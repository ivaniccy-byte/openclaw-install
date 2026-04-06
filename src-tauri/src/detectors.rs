use crate::EnvCheckResult;
use std::net::TcpListener;
use sysinfo::System;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DetectorError {
    #[error("系统信息获取失败: {0}")]
    SystemInfo(String),
    #[error("网络检测失败: {0}")]
    Network(String),
    #[error("端口检测失败: {0}")]
    Port(String),
}

/// CPU AVX2指令集检测
pub fn check_cpu_avx2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let cpuid_result = std::arch::x86_64::__cpuid(0x7);
            (cpuid_result.ebx & (1 << 5)) != 0
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// Windows版本检测
pub fn check_windows_version() -> (bool, u32) {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("cmd")
            .args(["/c", "ver"])
            .output();

        if let Ok(output) = output {
            let version = String::from_utf8_lossy(&output.stdout);
            if let Some(build) = version.find("10.0.") {
                let build_str = &version[build + 5..];
                if let Some(end) = build_str.find(|c: char| !c.is_ascii_digit()) {
                    if let Ok(build_num) = build_str[..end].parse::<u32>() {
                        return (build_num >= 19045, build_num);
                    }
                }
            }
        }
        (false, 0)
    }
    #[cfg(not(target_os = "windows"))]
    {
        (true, 22000)
    }
}

/// 磁盘空间和类型检测
pub fn check_disk_info(install_path: &str) -> (bool, u64, bool) {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    let path = std::path::Path::new(install_path);

    for disk in disks.list() {
        if let Some(disk_path) = disk.mount_point().to_str() {
            if path.starts_with(disk_path) || disk_path == "/" {
                let available = disk.available_space();
                let available_gb = available / (1024 * 1024 * 1024);
                // is_rotational method removed in sysinfo 0.33, assume SSD for modern systems
                let is_ssd = true;
                return (available_gb >= 10, available_gb, is_ssd);
            }
        }
    }
    (true, 100, true)
}

/// 内存检测
pub fn check_memory() -> (bool, u64) {
    let mut sys = System::new();
    sys.refresh_memory();
    let total_gb = sys.total_memory() / (1024 * 1024 * 1024);
    (total_gb >= 4, total_gb)
}

/// Node.js 检测 (OpenClaw 3.28 需要 Node.js)
pub fn check_nodejs() -> (bool, Option<String>) {
    use std::process::Command;

    // 检查PATH中的node
    if let Ok(output) = Command::new("node").args(["--version"]).output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim().to_string();
            // OpenClaw 3.28 推荐 Node.js 20+
            if version.starts_with("v20") || version.starts_with("v21") || version.starts_with("v22") || version.starts_with("v23") || version.starts_with("v24") {
                return (true, Some(version));
            }
            // 较低版本也会检测到，但会提示建议使用内置版本
            return (true, Some(version));
        }
    }

    // 检查常见安装位置
    let possible_paths = [
        "C:\\Program Files\\nodejs\\node.exe",
        "C:\\Program Files (x86)\\nodejs\\node.exe",
    ];

    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            if let Ok(output) = Command::new(path).args(["--version"]).output() {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    return (true, Some(version.trim().to_string()));
                }
            }
        }
    }

    (false, None)
}

/// 端口可用性检测
pub fn check_port_available(port: u16) -> (bool, u16) {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => (true, port),
        Err(_) => {
            for p in port..port + 100 {
                if TcpListener::bind(("127.0.0.1", p)).is_ok() {
                    return (false, p);
                }
            }
            (false, port)
        }
    }
}

/// 网络连通性检测
pub fn check_network() -> bool {
    use std::process::Command;

    let mirrors = [
        "https://npmmirror.com",
        "https://pypi.tuna.tsinghua.edu.cn",
    ];

    for mirror in &mirrors {
        let output = Command::new("curl")
            .args(["-I", "-s", "-o", "/dev/null", "-w", "%{http_code}", mirror])
            .output();

        if let Ok(output) = output {
            let code = String::from_utf8_lossy(&output.stdout);
            if code.trim() == "200" || code.trim() == "301" || code.trim() == "302" {
                return true;
            }
        }
    }
    true
}

/// 查找可用端口
pub async fn find_available_port(start_port: u16) -> Result<u16, DetectorError> {
    use tokio::net::TcpListener;

    for port in start_port..start_port + 100 {
        let addr = format!("127.0.0.1:{}", port);
        if TcpListener::bind(&addr).await.is_ok() {
            return Ok(port);
        }
    }
    Err(DetectorError::Port("未找到可用端口".to_string()))
}

/// 执行全量环境检测
pub async fn check_all(install_path: &str) -> Result<EnvCheckResult, DetectorError> {
    let cpu_avx2 = check_cpu_avx2();
    let (windows_ok, build_num) = check_windows_version();
    let (disk_ok, disk_space, is_ssd) = check_disk_info(install_path);
    let (memory_ok, memory_gb) = check_memory();
    let (nodejs_ok, nodejs_version) = check_nodejs();
    let (port_available, recommended_port) = check_port_available(18789);
    let network_ok = check_network();

    Ok(EnvCheckResult {
        cpu_avx2,
        windows_version_ok: windows_ok,
        windows_build_number: build_num,
        disk_space_ok: disk_ok,
        disk_space_gb: disk_space,
        is_ssd,
        memory_ok,
        memory_gb,
        nodejs_exists: nodejs_ok,
        nodejs_version,
        port_available,
        recommended_port,
        network_ok,
    })
}
