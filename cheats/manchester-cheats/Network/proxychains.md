# Proxychains

## proxychains - Scan with Nmap through proxy
```
proxychains nmap -v -sn 172.16.5.1-200
```

## proxychains - TCP scan through proxy
```
proxychains nmap -v -Pn -sT 172.16.5.19
```

## proxychains - Open Metasploit through proxy
```
proxychains msfconsole
```
## proxychains - RDP connection through proxy
```
proxychains xfreerdp /v:<IPaddressofTarget> /u:victor /p:pass@123
```
## proxychains - Open Firefox through proxy
```
proxychains firefox-esr <IPaddressofTargetWebServer>:80
```

