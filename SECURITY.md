# Security Policy

## Supported versions

Until the crate reaches 1.0, security fixes are expected only on the latest minor version.

## Reporting a vulnerability

Please do not open public issues for suspected vulnerabilities. Report privately through GitHub Security Advisories when the repository is available, or by email to the maintainer listed in `Cargo.toml`.

## Security model summary

`os-info` is a command-line tool that reads system information using standard system APIs. It does not require elevated privileges and does not process external network data.

## Known limitations

- Only supports standard operating systems.
- Accuracy depends on the underlying crates (`os_info` and `sysinfo`).
