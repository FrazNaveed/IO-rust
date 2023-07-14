# I/O Program

This is a simple I/O program implemented in Rust. The program performs basic input and output operations using the standard input and output streams.

## Prerequisites

To run this program, you need to have Rust and Cargo installed on your system. If you don't have Rust and Cargo installed, you can follow the official Rust installation guide at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

1. Clone the repository:

2. Navigate to the project directory:

3. Build the project using Cargo:

4. Run the program:

## Features

1. The program utilizes command line env arguments to specify the query string and the file path for the text search operation. This allows users to easily provide inputs and interact with the program.

2. The program offers a configuration option to ignore case sensitivity during the search. By setting the IGNORE_CASE env variable users can control whether the search should be case sensitive or case-insensitive.

3. The core functionality of the program is the text search operation. It reads the contents of the specified file and performs a search for the query string.

4. The program displays the matching lines that satisfy the search criteria. Each line is printed to the console.
