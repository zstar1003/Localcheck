@echo off
echo 创建新的Tauri项目...

REM 创建一个临时目录
mkdir temp_project
cd temp_project

REM 使用Tauri CLI创建一个新项目
npm create tauri@latest -- --template vanilla-ts

REM 复制新项目的配置文件
copy src-tauri\tauri.conf.json ..\src-tauri\tauri.conf.json

REM 返回上级目录
cd ..

REM 删除临时目录
rmdir /s /q temp_project

echo 新的tauri.conf.json文件已创建。