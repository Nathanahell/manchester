# nmap

#plateform/linux #target/remote #cat/RECON #tag/scan


## nmap - Scan local port
```
nmap -v -sV -p1234 localhost
```

## nmap - mssql script scan
```
sudo nmap --script ms-sql-info,ms-sql-empty-password,ms-sql-xp-cmdshell,ms-sql-config,ms-sql-ntlm-info,ms-sql-tables,ms-sql-hasdbaccess,ms-sql-dac,ms-sql-dump-hashes --script-args mssql.instance-port=1433,mssql.username=sa,mssql.password=,mssql.instance-name=MSSQLSERVER -sV -p 1433 <IP>
```

## nmap - SID bruteforcing
```
sudo nmap -p1521 -sV <IP> --open --script oracle-sid-brute
```
