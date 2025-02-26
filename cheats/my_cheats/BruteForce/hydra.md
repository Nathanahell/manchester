# Hydra

% hydra

## hydra - Crack passwords with user and password lists
```
hydra -L user.list -P password.list <service>://<ip>
```

## hydra - Crack passwords with a username and password list
```
hydra -l username -P password.list <service>://<ip>
```

## hydra - Crack passwords with a user list and password
```
hydra -L user.list -p password <service>://<ip>
```

## hydra - Credential stuffing attack
```
hydra -C <user_pass.list> ssh://<IP>
```
## hydra - Brute-force POP3
```
hydra -L users.txt -p 'Company01!' -f 10.10.110.20 pop3
```
