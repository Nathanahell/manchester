# Username anarchy

## username-anarchy - Generate possible usernames for "Jane Smith"
```
username-anarchy Jane Smith
```
## username-anarchy - Use a file ('names.txt') with names for input. Can handle space, CSV, or TAB delimited names.
```
username-anarchy -i names.txt
```

## username-anarchy - Automatically generate usernames using common names from the US dataset
```
username-anarchy -a --country us
```

## username-anarchy -Use specific format plugins for username generation (comma-separated).
```
username-anarchy -f format1,format2
```

## username-anarchy - Append ''@example.com' as a suffix to each username.
```
username-anarchy -@ example.com
```

## username-anarchy - Generate usernames in case-insensitive (lowercase) format
```
username-anarchy --case-insensitive
```
