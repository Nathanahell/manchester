# Bash one-liner

#plateform/linux #target/local #cat/UTILS

## bash - Recursively create .gitkeep files in empty folders
```
find . -type d -empty -not -path "./.git/*" -exec touch {}/.gitkeep \;
```
