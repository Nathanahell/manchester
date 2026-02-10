# john the ripper

% password recovery, password cracking

#plateform/linux  #target/local  #cat/CRACKING/PASSWORD 

i## office2john.py - Convert DOCX to hash
```
office2john.py Protected.docx > protected-docx.hash
```

## john - Crack DOCX hash
```
john --wordlist=rockyou.txt protected-docx.hash
```

## pdf2john.pl - Convert PDF to hash
```
pdf2john.pl PDF.pdf > pdf.hash
```

## john - Crack PDF hash
```
john --wordlist=rockyou.txt pdf.hash
```

## zip2john - Generate ZIP hash
```
zip2john ZIP.zip > zip.hash
```

## john - Crack ZIP hash
```
john --wordlist=rockyou.txt zip.hash
```

## bitlocker2john - Extract BitLocker hashes
```
bitlocker2john -i Backup.vhd > backup.hashes
```

## ssh2john - cracking passphrase
```
ssh2john.py id_XX > id_XX.2crack
```
