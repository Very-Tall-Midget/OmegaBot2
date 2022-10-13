@echo off
:start

set /p "environment=Count frames? "

IF "%environment%"=="Yes" (
	cargo +stable-i686-pc-windows-msvc build --release --lib --features count_frames
) ELSE IF "%environment%"=="No" (
	cargo +stable-i686-pc-windows-msvc build --release --lib
) ELSE (
	goto start
)
