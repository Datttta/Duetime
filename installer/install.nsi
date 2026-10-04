; --- Duetime NSIS Modern Installer Script ---

!define APPNAME "Duetime"
!define COMPANYNAME "Datttta"
!define DESCRIPTION "A terminal task and time management application"
!define VERSIONMAJOR 1
!define VERSIONMINOR 8
!define VERSIONBUILD 2

; The name of the installer you want to generate
OutFile "Duetime-Setup.exe"

; The default installation directory
InstallDir "$LOCALAPPDATA\Duetime"

; Request application privileges for standard user execution
RequestExecutionLevel user

!include "MUI2.nsh"

!define MUI_ABORTWARNING

; Insert pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    
    ; Include your compiled Windows executable from the release build folder
    File "..\target\release\Duetime.exe"
    
    ; Create the Start Menu shortcut
    CreateDirectory "$SMPROGRAMS\Duetime"
    CreateShortcut "$SMPROGRAMS\Duetime\Duetime.lnk" "$INSTDIR\Duetime.exe" "" "$INSTDIR\Duetime.exe" 0
    
    ; Create Uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    ; Write registry keys for Windows Add/Remove Programs
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime" "DisplayName" "$(^Name)"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime" "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime" "DisplayIcon" "$INSTDIR\Duetime.exe"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime" "Publisher" "${COMPANYNAME}"
SectionEnd

Section "Uninstall"
    ; Remove files
    Delete "$INSTDIR\Duetime.exe"
    Delete "$INSTDIR\uninstall.exe"
    
    ; Remove shortcuts
    Delete "$SMPROGRAMS\Duetime\Duetime.lnk"
    RMDir "$SMPROGRAMS\Duetime"
    
    ; Remove installation directory
    RMDir "$INSTDIR"
    
    ; Remove registry keys
    DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime"
SectionEnd
