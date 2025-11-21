@echo off
echo ========================================
echo  Dell XE9680 GPU 映射查询程序
echo  正在编译 Rust 桌面应用...
echo ========================================
echo.

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo  编译成功！启动程序...
    echo ========================================
    echo.
    start "" "target\release\xe9680-gpu-mapper.exe"
) else (
    echo.
    echo ========================================
    echo  编译失败！请检查错误信息
    echo ========================================
    pause
)
