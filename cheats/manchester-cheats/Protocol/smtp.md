# SMTP

## Interact with SMTP server - Footprint
```
telnet <FQDN/IP> 25
```

## Querying OIDs using snmpwalk - Footprinting
```
snmpwalk -v2c -c <community string> <FQDN/IP>
```

## Bruteforcing community strings of the SNMP service - Footprinting
```
onesixtyone -c community-strings.list <FQDN/IP>
```

## Bruteforcing SNMP service OIDs  - Footprinting
```
braa <community string>@<FQDN/IP>:.1.*
```

## telnet - Connect to SMTP server
```
telnet 10.10.110.20 25
```

## smtp-user-enum - SMTP user enumeration
```
smtp-user-enum -M RCPT -U userlist.txt -D inlanefreight.htb -t 10.129.203.7
```

## swaks - Test SMTP open-relay
```
swaks --from notifications@inlanefreight.com --to employees@inlanefreight.com --header 'Subject: Notification' --body 'Message' --server 10.10.11.213
```
