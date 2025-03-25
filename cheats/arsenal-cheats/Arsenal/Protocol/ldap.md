# ldap

#plateform/linux  #target/remote  #protocol/ldap  #port/639 #port/389

## ldap nmap
#cat/RECON 
```
nmap -n -sV --script "ldap* and not brute" -p 389 <ip>
```

## ldapsearch base
#cat/ATTACK/CONNECT 
```
ldapsearch -x -h <ip> -s base
```

## ldapsearch with base dn
#cat/ATTACK/CONNECT 
```
ldapsearch -x -h <ip> -b <basedn>
```

## ldapsearch base with authentication
#cat/ATTACK/CONNECT 
```
ldapsearch -x -h <ip> -D <domain>\\<username> -w <password> -b 'DC=<domain>,DC=<path>'
```

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
# ldap filter - find all DC
(&(objectCategory=Computer)(userAccountControl:1.2.840.113556.1.4.803:=8192))
```

## ldapsearch filter - find all servers in the directory that are not DC to pivot
```
# ldap filter - find all servers in the directory that are not DCs
(&(objectCategory=computer)(operatingSystem=*server*)(!(userAccountControl:1.2.840.113556.1.4.803:=8192)))
```

## ldapsearch filter - find accounts with a serviceprincipalname (SPN)
```
# ldap filter - find accounts with a serviceprincipalname (SPN)
"(&(objectClass=User)(serviceprincipalname=*)(samaccountname=*))" samaccountname serviceprincipalname
```
## ldapsearch filter - find accounts with constrained delegation
```
# ldap filter - find accounts with constrained delegation
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

## ldapdomaindump
#cat/ATTACK/CONNECT 
```
ldapdomaindump --no-json --no-grep --authtype SIMPLE -o ldap_dump -r <ip> -u <domain>\\<username> -p <password>
```
