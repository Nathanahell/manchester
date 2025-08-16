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
# Check the interface with 'ss -tnlp', if UP addr for the redirected port is 127.0.0.1, the port redirection failed.
# Change the ssh config file using the following tip, restart the svc, restart a fresh ssh connection.
# Si vous êtes confronté à des problèmes lors de la mise en place d’une redirection de port distant, ceci peut être dû à la configuration de votre serveur SSH. La redirection de port distant est en général désactivée par défaut. Vous pouvez modifier cela en activant GatewayPorts dans votre fichier de configuration de serveur SSH. Pour ce faire, ouvrez le fichier et définissez « GatewayPorts » sur « yes ».
# You want your forwarded port to listen to all IP i.e 0.0.0.0
# It is especially useful to foward smb connection to get NTLMv2 'hash' of a victim machine through a pivot machine
# cf pivoting
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
