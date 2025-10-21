# Everything cargo

## cargo-edit - udpating a single dependency
```
# check for newer incompatible versions
cargo upgrade -i --dry-run

# update a dependency to latest incompatible version:
cargo upgrade -i -p pyo3

# update a dependency to a specific incompatible version
cargo upgrade -i -p pyo3@0.21.2
```
