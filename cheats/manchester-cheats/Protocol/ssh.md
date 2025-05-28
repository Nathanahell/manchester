# SSH

## ssh - Remote security audit against the target SSH service - Audit
```
ssh-audit.py <FQDN/IP>
```

## ssh - Log in to the SSH server using the SSH client.
```
ssh <user>@<FQDN/IP>
```

## ssh - Log in to the SSH server using private key.
```
ssh -i private.key <user>@<FQDN/IP>
```

## ssh - Enforce password-based authentication.
```
ssh <user>@<FQDN/IP> -o PreferredAuthentications=password
```
## ssh - pivot, tunneling -  Forward data via port
```
ssh -L 1234:localhost:3306 8080:localhost:80 <victim username>@<IPaddressofTarget>
```

## ssh - pivot, tunneling Dynamic port forward
```
ssh -D 9050 ubuntu@<IPaddressofTarget>
```

## ssh - Reverse SSH tunnel
```
ssh -R <InternalIPofPivotHost>:<PivotHostPort2Forward>:0.0.0.0:<listener port> <victim username>@<ipAddressofTarget> -vN
```
## scp - Transfer file to target
```
scp backupscript.exe ubuntu@<ipAddressofTarget>:~/
```

## scp - Transfer directory to target
```
scp -r rpivot ubuntu@<IPaddressOfTarget>
```

## ssh - Connect through local port
```
ssh -p2222 -lubuntu 127.0.0.1
```
