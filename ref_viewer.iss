; Script partially generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define AppName "ref viewer"
#define AppPathName "ref_viewer"
#define AppVersion "0.1"
#define AppPublisher "andr00d"
#define AppURL "https://github.com/andr00d/ref_viewer"
#define AppExeName "ref_viewer.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{A414907C-0622-43BE-830C-5C1756A8F110}
AppName={#AppName}
AppVersion={#AppVersion}
;AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}
DefaultDirName={autopf}\ref_viewer
DisableProgramGroupPage=yes
LicenseFile={#SourcePath}\LICENSE
OutputBaseFilename=mysetup
ChangesAssociations=yes
Compression=lzma
SolidCompression=yes
WizardStyle=modern
Uninstallable=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourcePath}\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\exiftool.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#SourcePath}\media\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
; jpg
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpg\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpg\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpg\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

; jpeg
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpeg\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpeg\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.jpeg\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

; png
Root: "HKCR"; Subkey: "SystemFileAssociations\.png\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.png\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.png\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

; gif
Root: "HKCR"; Subkey: "SystemFileAssociations\.gif\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.gif\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.gif\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

; webp
Root: "HKCR"; Subkey: "SystemFileAssociations\.webp\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.webp\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "SystemFileAssociations\.webp\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

; folder
Root: "HKCR"; Subkey: "Directory\shell\{#AppPathName}"; ValueType: string; ValueName: ""; ValueData: "Open with {#AppName}"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "Directory\shell\{#AppPathName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\icon.ico"; Flags: uninsdeletekey
Root: "HKCR"; Subkey: "Directory\shell\{#AppPathName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}""  ""%1"""; Flags: uninsdeletekey

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExeName}"; 
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon
