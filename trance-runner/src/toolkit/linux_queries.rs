//! Linux-specific platform queries (disk drives, GPU names, monitor enumeration).
//! Bypasses lspci and df subprocess commands using native FFI (statvfs) and sysfs.

use crate::toolkit::platform::DiskDriveInfo;

#[cfg(target_os = "linux")]
#[repr(C)]
struct StatVfs {
    f_bsize: u64,
    f_frsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_favail: u64,
    f_fsid: u64,
    f_flag: u64,
    f_namemax: u64,
    __f_spare: [i32; 6],
}

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn statvfs(path: *const u8, buf: *mut StatVfs) -> i32;
}

#[cfg(target_os = "linux")]
pub fn query_disk_drives() -> Vec<DiskDriveInfo> {
    let mut drives = Vec::new();
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let dev = parts[0];
                let path = parts[1];
                if dev.starts_with("/dev/") || path == "/" {
                    let path_c = match std::ffi::CString::new(path) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                    unsafe {
                        let mut stat: StatVfs = std::mem::zeroed();
                        if statvfs(path_c.as_ptr() as *const u8, &mut stat) == 0 {
                            let block_size = if stat.f_frsize > 0 {
                                stat.f_frsize
                            } else {
                                stat.f_bsize
                            };
                            drives.push(DiskDriveInfo {
                                path: path.to_string(),
                                total_bytes: stat.f_blocks * block_size,
                                free_bytes: stat.f_bavail * block_size,
                            });
                        }
                    }
                }
            }
        }
    }
    if drives.is_empty() {
        drives.push(DiskDriveInfo {
            path: "/".to_string(),
            free_bytes: 50 * 1024 * 1024 * 1024,
            total_bytes: 100 * 1024 * 1024 * 1024,
        });
    }
    drives
}

#[cfg(not(target_os = "linux"))]
pub fn query_disk_drives() -> Vec<DiskDriveInfo> {
    vec![DiskDriveInfo {
        path: "/".to_string(),
        free_bytes: 50 * 1024 * 1024 * 1024,
        total_bytes: 100 * 1024 * 1024 * 1024,
    }]
}

#[cfg(target_os = "linux")]
pub fn query_gpu_names() -> Vec<String> {
    let mut gpus = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path().join("device").join("uevent");
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    for line in content.lines() {
                        if line.starts_with("DRIVER=") {
                            let driver = line.split('=').nth(1).unwrap_or("").to_string();
                            if !driver.is_empty() && !gpus.contains(&driver) {
                                gpus.push(driver);
                            }
                        }
                    }
                }
            }
        }
    }
    gpus
}

#[cfg(not(target_os = "linux"))]
pub fn query_gpu_names() -> Vec<String> {
    Vec::new()
}

#[cfg(target_os = "linux")]
pub fn query_all_monitors() -> Vec<String> {
    let mut monitors = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let modes_path = entry.path().join("modes");
            if modes_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&modes_path) {
                    if let Some(line) = content.lines().next() {
                        monitors.push(format!("Display: {}", line));
                    }
                }
            }
        }
    }
    if !monitors.is_empty() {
        return monitors;
    }
    vec!["Primary: 1920x1080".to_string()]
}

#[cfg(not(target_os = "linux"))]
pub fn query_all_monitors() -> Vec<String> {
    vec!["Primary: 1920x1080".to_string()]
}
