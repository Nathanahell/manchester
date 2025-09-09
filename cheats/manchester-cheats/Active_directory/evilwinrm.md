# windows evil-winrm

## Log in to the WinRM server.
```
evil-winrm -i <FQDN/IP> -u <user> -p <password>
```

## Login through proxychains as a domain user using kerberos
```
proxychains evil-winrm -k -u <User> --realm <DOMAIN.TLD> -i <DNS MACHINE NAME>.<DOMAIN.TLD>
```

## winrm - double-hop prb - creds in memory
```
# Winrm - persistant credentials for remoting
$secpasswd = Read-Host "Enter password" -AsSecureString
$cred = New-Object System.Management.Automation.PSCredential ("DOMAIN\svc_winrm", $secpasswd)
Enter-PSSession -ComputerName localhost -Credential $cred
```
