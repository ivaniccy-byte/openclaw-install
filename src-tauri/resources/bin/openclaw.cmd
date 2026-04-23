@echo off
setlocal
:: 获取当前脚本所在的 resources 目录
set "RESOURCES_DIR=%~dp0.."
set "NODE_EXE=%RESOURCES_DIR%\node-runtime\node.exe"
set "OPENCLAW_JS=%RESOURCES_DIR%\openclaw\start.js"

if not exist "%NODE_EXE%" (
    echo [Error] Node.js runtime not found at: %NODE_EXE%
    exit /b 1
)

if not exist "%OPENCLAW_JS%" (
    echo [Error] OpenClaw core script not found at: %OPENCLAW_JS%
    exit /b 1
)

:: 透传所有参数并启动 OpenClaw
"%NODE_EXE%" "%OPENCLAW_JS%" %*
endlocal
