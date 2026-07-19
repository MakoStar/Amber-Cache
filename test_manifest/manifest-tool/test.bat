@echo off
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    exit /b %ERRORLEVEL%
)

if not exist test mkdir test

copy /Y target\release\manifest-tool.exe test\
echo Successfully copied the manifest-tool.exe to the test directory!

cd .\test\
.\manifest-tool.exe > output.log 2>&1