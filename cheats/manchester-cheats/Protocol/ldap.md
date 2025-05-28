# ldap

#plateform/linux  #target/remote  #protocol/ldap  #port/639 #port/389

## ldapsearch anonymous, dump directory structure, simple login, ignore certificate validation
```
LDAPTLS_REQCERT=never  ldapsearch -x -LLL -H ldaps://<IP>:<PORT>  -b "" "(objectClass=*)"
````

## ldapsearch - users
```
ldapsearch -x -LLL -H ldap://<ldap-server> -b "dc=example,dc=com" "(objectClass=person)"
````

## ldapsearch - users fetch specific attributes
```
ldapsearch -x -LLL -H ldap://<ldap-server> -b "dc=example,dc=com" "(objectClass=person)" cn mail uid
```

## ldapsearch - mapping LDAP to network assets
```
ldapsearch -x -LLL -H ldap://<ldap-server> -b "dc=example,dc=com" "(objectClass=posixAccount)" uid homeDirectory
```

## ldapsearch filter - find all DC
```
// ldap filter - find all DC
(&(objectCategory=Computer)(userAccountControl:1.2.840.113556.1.4.803:=8192))
```

## ldapsearch filter - find all servers in the directory that are not DC to pivot
```
// ldap filter - find all servers in the directory that are not DCs
(&(objectCategory=computer)(operatingSystem=*server*)(!(userAccountControl:1.2.840.113556.1.4.803:=8192)))
```

## ldapsearch filter - find accounts with a serviceprincipalname (SPN)
```
// ldap filter - find accounts with a serviceprincipalname (SPN)
"(&(objectClass=User)(serviceprincipalname=*)(samaccountname=*))" samaccountname serviceprincipalname
```
## ldapsearch filter - find accounts with constrained delegation
```
// ldap filter - find accounts with constrained delegation
(&(objectClass=User)(msDS-AllowedToDelegateTo=*))
```

## ldapsearch filter - find accounts with unconstrained Delegation (will include DCs)
```
# ldap filter - find accounts with constrained delegation
(&(objectClass=User)(msDS-AllowedToDelegateTo=*))
```

## ldapsearch filter - more queries
```
ldap filter - more queries : https://www.politoinc.com/post/ldap-queries-for-offensive-and-defensive-operations
```

## ldapsearch - retrieve passpol
```
ldapsearch -h 172.16.5.5 -x -b "DC=INLANEFREIGHT,DC=LOCAL" -s sub "*" | grep -m 1 -B 10 pwdHistoryLength
```

## ldapsearch - anonymous - list users
```
ldapsearch -h 172.16.5.5 -x -b "DC=INLANEFREIGHT,DC=LOCAL" -s sub "(&(objectclass=user))"  | grep sAMAccountName: | cut -f2 -d" "
```

## windapsearch.py - anonymous - list users
```
./windapsearch.py --dc-ip 172.16.5.5 -u "" -U
```
