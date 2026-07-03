fn main() {
    use os_info;

    let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
    let info = os_info::get();
    let arch = info.architecture().unwrap_or("");
    let edtn = match info.edition() {
        Some(e) if !e.is_empty() => format!("{e} "),
        _ => String::new(),
    };
    let name = match info.codename() {
        Some(c) if !c.is_empty() => format!("{c} "),
        _ => String::new(),
    };

    if !verbose {
        // Print full information:
        println!("OS information: {info} {edtn}{name}{arch}");
    }
    else {
        // Print information separately:
        println!("Type: {}", info.os_type());
        println!("Version: {}", info.version());
        println!("Edition: {:?}", info.edition());
        println!("Codename: {:?}", info.codename());
        println!("Bitness: {}", info.bitness());
        println!("Architecture: {}", info.architecture().unwrap_or(""));
    }
}
