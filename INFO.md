# Debug : Cli-log
While setting up cli-log, app name used in derive the env var name giving the log level :
I named my app "MANCHESTER_APP", the env var to set is "MANCHESTER_APP".

One shot : $env:MANCHESTER_APP_LOG="debug"; cargo test
For powershell session : Set-Item -Path env:MANCHESTER_APP_LOG -Value "debug"


# Features
- Tags abbreviation : 

        tags_dict = {'target/local': 'Loc',
                     'target/remote': 'Rem',
                     'target/serve': 'Ser',
                     'plateform/linux': '[L] ',
                     'plateform/windows': '[W] ',
                     'plateform/mac': '[M] ',
                     'plateform/multiple': '[*] '}


# Clean-up job to do
- Remove unecessary Block widget used to visualize spaces for debugging
    - Add the instanciation of styled block using .block(..) methods on widget structures
- Remove UNEEDED

# Run test, do not capture stdout output
cargo test test_include_dir -- --nocapture