@echo off
setlocal

set "originalFile=extensao.exe"
set "newFile=extensao-new.exe"
set "backupFile=extensao-bkp.exe"

:: Download the new file using curl
curl -L --fail "https://github.com/kernel32dev/extensao/raw/master/bin/extensao.exe" -o "%newFile%"

:: Check if the download was successful
if not %errorlevel%==0 (
    echo Error: Failed to download the new file. Script will exit.
    exit /b 1
)

:: Compare the two files
fc /b "%originalFile%" "%newFile%" > nul

if errorlevel 1 (
    echo Files are different. Stopping "%originalFile%"...
    
    :: Call extensao.exe stop
    call "%originalFile%" stop
	if errorlevel 1 (
		echo Failed to stop service, try executing with administrator privileges
		exit /b 1
	)
    
    :: Move the old file to backup
    move "%originalFile%" "%backupFile%" > nul
    
    :: Move the new file to the original file's location
    move "%newFile%" "%originalFile%" > nul
    
    echo Starting "%originalFile%"...
    
    :: Call extensao.exe start
    call "%originalFile%" start
) else (
    echo Files are identical. No action needed.
	del "%newFile%"
)

endlocal
