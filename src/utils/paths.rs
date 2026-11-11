use std::path::PathBuf;

/// Resolve um caminho relativo dentro de `src/assets` (em desenvolvimento)
/// ou `assets` (em distribuição instalada).
pub fn resolver_caminho_asset(relativo: &str) -> Option<PathBuf> {
    let mut candidatos = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        candidatos.push(cwd.join("src").join("assets").join(relativo));
        candidatos.push(cwd.join("assets").join(relativo));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            candidatos.push(exe_dir.join("assets").join(relativo));
            candidatos.push(exe_dir.join("src").join("assets").join(relativo));
            if let Some(p1) = exe_dir.parent() {
                candidatos.push(p1.join("src").join("assets").join(relativo));
                candidatos.push(p1.join("assets").join(relativo));
                if let Some(p2) = p1.parent() {
                    candidatos.push(p2.join("src").join("assets").join(relativo));
                    candidatos.push(p2.join("assets").join(relativo));
                    if let Some(p3) = p2.parent() {
                        candidatos.push(p3.join("src").join("assets").join(relativo));
                        candidatos.push(p3.join("assets").join(relativo));
                    }
                }
            }
        }
    }

    candidatos.into_iter().find(|p| p.exists())
}
