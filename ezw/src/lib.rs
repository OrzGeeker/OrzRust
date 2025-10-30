use clap::{Parser, Subcommand};
use std::process::Command;
use std::env;

/// 命令行子命令枚举
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 配置Git环境
    Git = 0,
    /// 配置Shell环境
    Shell = 1,
    /// 配置Vim环境
    Vim = 2,
}

/// 命令行参数解析结构
#[derive(Parser, Debug)]
#[command(version, about, long_about = "ezw - 简易工作环境配置工具, 支持Git、Shell和Vim的快速配置")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// 执行命令行命令的主函数
pub fn run_app() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Git => {
            println!("开始配置Git环境...");
            configure_git();
        },
        Commands::Shell => {
            println!("开始配置Shell环境...");
            configure_shell();
        },
        Commands::Vim => {
            println!("开始配置Vim环境...");
            configure_vim();
        },
    }
}

/// 配置Git环境
fn configure_git() {
    // 检测操作系统类型
    let os = detect_os();
    println!("当前操作系统: {}", os);
    
    // 检查Git是否安装
    if !is_git_installed() {
        println!("错误: 未检测到Git。请先安装Git再运行此命令。");
        return;
    }
    
    // 这里可以添加Git配置逻辑
    println!("Git配置成功！");
}

/// 配置Shell环境
fn configure_shell() {
    let os = detect_os();
    println!("当前操作系统: {}", os);
    
    // 获取当前shell
    let shell = detect_shell();
    println!("当前Shell: {}", shell);
    
    // 这里可以添加Shell配置逻辑
    println!("Shell配置成功！");
}

/// 配置Vim环境
fn configure_vim() {
    let os = detect_os();
    println!("当前操作系统: {}", os);
    
    // 检查Vim是否安装
    if !is_vim_installed() {
        println!("错误: 未检测到Vim。请先安装Vim再运行此命令。");
        return;
    }
    
    // 这里可以添加Vim配置逻辑
    println!("Vim配置成功！");
}

/// 检测操作系统类型
fn detect_os() -> String {
    #[cfg(target_os = "windows")]
    return "Windows".to_string();
    
    #[cfg(target_os = "macos")]
    return "macOS".to_string();
    
    #[cfg(target_os = "linux")]
    return "Linux".to_string();
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Unknown".to_string();
}

/// 检测当前使用的Shell
fn detect_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        return shell;
    } else if let Ok(comspec) = env::var("COMSPEC") {
        return comspec;
    }
    return "Unknown Shell".to_string();
}

/// 检查Git是否已安装
fn is_git_installed() -> bool {
    let output = Command::new(
        #[cfg(target_os = "windows")]
        "where",
        #[cfg(not(target_os = "windows"))]
        "which",
    )
    .arg("git")
    .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// 检查Vim是否已安装
fn is_vim_installed() -> bool {
    let output = Command::new(
        #[cfg(target_os = "windows")]
        "where",
        #[cfg(not(target_os = "windows"))]
        "which",
    )
    .arg("vim")
    .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}