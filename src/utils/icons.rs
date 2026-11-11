use super::paths::resolver_caminho_asset;
use std::path::PathBuf;
use std::sync::OnceLock;

static ICON_DATA_CACHE: OnceLock<Option<egui::IconData>> = OnceLock::new();

/// Retorna o caminho do ícone caso ele exista em disco.
pub fn obter_caminho_icone(nome_icone: &str) -> Option<PathBuf> {
    if let Some(p) = resolver_caminho_asset(nome_icone) {
        return Some(p);
    }
    let in_icons = format!("icons/{}", nome_icone);
    if let Some(p) = resolver_caminho_asset(&in_icons) {
        return Some(p);
    }
    let in_assets = format!("assets/icons/{}", nome_icone);
    if let Some(p) = resolver_caminho_asset(&in_assets) {
        return Some(p);
    }
    None
}

/// Carrega o ícone oficial do Eolo Platform para uso na janela do eframe (barra de títulos, barra de tarefas e Alt+Tab).
/// Tenta primeiro carregar de arquivo em disco; se não encontrar, utiliza os bytes embutidos em tempo de compilação.
pub fn carregar_icone_janela() -> Option<egui::IconData> {
    ICON_DATA_CACHE
        .get_or_init(|| {
            // 1. Tentar ler do arquivo no disco
            if let Some(path) = obter_caminho_icone("eolo.ico") {
                if let Ok(img) = image::open(&path) {
                    let rgba = img.to_rgba8();
                    let (width, height) = rgba.dimensions();
                    return Some(egui::IconData {
                        rgba: rgba.into_raw(),
                        width,
                        height,
                    });
                }
            }

            // 2. Fallback garantido: bytes embutidos diretamente no executável em tempo de compilação
            if let Ok(img) = image::load_from_memory(include_bytes!("../assets/icons/eolo.ico")) {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                return Some(egui::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                });
            }

            None
        })
        .clone()
}
