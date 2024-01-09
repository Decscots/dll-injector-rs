# Rust DLL Injector

## Folders

### `dll_injector`

- The actual DLL injector using `LoadLibrary`

### `test_dll`

- An example DLL used for testing and demonstration purposes.

## Prerequisites

- [Rustup](<https://rustup.rs/>) (only needed for building, you can just download the binaries from the [releases page](<https://github.com/Decscots/dll-injector-rs/releases>)
- A Windows computer

## Building

1. Clone the repository

    ```bash
    git clone https://github.com/Decscots/dll-injector-rs.git
    ```

2. Navigate to the `dll_injector` directory

    ```bash
    cd dll-injector-rs/dll_injector
    ```

3. Build the `dll_injector` project

    ```bash
    cargo build --release
    ```
## Usage
1. Running the dll injector

    ```bash
    dll_injector.exe <target_process_name> <path_to_dll>
    ```

    Replace `<target_process_name>` with the name of the process you want to inject the DLL into (case insensitive), and `<path_to_dll>` with the path to the DLL you want to inject.

