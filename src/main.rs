#![windows_subsystem = "windows"]

use eframe::NativeOptions;
use eolo_rs::ui::EoloApp;
use eolo_rs::utils::icons::carregar_icone_janela;

fn main() -> eframe::Result<()> {
    #[cfg(target_os = "windows")]
    unsafe {
        // Desvincula o console caso o executável seja acionado por terminal ou duplo clique
        windows_sys::Win32::System::Console::FreeConsole();

        // Vincula o AppUserModelID explícito para que a barra de tarefas do Windows 11
        // use o ícone e agrupamento exclusivo do aplicativo, inclusive quando fixado.
        // IMPORTANTE: deve ser idêntico ao MyAppUserModelID de repo/EoloPlatform.iss,
        // caso contrário o atalho fixado e a janela aparecem como botões separados.
        let app_id: Vec<u16> = "fernandoncidade.EoloPlatform\0"
            .encode_utf16()
            .collect();
        windows_sys::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID(app_id.as_ptr());
    }

    let mut native_options = NativeOptions::default();
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Eolo Platform — Ações de Vento em Edificações e Coberturas (Estudo Acadêmico • Ref. ABNT NBR 6123:2023)")
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([960.0, 600.0])
        .with_active(true);

    // Vincula o ícone oficial à barra de títulos da janela, barra de tarefas e Alt+Tab
    if let Some(icon) = carregar_icone_janela() {
        viewport = viewport.with_icon(icon);
    }

    #[cfg(target_os = "windows")]
    {
        unsafe {
            let cx = windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(
                windows_sys::Win32::UI::WindowsAndMessaging::SM_CXSCREEN,
            );
            let cy = windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(
                windows_sys::Win32::UI::WindowsAndMessaging::SM_CYSCREEN,
            );
            if cx > 0 && cy > 0 {
                let x = ((cx as f32 - 1280.0) * 0.5).max(0.0);
                let y = ((cy as f32 - 800.0) * 0.5).max(0.0);
                viewport = viewport.with_position([x, y]);
            }
        }
    }

    native_options.viewport = viewport;

    eframe::run_native(
        "Eolo Platform",
        native_options,
        Box::new(|_cc| Ok(Box::new(EoloApp::default()))),
    )
}
