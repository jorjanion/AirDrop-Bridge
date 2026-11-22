fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        res.set(
            "FileDescription",
            "AirDrop Bridge - Native Apple Transfer Bridge",
        );
        res.set("ProductName", "AirDrop Bridge");
        res.set("OriginalFilename", "airdrop-bridge.exe");
        res.set(
            "LegalCopyright",
            "Copyright (c) 2026 AirDrop Bridge Contributors",
        );
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {}", e);
        }
    }
}
