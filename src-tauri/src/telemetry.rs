use crate::{Alert, AppConfig, HealthDetails, HealthScore, OpenClawStatus};
use sysinfo::System;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("系统信息获取失败: {0}")]
    SystemInfo(String),
}

/// 计算健康度评分
pub fn calculate_health_score(config: &AppConfig, status: &OpenClawStatus) -> Result<HealthScore, TelemetryError> {
    let mut score: i32 = 100;
    let mut alerts: Vec<Alert> = Vec::new();
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu();

    // 核心进程状态 (权重30%)
    let process_alive = status.running;
    if !process_alive {
        return Ok(HealthScore {
            score: 0,
            level: "异常".to_string(),
            details: HealthDetails {
                process_alive: false,
                port_listening: false,
                main_model_ok: false,
                embedding_model_ok: false,
                rerank_model_ok: false,
                memory_system_ok: false,
                plugins_ok: vec![],
                skills_ok: vec![],
                config_valid: true,
                cpu_usage: 0.0,
                memory_usage: 0.0,
            },
        });
    }

    let port_listening = status.running;

    // 内存使用率
    let total_mem = sys.total_memory() as f32;
    let used_mem = sys.used_memory() as f32;
    let memory_usage = if total_mem > 0.0 {
        (used_mem / total_mem) * 100.0
    } else {
        0.0
    };

    // CPU使用率
    let cpu_usage = status.cpu_percent;

    // 主模型状态 (权重20%)
    let main_model_ok = config.main_model.is_some();

    // Embedding模型状态 (Loseless Claw方案)
    let embedding_model_ok = match config.memory_system.as_str() {
        "loseless" => config.embedding_model.is_some(),
        _ => true,
    };

    // Rerank模型状态 (Loseless Claw方案)
    let rerank_model_ok = match config.memory_system.as_str() {
        "loseless" => config.rerank_model.is_some(),
        _ => true,
    };

    // 记忆系统状态
    let memory_system_ok = match config.memory_system.as_str() {
        "none" => true,
        "loseless" => config.embedding_model.is_some() && config.rerank_model.is_some(),
        _ => false,
    };

    // 配置文件有效性
    let config_valid = true; // 简化处理

    // 插件和Skill状态（简化处理）
    let plugins_ok: Vec<String> = vec![];
    let skills_ok: Vec<String> = vec![];

    // 构建详情
    let details = HealthDetails {
        process_alive,
        port_listening,
        main_model_ok,
        embedding_model_ok,
        rerank_model_ok,
        memory_system_ok,
        plugins_ok,
        skills_ok,
        config_valid,
        cpu_usage,
        memory_usage,
    };

    // 计算扣分
    if !port_listening {
        score -= 30;
        alerts.push(Alert {
            id: "port".to_string(),
            title: "端口未监听".to_string(),
            description: "OpenClaw服务未正常监听端口".to_string(),
            severity: "ERROR".to_string(),
            fix_type: Some("restart".to_string()),
        });
    }

    if !main_model_ok {
        score -= 20;
        alerts.push(Alert {
            id: "main_model".to_string(),
            title: "主模型未配置".to_string(),
            description: "请在配置中心配置大模型API".to_string(),
            severity: "ERROR".to_string(),
            fix_type: None,
        });
    }

    if !embedding_model_ok && config.memory_system == "loseless" {
        score -= 15;
        alerts.push(Alert {
            id: "embedding_model".to_string(),
            title: "Embedding模型未配置".to_string(),
            description: "Loseless记忆系统需要配置Embedding模型".to_string(),
            severity: "WARNING".to_string(),
            fix_type: None,
        });
    }

    if !rerank_model_ok && config.memory_system == "loseless" {
        score -= 10;
        alerts.push(Alert {
            id: "rerank_model".to_string(),
            title: "Rerank模型未配置".to_string(),
            description: "Loseless记忆系统需要配置Rerank模型".to_string(),
            severity: "WARNING".to_string(),
            fix_type: None,
        });
    }

    if memory_usage > 85.0 {
        score -= 5;
        alerts.push(Alert {
            id: "memory_high".to_string(),
            title: "内存占用过高".to_string(),
            description: format!("当前内存占用{:.1}%，建议开启低资源模式", memory_usage),
            severity: "WARNING".to_string(),
            fix_type: Some("low_memory".to_string()),
        });
    }

    if cpu_usage > 90.0 {
        score -= 5;
        alerts.push(Alert {
            id: "cpu_high".to_string(),
            title: "CPU占用过高".to_string(),
            description: format!("当前CPU占用{:.1}%，可能影响其他应用", cpu_usage),
            severity: "WARNING".to_string(),
            fix_type: Some("low_cpu".to_string()),
        });
    }

    // 确保分数在0-100范围内
    score = score.max(0).min(100);

    // 确定等级
    let level = match score {
        90..=100 => "优秀".to_string(),
        70..=89 => "正常".to_string(),
        60..=69 => "警告".to_string(),
        _ => "异常".to_string(),
    };

    Ok(HealthScore {
        score: score as u32,
        level,
        details,
    })
}
