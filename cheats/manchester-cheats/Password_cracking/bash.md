# Password cracking related bash cmd

% linux

## openssl - Extract files from archive
```
for i in $(cat rockyou.txt);do openssl enc -aes-256-cbc -d -in GZIP.gzip -k $i 2>/dev/null | tar xz;done
```
