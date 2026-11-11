use egui::{
    text::LayoutJob, CornerRadius, FontFamily, FontId, Frame, Margin, RichText, ScrollArea, Stroke,
    TextFormat, Ui,
};

use crate::core::project::EoloProject;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::locale::Localizer;
use crate::reports::MemorialGenerator;
use crate::ui::theme::EoloTheme;

#[derive(Default)]
pub struct MemorialTabState {
    pub search_query: String,
    pub status_message: Option<(String, bool)>, // (mensagem, is_error)
}

/// Constrói um `LayoutJob` interpretando `**negrito**` e `*itálico*` sem exibir asteriscos brutos
fn build_inline_markdown_job(
    prefix: &str,
    prefix_color: egui::Color32,
    text: &str,
    font_size: f32,
    dark: bool,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    let txt_primary = EoloTheme::text_primary(dark);
    let txt_bold = EoloTheme::accent_blue(dark);
    let txt_sec = EoloTheme::text_secondary(dark);

    if !prefix.is_empty() {
        job.append(
            prefix,
            0.0,
            TextFormat {
                font_id: FontId::new(font_size, FontFamily::Proportional),
                color: prefix_color,
                ..Default::default()
            },
        );
    }

    let mut rest = text;
    while !rest.is_empty() {
        if let Some(bold_start) = rest.find("**") {
            let before = &rest[..bold_start];
            if !before.is_empty() {
                append_italic_aware(&mut job, before, font_size, txt_primary, txt_sec);
            }
            let after_open = &rest[bold_start + 2..];
            if let Some(bold_end) = after_open.find("**") {
                let bold_content = &after_open[..bold_end];
                // Se o trecho em negrito for um rótulo (termina em ':'), usa cor primária forte;
                // se for um valor numérico destacado, usa azul técnico de destaque.
                let color = if bold_content.trim_end().ends_with(':') {
                    txt_primary
                } else {
                    txt_bold
                };
                job.append(
                    bold_content,
                    0.0,
                    TextFormat {
                        font_id: FontId::new(font_size + 0.3, FontFamily::Proportional),
                        color,
                        ..Default::default()
                    },
                );
                rest = &after_open[bold_end + 2..];
            } else {
                append_italic_aware(&mut job, after_open, font_size, txt_primary, txt_sec);
                break;
            }
        } else {
            append_italic_aware(&mut job, rest, font_size, txt_primary, txt_sec);
            break;
        }
    }

    job
}

fn append_italic_aware(
    job: &mut LayoutJob,
    segment: &str,
    font_size: f32,
    normal_color: egui::Color32,
    italic_color: egui::Color32,
) {
    let mut rem = segment;
    while !rem.is_empty() {
        if let Some(it_start) = rem.find('*') {
            let before = &rem[..it_start];
            if !before.is_empty() {
                job.append(
                    before,
                    0.0,
                    TextFormat {
                        font_id: FontId::new(font_size, FontFamily::Proportional),
                        color: normal_color,
                        ..Default::default()
                    },
                );
            }
            let after_open = &rem[it_start + 1..];
            if let Some(it_end) = after_open.find('*') {
                let it_content = &after_open[..it_end];
                job.append(
                    it_content,
                    0.0,
                    TextFormat {
                        font_id: FontId::new(font_size, FontFamily::Proportional),
                        color: italic_color,
                        italics: true,
                        ..Default::default()
                    },
                );
                rem = &after_open[it_end + 1..];
            } else {
                job.append(
                    after_open,
                    0.0,
                    TextFormat {
                        font_id: FontId::new(font_size, FontFamily::Proportional),
                        color: normal_color,
                        ..Default::default()
                    },
                );
                break;
            }
        } else {
            job.append(
                rem,
                0.0,
                TextFormat {
                    font_id: FontId::new(font_size, FontFamily::Proportional),
                    color: normal_color,
                    ..Default::default()
                },
            );
            break;
        }
    }
}

/// Renderiza uma tabela Markdown completa como um `egui::Grid` estruturado e listrado
fn render_markdown_table(ui: &mut Ui, table_lines: &[&str], table_idx: usize, dark: bool) {
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in table_lines {
        let trimmed = line.trim();
        if trimmed.contains(":---") {
            continue;
        }
        let inner = trimmed
            .strip_prefix('|')
            .unwrap_or(trimmed)
            .strip_suffix('|')
            .unwrap_or(trimmed);
        let cells: Vec<String> = inner.split('|').map(|c| c.trim().to_string()).collect();
        if !cells.is_empty() {
            rows.push(cells);
        }
    }

    if rows.is_empty() {
        return;
    }

    let num_cols = rows[0].len();
    let table_frame = Frame::NONE
        .fill(EoloTheme::card_bg(dark))
        .stroke(Stroke::new(1.0, EoloTheme::card_stroke(dark)))
        .corner_radius(CornerRadius::same(5))
        .inner_margin(Margin::symmetric(10, 8));

    ui.add_space(4.0);
    table_frame.show(ui, |ui| {
        egui::Grid::new(("memorial_abnt_table", table_idx))
            .striped(true)
            .num_columns(num_cols)
            .spacing([22.0, 7.0])
            .show(ui, |ui| {
                for (r_idx, row) in rows.iter().enumerate() {
                    for cell in row {
                        if r_idx == 0 {
                            let clean_hdr = MemorialGenerator::strip_markdown_inline(cell);
                            ui.label(
                                RichText::new(clean_hdr)
                                    .size(13.0)
                                    .color(EoloTheme::accent_blue(dark))
                                    .strong(),
                            );
                        } else {
                            let is_bold_cell = cell.contains("**");
                            let job = build_inline_markdown_job(
                                "",
                                EoloTheme::text_primary(dark),
                                cell,
                                12.5,
                                dark,
                            );
                            if is_bold_cell {
                                ui.label(job);
                            } else {
                                ui.label(
                                    RichText::new(MemorialGenerator::strip_markdown_inline(cell))
                                        .size(12.5)
                                        .color(EoloTheme::text_primary(dark)),
                                );
                            }
                        }
                    }
                    ui.end_row();
                }
            });
    });
    ui.add_space(4.0);
}

pub fn render_tab_memorial(
    ui: &mut Ui,
    project: &EoloProject,
    env: &WindEnvelopeResults,
    state: &mut MemorialTabState,
    localizer: &Localizer,
) {
    let dark = ui.visuals().dark_mode;
    let lang = localizer.current_lang;
    let txt_primary = EoloTheme::text_primary(dark);
    let md_content = MemorialGenerator::generate_markdown_i18n(project, env, lang);

    ui.horizontal_wrapped(|ui| {
        ui.heading(
            RichText::new(localizer.tr(
                "5. Memorial de Cálculo Estrutural & Exportação ABNT",
                "5. Structural Calculation Report & ABNT Export",
            ))
            .color(txt_primary)
            .strong(),
        );

        ui.add_space(12.0);

        if ui
            .button(
                RichText::new(localizer.tr(
                    "🖨️ Exportar HTML / PDF (A4)...",
                    "🖨️ Export HTML / PDF (A4)...",
                ))
                .color(txt_primary)
                .strong(),
            )
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("HTML / A4 PDF Print", &["html", "htm"])
                .set_file_name("Memorial_Calculo_Vento_NBR6123.html")
                .save_file()
            {
                let html = MemorialGenerator::generate_html_i18n(project, env, lang);
                match std::fs::write(&path, html) {
                    Ok(_) => {
                        state.status_message = Some((
                            format!(
                                "{} {}",
                                localizer.tr(
                                    "Memorial exportado com sucesso em:",
                                    "Report successfully exported to:"
                                ),
                                path.display()
                            ),
                            false,
                        ));
                    }
                    Err(e) => {
                        state.status_message = Some((
                            format!("{} {}", localizer.tr("Erro ao exportar:", "Export error:"), e),
                            true,
                        ));
                    }
                }
            }
        }

        if ui
            .button(localizer.tr(
                "💾 Salvar Markdown (.md)...",
                "💾 Save Markdown (.md)...",
            ))
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Markdown Document", &["md"])
                .set_file_name("Memorial_Calculo_Vento_NBR6123.md")
                .save_file()
            {
                match std::fs::write(&path, &md_content) {
                    Ok(_) => {
                        state.status_message = Some((
                            format!(
                                "{} {}",
                                localizer.tr(
                                    "Arquivo salvo com sucesso em:",
                                    "File successfully saved to:"
                                ),
                                path.display()
                            ),
                            false,
                        ));
                    }
                    Err(e) => {
                        state.status_message = Some((
                            format!("{} {}", localizer.tr("Erro ao salvar:", "Save error:"), e),
                            true,
                        ));
                    }
                }
            }
        }

        if ui
            .button(localizer.tr(
                "📦 Exportar Projeto (.json)...",
                "📦 Export Project (.json)...",
            ))
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Eolo Project", &["json"])
                .set_file_name("projeto_eolo_nbr6123.json")
                .save_file()
            {
                let json = serde_json::to_string_pretty(project).unwrap_or_default();
                match std::fs::write(&path, json) {
                    Ok(_) => {
                        state.status_message = Some((
                            format!(
                                "{} {}",
                                localizer.tr("Projeto salvo em JSON:", "Project saved to JSON:"),
                                path.display()
                            ),
                            false,
                        ));
                    }
                    Err(e) => {
                        state.status_message = Some((
                            format!(
                                "{} {}",
                                localizer.tr("Erro ao exportar JSON:", "JSON export error:"),
                                e
                            ),
                            true,
                        ));
                    }
                }
            }
        }

        if ui
            .button(localizer.tr("📋 Copiar Texto", "📋 Copy Text"))
            .clicked()
        {
            let plain_report = MemorialGenerator::generate_plain_text_i18n(project, env, lang);
            ui.ctx().copy_text(plain_report);
            state.status_message = Some((
                localizer
                    .tr(
                        "Memorial formatado copiado para a Área de Transferência!",
                        "Formatted Calculation Report copied to Clipboard!",
                    )
                    .to_string(),
                false,
            ));
        }
    });

    if let Some((msg, is_err)) = &state.status_message {
        ui.add_space(4.0);
        let color = if *is_err {
            EoloTheme::overpressure_red(dark)
        } else {
            EoloTheme::success_green(dark)
        };
        ui.label(RichText::new(msg).color(color).strong());
    }

    ui.separator();

    // Campo de Busca / Filtragem no Memorial
    ui.horizontal(|ui| {
        ui.label(localizer.tr(
            "🔍 Filtrar seções do relatório:",
            "🔍 Filter report sections:",
        ));
        ui.text_edit_singleline(&mut state.search_query);
        if !state.search_query.is_empty() && ui.button("✖").clicked() {
            state.search_query.clear();
        }
    });

    ui.add_space(6.0);

    // Visualizador de documento técnico ABNT com renderização rica de títulos, listas, equações e tabelas
    ScrollArea::vertical()
        .id_salt("memorial_abnt_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let doc_frame = Frame::NONE
                .fill(ui.visuals().window_fill)
                .stroke(Stroke::new(1.0, EoloTheme::card_stroke(dark)))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::symmetric(18, 14));

            doc_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                let query = state.search_query.trim().to_lowercase();
                let all_lines: Vec<&str> = md_content.lines().collect();
                let mut idx = 0usize;
                let mut table_counter = 0usize;

                while idx < all_lines.len() {
                    let line = all_lines[idx];
                    let trimmed = line.trim();

                    // Agrupa linhas consecutivas de tabela Markdown (`| ... |`)
                    if trimmed.starts_with('|') && trimmed.ends_with('|') {
                        let start = idx;
                        while idx < all_lines.len() {
                            let t = all_lines[idx].trim();
                            if t.starts_with('|') && t.ends_with('|') {
                                idx += 1;
                            } else {
                                break;
                            }
                        }
                        let table_slice = &all_lines[start..idx];
                        let matches_query = query.is_empty()
                            || table_slice
                                .iter()
                                .any(|l| l.to_lowercase().contains(&query));
                        if matches_query {
                            table_counter += 1;
                            render_markdown_table(ui, table_slice, table_counter, dark);
                        }
                        continue;
                    }

                    idx += 1;

                    if !query.is_empty() && !line.to_lowercase().contains(&query) {
                        continue;
                    }

                    if let Some(h1) = line.strip_prefix("# ") {
                        ui.add_space(4.0);
                        ui.heading(
                            RichText::new(MemorialGenerator::strip_markdown_inline(h1))
                                .size(19.0)
                                .color(EoloTheme::accent_blue(dark))
                                .strong(),
                        );
                    } else if let Some(h2) = line.strip_prefix("## ") {
                        ui.label(
                            RichText::new(MemorialGenerator::strip_markdown_inline(h2))
                                .size(14.0)
                                .color(EoloTheme::secondary_blue(dark))
                                .strong(),
                        );
                        ui.add_space(4.0);
                    } else if let Some(h3) = line.strip_prefix("### ") {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(MemorialGenerator::strip_markdown_inline(h3))
                                .size(14.5)
                                .color(EoloTheme::accent_blue(dark))
                                .strong(),
                        );
                        ui.add_space(3.0);
                    } else if let Some(h4) = line.strip_prefix("#### ") {
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(MemorialGenerator::strip_markdown_inline(h4))
                                .size(13.0)
                                .color(EoloTheme::secondary_blue(dark))
                                .strong(),
                        );
                        ui.add_space(2.0);
                    } else if let Some(eq) = line.strip_prefix("> ") {
                        let eq_frame = Frame::NONE
                            .fill(EoloTheme::card_bg(dark))
                            .stroke(Stroke::new(1.0, EoloTheme::accent_blue(dark)))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(12, 6));
                        ui.add_space(3.0);
                        eq_frame.show(ui, |ui| {
                            ui.label(
                                RichText::new(MemorialGenerator::strip_markdown_inline(eq))
                                    .size(13.5)
                                    .color(EoloTheme::accent_blue(dark))
                                    .strong(),
                            );
                        });
                        ui.add_space(3.0);
                    } else if line.starts_with("---") {
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                    } else if let Some(sub_item) = line.strip_prefix("  - ") {
                        let job = build_inline_markdown_job(
                            "      ◦  ",
                            EoloTheme::secondary_blue(dark),
                            sub_item,
                            12.5,
                            dark,
                        );
                        ui.label(job);
                    } else if let Some(item) = line.strip_prefix("- ") {
                        let job = build_inline_markdown_job(
                            "  •  ",
                            EoloTheme::accent_blue(dark),
                            item,
                            13.0,
                            dark,
                        );
                        ui.label(job);
                    } else if trimmed.starts_with("1. ") || trimmed.starts_with("2. ") {
                        let (num_part, rest_part) = trimmed.split_at(3);
                        let job = build_inline_markdown_job(
                            &format!("  {} ", num_part),
                            EoloTheme::accent_blue(dark),
                            rest_part,
                            12.8,
                            dark,
                        );
                        ui.label(job);
                    } else if !trimmed.is_empty() {
                        let job = build_inline_markdown_job(
                            "",
                            txt_primary,
                            trimmed,
                            13.0,
                            dark,
                        );
                        ui.label(job);
                    }
                }
            });
        });
}
