# Port Scanner in Rust

This is a simple port scanner implemented in Rust using multi-threading to scan for open ports on a specified IP address.

## Overview

The port scanner utilizes Rust's standard library modules like `std::io`, `std::net`, and `std::sync` to establish TCP connections and scan ports concurrently.

## Usage

To use this port scanner, follow these steps:

1. **Clone the Repository:** Clone this repository to your local machine.
2. **Compile the Code:** Use the Rust compiler to build the code.
    ```bash
    $ rustc main.rs
    ```
3. **Run the Scanner:** Execute the compiled binary with the following command:
    ```bash
    $ ./main <IP_ADDRESS> [-j THREADS]
    ```
    - `<IP_ADDRESS>`: Provide the target IP address to scan for open ports.
    - `[-j THREADS]`: Optionally, use the `-j` flag followed by the number of threads for concurrent scanning. If not provided, the default thread count is 4.

    **Example:**
    ```bash
    $ ./main 127.0.0.1 -j 8
    ```
4. **View Results:** The program will display a list of open ports on the specified IP address.

## Additional Notes

- The port scanner uses TCP connections to determine if a port is open.
- It employs multi-threading to speed up the scanning process by distributing the workload across multiple threads.
- Error handling is implemented for various scenarios, such as incorrect command-line arguments or failed port connections.

## Contribution

Feel free to contribute by forking this repository, making improvements, and creating pull requests. If you encounter any issues or have suggestions, please open an issue.

## License

This project is licensed under the [MIT License](LICENSE).
