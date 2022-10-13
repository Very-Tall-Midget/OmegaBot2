@echo off

tasklist /fi "ImageName eq OmegaBotUI.exe" /fo csv 2>NUL | find /I "OmegaBotUI.exe">NUL
set open=%ERRORLEVEL%
if %open% neq 0 call C:\Qt\5.15.2\msvc2019\bin\windeployqt.exe build-UI-32bit-Release\release\OmegaBotUI.exe
pushd OmegaBotDll
call build-debug.bat
popd
copy /Y OmegaBotDll\target\debug\omega_bot.dll build-UI-32bit-Release\release\OmegaBot.dll
if %open% neq 0 call run.bat
pause
