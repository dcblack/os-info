/// Prints the help message in a Unix-style format.
fn print_help() {
    println!(
        r#"NAME
    os-info - print information about the current operating system

SYNOPSIS
    os-info [OPTIONS]

DESCRIPTION
    Prints information about the current operating system.

    -v, --verbose
        Print details on separate lines.

    -h, --help
        Display this help and exit.

    --version
        Output version information and exit."#
    );
}

fn main() {
    // Collect command-line arguments.
    let args: Vec<String> = std::env::args().collect();

    // Check for help flag.
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    // Check for version flag.
    if args.iter().any(|arg| arg == "--version") {
        println!("os-info {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Check for verbose flag.
    let verbose = args.iter().any(|arg| arg == "--verbose" || arg == "-v");
    
    // Get OS information using the os_info crate.
    let info = os_info::get();
    
    // Extract architecture information, defaulting to an empty string if not found.
    let architecture = info.architecture().unwrap_or("");
    
    // Extract edition information, formatting it with a trailing space if present.
    let edition = match info.edition() {
        Some(e) if !e.is_empty() => format!("{e} "),
        _ => String::new(),
    };
    
    // Extract codename information, formatting it with a trailing space if present.
    let codename = match info.codename() {
        Some(c) if !c.is_empty() => format!("{c} "),
        _ => String::new(),
    };

    if !verbose {
        // Print full information in a single line summary.
        let summary = format!("{info} {edition}{codename}{architecture}");
        println!("{}", summary.trim());
    } else {
        // Print information separately on multiple lines.
        println!("Type: {}", info.os_type());
        println!("Version: {}", info.version());
        if !edition.is_empty() {
            println!("Edition: {}", edition.trim());
        }
        if !codename.is_empty() {
            println!("Codename: {}", codename.trim());
        }
        println!("Bitness: {}", info.bitness());
        if !architecture.is_empty() {
            println!("Architecture: {architecture}");
        }
    }
}
