@echo off
echo 正在启动论文本地校验工具...

REM 启动前端开发服务器
start cmd /k "npm run dev"

REM 等待前端服务器启动
echo 等待前端服务器启动...
timeout /t 5 /nobreak

REM 启动Tauri应用
cd src-tauri
cargo run