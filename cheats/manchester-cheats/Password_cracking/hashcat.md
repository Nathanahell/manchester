# hashcat

% password recovery, password cracking

#plateform/linux  #target/local  #cat/CRACKING/PASSWORD 

## hashcat - Crack NTLM hashes
```
hashcat -m 1000 dumpedhashes.txt /usr/share/wordlists/rockyou.txt
```

## hashcat - Crack single NTLM hash
```
hashcat -m 1000 64f12cddaa88057e06a81b54e73b949b /usr/share/wordlists/rockyou.txt --show
```

## unshadow - Combine passwd and shadow files
```
unshadow /tmp/passwd.bak /tmp/shadow.bak > /tmp/unshadowed.hashes
```

## hashcat - Crack unshadowed hashes
```
hashcat -m 1800 -a 0 /tmp/unshadowed.hashes rockyou.txt -o /tmp/unshadowed.cracked
```

## hashcat - Crack MD5 hashes
```
hashcat -m 500 -a 0 md5-hashes.list rockyou.txt
```

## hashcat - Crack BitLocker hashes
```
hashcat -m 22100 backup.hash /opt/useful/seclists/Passwords/Leaked-Databases/rockyou.txt -o backup.cracked
```


