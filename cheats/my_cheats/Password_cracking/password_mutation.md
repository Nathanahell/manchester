# Password mutation

% password

## Uses cewl to generate a wordlist based on keywords present on a website.
```
cewl https://www.inlanefreight.com -d 4 -m 6 --lowercase -w inlane.wordlist
```

## Uses Hashcat to generate a rule-based word list.
```
hashcat --force password.list -r custom.rule --stdout > mut_password.list
```

## Users username-anarchy tool in conjunction with a pre-made list of first and last names to generate a list of potential username.
```
./username-anarchy -i /path/to/listoffirstandlastnames.txt
```

## Uses Linux-based commands curl, awk, grep and tee to download a list of file extensions to be used in searching for files that could contain passwords.
```
curl -s https://fileinfo.com/filetypes/compressed | html2text | awk '{print tolower($1)}' | grep "\." | tee -a compressed_ext.txt
```
