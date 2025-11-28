# Manchester

## About

Manchester is a cross-platform command-line memo to quickly find a system command.
It is designed for portability, ease of use, and can be customized to suit your needs.
The main features of the tool are the following :
- **Portability**: the project is cross-platfrom as it can be built for different targets. Take the release binary and you are all set. No set-up required.
- **Does not require internet access**: your notes are embedded within the binary, therefore no internet access or install is required after building the project. Ideal for constrained environment.
- **Ease of use**: Use a fuzzy searcher for easier command matching. Navigation is done using the arrow keys.

This project was heavily inspired by OCD's [Arsenal](https://github.com/Orange-Cyberdefense/arsenal) tool. Do check them out.

## The author's note

I took a chance to practice Rust and to fiddle with the Ratatui library.
Any constructive feedback is much appreciated.
The usefulness of the tool is dependent on your contribution. This project aims to be a general purpose command/short cheatsheet finder for sysadmin, networking, architecture and hacking. As such, any command you regularily use to speed-up your workflow is eligible to a contribution.


Like the project ? Open a PR ! :)

## Demo

Search command and output in the terminal.

![demo](demo/demo.gif)

*Made with asciinema and agg*

## Build
```
cargo build --release
```

## Update the project

Since the default notes are taken from Arsenal's public repository a simple copy of the cheats directory is used to update the repository.

Any suggestion for a simpler & automated workflow is appreciated.

## Documentation

To do.

## Contribution

Any contribution is welcome. Be it documentation, code changes and especially commands.
### Code contribution ideas
- Allow for the completion of a command within a box prompting for input after a command has been selected
- Windows's Defender evasion : the binary is flagged a malicious as it contains malicious commands. Loading encrypted notes at compile-time and decrypting them at runtime would be a way to trivial detection.
- Not every fields (based on a Arsenal cheatsheet) appear in the tool because I lacked the time to fully display everything.

### Cheatsheets
- Add additional commands.
  - For adding commands, a template is present inside cheats/arsenal-cheats folder.
  - Multiple commands in a code block is supported.
