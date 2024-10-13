# rusfind

**rusfind** is a simple command-line tool written in Rust that functions similarly to the Unix/Linux `find` command. It allows you to search for files and directories based on name patterns and file types. Additionally, it leverages lazy iteration to only load the next directory or file when necessary, reducing memory usage. It also includes metadata caching to avoid redundant file system calls, improving performance. Moreover, the tool uses parallelism with the Rayon library to speed up searches in large directories by processing multiple files simultaneously using multiple threads.

## Features

- Search for files or directories using a specified name pattern.
- Filter results by file type (files or directories).
- Leverage parallelism with customizable thread count to speed up the search process.

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/rusfind.git
    cd rusfind
    ```

2. **Build the project**:
   You need to have Rust installed. You can install Rust [here](https://www.rust-lang.org/tools/install).

   After that, run the following command to build the project:
    ```bash
    cargo build --release
    ```

3. **Run the tool**:
   After building, you can run the tool from the `target/release` directory:
    ```bash
    ./target/release/rusfind --help
    ```

## Usage

You can use `rusfind` to search for files and directories in a specific path with various options.

### Basic Commands

- **Search for a file by name**:
    ```bash
    rusfind -p /path/to/directory -n "filename"
    ```

- **Search for a directory by name**:
    ```bash
    rusfind -p /path/to/directory -n "dirname" -t d
    ```

- **Specify the number of threads for parallelism**:
    ```bash
    rusfind -p /path/to/directory -n "filename" -t f -r 4
    ```

### Options

| Option              | Short  | Default | Description                                                    |
|---------------------|--------|---------|----------------------------------------------------------------|
| `--path`            | `-p`   | `.`     | The directory path to start searching in (defaults to current directory). |
| `--name`            | `-n`   | `None`  | The name or pattern to search for.                             |
| `--f_type`          | `-t`   | `None`  | Specify 'f' to search for files, 'd' for directories.          |
| `--threads`         | `-r`   | `2`     | The number of threads to use for parallelism.                  |
| `--help`            | `-h`   |         | Show help information.                                         |

### Examples

1. **Find all files with `.txt` extension**:
    ```bash
    rusfind -p /home/user -n ".txt" -t f
    ```

2. **Find all directories with the name "backup"**:
    ```bash
    rusfind -p /home/user -n "backup" -t d
    ```

3. **Search with 4 threads for faster performance**:
    ```bash
    rusfind -p /var/log -n ".log" -t f -r 4
    ```

## How it Works

- **Breadth-First Search**: The tool uses a breadth-first search (BFS) algorithm to traverse the directory tree and find the files or directories that match the provided pattern and type.

- **Parallelism with Rayon**: By leveraging Rayon, the tool processes directory entries in parallel, significantly speeding up the search process in large directories.

## Contribution

Feel free to submit pull requests or open issues to contribute to the project. You can fork the repository, create your feature branch, and submit a pull request when you're ready.

1. Fork the repository.
2. Create your feature branch:
    ```bash
    git checkout -b feature/new-feature
    ```
3. Commit your changes:
    ```bash
    git commit -m 'Add new feature'
    ```
4. Push to the branch:
    ```bash
    git push origin feature/new-feature
    ```
5. Open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

* Raphael Benzi - [raphael_benzi@hotmail.com](mailto:raphael_benzi@hotmail.com)
