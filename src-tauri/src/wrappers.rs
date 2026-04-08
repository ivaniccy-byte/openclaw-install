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
    #[error("安装失败: {0}")]
    InstallError(String),
    #[error("资源获取失败: {0}")]
    ResourceError(String),
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
        let node_sub_path = if cfg!(target_os = "windows") {
            "node-runtime/win-x64/node.exe"
        } else if cfg!(target_os = "macos") {
            "node-runtime/mac-arm64/bin/node" // 假设结构
        } else {
            "node-runtime/linux-x64/bin/node"
        };

        let built_in_node = std::path::Path::new(&self.install_path)
            .join("resources")
            .join(node_sub_path);

        if built_in_node.exists() {
            return Ok(built_in_node.to_string_lossy().to_string());
        }

        // 2. 检查系统PATH
        let node_cmd = if cfg!(target_os = "windows") { "node.exe" } else { "node" };
        if let Ok(output) = Command::new(node_cmd).arg("--version").output() {
            if output.status.success() {
                // 在非Windows上使用 which，Windows上使用 where
                let which_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
                if let Ok(which_output) = Command::new(which_cmd).arg(node_cmd).output() {
                    let path = String::from_utf8_lossy(&which_output.stdout);
                    if let Some(first_line) = path.lines().next() {
                        return Ok(first_line.trim().to_string());
                    }
                }
            }
        }

        Err(WrapperError::StartFailed("未找到Node.js运行时，请确保已安装或内置资源完整".to_string()))
    }
}

/// 执行物理安装逻辑
pub fn perform_install(
    app_handle: &tauri::AppHandle,
    options: &crate::InstallOptions,
) -> Result<(), WrapperError> {
    use std::fs;
    use tauri::Manager;

    let target_dir = std::path::Path::new(&options.install_path);
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).map_err(|e| WrapperError::InstallError(e.to_string()))?;
    }

    let resource_dir = app_handle.path().resource_dir()
        .map_err(|e| WrapperError::ResourceError(e.to_string()))?;
    
    let src_resources = resource_dir.join("resources");

    // 1. 复制必备组件
    copy_dir_recursive(&src_resources.join("openclaw"), &target_dir.join("resources").join("openclaw"))?;
    
    // 2. 根据平台复制Node运行时
    let node_dir_name = if cfg!(target_os = "windows") { "win-x64" } else { "mac-arm64" };
    copy_dir_recursive(
        &src_resources.join("node-runtime").join(node_dir_name),
        &target_dir.join("resources").join("node-runtime").join(node_dir_name)
    )?;

    // 3. 复制可选组件
    if options.selected_memory == "loseless" {
        copy_dir_recursive(&src_resources.join("lossless-claw"), &target_dir.join("resources").join("lossless-claw"))?;
        copy_dir_recursive(&src_resources.join("lancedb-pro"), &target_dir.join("resources").join("lancedb-pro"))?;
    }

    if !options.selected_skills.is_empty() {
        // 如果选择了技能，复制 skills 目录
        copy_dir_recursive(&src_resources.join("skills"), &target_dir.join("resources").join("skills"))?;
    }

    // 4. 生成初始配置
    let initial_config = crate::AppConfig {
        port: 18789,
        auto_start: false,
        auto_restart: true,
        low_power_mode: false,
        memory_system: options.selected_memory.clone(),
        main_model: None,
        embedding_model: None,
        rerank_model: None,
    };
    save_config(&options.install_path, &initial_config)?;

    Ok(())
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), WrapperError> {
    if !src.exists() {
        log::warn!("源路径不存在: {:?}", src);
        return Ok(());
    }

    if !dst.exists() {
        std::fs::create_dir_all(dst).map_err(|e| WrapperError::InstallError(e.to_string()))?;
    }

    for entry in std::fs::read_dir(src).map_err(|e| WrapperError::InstallError(e.to_string()))? {
        let entry = entry.map_err(|e| WrapperError::InstallError(e.to_string()))?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &target_path)?;
        } else {
            std::fs::copy(&entry_path, &target_path).map_err(|e| WrapperError::InstallError(e.to_string()))?;
        }
    }
    Ok(())
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
