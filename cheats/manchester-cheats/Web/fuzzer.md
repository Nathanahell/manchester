# Web Fuzzing

## Bash - manual vhost fuzzing
```
cat ./vhosts | while read vhost;do echo "\n********\nFUZZING: ${vhost}\n********";curl -s -I http://192.168.10.10 -H "HOST: ${vhost}.randomtarget.com" | grep "Content-Length: ";done
```

## Gobuster - vhost fuzzing
```
gobuster vhost -u http://<target_IP_address> -w <wordlist_file> --append-domain
```

## ffuf - vhost
```
ffuf -w ./vhosts -u http://<IP> -H "HOST: FUZZ.randomtarget.com" -fs <Exclude response size>
```

## ffuf - parameter fuzzing
```
ffuf -w /opt/useful/seclists/Discovery/Web-Content/burp-parameter-names.txt:FUZZ -u http://admin.academy.htb:PORT/admin/admin.php -X POST -d 'FUZZ=key' -H 'Content-Type: application/x-www-form-urlencoded' -fs xxx
```

## ffuf - value fuzzing
```
ffuf -w ids.txt:FUZZ -u http://admin.academy.htb:PORT/admin/admin.php -X POST -d 'id=FUZZ' -H 'Content-Type: application/x-www-form-urlencoded' -fs xxx
```

## gobuster - vhost
```
gobuster vhost -u http://<target_IP_address> -w <wordlist_file> --append-domain
```

