use egui::{CornerRadius, Frame, Margin, RichText, Stroke, Ui};

use crate::core::project::EoloProject;
use crate::core::scenarios::get_preset_scenarios;
use crate::core::types::PressureUnit;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::locale::{Language, Localizer};
use crate::ui::theme::EoloTheme;

pub fn render_toolbar(
    ui: &mut Ui,
    project: &mut EoloProject,
    env: &WindEnvelopeResults,
    dark_mode: &mut bool,
    localizer: &mut Localizer,
) {
    let dark = ui.visuals().dark_mode;
    let lang = localizer.current_lang;

    // Moldura retangular fina envolvendo todos os widgets abaixo da barra de menus
    let moldura = Frame::NONE
        .fill(ui.visuals().widgets.noninteractive.bg_fill)
        .stroke(Stroke::new(
            1.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(8, 6));

    moldura.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);

        // Layout responsivo em fluxo contínuo (horizontal_wrapped) para zero sobreposição
        // mesmo nas dimensões mínimas da janela
        ui.horizontal_wrapped(|ui| {
            // 1. Ações de Arquivo
            if ui
                .button(localizer.tr("📄 Novo", "📄 New"))
                .on_hover_text(localizer.tr(
                    "Criar novo projeto em branco",
                    "Create a new blank project",
                ))
                .clicked()
            {
                *project = EoloProject::default();
            }

            if ui
                .button(localizer.tr("📂 Abrir", "📂 Open"))
                .on_hover_text(localizer.tr(
                    "Carregar projeto JSON (.json)",
                    "Load JSON project (.json)",
                ))
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
            }

            if ui
                .button(localizer.tr("💾 Salvar", "💾 Save"))
                .on_hover_text(localizer.tr(
                    "Salvar projeto atual em JSON",
                    "Save current project to JSON",
                ))
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
            }

            ui.separator();

            // 2. Seletor Rápido dos 5 Cenários de Engenharia
            ui.label(
                RichText::new(localizer.tr("Cenário:", "Scenario:"))
                    .strong()
                    .color(EoloTheme::text_primary(dark)),
            );
            for sc in get_preset_scenarios() {
                if ui
                    .button(format!("#{}", sc.id))
                    .on_hover_text(format!(
                        "{}\n\n{}",
                        sc.name(lang),
                        sc.description(lang)
                    ))
                    .clicked()
                {
                    *project = sc.project;
                }
            }

            ui.separator();

            // 3. Seletor de Unidade de Pressão
            ui.label(
                RichText::new(localizer.tr("Unidade:", "Unit:"))
                    .color(EoloTheme::text_primary(dark)),
            );
            egui::ComboBox::from_id_salt("toolbar_unit_combo")
                .width(74.0)
                .selected_text(project.display_unit.symbol())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut project.display_unit,
                        PressureUnit::KiloNewtonPerSquareMeter,
                        "kN/m²",
                    );
                    ui.selectable_value(
                        &mut project.display_unit,
                        PressureUnit::KilogramForcePerSquareMeter,
                        "kgf/m²",
                    );
                    ui.selectable_value(
                        &mut project.display_unit,
                        PressureUnit::Pascal,
                        "Pa",
                    );
                    ui.selectable_value(
                        &mut project.display_unit,
                        PressureUnit::KiloPascal,
                        "kPa",
                    );
                });

            // 4. Botão de Alternância Dinâmica de Tema (Escuro / Claro)
            let theme_label = if *dark_mode {
                localizer.tr("☀️ Claro", "☀️ Light")
            } else {
                localizer.tr("🌙 Escuro", "🌙 Dark")
            };
            if ui
                .button(theme_label)
                .on_hover_text(localizer.tr(
                    "Alternar dinamicamente entre Tema Escuro e Tema Claro",
                    "Dynamically toggle between Dark Theme and Light Theme",
                ))
                .clicked()
            {
                *dark_mode = !*dark_mode;
            }

            // 5. Botão de Alternância Dinâmica de Idioma (PT-BR / EN-US)
            let lang_label = match localizer.current_lang {
                Language::PtBr => "🇧🇷 PT-BR",
                Language::EnUs => "🇺🇸 EN-US",
            };
            if ui
                .button(lang_label)
                .on_hover_text(localizer.tr(
                    "Alternar idioma em tempo real (Português BR / Inglês US)",
                    "Switch language in real time (Portuguese BR / English US)",
                ))
                .clicked()
            {
                localizer.current_lang = match localizer.current_lang {
                    Language::PtBr => Language::EnUs,
                    Language::EnUs => Language::PtBr,
                };
            }

            ui.separator();

            // 6. Indicadores de Telemetria Estrutural em Tempo Real (Sem sobreposição)
            let unit_sym = project.display_unit.symbol();
            let q_disp = project.display_unit.from_pa(env.wind_state.q_pa);
            let uplift_disp = project
                .display_unit
                .from_pa(env.roof_max_uplift_kn_m2 * 1000.0);

            let badge_frame = Frame::NONE
                .fill(EoloTheme::card_bg(dark))
                .stroke(Stroke::new(1.0, EoloTheme::card_stroke(dark)))
                .corner_radius(CornerRadius::same(4))
                .inner_margin(Margin::symmetric(6, 2));

            badge_frame.show(ui, |ui| {
                ui.label(
                    RichText::new(format!(
                        "Vk = {:.1} m/s ({:.0} km/h)",
                        env.wind_state.vk_ms, env.wind_state.vk_kmh
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
            });

            badge_frame.show(ui, |ui| {
                ui.label(
                    RichText::new(format!("q = {:.2} {}", q_disp, unit_sym))
                        .color(EoloTheme::text_primary(dark))
                        .strong(),
                );
            });

            badge_frame.show(ui, |ui| {
                let uplift_lbl = localizer.tr("Arrancamento:", "Max Uplift:");
                ui.label(
                    RichText::new(format!(
                        "{} {:.2} {}",
                        uplift_lbl, uplift_disp, unit_sym
                    ))
                    .color(EoloTheme::suction_cyan(dark))
                    .strong(),
                );
            });
        });
    });
}
