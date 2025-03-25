# socat

## socat - Listen and fork connections
```
socat TCP4-LISTEN:8080,fork TCP4:<IPaddressofAttackHost>:80
```

## socat - Forward to target host
```
socat TCP4-LISTEN:8080,fork TCP4:<IPaddressofTarget>:8443
```


## chisel - Start server with SOCKS5
```
./chisel server -v -p 1234 --socks5
```

## chisel - Connect to server
```
./chisel client -v 10.129.202.64:1234 socks
```
