use crate::AppConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FixError {
    #[error("修复失败: {0}")]
    Failed(String),
}

/// 执行一键修复
pub fn execute_fix(fix_type: &str, _config: &AppConfig) -> Result<(), FixError> {
    match fix_type {
        "restart" => {
            // 进程重启修复
            log::info!("执行进程重启修复");
            Ok(())
        }
        "port" => {
            // 端口更换修复
            log::info!("执行端口更换修复");
            Ok(())
        }
        "config" => {
            // 配置还原修复
            log::info!("执行配置还原修复");
            Ok(())
        }
        "network" => {
            // 网络检测修复
            log::info!("执行网络检测修复");
            Ok(())
        }
        "low_memory" => {
            // 低内存模式修复
            log::info!("执行低内存模式修复");
            Ok(())
        }
        "low_cpu" => {
            // 低CPU模式修复
            log::info!("执行低CPU模式修复");
            Ok(())
        }
        "model_connection" => {
            // 模型连通性修复
            log::info!("执行模型连通性修复");
            Ok(())
        }
        _ => Err(FixError::Failed(format!("未知的修复类型: {}", fix_type))),
    }
}

/// 获取修复脚本列表
pub fn get_fix_scripts() -> Vec<FixScript> {
    vec![
        FixScript {
            id: "restart".to_string(),
            name: "进程崩溃重启".to_string(),
            description: "当OpenClaw进程意外退出时，自动重启服务".to_string(),
            severity: "high".to_string(),
        },
        FixScript {
            id: "port".to_string(),
            name: "端口占用修复".to_string(),
            description: "当默认端口被占用时，自动更换到可用端口".to_string(),
            severity: "medium".to_string(),
        },
        FixScript {
            id: "config".to_string(),
            name: "配置损坏还原".to_string(),
            description: "当配置文件损坏时，自动还原为默认配置".to_string(),
            severity: "high".to_string(),
        },
        FixScript {
            id: "dependency".to_string(),
            name: "依赖缺失修复".to_string(),
            description: "当检测到缺少运行时依赖时，自动重新安装".to_string(),
            severity: "high".to_string(),
        },
        FixScript {
            id: "network".to_string(),
            name: "网络异常检测".to_string(),
            description: "检测并修复网络连接问题".to_string(),
            severity: "low".to_string(),
        },
        FixScript {
            id: "model_connection".to_string(),
            name: "模型连通性排查".to_string(),
            description: "检测并修复大模型API连接问题".to_string(),
            severity: "high".to_string(),
        },
    ]
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FixScript {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: String,
}
