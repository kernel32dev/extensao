@echo off
setlocal

set "originalFile=extensao.exe"
set "newFile=extensao-new.exe"
set "backupFile=extensao-bkp.exe"


if exist "%originalFile%" (

    curl -L --fail "https://github.com/kernel32dev/extensao/raw/master/bin/extensao.exe" -o "%newFile%"

    if not %errorlevel%==0 (
        echo Error: Failed to download the new file. Script will exit.
		pause
        exit /b 1
    )

    fc /b "%originalFile%" "%newFile%" > nul

    if errorlevel 1 (
        echo Files are different. Stopping "%originalFile%"...
        
        call "%originalFile%" stop
        if errorlevel 1 (
            echo Failed to stop service, try executing with administrator privileges
			pause
            exit /b 1
        )
        
        move "%originalFile%" "%backupFile%" > nul
        
        move "%newFile%" "%originalFile%" > nul
        
        echo Starting "%originalFile%"...
        
        call "%originalFile%" start
        if errorlevel 1 (
            echo Failed to start service, try executing with administrator privileges
			pause
            exit /b 1
        )
    ) else (
        echo Files are identical. No action needed.
        del "%newFile%"
    )
) else (
    curl -L --fail "https://github.com/kernel32dev/extensao/raw/master/bin/extensao.exe" -o "%originalFile%"
    if not %errorlevel%==0 (
        echo Error: Failed to download the new file. Script will exit.
		pause
        exit /b 1
    )
    call "%originalFile%" install
    call "%originalFile%" start
    if errorlevel 1 (
        echo Failed to start service, try executing with administrator privileges
		pause
        exit /b 1
    )
)
pause
endlocal
