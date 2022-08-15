# stacker
Little CLI utility to calculate how many Minecraft stacks are the given amount of items.

# How to use:
### Download precompiled executables (MacOS - ARM, Windows, Linux - x86-64)
- Go to Actions
- Select the latest successfull build (the last one is on top, and the successfulls have a green check mark)
- Scroll down and find the package for your operating system. (macos build are for ARM (Apple Silicon compatable) and linux and windows builds are x86-64)
- Extract it, open terminal in the same folder, then run `./stacker`
- Add it to PATH, if you want to access it from anywhere just using `stacker`

### Compile from source (any platform listed [here](https://doc.rust-lang.org/nightly/rustc/platform-support.html))
- Install Rust to your local machine!
- Clone repo: `git clone https://github.com/ubionexd/stacker`
- Build it: `cargo build --release`
- Your built executable can be found at `./target/release`!
