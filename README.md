## Mining Client

Welcome to the Off Blockway mining client!

### Usage

The mining client handles a lot of the functionality for you, just boot up the CLI from the parent directory using `cargo run` and follow the directions to mine!

The way the CLI gets the user's IP is through `ipconfig getifaddr en0`, if your operating system doesn't support this command then just replace it with the system call that finds the host's IP address.

The post request to the client is made with the IP I was on when testing the functionality. Change this out to your IP when using the exectuable. 