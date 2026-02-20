# Bash tricks - tricks using shells in general

## symlink abuse
```
If a script do not use absolute path to run binaries, to access file, then leverage symlinks : 'ln -s ..'
```

## bash - code execution in arithmetic expression
```
See
https://unix.stackexchange.com/questions/172103/security-implications-of-using-unsanitized-data-in-shell-arithmetic-evaluation
https://github.com/koalaman/shellcheck/issues/3088
```
