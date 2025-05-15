# Manchester

## About

Manchester is a cross-platform command-line memo to quickly find a system command.
It is designed for portability, ease of use, and can be customized to suit your needs.
The main features of the tool are the following :
- **Portability**: the project is cross-platfrom as it can be built for different targets. Take the release binary and you are all set. No set-up required.
- **Does not require internet access**: your notes are embedded within the binary, therefore no internet access or install is required after building the porject. Ideal for constrained environment.
- **Ease of use**: Use a fuzzy searcher for easier command matching. Navigation is done using the arrow keys.

This project was heavily inspired by OCD's [Arsenal](https://github.com/Orange-Cyberdefense/arsenal) tool. Do check them out :D


## The author's note

This is my first ever open-source project. I took a chance to practice Rust and to fiddle with the Ratatui library.
I am open for any improvements in my code as I am not quite a software developper yet. Any constructive feedback is much appreciated. :)

Like the project ? Don't hesitate to contribute !

## Demo

Search command and output in the terminal.

![demo](demo/demo.gif)

*Made with asciinema and agg*

## Build
```
cargo build --release
```

## Update the project

Since the default notes are token from Arsenal's public repository a simple copy of the cheats directory is used to update the repository.

Any suggestion for a simpler & automated workflow is appreciated.

## Documentation

To do.

## Contribution

Any contribution is welcome. Be it documentation, code changes and especially commands.
### Code contribution
- Allow for the completion of a command within a box prompting for input after a command has been selected
- Windows's Defender evasion : the binary is flagged a malicious as it contains malicious commands.
- Not every fields (based on a Arsenal cheatsheet) appear in the tool because I lacked the time to fully display everything.

### Cheatsheets
- Add additional commands.
  - For adding commands, a template is present inside cheats/arsenal-cheats folder.
  - Multiple commands in a code block is supported.
