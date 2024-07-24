@echo off
setlocal enableDelayedExpansion

echo current path: !cd!
cd /d !cd!

call :cargo_test
call :cargo_deny

endlocal
exit /b

:cargo_test
echo $ cargo test --verbose -- --show-output
call cargo test --verbose --features "asynchronous" -- --show-output
goto :eof

:cargo_deny
echo $ cargo deny check bans licenses sources
call cargo deny check bans licenses sources
goto :eof
