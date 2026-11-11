use egui::Ui;

use crate::core::project::EoloProject;
use crate::core::scenarios::get_preset_scenarios;
use crate::locale::{Language, Localizer};
use crate::ui::dialogs::*;

#[allow(clippy::too_many_arguments)]
pub fn render_menu_bar(
    ui: &mut Ui,
    project: &mut EoloProject,
    localizer: &mut Localizer,
    dark_mode: &mut bool,
    active_tab: &mut usize,
    dialog_about: &mut AboutDialog,
    dialog_manual: &mut ManualDialog,
    dialog_audit: &mut AuditDialog,
    dialog_quick_calc: &mut QuickCalcDialog,
    dialog_units: &mut UnitsDialog,
    dialog_settings: &mut SettingsDialog,
) {
    let lang = localizer.current_lang;

    egui::MenuBar::new().ui(ui, |ui| {
        // Menu Arquivo / File
        ui.menu_button(localizer.t("menu_file"), |ui| {
            if ui
                .button(localizer.tr("📄 Novo Projeto Padrão", "📄 New Default Project"))
                .clicked()
            {
                *project = EoloProject::default();
                ui.close();
            }

            if ui
                .button(localizer.tr("📂 Abrir Projeto (.json)...", "📂 Open Project (.json)..."))
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Eolo Project", &["json"])
                    .pick_file()
                {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if let Ok(loaded) = serde_json::from_str::<EoloProject>(&content) {
                            *project = loaded;
                        }
                    }
                }
                ui.close();
            }

            if ui
                .button(localizer.tr("💾 Salvar Projeto (.json)...", "💾 Save Project (.json)..."))
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Eolo Project", &["json"])
                    .set_file_name("projeto_eolo_nbr6123.json")
                    .save_file()
                {
                    if let Ok(json) = serde_json::to_string_pretty(project) {
                        let _ = std::fs::write(path, json);
                    }
                }
                ui.close();
            }

            ui.separator();

            if ui
                .button(localizer.tr(
                    "🖨️ Ir para Exportação de Memorial (Aba 5)",
                    "🖨️ Go to Report Export (Tab 5)",
                ))
                .clicked()
            {
                *active_tab = 4;
                ui.close();
            }

            ui.separator();

            if ui.button(localizer.tr("🚪 Sair", "🚪 Exit")).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        // Menu Cenários Pré-Configurados / Preset Scenarios
        ui.menu_button(localizer.t("menu_scenarios"), |ui| {
            ui.label(localizer.tr(
                "Carregar Caso Prático da Engenharia:",
                "Load Practical Engineering Case:",
            ));
            ui.separator();

            for sc in get_preset_scenarios() {
                if ui
                    .button(sc.name(lang))
                    .on_hover_text(sc.description(lang))
                    .clicked()
                {
                    *project = sc.project;
                    ui.close();
                }
            }
        });

        // Menu Ferramentas / Tools
        ui.menu_button(localizer.t("menu_tools"), |ui| {
            if ui
                .button(localizer.tr(
                    "🧮 Calculadora Rápida de Vk e Pressão q...",
                    "🧮 Quick Vk & Dynamic Pressure q Calculator...",
                ))
                .clicked()
            {
                dialog_quick_calc.open();
                ui.close();
            }

            if ui
                .button(localizer.tr(
                    "⚖️ Conversor de Unidades de Pressão (Pa, kN/m², kgf/m²)...",
                    "⚖️ Pressure Unit Converter (Pa, kN/m², kgf/m²)...",
                ))
                .clicked()
            {
                dialog_units.open();
                ui.close();
            }

            ui.separator();

            if ui
                .button(localizer.tr(
                    "✅ Matriz de Rastreabilidade Acadêmica (Ref. NBR 6123:2023)...",
                    "✅ Academic Traceability Matrix (Ref. NBR 6123:2023)...",
                ))
                .clicked()
            {
                dialog_audit.open();
                ui.close();
            }
        });

        // Menu Configurações / Settings
        ui.menu_button(localizer.t("menu_settings"), |ui| {
            if ui
                .button(localizer.tr(
                    "⚙️ Dados do Projeto, ART e Preferências...",
                    "⚙️ Project Metadata, License & Preferences...",
                ))
                .clicked()
            {
                dialog_settings.open();
                ui.close();
            }

            ui.separator();

            let theme_text = if *dark_mode {
                localizer.tr("☀️ Alternar para Tema Claro", "☀️ Switch to Light Theme")
            } else {
                localizer.tr("🌙 Alternar para Tema Escuro", "🌙 Switch to Dark Theme")
            };
            if ui.button(theme_text).clicked() {
                *dark_mode = !*dark_mode;
                ui.close();
            }

            ui.separator();

            ui.label(localizer.tr("🌐 Idioma / Language:", "🌐 Language / Idioma:"));
            if ui
                .radio_value(
                    &mut localizer.current_lang,
                    Language::PtBr,
                    "🇧🇷 Português (Brasil)",
                )
                .clicked()
            {
                ui.close();
            }
            if ui
                .radio_value(
                    &mut localizer.current_lang,
                    Language::EnUs,
                    "🇺🇸 English (US)",
                )
                .clicked()
            {
                ui.close();
            }
        });

        // Menu Ajuda / Help
        ui.menu_button(localizer.t("menu_help"), |ui| {
            if ui
                .button(localizer.tr(
                    "📖 Manual do Usuário & Guia de Estudo (Ref. NBR 6123)...",
                    "📖 User Manual & Study Guide (Ref. NBR 6123)...",
                ))
                .clicked()
            {
                dialog_manual.open();
                ui.close();
            }

            if ui
                .button(localizer.tr(
                    "✅ Rastreabilidade Acadêmica (Ref. ABNT NBR 6123:2023)...",
                    "✅ Academic Traceability (Ref. ABNT NBR 6123:2023)...",
                ))
                .clicked()
            {
                dialog_audit.open();
                ui.close();
            }

            ui.separator();

            if ui
                .button(localizer.tr("ℹ️ Sobre o Eolo Platform...", "ℹ️ About Eolo Platform..."))
                .clicked()
            {
                dialog_about.open();
                ui.close();
            }
        });
    });
}
