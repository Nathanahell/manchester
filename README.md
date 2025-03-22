# Manchester

## About

Manchester is a cross-platform command-line memo one can use to quickly find a system command.
It is designed for portability, ease of use, and can be customized to suit your needs.
The main features of the tool are the following :
- **Portability**: the project is cross-platfrom as it can be built for different targets.
- **Does not require internet access**: your notes are embedded within the binary, thereforce no internet access or install is required after building the porject. Ideal for constrained environment.
- **Ease of use**

This project was heavily inspired by OCD's [Arsenal](https://github.com/Orange-Cyberdefense/arsenal) tool. Shoutout to them !

## Build
```
cargo build --release
```

## Update the project

Since the default notes are token from Arsenal's public repository the following workaround can be used to pull its changes into the project.

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

## Documentation

To do.

## Contribution

Any contribution is welcome. Be it documentation, code changes and especially commands.
Right now the focus would be to :
- Add additional commands.
- Allow for the completion of a command within a box prompting for input after a command has been selected
- Windows's Defender evasion if the binary is flagged a malicious since it may contains malicious commands. Maybe through the encryption of the embedded files at rest ?

For adding commands, a template is present inside cheats/arsenal-cheats folder. Should every tag feature be implemented in the future, everything written in the commands description should appear.

## The author's note

This is my first ever open-source project. I took a chance to practice Rust and to fiddle with the Ratatui library.
I am open for any improvements in my code as I am not quite a software developper yet. Any constructive feedback is much appreciated. :)
