@ECHO OFF

SET INSTALL_FOLDER="%LOCALAPPDATA%\Programs\sunrs-tray"

cargo build --release

IF %ERRORLEVEL% NEQ 0 ( 
   EXIT /B
)

if not exist %INSTALL_FOLDER% mkdir %INSTALL_FOLDER%

copy /b/v/y ".\assets\sunrs-tray.ico" %INSTALL_FOLDER% && copy /b/v/y ".\target\release\sunrs-tray.exe" %INSTALL_FOLDER% && echo Installed in %INSTALL_FOLDER%
