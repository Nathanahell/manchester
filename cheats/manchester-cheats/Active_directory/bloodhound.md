# Bloodhound ingestor + GUI

## bloodhound-python - Default enumeration with IP + Domain name
```
bloodhound-ce-python -u '<username>' -p '<password>' -d <blabla.local> -dc <DOMAIN NAME> -ns <DC IP> -c All --zip
```
## bloodhoud-python - Enumeration + Kerberos
```
# Set export KRB5CCNAME=user.ccache
# Setup faketime + shell alias
faketime-shell bloodhound-python -u '<USERNAME>@<DOMAIN>' -p <PASS> -k -ns <DC IP> -c All -d <DOMAIN> --zip
```

## bloodhound - jq - extract domain user/computer list from JSON file
```
cat 20260206150643_{users,computers}.json | jq ".data | .[] | .Properties.name" | tr -d '"' >> users.lst
```
