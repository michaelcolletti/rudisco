use std::process::Command;
use std::io;
use std::fs::File;
use std::io::Write;
use std::env;

fn main() {
    let mut report = String::new();

    report.push_str("# System Information Report\n\n");

    let protocols = env::var("SUPPORTED_PROTOCOLS").unwrap_or_else(|_| "OS,CPU,Memory,Storage,Network,ICMP,SNMP,SMTP,NFS,Services,GPU".to_string());
    let protocols: Vec<&str> = protocols.split(',').collect();

    if protocols.contains(&"OS") {
        match get_os_info() {
            Ok(info) => report.push_str(&format!("## OS Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get OS information: {}", e),
        }
    }
    if protocols.contains(&"CPU") {
        match get_cpu_info() {
            Ok(info) => report.push_str(&format!("## CPU Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get CPU information: {}", e),
        }
    }
    if protocols.contains(&"Memory") {
        match get_memory_info() {
            Ok(info) => report.push_str(&format!("## Memory Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get Memory information: {}", e),
        }
    }
    if protocols.contains(&"Storage") {
        match get_storage_info() {
            Ok(info) => report.push_str(&format!("## Storage Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get Storage information: {}", e),
        }
    }
    if protocols.contains(&"Network") {
        match get_network_info() {
            Ok(info) => report.push_str(&format!("## Network Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get Network information: {}", e),
        }
    }
    if protocols.contains(&"ICMP") {
        match get_icmp_info() {
            Ok(info) => report.push_str(&format!("## ICMP Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get ICMP information: {}", e),
        }
    }
    if protocols.contains(&"SNMP") {
        match get_snmp_info() {
            Ok(info) => report.push_str(&format!("## SNMP Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get SNMP information: {}", e),
        }
    }
    if protocols.contains(&"SMTP") {
        match get_smtp_info() {
            Ok(info) => report.push_str(&format!("## SMTP Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get SMTP information: {}", e),
        }
    }
    if protocols.contains(&"NFS") {
        match get_nfs_info() {
            Ok(info) => report.push_str(&format!("## NFS Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get NFS information: {}", e),
        }
    }
    if protocols.contains(&"Services") {
        match get_services_info() {
            Ok(info) => report.push_str(&format!("## Services Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get Services information: {}", e),
        }
    }
    if protocols.contains(&"GPU") {
        match get_gpu_info() {
            Ok(info) => report.push_str(&format!("## GPU Information\n{}\n\n", summarize_info(&info))),
            Err(e) => eprintln!("Failed to get GPU information: {}", e),
        }
    }

    report.push_str("## Components and Services\n\n");
    report.push_str("```mermaid\n");
    report.push_str("graph TD;\n");
    if protocols.contains(&"OS") { report.push_str("    A[OS] -->|Info| B[CPU];\n"); }
    if protocols.contains(&"Memory") { report.push_str("    A -->|Info| C[Memory];\n"); }
    if protocols.contains(&"Storage") { report.push_str("    A -->|Info| D[Storage];\n"); }
    if protocols.contains(&"Network") { report.push_str("    A -->|Info| E[Network];\n"); }
    if protocols.contains(&"ICMP") { report.push_str("    A -->|Info| F[ICMP];\n"); }
    if protocols.contains(&"SNMP") { report.push_str("    A -->|Info| G[SNMP];\n"); }
    if protocols.contains(&"SMTP") { report.push_str("    A -->|Info| H[SMTP];\n"); }
    if protocols.contains(&"NFS") { report.push_str("    A -->|Info| I[NFS];\n"); }
    if protocols.contains(&"Services") { report.push_str("    A -->|Info| J[Services];\n"); }
    if protocols.contains(&"GPU") { report.push_str("    A -->|Info| K[GPU];\n"); }
    report.push_str("```\n");

    let mut file = File::create("RUDISCO.md").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");
}

fn summarize_info(info: &str) -> String {
    let lines: Vec<&str> = info.lines().collect();
    if lines.len() > 5 {
        lines[..5].join("\n") + "\n..."
    } else {
        info.to_string()
    }
}

fn get_os_info() -> Result<String, io::Error> {
    let output = Command::new("uname")
        .arg("-a")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_cpu_info() -> Result<String, io::Error> {
    let output = if cfg!(target_os = "macos") {
        Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()?
    } else {
        Command::new("lscpu")
            .output()?
    };
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_memory_info() -> Result<String, io::Error> {
    let output = if cfg!(target_os = "macos") {
        Command::new("vm_stat")
            .output()?
    } else {
        Command::new("free")
            .arg("-h")
            .output()?
    };
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_storage_info() -> Result<String, io::Error> {
    let mut storage_info = String::new();

    let df_output = Command::new("df")
        .arg("-h")
        .output()?;
    storage_info.push_str(&String::from_utf8_lossy(&df_output.stdout).to_string());

    let lsblk_output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,UUID")
        .output()?;
    storage_info.push_str("\nUUID Information:\n");
    storage_info.push_str(&String::from_utf8_lossy(&lsblk_output.stdout).to_string());

    let wwid_output = Command::new("ls -l /dev/disk/by-id")
        .output()?;
    storage_info.push_str("\nWWID Information:\n");
    storage_info.push_str(&String::from_utf8_lossy(&wwid_output.stdout).to_string());

    Ok(storage_info)
}

fn get_network_info() -> Result<String, io::Error> {
    let output = Command::new("ifconfig")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_icmp_info() -> Result<String, io::Error> {
    let output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg("127.0.0.1")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_snmp_info() -> Result<String, io::Error> {
    let output = Command::new("snmpwalk")
        .arg("-v")
        .arg("2c")
        .arg("-c")
        .arg("public")
        .arg("localhost")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_smtp_info() -> Result<String, io::Error> {
    let output = Command::new("telnet")
        .arg("localhost")
        .arg("25")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_nfs_info() -> Result<String, io::Error> {
    let output = Command::new("showmount")
        .arg("-e")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_services_info() -> Result<String, io::Error> {
    let output = if cfg!(target_os = "macos") {
        Command::new("launchctl")
            .arg("list")
            .output()?
    } else {
        Command::new("systemctl")
            .arg("list-units")
            .arg("--type=service")
            .output()?
    };
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_gpu_info() -> Result<String, io::Error> {
    let output = if cfg!(target_os = "macos") {
        Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()?
    } else {
        Command::new("lspci")
            .arg("| grep -i 'vga\\|3d\\|2d'")
            .output()?
    };
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
