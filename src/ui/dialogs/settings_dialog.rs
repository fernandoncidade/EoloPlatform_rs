use egui::RichText;

use crate::core::project::EoloProject;
use crate::core::types::PressureUnit;
use crate::locale::Language;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;

#[derive(Default)]
pub struct SettingsDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
}

impl SettingsDialog {
    pub fn open(&mut self) {
        self.is_open = true;
        self.minimized = false;
        self.raise_z_order = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.minimized = false;
        self.raise_z_order = false;
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        project: &mut EoloProject,
        dark_mode: &mut bool,
        current_lang: &mut Language,
    ) {
        if !self.is_open {
            return;
        }

        let mut state = ModelessState {
            is_open: self.is_open,
            raise_z_order: self.raise_z_order,
            maximized: self.maximized,
            minimized: self.minimized,
        };

        let is_pt = *current_lang == Language::PtBr;
        let tr = |pt: &'static str, en: &'static str| if is_pt { pt } else { en };
        let initial_dark = *dark_mode;

        show_modeless_dialog(
            ctx,
            "eolo_dialog_settings",
            tr(
                "⚙️ Configurações do Projeto e Preferências",
                "⚙️ Project Settings & Preferences",
            ),
            [540.0, 440.0],
            [440.0, 380.0],
            initial_dark,
            &mut state,
            |ui, close_clicked| {
                let dark = *dark_mode;
                ui.heading(
                    RichText::new(tr(
                        "Dados do Projeto e Responsabilidade Técnica",
                        "Project Identification & Technical Responsibility",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.label(tr(
                    "Estes dados serão incorporados no cabeçalho formal do Memorial de Cálculo:",
                    "These fields will be included in the formal header of the Calculation Report:",
                ));
                ui.add_space(4.0);

                egui::Grid::new("settings_meta_grid")
                    .num_columns(2)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(tr("Título do Projeto:", "Project Title:"));
                        ui.text_edit_singleline(&mut project.metadata.title);
                        ui.end_row();

                        ui.label(tr("Engenheiro Responsável:", "Structural Engineer:"));
                        ui.text_edit_singleline(&mut project.metadata.engineer);
                        ui.end_row();

                        ui.label(tr("Registro CREA / ART:", "License CREA / ART:"));
                        ui.text_edit_singleline(&mut project.metadata.crea_art);
                        ui.end_row();

                        ui.label(tr("Cliente / Contratante:", "Client / Owner:"));
                        ui.text_edit_singleline(&mut project.metadata.client);
                        ui.end_row();

                        ui.label(tr("Cidade:", "City:"));
                        ui.text_edit_singleline(&mut project.metadata.location_city);
                        ui.end_row();

                        ui.label(tr("Estado (UF):", "State (UF):"));
                        ui.text_edit_singleline(&mut project.metadata.location_state);
                        ui.end_row();
                    });

                ui.separator();
                ui.heading(
                    RichText::new(tr(
                        "Preferências de Interface e Unidades",
                        "Interface & Unit Preferences",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );

                ui.horizontal(|ui| {
                    ui.label(tr("Tema Visual:", "Visual Theme:"));
                    if ui
                        .selectable_label(*dark_mode, tr("🌙 Modo Escuro", "🌙 Dark Mode"))
                        .clicked()
                    {
                        *dark_mode = true;
                    }
                    if ui
                        .selectable_label(!*dark_mode, tr("☀️ Modo Claro", "☀️ Light Mode"))
                        .clicked()
                    {
                        *dark_mode = false;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Idioma / Language:");
                    if ui
                        .selectable_label(*current_lang == Language::PtBr, "🇧🇷 Português (Brasil)")
                        .clicked()
                    {
                        *current_lang = Language::PtBr;
                    }
                    if ui
                        .selectable_label(*current_lang == Language::EnUs, "🇺🇸 English (US)")
                        .clicked()
                    {
                        *current_lang = Language::EnUs;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(tr(
                        "Unidade Padrão de Pressão:",
                        "Default Pressure Unit:",
                    ));
                    egui::ComboBox::from_id_salt("settings_unit_combo")
                        .selected_text(project.display_unit.symbol())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut project.display_unit,
                                PressureUnit::KiloNewtonPerSquareMeter,
                                tr("kN/m² (Recomendado NBR)", "kN/m² (Recommended NBR)"),
                            );
                            ui.selectable_value(
                                &mut project.display_unit,
                                PressureUnit::KilogramForcePerSquareMeter,
                                "kgf/m²",
                            );
                            ui.selectable_value(
                                &mut project.display_unit,
                                PressureUnit::Pascal,
                                "Pascal (Pa)",
                            );
                            ui.selectable_value(
                                &mut project.display_unit,
                                PressureUnit::KiloPascal,
                                "kPa",
                            );
                        });
                });

                ui.separator();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            RichText::new(tr("Concluir", "Done"))
                                .color(EoloTheme::text_primary(dark))
                                .strong(),
                        )
                        .clicked()
                    {
                        *close_clicked = true;
                    }
                });
            },
        );

        self.is_open = state.is_open;
        self.raise_z_order = state.raise_z_order;
        self.maximized = state.maximized;
        self.minimized = state.minimized;
    }
}
