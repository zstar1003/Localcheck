use std::process::Command;
use std::path::Path;
use std::time::Duration;
use std::thread;

fn find_npm_command() -> String {
    // 在 Windows 上尝试不同的 npm 命令
    let npm_commands = if cfg!(target_os = "windows") {
        vec!["npm.cmd", "npm.exe", "npm"]
    } else {
        vec!["npm"]
    };
    
    for cmd in npm_commands {
        if Command::new(cmd).arg("--version").output().is_ok() {
            return cmd.to_string();
        }
    }
    
    // 如果都找不到，返回默认的 npm
    "npm".to_string()
}

fn main() {
    println!("准备启动前端开发服务器...");
    
    let npm_cmd = find_npm_command();
    println!("使用 npm 命令: {}", npm_cmd);
    
    // 检查 npm 是否可用
    match Command::new(&npm_cmd).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                println!("npm 版本: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                eprintln!("错误: npm 命令不可用");
                eprintln!("请确保已安装 Node.js 和 npm");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("错误: 找不到 npm 命令: {}", e);
            eprintln!("请确保已安装 Node.js 和 npm，并且已添加到系统 PATH 中");
            std::process::exit(1);
        }
    }
    
    // 检查是否存在 package.json
    if !Path::new("package.json").exists() {
        eprintln!("错误: 未找到 package.json 文件");
        eprintln!("请确保在项目根目录中运行此命令");
        std::process::exit(1);
    }
    
    // 检查是否存在 node_modules
    if !Path::new("node_modules").exists() {
        println!("安装前端依赖...");
        let output = Command::new(&npm_cmd)
            .args(&["install"])
            .output()
            .map_err(|e| {
                eprintln!("执行 npm install 失败: {}", e);
                std::process::exit(1);
            })
            .unwrap();
        
        if !output.status.success() {
            eprintln!("npm install 失败: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
        println!("依赖安装完成");
    }
    
    // 启动前端开发服务器（在后台运行）
    println!("启动前端开发服务器...");
    
    // 使用 start 命令在后台启动 npm run dev
    if cfg!(target_os = "windows") {
        match Command::new("cmd")
            .args(&["/c", "start", "/b", &npm_cmd, "run", "dev"])
            .status()
        {
            Ok(_) => {
                println!("前端开发服务器启动命令已执行");
            }
            Err(e) => {
                eprintln!("启动前端开发服务器失败: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match Command::new(&npm_cmd)
            .args(&["run", "dev"])
            .spawn()
        {
            Ok(_) => {
                println!("前端开发服务器启动命令已执行");
            }
            Err(e) => {
                eprintln!("启动前端开发服务器失败: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    // 等待一段时间确保服务器开始启动
    thread::sleep(Duration::from_secs(2));
    println!("前端开发服务器准备就绪，Tauri将继续启动");
    
    // 正常退出，让 Tauri 继续执行
}