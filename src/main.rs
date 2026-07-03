fn main() {
    use os_info;

    let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let info = os_info::get();
    let architecture = info.architecture().unwrap_or("");
    let edition = match info.edition() {
        Some(e) if !e.is_empty() => format!("{e} "),
        _ => String::new(),
    };
    let codename = match info.codename() {
        Some(c) if !c.is_empty() => format!("{c} "),
        _ => String::new(),
    };

    if !verbose {
        // Print full information:
        println!("{info} {edition}{codename}{architecture}");
    } else {
        // Print information separately:
        println!("Type: {}", info.os_type());
        println!("Version: {}", info.version());
        if !edition.is_empty() {
            println!("Edition: {edition}");
        }
        if !codename.is_empty() {
            println!("Codename: {codename}");
        }
        println!("Bitness: {}", info.bitness());
        if !architecture.is_empty() {
            println!("Architecture: {architecture}");
        }
    }
}
