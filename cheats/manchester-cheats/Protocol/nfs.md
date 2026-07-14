# NFS

## Show available NFS shares - Footprinting
```
showmount -e <FQDN/IP>
```

## Mount the specific NFS share to ./target-NFS - Footprinting
```
mount -t nfs <FQDN/IP>:/<share> ./target-NFS/ -o nolock
# RW
mount -t nfs -o rw cicada.vl:/profiles /mnt
```

## Unmount the specific NFS share - Footprinting
```
umount ./target-NFS
```
