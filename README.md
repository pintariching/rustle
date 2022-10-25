# Rustle

The Svelte compiler, rewritten in Rust.

## Description

This projects aims to make `Svelte` usable without `Node.js` and make the compiler _blazingly fast_.

## Work in progress

This is still a big work in progress. Only a few parts of Svelte are working now and the CLI tool still needs some work.

# Getting started

### Installation

To install with cargo, run `cargo install rustle_cli --version "0.0.2-alpha"` to install the alpha version of the CLI.

### Using railwind_cli

Run `rustle_cli app.rustle` to generate an `app.js` file. You can optionally specify a different output file with the `-o` flag.

You can also specify a directory for example `rustle_cli src` to parse all the files in that directory.

For debugging, you can print the generated AST with the `-a` or `--ast` flag and pretty print it with `-p`.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

* A big thank you to [lihautan](https://www.youtube.com/c/lihautan) on Youtube, for making the video [Build your own Svelte](https://www.youtube.com/watch?v=mwvyKGw2CzU) which helped me a lot in understanding how the Svelte compiler actually works!
