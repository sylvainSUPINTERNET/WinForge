Windows Svc

```` bash
cargo build --release
````

```` bash 
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v TestDaemon /t REG_SZ /d '"C:\Workspace\perso\WinForge\winforge-background-service\target\release\winforge-background-service.exe"' /f

# shutdown


# check
Get-Process | Where-Object ProcessName -like "*winforge*"


# remove key
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v TestDaemon /f

reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v TestDaemon

reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Run"


$p=New-Object IO.Pipes.NamedPipeClientStream(".","winforge",[IO.Pipes.PipeDirection]::Out);$p.Connect();$b=[Text.Encoding]::UTF8.GetBytes('{"cmd":"ping"}');$p.Write($b,0,$b.Length);$p.Dispose()

````