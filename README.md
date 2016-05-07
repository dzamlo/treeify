# treeify

treeify converts the output of a command that lists files into a tree representation similar to the output of the command tree.

## Installation

You can install the latest published version using cargo with the following command:
```bash
cargo install treeify
```

You can also install the latest version from the git repository:
```bash
cargo install --git https://github.com/dzamlo/treeify.git
```

## Usage

Simply pipe the output of a command to treeify. For example:
```bash
find ~ -name '*.rs' | treeify
```

To prevent issue with filename containing new line characters, you can use the -0 or --null option:
```bash
find ~ -name '*.rs' -print0 | treeify --null
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
