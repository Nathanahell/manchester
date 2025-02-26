# Recon Bash cmd
% infrastructure, reco

## Certificate transparency - Infrastructure enumeration - Recon
```
export TARGET="facebook.com"; curl -s "https://crt.sh/?q=${TARGET}&output=json" | jq -r '.[] | "\(.name_value)\n\(.common_name)"' | sort -u > "${TARGET}_crt.sh.txt"
```

## Certificate transparency - Infrastructure enumeration bis - Recon
```
curl -s https://crt.sh/\?q\=<target-domain>\&output\=json | jq .
```

## Certificate transparency - 'dev' domain lookup on crt.sh
```
curl -s "https://crt.sh/?q=facebook.com&output=json" | jq -r '.[] | select(.name_value | contains("dev")) | .name_value' | sort -u
```

## Manual certificate transparency against a website
```
export TARGET="facebook.com"
export PORT="443"
openssl s_client -ign_eof 2>/dev/null <<<$'HEAD / HTTP/1.0\r\n\r' -connect "${TARGET}:${PORT}" | openssl x509 -noout -text -in - | grep 'DNS' | sed -e 's|DNS:|\n|g' -e 's|^\*.*||g' | tr -d ',' | sort -u
````
## Extract subdomains from a list and sort
```
cat *.json | jq -r '.hosts[]' 2>/dev/null | cut -d':' -f 1 | sort -u > "${TARGET}_theHarvester.txt"
```

## Scan each IP address in a list using Shodan - Infrastructure enumeration - Recon
```
for i in $(cat ip-addresses.txt);do shodan host $i;done
```

## Wayback machine - Get all snapshot of a site - Passive infra recon
```
waybackurls -dates https://facebook.com > waybackurls.txt
```

## Whatweb - Common server characteristics
```
whatweb -a3 https://www.facebook.com -v
```

## netstat - Display routing table
```
netstat -r
```

## ssh - Create SSH tunnel
```
ssh -L 1234:localhost:3306 Ubuntu@<IPaddressofTarget>
```

## netstat - Display tunnel connections
```
netstat -antp | grep 1234
```

## Discover devices on network (Linux)
```
for i in {1..254} ;do (ping -c 1 172.16.5.$i | grep "bytes from" &) ;done
```

## socks4 - Configure SOCKS4 proxy
```
socks4 127.0.0.1 9050
```

## Socks5 - Configure SOCKS5 proxy
```
Socks5 127.0.0.1 1080
```

## xfreerdp - Connect via port forwarding
```
xfreerdp /v:localhost:3300 /u:victor /p:pass@123
```

## netstat - Display active network connections
```
netstat -antp
```

## netstat - List TCP connections on port
```
netstat -antb |findstr 1080
```
