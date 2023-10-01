# Regno

A project for handling vehicle registration numbers and car information.

## Project Structure

```
regno
├── src
│   ├── biluppgifter
│   │   └── types.rs  // Types for car information
│   ├── biluppgifter.rs // Handles car information
│   ├── main.rs // Main program
│   └── utils.rs // Utility functions
└── README.md
```

## Prerequisites

- Rust 1.52 or higher

## Installation

1. Clone this repository:

    ```
    git clone https://github.com/your-username/regno.git
    ```

2. Build the project:

    ```bash
    cd regno
    cargo build
    ```

## Usage

Run the program with:

```bash
cargo run
```

## Features

- `biluppgifter/types.rs`: Contains the types used for describing car information.
- `biluppgifter.rs`: Performs operations for fetching and processing car information.
- `main.rs`: The main program where everything runs.
- `utils.rs`: Helper functions and utilities used throughout the project.

## Contributing

If you'd like to contribute, please create a Pull Request.

## License

This project is licensed under the MIT License.