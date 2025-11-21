fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("ProductName", "Dell XE9680 GPU Mapper");
        res.set("FileDescription", "Dell XE9680 GPU Mapping Query Tool");
        res.set("CompanyName", "Wharton Wang");
        res.set("LegalCopyright", "Copyright (c) 2024 Wharton Wang");
        res.set("OriginalFilename", "xe9680-gpu-mapper.exe");
        res.compile().unwrap();
    }
}
