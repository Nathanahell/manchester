# feroxbuster

## feroxbuster - all-in-one enum
```
feroxbuster -vv -w $(fzf --walker-root=/usr/share/seclist)  -u "<URL>" -ac --rate-limit 100 --collect-extensions --collect-backups --collect-words -o ferox-<URL> --dont-scan /static
```
