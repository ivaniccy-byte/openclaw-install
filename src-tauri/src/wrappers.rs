use crate::{ModelConfig, OpenClawStatus};
use reqwest::Client;
use serde_json::json;
use std::process::Command;
use sysinfo::System;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrapperError {
    #[error("进程启动失败: {0}")]
    StartFailed(String),
    #[error("进程停止失败: {0}")]
    StopFailed(String),
    #[error("进程未运行")]
    NotRunning,
    #[error("HTTP请求失败: {0}")]
    HttpFailed(String),
    #[error("配置错误: {0}")]
    ConfigError(String),
}

/// OpenClaw进程管理（基于Node.js运行时）
pub struct OpenClawProcess {
    install_path: String,
    port: u16,
    node_child: Option<std::process::Child>,
    start_time: std::time::Instant,
}

impl OpenClawProcess {
    pub fn new(install_path: &str, port: u16) -> Self {
        Self {
            install_path: install_path.to_string(),
            port,
            node_child: None,
            start_time: std::time::Instant::now(),
        }
    }

    /// 启动OpenClaw进程
    pub fn start(&mut self) -> Result<(), WrapperError> {
        let openclaw_dir = std::path::Path::new(&self.install_path);

        // 1. 查找Node.js可执行文件
        let node_exe = self.find_node_exe()?;

        // 2. 启动OpenClaw核心（Node.js）
        let mut node_cmd = Command::new(node_exe);
        node_cmd.arg("start.js")
           .current_dir(openclaw_dir)
           .env("PORT", self.port.to_string());

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            node_cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        let node_child = node_cmd.spawn().map_err(|e| WrapperError::StartFailed(e.to_string()))?;
        self.node_child = Some(node_child);

        self.start_time = std::time::Instant::now();

        // 等待服务就绪
        std::thread::sleep(std::time::Duration::from_secs(3));

        Ok(())
    }

    /// 停止OpenClaw进程
    pub fn stop(&mut self) -> Result<(), WrapperError> {
        if let Some(ref mut child) = self.node_child {
            child.kill().map_err(|e| WrapperError::StopFailed(e.to_string()))?;
            child.wait().map_err(|e| WrapperError::StopFailed(e.to_string()))?;
        }
        self.node_child = None;
        Ok(())
    }

    /// 获取进程状态
    pub fn get_status(&self) -> Result<OpenClawStatus, WrapperError> {
        let running = self.node_child.is_some();

        let (uptime, memory, cpu) = if running {
            let uptime = self.start_time.elapsed().as_secs();
            let mut sys = System::new();
            sys.refresh_memory();

            let mut memory_mb = 0u64;
            let mut cpu_percent = 0.0f32;

            for process in sys.processes().values() {
                if let Some(cmd) = process.cmd().first() {
                    let cmd_str = cmd.to_string_lossy().to_string();
                    if cmd_str.contains("node") && cmd_str.contains("openclaw") {
                        memory_mb = process.memory() / (1024 * 1024);
                        cpu_percent = process.cpu_usage();
                        break;
                    }
                }
            }

            (uptime, memory_mb, cpu_percent)
        } else {
            (0, 0, 0.0)
        };

        Ok(OpenClawStatus {
            running,
            port: self.port,
            uptime_seconds: uptime,
            memory_mb: memory,
            cpu_percent: cpu,
        })
    }

    /// 查找Node.js可执行文件
    fn find_node_exe(&self) -> Result<String, WrapperError> {
        // 1. 先检查内置运行时
        let built_in_node = std::path::Path::new(&self.install_path)
            .join("resources")
            .join("node-runtime")
            .join("win-x64")
            .join("node.exe");

        if built_in_node.exists() {
            return Ok(built_in_node.to_string_lossy().to_string());
        }

        // 2. 检查PATH中的node
        if let Ok(output) = Command::new("node").arg("--version").output() {
            if output.status.success() {
                if let Ok(which) = Command::new("where").arg("node").output() {
                    let path = String::from_utf8_lossy(&which.stdout);
                    if let Some(first_line) = path.lines().next() {
                        let node_path = first_line.trim();
                        if std::path::Path::new(node_path).exists() {
                            return Ok(node_path.to_string());
                        }
                    }
                }
            }
        }

        // 3. 检查常见安装位置
        let possible_paths = [
            "C:\\Program Files\\nodejs\\node.exe",
            "C:\\Program Files (x86)\\nodejs\\node.exe",
        ];

        for path in &possible_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }

        Err(WrapperError::StartFailed("未找到Node.js运行时".to_string()))
    }
}

/// 测试模型API连通性
pub async fn test_model_api(model: &ModelConfig) -> Result<bool, WrapperError> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| WrapperError::HttpFailed(e.to_string()))?;

    let request = json!({
        "model": model.model_name,
        "messages": [{"role": "user", "content": "hello"}],
        "max_tokens": 10
    });

    let response = client
        .post(&model.endpoint)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| WrapperError::HttpFailed(e.to_string()))?;

    Ok(response.status().is_success())
}

/// 加载OpenClaw配置
pub fn load_config(install_path: &str) -> Result<crate::AppConfig, WrapperError> {
    let config_path = std::path::Path::new(install_path)
        .join("resources")
        .join("openclaw")
        .join("config.yaml");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

        serde_yaml_ng::from_str(&content)
            .map_err(|e| WrapperError::ConfigError(e.to_string()))
    } else {
        Ok(crate::AppConfig {
            port: 18789,
            auto_start: false,
            auto_restart: true,
            low_power_mode: false,
            memory_system: "none".to_string(),
            main_model: None,
            embedding_model: None,
            rerank_model: None,
        })
    }
}

/// 保存OpenClaw配置
pub fn save_config(install_path: &str, config: &crate::AppConfig) -> Result<(), WrapperError> {
    let config_dir = std::path::Path::new(install_path)
        .join("resources")
        .join("openclaw");

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    let config_path = config_dir.join("config.yaml");
    let content = serde_yaml_ng::to_string(config)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    std::fs::write(&config_path, content)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    Ok(())
}
