# Bash - File transfert

## Bash - download
```
// connect to desired webserver
exec 3<>/dev/tcp/<IP>/<PORT>
// HTTP get request
echo -e "GET /<File> HTTP/1.1\n\n">&3
// print response
cat <&3
```
