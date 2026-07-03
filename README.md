# os-info

`os-info` is a small Rust command-line app that prints information about the current operating system.

By default, it shows a single summary line with the OS name, version, edition, codename, architecture, and kernel version when available.

Use `--verbose` or `-v` to print the details on separate lines, including the OS type, version, kernel version, edition, codename, bitness, and architecture.

Use `--help` or `-h` to obtain help.

Use `--version` for the current version.

## Testing

Run the integration tests using Cargo:

```bash
cargo test
```

