# Silence

## Build dependencies

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have rustup, you can install it like this:

- Linux and OSX:
	```bash
	$ curl https://sh.rustup.rs -sSf | sh
	```

- Windows

    Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the rustup installer from
	https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the msvc toolchain:
    ```
	$ rustup default stable-x86_64-pc-windows-msvc
    ```
