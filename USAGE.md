# microlaunch Usage Guide

## Step 1: [Compile the program](/BUILDING.md)

## Step 2: Configuration
*I'm assuming you are on a Linux system.*
- Create a directory at `~/.config/microlaunch`. (`mkdir ~/.config/microlaunch`)
- Copy `config_examples/proton_example.toml` to `~/.config/microlaunch` as `microlaunch.toml` (`cp config_examples/proton_example.toml ~/.config/microlaunch/microlaunch.toml`).
- Edit `microlaunch.toml` and make sure its paths point to the correct locations. Of note is `game_path` - it is used to check for integrity and is required by Square Enix to log in.
- Ensure you have a working Proton installation (with the `proton` binary) in the folder pointed to by `proton_root_path`.
- Double-check your work.

## Step 3: Launching
- Run `microlaunch --gui` (or `cargo r -p microlaunch -- --gui`). The client should launch, and its graphical interface should appear.
- Enter your Square Enix username, password and optionally one-time password. Make sure to select your correct platform and account type.
- Optionally, check the "Save information" box **and read the disclaimer.**
- Click "Log in".
- It will take a bit - the launcher is not frozen. You will receive diagnostic info in the terminal, **but this contains sensitive information such as login tokens, so** ***do not share it!***
- Ensure everything is correct - ***do not share your Unique Patch ID, as this can be used to log in as you*** - and click "Launch game!"
- Wait a bit - FINAL FANTASY XIV should start.

## Step 4 (optional): Automatic login
- Once you have logged in at least once with "Save information" checked, the program will store your username and password *in encrypted form* at `~/.config/microlaunch/sensitive_data.enc`.
- To automatically log in, simply run `microlaunch` (or `cargo r -p microlaunch --`).
- To get back to the GUI, run `microlaunch --gui` (or `cargo r -p microlaunch -- --gui`).
- You will see a bunch of terminal output, and after a while, FINAL FANTASY XIV should start!