fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        res.set(
            "FileDescription",
            "Drop Bridge - Native Apple Transfer Bridge",
        );
        res.set("ProductName", "Drop Bridge");
        res.set("OriginalFilename", "drop-bridge.exe");
        res.set(
            "LegalCopyright",
            "Copyright (c) 2026 Drop Bridge Contributors",
        );
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {}", e);
        }
    }
}
