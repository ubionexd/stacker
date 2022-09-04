# stacker
Little CLI utility to calculate how many Minecraft stacks are the given amount of items.

# How to use:
### Download precompiled executables (MacOS, Windows, Linux - x86-64)
- Go to Actions
- Select the latest successfull build (the last one is on top, and the successfulls have a green check mark)
- Scroll down and find the package for your operating system. (MacOS builds are not compatible with ARM (Apple Silicon: M1, M2) yet)
- Extract it, open terminal in the same folder, then run `./stacker`
- Add it to PATH, if you want to access it from anywhere just using `stacker`

### Compile from source (any platform listed [here](https://doc.rust-lang.org/nightly/rustc/platform-support.html))
- Install Rust to your local machine!
- Clone repo: `git clone https://github.com/ubionexd/stacker`
- Build it: `cargo build --release`
- Your built executable can be found at `./target/release`!
