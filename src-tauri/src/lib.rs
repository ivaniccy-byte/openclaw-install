use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// ============================================================================
// 模块定义
// ============================================================================

mod detectors;
mod wrappers;
mod telemetry;
mod fixes;

// ============================================================================
// 数据结构定义
// ============================================================================

/// 环境检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvCheckResult {
    pub cpu_avx2: bool,
    pub windows_version_ok: bool,
    pub windows_build_number: u32,
    pub disk_space_ok: bool,
    pub disk_space_gb: u64,
    pub is_ssd: bool,
    pub memory_ok: bool,
    pub memory_gb: u64,
    pub nodejs_exists: bool,
    pub nodejs_version: Option<String>,
    pub port_available: bool,
    pub recommended_port: u16,
    pub network_ok: bool,
}

/// OpenClaw运行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawStatus {
    pub running: bool,
    pub port: u16,
    pub uptime_seconds: u64,
    pub memory_mb: u64,
    pub cpu_percent: f32,
}

/// 健康度评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub score: u32,
    pub level: String,
    pub details: HealthDetails,
}

/// 健康详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthDetails {
    pub process_alive: bool,
    pub port_listening: bool,
    pub main_model_ok: bool,
    pub embedding_model_ok: bool,
    pub rerank_model_ok: bool,
    pub memory_system_ok: bool,
    pub plugins_ok: Vec<String>,
    pub skills_ok: Vec<String>,
    pub config_valid: bool,
    pub cpu_usage: f32,
    pub memory_usage: f32,
}

/// 大模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub api_key: String,
    pub endpoint: String,
    pub model_name: String,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub auto_start: bool,
    pub auto_restart: bool,
    pub low_power_mode: bool,
    pub memory_system: String, // "none" 或 "loseless"
    pub main_model: Option<ModelConfig>,
    pub embedding_model: Option<ModelConfig>, // Loseless Claw 需要 embedding 模型
    pub rerank_model: Option<ModelConfig>, // Loseless Claw 需要 rerank 模型
}

/// 异常告警
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub fix_type: Option<String>,
}

/// 安装选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallOptions {
    pub install_path: String,
    pub selected_plugins: Vec<String>,
    pub selected_memory: String,
    pub selected_skills: Vec<String>,
}

// ============================================================================
// 应用状态
// ============================================================================

pub struct AppState {
    pub openclaw_process: Mutex<Option<wrappers::OpenClawProcess>>,
    pub config: Mutex<AppConfig>,
    pub install_path: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            openclaw_process: Mutex::new(None),
            config: Mutex::new(AppConfig {
                port: 18789,
                auto_start: false,
                auto_restart: true,
                low_power_mode: false,
                memory_system: "none".to_string(),
                main_model: None,
                embedding_model: None,
                rerank_model: None,
            }),
            install_path: Mutex::new(None),
        }
    }
}

// ============================================================================
// Tauri 命令
// ============================================================================

/// 环境预检测
#[tauri::command]
async fn check_environment(install_path: String) -> Result<EnvCheckResult, String> {
    log::info!("开始环境预检测，安装路径: {}", install_path);
    detectors::check_all(&install_path).await.map_err(|e| e.to_string())
}

/// 执行安装
#[tauri::command]
async fn perform_installation(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    options: InstallOptions,
) -> Result<(), String> {
    log::info!("开始执行安装流程，路径: {}", options.install_path);
    
    // 执行物理安装逻辑
    wrappers::perform_install(&app_handle, &options).map_err(|e| e.to_string())?;
    
    // 更新全局状态中的安装路径
    *state.install_path.lock().unwrap() = Some(options.install_path);
    
    Ok(())
}

/// 启动OpenClaw
#[tauri::command]
async fn start_openclaw(
    state: State<'_, AppState>,
    install_path: String,
    port: u16,
) -> Result<(), String> {
    log::info!("启动OpenClaw，端口: {}", port);
    let mut process = wrappers::OpenClawProcess::new(&install_path, port);
    process.start().map_err(|e| e.to_string())?;
    *state.openclaw_process.lock().unwrap() = Some(process);
    Ok(())
}

/// 停止OpenClaw
#[tauri::command]
async fn stop_openclaw(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("停止OpenClaw");
    if let Some(ref mut process) = *state.openclaw_process.lock().unwrap() {
        process.stop().map_err(|e| e.to_string())?;
    }
    *state.openclaw_process.lock().unwrap() = None;
    Ok(())
}

/// 获取OpenClaw状态
#[tauri::command]
async fn get_openclaw_status(state: State<'_, AppState>) -> Result<OpenClawStatus, String> {
    if let Some(ref process) = *state.openclaw_process.lock().unwrap() {
        process.get_status().map_err(|e| e.to_string())
    } else {
        Ok(OpenClawStatus {
            running: false,
            port: 18789,
            uptime_seconds: 0,
            memory_mb: 0,
            cpu_percent: 0.0,
        })
    }
}

/// 获取健康度评分
#[tauri::command]
async fn get_health_score(state: State<'_, AppState>) -> Result<HealthScore, String> {
    let config = state.config.lock().unwrap().clone();
    let status = get_openclaw_status(state).await.unwrap_or_else(|_| OpenClawStatus {
        running: false,
        port: 18789,
        uptime_seconds: 0,
        memory_mb: 0,
        cpu_percent: 0.0,
    });
    telemetry::calculate_health_score(&config, &status).map_err(|e| e.to_string())
}

/// 读取配置
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    Ok(state.config.lock().unwrap().clone())
}

/// 保存配置
#[tauri::command]
async fn save_config(state: State<'_, AppState>, config: AppConfig) -> Result<(), String> {
    *state.config.lock().unwrap() = config;
    Ok(())
}

/// 一键修复
#[tauri::command]
async fn auto_fix(fix_type: String, state: State<'_, AppState>) -> Result<(), String> {
    log::info!("执行一键修复: {}", fix_type);
    let config = state.config.lock().unwrap().clone();
    fixes::execute_fix(&fix_type, &config).map_err(|e| e.to_string())
}

/// 测试模型连通性
#[tauri::command]
async fn test_model_connection(model: ModelConfig) -> Result<bool, String> {
    log::info!("测试模型连通性: {}", model.provider);
    wrappers::test_model_api(&model).await.map_err(|e| e.to_string())
}

/// 获取可用端口
#[tauri::command]
async fn get_available_port(start_port: u16) -> Result<u16, String> {
    detectors::find_available_port(start_port).await.map_err(|e| e.to_string())
}

// ============================================================================
// 应用入口
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!("OpenClaw职场版启动中...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            check_environment,
            perform_installation,
            start_openclaw,
            stop_openclaw,
            get_openclaw_status,
            get_health_score,
            get_config,
            save_config,
            auto_fix,
            test_model_connection,
            get_available_port,
        ])
        .run(tauri::generate_context!())
        .expect("启动Tauri应用时发生错误");
}
