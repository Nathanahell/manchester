# rpivot

## rpivot - Run rpivot server
```
python2.7 server.py --proxy-port 9050 --server-port 9999 --server-ip 0.0.0.0
```

## rpivot - Connect to rpivot server
```
python2.7 client.py --server-ip 10.10.14.18 --server-port 9999
```

## rpivot - Connect to NTLM proxy
```
python client.py --server-ip <IPaddressofTargetWebServer> --server-port 8080 --ntlm-proxy-ip IPaddressofProxy> --ntlm-proxy-port 8081 --domain <nameofWindowsDomain> --username <username> --password <password>
```
