# DNS

## NS request to the specific nameserver - Footprinting
```
dig ns <domain.tld> @<nameserver>
```

## ANY request to the specific nameserver - Footprinting
```
dig any <domain.tld> @<nameserver>
```

## AXFR request to the specific nameserver - Footprinting
```
dig axfr <domain.tld> @<nameserver>
```

## Subdomain brute forcing - Footprinting
```
dnsenum --dnsserver <nameserver> --enum -p 0 -s 0 -o found_subdomains.txt -f ~/subdomains.list <domain.tld>
```

## DNS force zone transfert - 1. Identify nameservers
```
nslookup -type=NS zonetransfer
```
  
## DNS force zone transfert - 2. Testing for ANY and AXFR Zone Transfer
```
nslookup -type=any -query=AXFR zonetransfer.me nsztm1.digi.ninja
```

## DNS force zone transfert - 3. Force zone transfert
```
dig axfr @nsztm1.digi.ninja zonetransfer.me # If the server is misconfigured and allows the transfer, you'll receive a complete list of DNS records for the domain, including all subdomains
```

## DNS lookup for mail servers
```
host -t MX microsoft.com
```

## dig - DNS lookup for mail servers
```
dig mx inlanefreight.com | grep "MX" | grep -v ";"
```

## host - DNS lookup for IPv4 address
```
host -t A mail1.inlanefreight.htb.
```

