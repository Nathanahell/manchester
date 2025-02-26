# FTP

## Interact with the FTP service on the target - Footprint
```
ftp <FQDN/IP>
```

## Interact with the FTP service on the target  - Footprint
```
nc -nv <FQDN/IP> 21
```

## Interact with the FTP service on the target  - Footprint
```
telnet <FQDN/IP> 21
```

## Interact with the FTP service on the target using encrypted connection - Footprint
```
openssl s_client -connect <FQDN/IP>:21 -starttls ftp
```

## Download all available files on the target FTP server - Footprint
```
wget -m --no-passive ftp://anonymous:anonymous@<target>
```

