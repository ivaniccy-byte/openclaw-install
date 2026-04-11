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
        // 1. 先检查内置运行时 (基于安装目录)
        let node_sub_path = if cfg!(target_os = "windows") {
            "node-runtime/node.exe"
        } else {
            "node-runtime/bin/node"
        };

        let built_in_node = std::path::Path::new(&self.install_path)
            .join(node_sub_path);

        if built_in_node.exists() {
            return Ok(built_in_node.to_string_lossy().to_string());
        }

        // 2. 检查全局环境变量 OPENCLAW_HOME
        if let Ok(home) = std::env::var("OPENCLAW_HOME") {
            let env_node = std::path::Path::new(&home).join(node_sub_path);
            if env_node.exists() {
                return Ok(env_node.to_string_lossy().to_string());
            }
        }

        // 3. 特殊处理：检查用户家目录默认路径
        if let Ok(home) = std::env::var("USERPROFILE") {
            let user_node = std::path::Path::new(&home)
                .join(".openclaw")
                .join(node_sub_path);
            if user_node.exists() {
                return Ok(user_node.to_string_lossy().to_string());
            }
        }

        // 4. 检查系统PATH
        let node_cmd = if cfg!(target_os = "windows") { "node.exe" } else { "node" };
        if let Ok(output) = Command::new(node_cmd).arg("--version").output() {
            if output.status.success() {
                return Ok(node_cmd.to_string());
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
    
    // tauri.conf.json 配置 "../resources/**/*" 会将 resources/ 下的内容
    // 直接平铺到 resource_dir() 根目录下，所以不需要再 join("resources")
    let src_resources = &resource_dir;

    log::info!("资源目录: {:?}", src_resources);
    log::info!("目标安装目录: {:?}", target_dir);

    // 1. 复制必备组件
    let openclaw_src = src_resources.join("openclaw");
    log::info!("OpenClaw源路径: {:?}, 存在: {}", openclaw_src, openclaw_src.exists());
    copy_dir_recursive(&openclaw_src, &target_dir.join("openclaw"))?;
    
    // 2. 复制Node运行时
    copy_dir_recursive(
        &src_resources.join("node-runtime"),
        &target_dir.join("node-runtime")
    )?;

    // 3. 复制Python运行时
    copy_dir_recursive(
        &src_resources.join("python-runtime"),
        &target_dir.join("python-runtime")
    )?;

    // 3. 复制可选组件
    if options.selected_memory == "lossless-enhanced" {
        copy_dir_recursive(&src_resources.join("lossless-claw-enhanced"), &target_dir.join("lossless-claw-enhanced"))?;
        copy_dir_recursive(&src_resources.join("memories"), &target_dir.join("workspace").join("memories"))?;
    }

    if !options.selected_skills.is_empty() {
        // 如果选择了技能，复制到 workspace/skills 目录 (OpenClaw v3.28 规范)
        let dst_skills = target_dir.join("workspace").join("skills");
        copy_dir_recursive(&src_resources.join("skills"), &dst_skills)?;
    }

    // 4. 初始化配置文件 (JSON5 处理)
    provision_initial_config(&target_dir, &options)?;
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

    let mut url = model.endpoint.trim_end_matches('/').to_string();
    if !url.contains("/chat/completions") && !url.contains("/embeddings") && !url.contains("/rerank") {
        url.push_str("/chat/completions");
    }

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| WrapperError::HttpFailed(e.to_string()))?;

    let status = response.status();
    // 只要不是 401(未授权), 403(禁止) 或 404(未找到)，就认为网络连通且端点正常
    Ok(status.is_success() || status.as_u16() == 400 || status.as_u16() == 422)
}

/// 加载OpenClaw配置
pub fn load_config(install_path: &str) -> Result<crate::AppConfig, WrapperError> {
    let config_path = std::path::Path::new(install_path)
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

/// 初始配置注入 (对齐 openclaw.json 规范)
fn provision_initial_config(target_dir: &std::path::Path, options: &crate::InstallOptions) -> Result<(), WrapperError> {
    let config_path = target_dir.join("openclaw.json"); // v3.28 默认主配置
    
    // 构造满足 OpenClaw 规范的插件和工作区配置
    let mut plugins_entries = serde_json::Map::new();
    
    // 激活选定的插件
    for plugin_id in &options.selected_plugins {
        plugins_entries.insert(
            plugin_id.clone(),
            json!({
                "enabled": true,
                "config": {}
            })
        );
    }
    
    let config_data = json!({
        "agent": {
            "workspace": "./workspace",
            "home": target_dir.to_string_lossy().to_string()
        },
        "plugins": {
            "entries": plugins_entries
        },
        "models": {
            "chat": "deepseek-chat",
            "quick": "gpt-4o-mini",
            "cheap": "gpt-4o-mini"
        },
        "memory": {
            "system": options.selected_memory
        }
    });

    let content = serde_json::to_string_pretty(&config_data)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    std::fs::write(&config_path, content)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    Ok(())
}

/// 保存配置 (同步到核心 openclaw.json)
pub fn save_config(install_path: &str, config: &crate::AppConfig) -> Result<(), WrapperError> {
    let home_path = std::path::Path::new(install_path);
    let config_path = home_path.join("openclaw.json");

    // 如果文件已存在，尝试合并而非覆盖（简单处理：使用标准JSON写回）
    let mut config_data = if config_path.exists() {
        let existing = std::fs::read_to_string(&config_path).unwrap_or_default();
        serde_json::from_str::<serde_json::Value>(&existing).unwrap_or(json!({}))
    } else {
        json!({})
    };

    // 更新关键字段
    if let Some(ref m) = config.main_model {
        config_data["models"]["chat"] = json!(m.model_name);
        // 这里可以扩展更多提供商同步
    }
    
    config_data["port"] = json!(config.port);

    let content = serde_json::to_string_pretty(&config_data)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    std::fs::write(&config_path, content)
        .map_err(|e| WrapperError::ConfigError(e.to_string()))?;

    Ok(())
}
