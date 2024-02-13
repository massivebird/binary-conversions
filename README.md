# Garrett's binary conversion program

This command line application converts decimal values to and from the following 8-bit binary notations:

1. Signed magnitude
2. Ones complement
3. Twos complement
4. Excess-128

## Building

Run this project online [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7cb15b7883b17549703e002251e8207b)!

Click `Run` at the top of the page to run the program. Input can be provided at the bottom of the "Execution" pane.

> Link last updated: 12 February 2023 @ 19:22

> [!NOTE]
> Code present in the Rust Playground link belongs to a subset of the actual source code; functions not critical to the interactive mode have been removed in order to improve readability.

### Building manually

To manually build this project, you must first install [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/downloads).

Once you have Rust installed, run the following commands:

```bash
git clone https://github.com/massivebird/binary-conversions
cd binary-conversions
cargo run -- --interactive
```
