# oauth2-rust-aws-lambda
OAuth 2.0 Server implementation in Rust

Dieses Projekt wird unter Windows mit Visual Studio Code entwickelt.

## Special notes

The OAuth client ID cannot contain ':' characters. Because of a limitation of the HTTP Basic Authentication Base64 encoding process.

## Visual Studio Code setup

Installed Extension: <br>
rust-analyzer by rust-lang.org

## Installing Cargo Lambda

See: [Cargo Lambda installation guide](https://www.cargo-lambda.info/guide/installation.html)

Attention: The console inputs and outputs were created under Windows with PowerShell.

### Installing Git

I installed Git with Cygwin and added `C:\cygwin64\bin` to the system PATH so that Git can be run in PowerShell. <br>
Restart the system to take the changed PATH variable into account.

### Installing Scoop

See: [Scoop Homepage with Quickstart](https://scoop.sh/)

    > Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
    > irm get.scoop.sh | iex
    Initializing...
    Downloading ...
    Extracting...
    Creating shim...
    Adding ~\scoop\shims to your path.
    Scoop was installed successfully!
    Type 'scoop help' for instructions.

Restart the system to take the changed PATH variable into account.

### Installing Cargo Lambda using Scoop

    > scoop bucket add cargo-lambda https://github.com/cargo-lambda/scoop-cargo-lambda
    Checking repo... OK
    The cargo-lambda bucket was added successfully.
    > scoop install cargo-lambda/cargo-lambda
    Installing 'zig' (0.11.0) [64bit] from main bucket
    zig-windows-x86_64-0.11.0.zip (73,6 MB)     [=========================================] 100%
    Checking hash of zig-windows-x86_64-0.11.0.zip ... ok.
    Extracting zig-windows-x86_64-0.11.0.zip ... done.
    Linking ~\scoop\apps\zig\current => ~\scoop\apps\zig\0.11.0
    Creating shim for 'zig'.
    'zig' (0.11.0) was installed successfully!
    Installing 'cargo-lambda' (1.0.0) [64bit] from cargo-lambda bucket
    cargo-lambda-v1.0.0.windows-x64.zip (10,2 MB)     [===================================] 100%
    Checking hash of cargo-lambda-v1.0.0.windows-x64.zip ... ok.
    Extracting cargo-lambda-v1.0.0.windows-x64.zip ... done.
    Linking ~\scoop\apps\cargo-lambda\current => ~\scoop\apps\cargo-lambda\1.0.0
    Creating shim for 'cargo-lambda'.
    'cargo-lambda' (1.0.0) was installed successfully!
    'zig' suggests installing 'extras/vcredist2022'.

## Create Lambda with Cargo Lambda

See: [Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)

    > cd token
    > cargo lambda new oauth_token
