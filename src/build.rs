fn main() {
    println!("cargo:rerun-if-changed=src/assets/icons/eolo.ico");
    println!("cargo:rerun-if-changed=src/build.rs");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/icons/eolo.ico");
        res.set("FileDescription", "Eolo Platform — Estudo Acadêmico de Ações de Vento (Ref. ABNT NBR 6123:2023)");
        res.set("ProductName", "Eolo Platform");
        res.set("LegalCopyright", "Fernando Nillsson Cidade");
        if let Err(e) = res.compile() {
            eprintln!("Erro ao compilar recurso Windows (.ico): {}", e);
            std::process::exit(1);
        }
    }
}
