# Imap Pop3

## Log in to the IMAPS service using cURL - Footprinting
```
curl -k 'imaps://<FQDN/IP>' --user <user>:<password>
```

## Connect to the IMAPS service - Footprinting
```
openssl s_client -connect <FQDN/IP>:imaps
```

## Connect to the POP3s service - Footprinting
```
openssl s_client -connect <FQDN/IP>:pop3s
```
