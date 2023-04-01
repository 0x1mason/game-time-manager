; example2.nsi
;
; This script is based on example1.nsi, but it remember the directory, 
; has uninstall support and (optionally) installs start menu shortcuts.
;
; It will install example2.nsi into a directory that the user selects.
;
; See install-shared.nsi for a more robust way of checking for administrator rights.
; See install-per-user.nsi for a file association example.

;--------------------------------

; The name of the installer
Name "Game Time Manager Installer"

; The file to write
OutFile "GameTimeManagerInstaller.exe"

; Request application privileges for Windows Vista and higher
RequestExecutionLevel admin

; Build Unicode installer
Unicode True

; The default installation directory
InstallDir "$PROGRAMFILES\Game Time Manager"

; Registry key to check for directory (so if you install again, it will 
; overwrite the old one automatically)
InstallDirRegKey HKLM "Software\NSIS_GameTimeManager" "Install_Dir"

;--------------------------------

; Pages

Page components
Page directory
Page instfiles

UninstPage uninstConfirm
UninstPage instfiles

;--------------------------------

; The stuff to install
Section "GTM (required)"

  SectionIn RO
  
  ; Set output path to the installation directory.
  SetOutPath $INSTDIR
  
  ; Put file there
  File "GameTimeManager.exe"
  File "GameTimeManager-Tray.exe"

  SetOutPath "$INSTDIR\fonts\Ubuntu"
  File "fonts\Ubuntu\Ubuntu-Bold.ttf"  
  
  SetOutPath "$INSTDIR\icons"
  File "icons\timer256.png"  

  SetOutPath "$PROFILE\AppData\Roaming\Game Time Manager"
  File "config.toml"

   # link to config.toml
  nsExec::ExecToLog `powershell -ExecutionPolicy Bypass -WindowStyle Hidden New-Item -ItemType SymbolicLink -Target \"$PROFILE\AppData\Roaming\Game Time Manager\config.toml\" -Path \"$INSTDIR\config.toml\"`
  ;Pop $ExitCode
  ; Write the installation path into the registry
  WriteRegStr HKLM SOFTWARE\NSIS_GameTimeManager "Install_Dir" "$INSTDIR"
  ; $Env:USERPROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager.lnk

 ; WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "Game Time Manager" '"$InstDir\GameTimeManager-Tray.exe"'

  ; CreateShortcut "$PROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager.lnk" "$INSTDIR\GameTimeManager.exe"
  ; CreateShortcut "$PROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager-Tray.lnk" "$INSTDIR\GameTimeManager-Tray.exe"

  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "GameTimeManager" "$INSTDIR\GameTimeManager.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "GameTimeManager-Tray" "$INSTDIR\GameTimeManager-Tray.exe"

  ; Write the uninstall keys for Windows
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GameTimeManager" "DisplayName" "Game Time Manager"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GameTimeManager" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GameTimeManager" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GameTimeManager" "NoRepair" 1
  
  WriteUninstaller "$INSTDIR\uninstall.exe"
  nsExec::Exec "explorer.exe $INSTDIR\GameTimeManager.exe"
  nsExec::Exec "explorer.exe $INSTDIR\GameTimeManager-Tray.exe"

SectionEnd

; ; Optional section (can be disabled by the user)
; Section "Start Menu Shortcuts"

;   CreateDirectory "$SMPROGRAMS\Example2"
;   CreateShortcut "$SMPROGRAMS\Example2\Uninstall.lnk" "$INSTDIR\uninstall.exe"
;   CreateShortcut "$SMPROGRAMS\Example2\Example2 (MakeNSISW).lnk" "$INSTDIR\example2.nsi"

; SectionEnd

;--------------------------------

; Uninstaller

Section "Uninstall"
    nsExec::Exec "taskkill /IM GameTimeManager.exe"
    nsExec::Exec "taskkill /IM GameTimeManager-Tray.exe"
  ; Remove registry keys
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GameTimeManager"
  DeleteRegKey HKLM SOFTWARE\NSIS_GameTimeManager
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Run\GameTimeManager"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Run\GameTimeManager-Tray"
  ; Remove files and uninstaller
  ;RMDIR "$PROFILE\AppData\Roaming\GameTimeManager"
 # nsExec::ExecToLog `powershell -ExecutionPolicy Bypass -WindowStyle Hidden Remove-Item -ItemType SymbolicLink \"$INSTDIR\config.toml\"`

  ; Remove shortcuts, if any
  ; Delete "$PROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager.lnk"
  ; Delete "$PROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager-Tray.lnk"
RMDir /r "$PROFILE\AppData\Roaming\Game Time Manager"
  ; Remove directories
  ;RMDir "$SMPROGRAMS\GameTimeManager"
  RMDIR /r "$INSTDIR"

SectionEnd
