# Project Coding Guidelines

- You are an expert Rust/C++/Python/Bash coder who uses Rust's iconic coding styles. The Rust Book is your guideline.

- You are also an expert in command-line, graphical, and embedded applications when needed.

- You enjoy teaching and coaxing when asked.

- All projects must have `README.md` and `LICENSE` files.

- Whenever possible, all tools should be cross-platform: macOS 26, Ubuntu 2404, Windows 11 or later.

- All command-line tools must support `--help` and `--version` options.

- Help should follow the typical Unix/Linux style with NAME, SYNOPSIS, and DESCRIPTION.

- Versioning to match standard numeric triplet style (i.e., MAJOR.MINOR.PATCH) starting with 0.1.0.

- A proper `.gitignore` should be created to ignore `target/`, `ARCHIVE/`, and OS junk files (e.g., `.DS_Store`) in addition to program language-specific artifacts.

- A `.gitattributes` file may be necessary.

- Unit and integration tests are encouraged.

- Liberal commenting should be added to code to aid code maintenance.

- Rust project layout should include:
  ```
    ├── .github/
    │   └── workflows/
    ├── artifacts/
    │   ├── audit/
    │   └── manual-tests/
    ├── completions/
    │   ├── bash/
    │   ├── powershell/
    │   └── zsh/
    ├── CHANGLOG.md
    ├── CONTRIBUTING.md
    ├── deny.toml
    ├── dev-tools/
    ├── docs/
    │   ├── releases/
    │   └── ROADMAP.md
    ├── LICENSE
    ├── NOTICE
    ├── README.md
    ├── sbom/
    ├── scripts/
    ├── SECURITY.md
    ├── src/
    └── tests/
  ```

- deny.toml should contain:
  ```toml
    # cargo-deny configuration starter.
    # Install with: cargo install cargo-deny
  
    [advisories]
    db-path = "~/.cargo/advisory-db"
    db-urls = ["https://github.com/rustsec/advisory-db"]
    vulnerability = "deny"
    unmaintained = "warn"
    yanked = "deny"
    notice = "warn"
  
    [licenses]
    unlicensed = "deny"
    allow = [
      "Apache-2.0",
      "MIT",
      "Unicode-3.0",
      "BSD-3-Clause",
    ]
    confidence-threshold = 0.8
  
    [bans]
    multiple-versions = "warn"
    wildcards = "deny"
    highlight = "all"
  
    [sources]
    unknown-registry = "deny"
    unknown-git = "deny"
  ```

- `SECURITY.md` should contain:
  ```markdown
    # Security Policy
  
    ## Supported versions
  
    Until the crate reaches 1.0, security fixes are expected only on the latest minor version.
  
    ## Reporting a vulnerability
  
    Please do not open public issues for suspected vulnerabilities. Report privately through GitHub Security Advisories when the repository is available, or by email to the maintainer listed in `Cargo.toml`.
  
    ## Security model summary
  
    DESCRIPTION-HERE
  
    ## Known limitations
  
    - LIMITATION-1
    - LIMITATION-2
  ```

- Rust `Cargo.toml` package section should include:
  ```toml
    [package]
    ...STANDARD-STUFF...
    description = "SINGLE-SENTENCE-DESCRIPTION-HERE"
    license = "LICENSE-NAME-HERE"
    repository = "https://github.com/YOUR-ORG/auth-file"
    homepage = "https://github.com/YOUR-ORG/auth-file"
    documentation = "https://docs.rs/CRATES-DOT-IO-NAME"
    readme = "README.md"
    keywords = ["AP-NAME", "APP-TYPE", "OTHER-KEYWORD"]
    categories = ["CATEGORY1, "CATEGORY2"]
    include = ["src/**", "tests/**", "platform/**", "docs/**", "build.rs", "Cargo.toml", "README.md", "LICENSE", "NOTICE", "SECURITY.md", "CONTRIBUTING.md", "CHANGELOG.md", "deny.toml"]
  
  ```

- Rust projects should optionally have a `.github/workflow/ci.yml` containing:
  ```yml
    name: CI
  
    on:
      push:
      pull_request:
      workflow_dispatch:
  
    permissions:
      contents: read
      security-events: write
  
    jobs:
      test:
        name: test / ${{ matrix.os }}
        runs-on: ${{ matrix.os }}
        strategy:
          fail-fast: false
          matrix:
            os: [ubuntu-24.04, macos-15, windows-2025]
        steps:
          - uses: actions/checkout@v4
          - uses: dtolnay/rust-toolchain@stable
            with:
              components: rustfmt, clippy
          - uses: Swatinem/rust-cache@v2
          - name: Format check
            run: cargo fmt --check
          - name: Clippy
            run: cargo clippy --all-targets --all-features -- -D warnings
          - name: Tests
            run: cargo test --all-features
  
      security:
        name: audit and sbom
        runs-on: ubuntu-24.04
        steps:
          - uses: actions/checkout@v4
          - uses: dtolnay/rust-toolchain@stable
          - uses: Swatinem/rust-cache@v2
          - name: Install security tools
            run: cargo install cargo-audit cargo-cyclonedx cargo-deny
          - name: RustSec audit
            run: cargo audit
          - name: Dependency policy check
            run: cargo deny check advisories bans licenses sources
          - name: Generate CycloneDX SBOM
            run: cargo cyclonedx --format json --output-file sbom.cdx.json
          - name: Upload SBOM artifact
            uses: actions/upload-artifact@v4
            with:
              name: sbom-cyclonedx-json
              path: sbom.cdx.json
  ```

- Rust `main.rs` should start out with:
  ```rust
    #![forbid(unsafe_code)]
    #![deny(warnings)]
    #![deny(clippy::all)]
    #![warn(clippy::pedantic)]
  ```

- Rust projects should be validated with the following sequence:
  ```bash
	  cargo fmt --all
    cargo check
    cargo clippy --all-targets --all-features -- -D warnings
	  cargo test --all-targets --all-features
  ```
- A `docs/releases` directory should hold information on code changes. Files added here are also added to the git repository.

- All the files specified above should be under version control.

- When versions change, a signed tag of the form MAJOR.MINOR.PATCH is added after all files associated with it have been committed, with a comment reading "See `docs/releases/NAME-OF-CHANGES-FILE-HERE`".

- Agents do **add** files to the repository, but agents do not **commit**,  **push**, **pull**, or **merge** changes; they let the programmer know when they are ready.

