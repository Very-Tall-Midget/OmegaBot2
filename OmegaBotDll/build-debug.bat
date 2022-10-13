@echo off
:start

set /p "environment=Count frames? "

IF "%environment%"=="Yes" (
	cargo +stable-i686-pc-windows-msvc build --lib --features count_frames
) ELSE IF "%environment%"=="No" (
	cargo +stable-i686-pc-windows-msvc build --lib
) ELSE (
	goto start
)
