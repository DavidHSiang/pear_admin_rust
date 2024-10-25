use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct SystemInfo {
    hostname: String,
    system_version: String,
    cpu_count: usize,
    cpus_percent: String,
    cpus_percent_f32: f32,
    memory_usage: String,
    memory_usage_f32: f32,
    memory_used: String,
    memory_total: String,
    memory_free: String,
    boot_time: String,
    up_time_format: String,
    time_now: String,
}

#[derive(Serialize)]
pub struct PollingData {
    time_now: String,
    cpus_percent_f32: f32,
    memory_usage_f32: f32,
}

pub struct MonitorService {}

impl MonitorService {
    pub async fn system_info() -> SystemInfo {
        let mut system = System::new_all();
        system.refresh_all();

        // 主机名称
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        // 系统版本
        let system_version = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());

        // 逻辑CPU数量
        let cpu_count = system.cpus().len();

        // CPU使用率
        let cpus_percent_f32: f32 = system.global_cpu_usage();
        let cpus_percent = format!("{:.2}%", cpus_percent_f32);

        // 内存
        let total_memory = system.total_memory(); // KB
        let used_memory = system.used_memory(); // KB
        let free_memory = system.free_memory(); // KB
        let memory_usage_f32 = if total_memory > 0 {
            (used_memory as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };
        let memory_usage = format!("{:.2}%", memory_usage_f32);

        // memory_used 使用GB显示
        let used_memory = used_memory as f32 / 1024.0 / 1024.0 / 1024.0;
        let used_memory = format!("{:.2}GB", used_memory);

        let total_memory = total_memory as f32 / 1024.0 / 1024.0 / 1024.0;
        let total_memory = format!("{:.2}GB", total_memory);

        let free_memory = free_memory as f32 / 1024.0 / 1024.0 / 1024.0;
        let free_memory = format!("{:.2}GB", free_memory);

        // 开机时间
        let boot_time = System::boot_time();
        let boot_datetime: DateTime<Local> =
            DateTime::from(UNIX_EPOCH + std::time::Duration::from_secs(boot_time));
        let boot_time_str = boot_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        // 运行时间
        let now = SystemTime::now();
        let up_time_duration = now
            .duration_since(UNIX_EPOCH + std::time::Duration::from_secs(boot_time))
            .unwrap_or_else(|_| std::time::Duration::new(0, 0));
        let up_time_seconds = up_time_duration.as_secs();
        let hours = up_time_seconds / 3600;
        let minutes = (up_time_seconds % 3600) / 60;
        let seconds = up_time_seconds % 60;
        let up_time_format = format!("{} 小时 {} 分钟 {} 秒", hours, minutes, seconds);

        // 当前时间
        let time_now = Local::now().format("%H:%M:%S").to_string();

        SystemInfo {
            hostname,
            system_version,
            cpu_count,
            cpus_percent,
            cpus_percent_f32,
            memory_usage,
            memory_usage_f32,
            memory_used: used_memory,   // 转换为字节
            memory_total: total_memory, // 转换为字节
            memory_free: free_memory,   // 转换为字节
            boot_time: boot_time_str,
            up_time_format,
            time_now,
        }
    }

    pub async fn monitor_polling() -> PollingData {
        let mut system = System::new_all();
        system.refresh_all();

        // CPU使用率
        let cpus_percent_f32: f32 = system.global_cpu_usage();

        // 内存
        let total_memory = system.total_memory(); // KB
        let used_memory = system.used_memory(); // KB
        let memory_usage_f32 = if total_memory > 0 {
            (used_memory as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };
        PollingData {
            time_now: Local::now().format("%H:%M:%S").to_string(),
            cpus_percent_f32,
            memory_usage_f32,
        }
    }
}
