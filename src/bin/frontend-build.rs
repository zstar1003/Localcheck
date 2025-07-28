use std::process::Command;
use std::path::Path;

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
    println!("构建前端项目...");
    
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
    
    // 检查是否存在 node_modules
    if !Path::new("node_modules").exists() {
        println!("安装前端依赖...");
        let output = Command::new(&npm_cmd)
            .args(&["install"])
            .output()
            .expect("执行 npm install 失败");
        
        if !output.status.success() {
            eprintln!("npm install 失败: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    }
    
    // 构建前端
    println!("执行前端构建...");
    let output = Command::new(&npm_cmd)
        .args(&["run", "build"])
        .output()
        .expect("执行 npm run build 失败");
    
    if !output.status.success() {
        eprintln!("npm run build 失败: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    println!("前端构建完成！");
}