[Setup]
AppName=%%APP_NAME%%
AppVersion=%%APP_VERSION%%
AppPublisher=Idria
AppPublisherURL=https://www.idria.com.ar
WizardStyle=modern
ArchitecturesAllowed=x64
DefaultDirName={autopf64}\%%APP_NAME%%
DefaultGroupName=%%APP_NAME%%
OutputBaseFilename=%%OUTPUT_BASE_FILENAME%%
PrivilegesRequired=admin
OutputDir=..\

[Dirs]
Name: "{app}"; Permissions: users-full

[Files]
Source: "..\%%EXE_NAME%%.exe"; DestDir: "{app}"; Permissions: users-readexec
Source: "..\..\..\build\win\nssm.exe"; DestDir: "{app}"; Permissions: users-readexec
Source: "..\..\..\front\*"; DestDir: "{app}\front"; Flags: ignoreversion recursesubdirs; Permissions: users-readexec

[run]
Filename: {app}\nssm.exe; Parameters: "install %%APP_NAME%% ""{app}\%%EXE_NAME%%.exe"""; Flags: runhidden
Filename: {app}\nssm.exe; Parameters: "set %%APP_NAME%% AppDirectory ""{app}"""; Flags: runhidden
Filename: {app}\nssm.exe; Parameters: "set %%APP_NAME%% Start SERVICE_AUTO_START"; Flags: runhidden
Filename: {app}\nssm.exe; Parameters: "start %%APP_NAME%%"; Flags: runhidden

[UninstallRun]
Filename: {app}\nssm.exe; Parameters: "stop %%APP_NAME%%"; Flags: runhidden; RunOnceId: "StopService"
Filename: {app}\nssm.exe; Parameters: "remove %%APP_NAME%% confirm"; Flags: runhidden; RunOnceId: "RemoveService"
