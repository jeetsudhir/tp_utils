# Teleport Utility (tp_utils)

## Description
`tp_utils` is a command-line tool designed to facilitate easy switching to different directories within the terminal. It allows users to set, list, delete, and move to predefined points in the file system.

## Features
- **Set Teleport Point**: Set a new teleport point with a given name.
- **List Teleport Points**: View a list of all available teleport points.
- **Delete Teleport Point**: Remove a specific teleport point or delete all teleport points.
- **Teleport**: Instantly move to a predefined teleport point in the file system.

## Installation
To use `tp_utils`, follow these steps:
1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo build --release` to build the executable.
4. Optionally, you can run `cargo install --path .` to install the utility globally.

## Usage
### Setting a Teleport Point
```tp_utils set <name>```
Example:
```tp_utils set "work"```

### Listing Teleport Points
```tp_utils list```

### Deleting Teleport Points
To delete a specific teleport point:
```tp_utils delete <name>```
To delete all teleport points:
```tp_utils delete --all```

### Teleporting
```tp_utils tp <name>```
Example:
```tp_utils tp "work"```
