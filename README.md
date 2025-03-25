# Manchester

## About

Manchester is a cross-platform command-line memo one can use to quickly find a system command.
It is designed for portability, ease of use, and can be customized to suit your needs.
The main features of the tool are the following :
- **Portability**: the project is cross-platfrom as it can be built for different targets. Take the release binary and you are all set ! No set-up required !
- **Does not require internet access**: your notes are embedded within the binary, thereforce no internet access or install is required after building the porject. Ideal for constrained environment.
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

Since the default notes are token from Arsenal's public repository the following workaround is used to pull its changes into the project.

- To add a remote repository
```
# add the repository you want to embed as a remote to the Git project.
git remote add -f arsenal-repo https://github.com/Orange-Cyberdefense/arsenal.git

## pull in the specific folder you want from the remote repository using git subtree
git subtree add --prefix=<path/to/your/folder> repo_name branch_name path/to/folder
```

- To update the subtree
```
# pull the latest changes from the specified folder and update your project.
git subtree pull --prefix=path/to/your/folder repo_name branch_name --squash
```

If you have any idea for an easier updating of the arsenal-cheats/ repository. I'm all ears.

## Documentation

To do.

## Contribution

Any contribution is welcome. Be it documentation, code changes and especially commands.
### Code contribution
Right now the focus would be to :
- Allow for the completion of a command within a box prompting for input after a command has been selected
- Windows's Defender evasion if the binary is flagged a malicious since it may contains malicious commands. Maybe through the encryption of the embedded files at rest ?
- Not every fields (based on a Arsenal cheatsheet) appear in the tool because I lacked the time to fully display everything.

### Cheatsheets
- Add additional commands.
  - For adding commands, a template is present inside cheats/arsenal-cheats folder.
  - C-style inline comment (use of `//`) is preferred so as not to interfere with the parsing of the markdown headers using `#`
