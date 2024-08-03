use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use std::sync::Arc;
use sysinfo::{SystemExt, ProcessExt, CpuExt, DiskExt, NetworksExt, NetworkExt};
use reqwest;
use serde_json::json;
use chrono::Local;
use tokio;

// Discord Notification
const DISCORD_WEBHOOK_URL: &str = "URL"; // Replace the 'URL' with your Discord Webhook URL

// Alerts and normal pings
const ALERT_ROLE_ID: &str = "ID"; // Replace the 'ID' with your Role ID for critical alerts
const NORMAL_ROLE_ID: &str = "ID"; // Replace the 'ID' with your Role ID for alert state changes

// Thresholds for resource usage
const CPU_THRESHOLD: f32 = 90.0; // Replace 90.0 with your preferred percentage number for CPU usage threshold
const RAM_THRESHOLD: f32 = 90.0; // Replace 90.0 with your preferred percentage number for RAM usage threshold
const PROCESS_CPU_THRESHOLD: f32 = 80.0; // Replace 80.0 with your preferred percentage number for individual process CPU usage threshold
const PROCESS_RAM_THRESHOLD: f32 = 80.0; // Replace 80.0 with your preferred percentage number for individual process RAM usage threshold

// DDoS detection parameters
// If you don't want alerts for DDoS Attack then set change the value 100.0 to 1000000000.0
const DDOS_THRESHOLD_MB_PER_SEC: f64 = 100.0; // Replace 100.0 with your preferred MB/s for DDoS detection threshold
const DDOS_CHECK_PERIODS: usize = 5; // Replace 5 with the number of times the script as to check before triggering DDoS Attack Alert


struct AlertState {
    cpu_alert: bool,
    ram_alert: bool,
    ddos_alert: bool,
    process_alert: bool,
}

#[tokio::main]
async fn main() {
    let system = Arc::new(Mutex::new(sysinfo::System::new_all()));
    let alert_state = Arc::new(Mutex::new(AlertState {
        cpu_alert: false,
        ram_alert: false,
        ddos_alert: false,
        process_alert: false,
    }));

    let system_clone = Arc::clone(&system);
    let alert_state_clone = Arc::clone(&alert_state);

    tokio::task::spawn(async move {
        let mut last_network_rx = 0;
        let mut last_check = Instant::now();
        let mut network_rates = Vec::with_capacity(DDOS_CHECK_PERIODS);

        loop {
            {
                let sys = system_clone.lock().await;
                check_resources(&sys, &alert_state_clone, &mut last_network_rx, &mut last_check, &mut network_rates).await;
            }
            tokio::time::sleep(Duration::from_secs(3)).await; // Customize this duration to adjust the monitoring frequency - By Deafult it is set to 3 seconds
            system_clone.lock().await.refresh_all();
        }
    });

    loop {
        {
            let sys = system.lock().await;
            send_summary(&sys).await;
        }
        tokio::time::sleep(Duration::from_secs(1800)).await; // Customize this duration to adjust the summary reporting frequency - By Deafult it is set to 30mins 
    }
}

async fn check_resources(system: &sysinfo::System, alert_state: &Arc<Mutex<AlertState>>, last_network_rx: &mut u64, last_check: &mut Instant, network_rates: &mut Vec<f64>) {
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let ram_usage = system.used_memory() as f32 / system.total_memory() as f32 * 100.0;
    
    let mut state = alert_state.lock().await;

    // Check CPU usage
    if cpu_usage > CPU_THRESHOLD && !state.cpu_alert {
        send_alert("High CPU Usage", &format!("CPU usage exceeded Threshold limit, current usage is {:.2}%", cpu_usage)).await;
        state.cpu_alert = true;
    } else if cpu_usage <= CPU_THRESHOLD && state.cpu_alert {
        send_normal_ping("CPU usage returned to normal").await;
        state.cpu_alert = false;
    }

    // Check RAM usage
    if ram_usage > RAM_THRESHOLD && !state.ram_alert {
        send_alert("High RAM Usage", &format!("RAM usage exceeded Threshold limit, current usage is {:.2}%", ram_usage)).await;
        state.ram_alert = true;
    } else if ram_usage <= RAM_THRESHOLD && state.ram_alert {
        send_normal_ping("RAM usage returned to normal").await;
        state.ram_alert = false;
    }

    // Check for high resource-consuming processes
    for (pid, process) in system.processes() {
        let process_cpu = process.cpu_usage();
        let process_ram = process.memory() as f32 / system.total_memory() as f32 * 100.0;

        if (process_cpu > PROCESS_CPU_THRESHOLD || process_ram > PROCESS_RAM_THRESHOLD) && !state.process_alert {
            send_alert("High Resource-Consuming Process", &format!("Process {} (PID: {}) is using {:.2}% CPU and {:.2}% RAM", process.name(), pid, process_cpu, process_ram)).await;
            state.process_alert = true;
            break;
        }
    }

    // Check for potential DDoS
    let current_network_rx: u64 = system.networks().iter().map(|(_, network)| network.total_received()).sum();
    let elapsed = last_check.elapsed();
    let network_rate_mb_per_s = if elapsed.as_secs() > 0 {
        (current_network_rx - *last_network_rx) as f64 / elapsed.as_secs_f64() / 1_000_000.0
    } else {
        0.0
    };

    // Track network rates for averaging
    network_rates.push(network_rate_mb_per_s);
    if network_rates.len() > DDOS_CHECK_PERIODS {
        network_rates.remove(0);
    }
    let average_rate = network_rates.iter().sum::<f64>() / network_rates.len() as f64;

    if average_rate > DDOS_THRESHOLD_MB_PER_SEC && !state.ddos_alert {
        let traffic_message = format!("Unusual high network traffic detected. Average: {:.2} MB/s", average_rate);
        send_alert("Potential DDoS Attack", &traffic_message).await;
        state.ddos_alert = true;
    } else if average_rate <= DDOS_THRESHOLD_MB_PER_SEC && state.ddos_alert {
        send_normal_ping("Network traffic returned to normal levels").await;
        state.ddos_alert = false;
    }

    *last_network_rx = current_network_rx;
    *last_check = Instant::now();
}

async fn send_summary(system: &sysinfo::System) {
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let ram_usage = system.used_memory() as f32 / system.total_memory() as f32 * 100.0;
    let total_disk_space: u64 = system.disks().iter().map(|disk| disk.total_space()).sum();
    let used_disk_space: u64 = system.disks().iter().map(|disk| disk.total_space() - disk.available_space()).sum();
    let disk_usage = used_disk_space as f32 / total_disk_space as f32 * 100.0;
    
    let disk_usage_gb = used_disk_space as f32 / 1e9;
    let total_disk_space_gb = total_disk_space as f32 / 1e9;
    
    let network_tx: u64 = system.networks().iter().map(|(_, network)| network.total_transmitted()).sum();
    let network_rx: u64 = system.networks().iter().map(|(_, network)| network.total_received()).sum();

    let summary = json!({
        "embeds": [{
            "title": "Resource Usage Summary",
            "color": 14869218, // Customize the color here -- 14869218 is Decimel code for grey color (Default color is grey)
            "fields": [
                {
                    "name": "CPU Usage",
                    "value": format!("- Current Usage: {:.2}%\n- Load Average: {:.2} / {:.2} / {:.2}", 
                                     cpu_usage, system.load_average().one, system.load_average().five, system.load_average().fifteen),
                    "inline": false
                },
                {
                    "name": "Memory Usage",
                    "value": format!("- Current Usage: {:.2}% ({:.2}GB)\n- Total Memory: {:.2} GB\n- Swap Usage: {:.2} / {:.2} GB",
                                     ram_usage, system.used_memory() as f32 / 1e9, 
                                     system.total_memory() as f32 / 1e9,
                                     system.used_swap() as f32 / 1e9, system.total_swap() as f32 / 1e9),
                    "inline": false
                },
                {
                    "name": "Disk Usage",
                    "value": format!("- Current Usage: {:.2}% ({:.2} GB)\n- Total Disk Space: {:.2} GB",
                                     disk_usage, disk_usage_gb,
                                     total_disk_space_gb),
                    "inline": false
                },
                {
                    "name": "Network Usage",
                    "value": format!("- Upstream: {:.2} GB\n- Downstream: {:.2} GB",
                                     network_tx as f32 / 1e9, network_rx as f32 / 1e9),
                    "inline": false
                },
                {
                    "name": "Total Process Count",
                    "value": format!("{} processes", system.processes().len()),
                    "inline": false
                },
                {
                    "name": "Top Processes",
                    "value": get_top_processes(system),
                    "inline": false
                },
                {
                    "name": "System Uptime",
                    "value": format!("- {} hours, {} minutes", system.uptime() / 3600, (system.uptime() % 3600) / 60),
                    "inline": false
                }
            ],
            "timestamp": Local::now().to_rfc3339()
        }]
    });

    send_discord_message(&summary).await;
}

fn get_top_processes(system: &sysinfo::System) -> String {
    let mut processes: Vec<_> = system.processes().iter().collect();
    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
    
    processes.iter().take(3)
        .map(|(_, process)| {
            format!("• {}: {:.2}% CPU | {:.2}GB RAM", 
                    process.name(), 
                    process.cpu_usage(), 
                    process.memory() as f32 / 1e9)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

async fn send_alert(title: &str, message: &str) {
    let content = format!("<@&{}>", ALERT_ROLE_ID);

    let payload = json!({
        "content": content,
        "embeds": [{
            "title": title,
            "description": message,
            "color": 15158332 // Customize the color here -- 15158332 is Decimel code for red color (Default color is red)
        }]
    });

    send_discord_message(&payload).await;
}

async fn send_normal_ping(message: &str) {
    let content = format!("<@&{}>", NORMAL_ROLE_ID);
    let payload = json!({
        "content": content,
        "embeds": [{
            "description": message,
            "color": 3066993 // Customize the color here -- 3066993 is Decimel code for green color (Default color is green)
        }]
    });

    send_discord_message(&payload).await;
}

async fn send_discord_message(payload: &serde_json::Value) {
    let client = reqwest::Client::new();
    let res = client.post(DISCORD_WEBHOOK_URL)
        .json(payload)
        .send()
        .await;

    if let Err(e) = res {
        eprintln!("Failed to send Discord message: {}", e);
    }
}
