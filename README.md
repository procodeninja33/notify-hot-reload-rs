# Notify Hot Reload

This project automatically reloads the configuration file if there are any changes made to it, without needing to restart the binary. To achieve this functionality, we use the notify crate.

## How It Works

The [notify](https://crates.io/crates/notify) crate is used to monitor the filesystem for changes. When a change is detected in the configuration file, the application reloads the configuration without needing a restart. This is particularly useful for applications that require real-time configuration updates.

## Usage

To run the project, use the following command:

```
cargo run --release
```
