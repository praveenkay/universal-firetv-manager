# Windows guide

Download the MSI or setup EXE from the GitHub release and follow the installer. Windows 10/11 normally includes WebView2; install the Microsoft WebView2 Runtime if the window does not open.

Enter the Fire TV's numeric IP, accept the RSA prompt on the TV, inspect compatibility, and create the required backup before any change. The portable CLI is `firetvctl-windows-x86_64.exe` and can be run from PowerShell.

Verify downloads in PowerShell with `Get-FileHash -Algorithm SHA256 <file>` and compare it with `SHA256SUMS`.
