![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.72.1-orange.svg)

# Super Fast CSV Splitter

This is a Rust program that splits a CSV file into multiple files. It is designed to be as fast as possible.

This is also a work in progress, so your mileage may vary.

## Installation

```bash
$ cargo install csv-splitter
```

or:
    
```bash
$ git clone git@github.com:patterns-complexity/csv-splitter
$ cd csv-splitter
$ cargo build --release
$ cd ./target/release/
$ ./csv-splitter -h
```

## Usage and help

```bash
$ csv-splitter -i <input_directory> -o <output_directory> -g <granularity>
```

or:

```bash
$ csv-splitter --input <input_directory> --output <output_directory> --granularity <granularity>
```

or (with AMQP):

```bash
$ csv-splitter -i <input_directory> -o <output_directory> -g <granularity> -q <queue_name> -u <amqp_url>
```

or (with AMQP):

```bash
$ csv-splitter --input <input_directory> --output <output_directory> --granularity <granularity> --queue_name <queue_name> --amqp_url <amqp_url>
```

help:

```bash
$ csv-splitter -h
```

## Arguments
| Short form | Long form | Description | Required |
| --- | --- | --- | --- |
| -h | --help | Prints help information | No |
| -i | --input | The directory containing the CSV files to split | Yes |
| -o | --output | The directory to output the chunks to | Yes |
| -g | --granularity | Chunk line count | Yes |
| -q | --queue_name | AMQP queue name, needs -u if supplied | No |
| -u | --amqp_url | The URI of the AMQP queue (amqp://user:password@host:port) | No |

## Example
```bash
$ csv-splitter -i ./input -o ./output -g 500
```

## Example with AMQP
```bash
$ csv-splitter -i ./input -o ./output -g 500 -q my_queue -u amqp://user:password@host:port
```

## License
[MIT License]('https://opensource.org/license/mit/)

## Author
[Patterns Complexity](https://github.com/patterns-complexity)