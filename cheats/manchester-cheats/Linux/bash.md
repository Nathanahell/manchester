# Bash one-liner

#plateform/linux #target/local #cat/UTILS

## bash - Recursively create .gitkeep files in empty folders
```
find . -type d -empty -not -path "./.git/*" -exec touch {}/.gitkeep \;
```

## ps - list processes
```
ps -ef --forest
```

## bash - debugging scripts tricks
```
# set ...
set +/-x

# trapping errors
https://unix.stackexchange.com/questions/39623/trap-err-and-echoing-the-error-line
```

# tar - extract .tar.gz archive
```
tar -xvzf archive.tar.gz -C custome_folder
```
