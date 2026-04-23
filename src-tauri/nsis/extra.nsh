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

    # 2. Unpack node_modules from tar.gz (with file existence check)
    DetailPrint "Checking for node_modules.tar.gz..."
    IfFileExists "$INSTDIR\resources\openclaw\node_modules.tar.gz" do_unpack skip_unpack

    do_unpack:
        DetailPrint "Unpacking node_modules (this may take a minute)..."
        # Use nsExec for better control (doesn't wait for cmd window)
        nsExec::ExecToStack 'cmd /c cd /d "$INSTDIR\resources\openclaw" && tar -xzf node_modules.tar.gz'
        Pop $0  ; exit code
        Pop $1  ; output
        IntCmp $0 0 unpack_ok
            DetailPrint "Warning: tar exited with code $0"
            Goto unpack_done
        unpack_ok:
            DetailPrint "node_modules unpacked successfully."
            # Delete the tar.gz after successful extraction
            Delete "$INSTDIR\resources\openclaw\node_modules.tar.gz"
        Goto unpack_done

    skip_unpack:
        DetailPrint "node_modules.tar.gz not found, skipping unpack."

    unpack_done:

    # 3. Add to PATH (only if not already present)
    DetailPrint "Updating PATH..."
    ReadRegStr $0 HKCU "Environment" "PATH"

    # Check and add node-runtime
    Push "$INSTDIR\resources\node-runtime"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_node
        StrCpy $0 "$0;$INSTDIR\resources\node-runtime"
    skip_node:

    # Check and add bin
    Push "$INSTDIR\resources\bin"
    Push $0
    Call StrStr
    Pop $1
    StrCmp $1 "" 0 skip_bin
        StrCpy $0 "$0;$INSTDIR\resources\bin"
    skip_bin:

    WriteRegStr HKCU "Environment" "PATH" $0

    # 4. Broadcast environment change (with short timeout)
    # Using 1 second timeout to avoid hanging on unresponsive windows
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=1000

    DetailPrint "Installation complete!"
!macroend

# Uninstall section
Section "-UninstallExtra"
    # Remove environment variables
    DeleteRegValue HKCU "Environment" "OPENCLAW_HOME"
    DeleteRegValue HKCU "Environment" "OPENCLAW_CONFIG_PATH"

    # Remove from PATH
    ReadRegStr $0 HKCU "Environment" "PATH"

    Push "$INSTDIR\resources\node-runtime"
    Push ""
    Push $0
    Call StrReplace
    Pop $0

    Push "$INSTDIR\resources\bin"
    Push ""
    Push $0
    Call StrReplace
    Pop $0

    # Clean up double semicolons
    Push ";;"
    Push ";"
    Push $0
    Call StrReplace
    Pop $0

    WriteRegStr HKCU "Environment" "PATH" $0

    # Remove installation directory
    RMDir /r "$INSTDIR"

    # Remove AppData cache
    RMDir /r "$LOCALAPPDATA\com.openclaw.workplace"

    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=1000
SectionEnd

# Helper function: StrStr (check if string contains substring)
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

# Helper function: StrReplace
Function StrReplace
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
