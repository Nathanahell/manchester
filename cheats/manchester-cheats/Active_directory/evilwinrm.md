# windows evil-winrm

## Log in to the WinRM server.
```
evil-winrm -i <FQDN/IP> -u <user> -p <password>
```

## Login through proxychains as a domain user using kerberos
```
proxychains evil-winrm -k -u <User> -r <DOMAIN.TLD> -i <DNS MACHINE NAME>.<DOMAIN.TLD>
```
