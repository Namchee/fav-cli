# Favify

A command-line application version of [Fav](https://github.com/Namchee/fav). Generate a complete favicon set for your web applications with the desired target platforms.

## Installation

To install this project locally on your machine, execute this command on your terminal

```shell
cargo install favify
```

## Usage

```shell
Usage: favify.exe [OPTIONS] <source_image>

Arguments:
  <source_image>  Path to the source image

Options:
  -p, --platforms <platforms>  Platforms that should be supported [possible values: web, modern, android, apple]
  -o, --output <output>        Output folder destination, will be created if it does not exist
  -t, --template               Generate a quick-start HTML template
  -h, --help                   Print help information
  -V, --version                Print version information
```

## License

This project is licensed under the [MIT License](./LICENSE)