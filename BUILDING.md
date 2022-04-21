# Compiling microlaunch

- **Clone the submodules for this repository.** `git submodule init && git submodule update --recursive`. You need this for *steamworks-rs*, which is what is used to connect to Steam.
- Install the latest stable version of [the Rust programming language](https://rust-lang.org) (version `1.60.0` at this time)
- Acquire [the Steamworks SDK](https://partner.steamgames.com/doc/sdk)
- Point environment variable `STEAM_SDK_LOCATION` to the folder where you have the Steamworks SDK
- Run `cargo b`