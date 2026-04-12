!macro NSIS_HOOK_PREINSTALL
    # Force installation directory to %USERPROFILE%\.openclaw
    # Even if the user selected something else, we align with OpenClaw standard
    StrCpy $INSTDIR "$PROFILE\.openclaw"
    DetailPrint "Target Installation Directory: $INSTDIR"
!macroend

!macro NSIS_HOOK_POSTINSTALL
    # 1. Set OPENCLAW_HOME and OPENCLAW_CONFIG_PATH
    DetailPrint "Configuring Environment Variables..."
    WriteRegStr HKCU "Environment" "OPENCLAW_HOME" "$INSTDIR"
    WriteRegStr HKCU "Environment" "OPENCLAW_CONFIG_PATH" "$INSTDIR\openclaw.json"

    # 2. Unpack node_modules from tar.gz
    DetailPrint "Unpacking OpenClaw core dependencies (node_modules)..."
    ExecWait 'cmd /c tar -xzf "$INSTDIR\openclaw\node_modules.tar.gz" -C "$INSTDIR\openclaw" && del /f "$INSTDIR\openclaw\node_modules.tar.gz"' $0
    IntCmp $0 0 node_unpack_ok
        DetailPrint "Warning: Failed to unpack node_modules (error code: $0). OpenClaw may not function correctly."
        Goto node_unpack_done
    node_unpack_ok:
        DetailPrint "node_modules unpacked successfully."
    node_unpack_done:

    # 3. Add Binaries to PATH (Node and Python)
    ReadRegStr $0 HKCU "Environment" "PATH"

    # Simple check and append
    # Note: For a more robust PATH management, normally we'd use a dedicated NSH
    # but here we do a basic check to avoid obvious duplicates

    # Append Node Runtime
    Push "$INSTDIR\node-runtime"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_node_path
        StrCpy $0 "$0;$INSTDIR\node-runtime"
    skip_node_path:

    # Append Python Runtime
    Push "$INSTDIR\python-runtime"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_python_path
        StrCpy $0 "$0;$INSTDIR\python-runtime"
    skip_python_path:

    WriteRegStr HKCU "Environment" "PATH" $0

    # Notify system of environment changes
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000

    DetailPrint "Environment variables configured successfully."
!macroend

# Uninstallation is handled by the standard uninstaller, 
# but we can add custom cleanup in a hook if supported or global space.
# In Tauri v2, uninstaller hooks are separate or can be added to the global scope.

Section "-UninstallExtra"
    # This runs during uninstallation
    DeleteRegValue HKCU "Environment" "OPENCLAW_HOME"
    
    # Notify system
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

# Helper function StrStr
Function StrStr
  Exch $R1 ; string to search for
  Exch
  Exch $R2 ; string to search in
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

