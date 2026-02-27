# Passowrd tips - Everything related to passwords

## Hybrid attack - random + bruteforce password generation using wordlists
```
# Extract passwords matchin the passpol
grep -E '^.{8,}$' darkweb2017-top10000.txt > darkweb2017-minlength.txt

# Apply filter corresponding to password criteria
# One uppercase character
grep -E '[A-Z]' darkweb2017-minlength.txt > darkweb2017-uppercase.txt

# One lowercase
grep -E '[a-z]' darkweb2017-uppercase.txt > darkweb2017-lowercase.txt

# One digit
grep -E '[0-9]' darkweb2017-lowercase.txt > darkweb2017-number.txt
```

## Password spraying tips
```
- Spraying
  - spray systematically. On every endpoint. Check the passpol before spraying.

- Password guessing
  - Identify common username naming convention :
    - albert wesker > awesker, walbert
    - test evident combination outside of the naming convention
      ex : if convention is {1st letter of family name}{name of employee}, test {name of the employee}

```
