# DNS

## dig - NS request to the specific nameserver - Footprinting
```
dig ns <domain.tld> @<nameserver>
```

## dig - ANY request to the specific nameserver - Footprinting
```
dig any <domain.tld> @<nameserver>
```

## dig -AXFR request to the specific nameserver - Footprinting
```
dig axfr <domain.tld> @<nameserver>
```

## dig - DNS lookup for mail servers
```
dig mx inlanefreight.com | grep "MX" | grep -v ";"
```

## dig -DNS force zone transfert - 3. Force zone transfert
```
dig axfr @nsztm1.digi.ninja zonetransfer.me # If the server is misconfigured and allows the transfer, you'll receive a complete list of DNS records for the domain, including all subdomains
```

## dig - Specifies a specific name server to query
```
dig @<X.X.X.X> <domain>
```

## dig - Show full path of DNS resolution
```
dig +trace <domain>
```

## dnsenum - Subdomain brute forcing - Footprinting
```
dnsenum --dnsserver <nameserver> --enum -p 0 -s 0 -o found_subdomains.txt -f ~/subdomains.list <domain.tld>
```

## dnseum - Recursive subdomain brute forcing
```
dnsenum --enum <subdomain> -f </path/to/wordlist> -r
```

## DNS force zone transfert - 1. Identify nameservers
```
nslookup -type=NS zonetransfer
```
  
## DNS force zone transfert - 2. Testing for ANY and AXFR Zone Transfer
```
nslookup -type=any -query=AXFR zonetransfer.me nsztm1.digi.ninja
```

## DNS lookup for mail servers
```
host -t MX microsoft.com
```


## host - DNS lookup for IPv4 address
```
host -t A mail1.inlanefreight.htb.
```

