!macro NSIS_HOOK_PREINSTALL
    # Hard-enforce installation directory to %USERPROFILE%\.openclaw
    StrCpy $INSTDIR "$PROFILE\.openclaw"
    SetOutPath "$INSTDIR"
    DetailPrint "[Hardened] Target Installation Directory redirected to: $INSTDIR"
!macroend

!macro NSIS_HOOK_POSTINSTALL
    # 1. Set OPENCLAW_HOME environment variable
    DetailPrint "Setting OPENCLAW_HOME environment variable..."
    WriteRegStr HKCU "Environment" "OPENCLAW_HOME" "$INSTDIR"
    WriteRegStr HKCU "Environment" "OPENCLAW_CONFIG_PATH" "$INSTDIR\openclaw.json"

    # 2. Unpack node_modules from tar.gz
    DetailPrint "Checking for node_modules.tar.gz..."
    IfFileExists "$INSTDIR\resources\openclaw\node_modules.tar.gz" do_unpack skip_unpack

    do_unpack:
        DetailPrint "Unpacking node_modules (this may take a minute)..."
        nsExec::ExecToStack 'cmd /c cd /d "$INSTDIR\resources\openclaw" && tar -xzf node_modules.tar.gz'
        Pop $0
        Pop $1
        IntCmp $0 0 unpack_ok
            DetailPrint "Warning: tar exited with code $0"
            Goto unpack_done
        unpack_ok:
            DetailPrint "node_modules unpacked successfully."
            Delete "$INSTDIR\resources\openclaw\node_modules.tar.gz"
        Goto unpack_done

    skip_unpack:
        DetailPrint "node_modules.tar.gz not found, skipping unpack."

    unpack_done:

    # 3. Add to PATH
    DetailPrint "Updating PATH..."
    ReadRegStr $0 HKCU "Environment" "PATH"

    Push "$INSTDIR\resources\node-runtime"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_node
        StrCpy $0 "$0;$INSTDIR\resources\node-runtime"
    skip_node:

    Push "$INSTDIR\resources\bin"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_bin
        StrCpy $0 "$0;$INSTDIR\resources\bin"
    skip_bin:

    WriteRegStr HKCU "Environment" "PATH" $0

    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=1000

    DetailPrint "Installation complete!"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
    DetailPrint "=========================================="
    DetailPrint "OpenClaw 卸载程序"
    DetailPrint "=========================================="
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
    # 询问用户是否删除各组件
    DetailPrint "正在询问卸载选项..."

    # 停止运行中的进程
    DetailPrint "停止运行中的进程..."
    nsExec::ExecToStack 'taskkill /f /im "openclaw-workplace.exe" 2>nul'
    Pop $0
    Pop $1

    # 清理环境变量
    DetailPrint "清理环境变量..."
    DeleteRegValue HKCU "Environment" "OPENCLAW_HOME"
    DeleteRegValue HKCU "Environment" "OPENCLAW_CONFIG_PATH"

    # 清理 PATH
    ReadRegStr $0 HKCU "Environment" "PATH"

    Push "$INSTDIR\resources\node-runtime"
    Push ""
    Push $0
    Call un.StrReplace
    Pop $0

    Push "$INSTDIR\resources\bin"
    Push ""
    Push $0
    Call un.StrReplace
    Pop $0

    Push "$INSTDIR\resources\python-runtime"
    Push ""
    Push $0
    Call un.StrReplace
    Pop $0

    Push ";;"
    Push ";"
    Push $0
    Call un.StrReplace
    Pop $0

    WriteRegStr HKCU "Environment" "PATH" $0

    # 移除开机自启
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "OpenClawWorkplace"

    # 询问删除 Node.js 运行时
    MessageBox MB_YESNO "是否删除 Node.js v22 运行时？$\r$\n$\r$\n选择「否」可保留供其他程序使用。" IDYES del_node IDNO skip_node
    del_node:
        DetailPrint "删除 Node.js 运行时..."
        RMDir /r "$INSTDIR\resources\node-runtime"
        Goto node_done
    skip_node:
        DetailPrint "保留 Node.js 运行时"
    node_done:

    # 询问删除 Python 运行时
    MessageBox MB_YESNO "是否删除 Python 3.10 运行时？$\r$\n$\r$\n选择「否」可保留供其他程序使用。" IDYES del_python IDNO skip_python
    del_python:
        DetailPrint "删除 Python 运行时..."
        RMDir /r "$INSTDIR\resources\python-runtime"
        Goto python_done
    skip_python:
        DetailPrint "保留 Python 运行时"
    python_done:

    # 询问删除 OpenClaw 核心
    MessageBox MB_YESNO "是否删除 OpenClaw 核心？$\r$\n$\r$\n选择「否」可保留配置和数据。" IDYES del_core IDNO skip_core
    del_core:
        DetailPrint "删除 OpenClaw 核心..."
        RMDir /r "$INSTDIR\resources\openclaw"
        RMDir /r "$INSTDIR\resources\skills"
        RMDir /r "$INSTDIR\resources\memories"
        Delete "$INSTDIR\openclaw.json"
        Goto core_done
    skip_core:
        DetailPrint "保留 OpenClaw 核心和配置"
    core_done:

    # 删除 CLI 工具（总是删除）
    RMDir /r "$INSTDIR\resources\bin"

    # 清理缓存
    RMDir /r "$LOCALAPPDATA\com.openclaw.workplace"

    # 尝试清理空目录
    RMDir "$INSTDIR\resources"
    RMDir "$INSTDIR"

    # 广播环境变量变更
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=1000

    DetailPrint "=========================================="
    DetailPrint "OpenClaw 卸载完成"
    DetailPrint "=========================================="
!macroend

# ============================================================================
# 辅助函数
# ============================================================================

Function StrStr
  Exch $R1
  Exch
  Exch $R2
  Push $R3
  Push $R4
  Push $R5
  Push $R6
  StrLen $R3 $R1
  StrCpy $R4 0
  loop:
    StrCpy $R5 $R2 $R3 $R4
    StrCmp $R5 $R1 done
    StrCmp $R5 "" notfound
    IntOp $R4 $R4 + 1
    Goto loop
  notfound:
    StrCpy $R1 ""
    Goto end
  done:
    StrCpy $R1 $R2 "" $R4
  end:
    Pop $R6
    Pop $R5
    Pop $R4
    Pop $R3
    Pop $R2
    Exch $R1
FunctionEnd

Function un.StrReplace
  Exch $R0
  Exch
  Exch $R1
  Exch 2
  Exch $R2
  Push $R3
  Push $R4
  Push $R5
  Push $R6
  StrLen $R3 $R1
  StrLen $R4 $R2
  StrCpy $R5 0
  loop:
    StrCpy $R6 $R0 $R3 $R5
    StrCmp $R6 $R1 found
    StrCmp $R6 "" done
    IntOp $R5 $R5 + 1
    Goto loop
  found:
    StrCpy $R6 $R0 $R5
    StrCpy $R0 $R0 "" $R5
    StrCpy $R0 $R0 "" $R3
    StrCpy $R0 "$R6$R2$R0"
    IntOp $R5 $R5 + $R4
    Goto loop
  done:
    Pop $R6
    Pop $R5
    Pop $R4
    Pop $R3
    Pop $R2
    Pop $R1
    Exch $R0
FunctionEnd
