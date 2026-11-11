use crate::locale::Localizer;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;
use crate::utils::paths::resolver_caminho_asset;
use egui::{text::LayoutJob, Color32, FontId, RichText, ScrollArea, TextFormat, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbaSobre {
    Sobre,
    Historico,
    Clc,
    Copyright,
    Eula,
    Licencas,
    Avisos,
    Privacidade,
    Release,
}

pub struct AboutDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub aba_ativa: AbaSobre,
    pub search_query: String,
    pub current_hit: usize,
    pub total_hits: usize,
    scroll_to_current_hit: bool,
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            raise_z_order: false,
            maximized: false,
            minimized: false,
            aba_ativa: AbaSobre::Sobre,
            search_query: String::new(),
            current_hit: 0,
            total_hits: 0,
            scroll_to_current_hit: false,
        }
    }
}

impl AboutDialog {
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

    pub fn show(&mut self, ctx: &egui::Context, loc: &Localizer, dark_mode: bool) {
        if !self.is_open {
            return;
        }

        let mut state = ModelessState {
            is_open: self.is_open,
            raise_z_order: self.raise_z_order,
            maximized: self.maximized,
            minimized: self.minimized,
        };

        show_modeless_dialog(
            ctx,
            "eolo_dialog_about",
            loc.t("about_window_title"),
            [860.0, 620.0],
            [640.0, 460.0],
            dark_mode,
            &mut state,
            |ui, close_clicked| {
                self.render_contents(ui, loc, dark_mode, close_clicked);
            },
        );

        self.is_open = state.is_open;
        self.raise_z_order = state.raise_z_order;
        self.maximized = state.maximized;
        self.minimized = state.minimized;
    }

    pub fn render_contents(
        &mut self,
        ui: &mut Ui,
        loc: &Localizer,
        dark: bool,
        close_clicked: &mut bool,
    ) {
        let lang_code = loc.codigo_idioma();
        let texto_aba = self.carregar_texto_aba(self.aba_ativa, lang_code);

        // Recalcula total de hits na aba ativa
        let query_trimmed = self.search_query.trim().to_lowercase();
        let mut hits_total = 0usize;
        if !query_trimmed.is_empty() {
            if self.aba_ativa == AbaSobre::Licencas {
                for (_, fname) in Self::arquivos_licencas() {
                    let txt = Self::carregar_licenca(fname);
                    hits_total += Self::contar_ocorrencias(&txt, &query_trimmed);
                }
            } else {
                hits_total = Self::contar_ocorrencias(&texto_aba, &query_trimmed);
            }
        }
        self.total_hits = hits_total;
        if self.total_hits == 0 || self.current_hit >= self.total_hits {
            self.current_hit = 0;
        }

        // Cabeçalho Fixo da Aplicação
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.heading(
                    RichText::new("🌬️ Eolo Platform")
                        .size(22.0)
                        .color(EoloTheme::accent_blue(dark))
                        .strong(),
                );
                ui.add_space(8.0);
                let badge_bg = if dark {
                    Color32::from_rgb(20, 65, 42)
                } else {
                    Color32::from_rgb(220, 245, 228)
                };
                egui::Frame::NONE
                    .fill(badge_bg)
                    .corner_radius(4.0)
                    .inner_margin(egui::Margin::symmetric(8, 3))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(
                                "🛡️ 100% Local-First • Offline • Zero Cloud • Zero Telemetry",
                            )
                            .color(EoloTheme::success_green(dark))
                            .strong()
                            .size(11.5),
                        );
                    });
            });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{} 2026.11.11.0",
                        loc.t("about_version_label")
                    ))
                    .color(EoloTheme::text_primary(dark))
                    .strong(),
                );
                ui.separator();
                ui.label(
                    RichText::new(loc.t("about_authors"))
                        .color(EoloTheme::text_primary(dark))
                        .strong(),
                );
            });
            ui.label(
                RichText::new(loc.t("about_description"))
                    .color(EoloTheme::text_secondary(dark)),
            );
            ui.add_space(4.0);
            let warn_bg = if dark {
                Color32::from_rgb(65, 42, 16)
            } else {
                Color32::from_rgb(255, 243, 205)
            };
            egui::Frame::NONE
                .fill(warn_bg)
                .stroke(egui::Stroke::new(1.0, EoloTheme::overpressure_red(dark)))
                .corner_radius(4.0)
                .inner_margin(egui::Margin::symmetric(8, 5))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(loc.tr(
                            "⚠️ USO EXCLUSIVO PARA ESTUDO ACADÊMICO: Aplicação didática sujeita a possíveis erros. Não serve para decisões definitivas em produção; confira manualmente ou com ferramentas consolidadas de mercado.",
                            "⚠️ ACADEMIC STUDY PURPOSE ONLY: Educational application subject to possible errors. Not for definitive production decisions; verify results manually or with established industry tools.",
                        ))
                        .size(11.5)
                        .color(EoloTheme::text_primary(dark))
                        .strong(),
                    );
                });
        });

        ui.add_space(6.0);

        // Barra de Pesquisa Interativa nas Abas
        ui.horizontal(|ui| {
            ui.label("🔍");
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text(loc.t("search_placeholder"))
                    .desired_width(260.0),
            );
            if resp.changed() {
                self.current_hit = 0;
                self.scroll_to_current_hit = true;
            }
            if !self.search_query.is_empty() && ui.button("✖").clicked() {
                self.search_query.clear();
                self.current_hit = 0;
            }

            if !query_trimmed.is_empty() {
                ui.separator();
                if ui
                    .add_enabled(self.total_hits > 0, egui::Button::new("◀"))
                    .clicked()
                {
                    if self.current_hit == 0 {
                        self.current_hit = self.total_hits.saturating_sub(1);
                    } else {
                        self.current_hit -= 1;
                    }
                    self.scroll_to_current_hit = true;
                }
                if ui
                    .add_enabled(self.total_hits > 0, egui::Button::new("▶"))
                    .clicked()
                {
                    self.current_hit = (self.current_hit + 1) % self.total_hits.max(1);
                    self.scroll_to_current_hit = true;
                }

                let badge_color = if self.total_hits > 0 {
                    EoloTheme::success_green(dark)
                } else {
                    EoloTheme::overpressure_red(dark)
                };
                let label_txt = if self.total_hits > 0 {
                    format!("{} / {}", self.current_hit + 1, self.total_hits)
                } else {
                    "0 / 0".to_string()
                };
                ui.colored_label(badge_color, RichText::new(label_txt).strong());
            }
        });

        ui.add_space(4.0);

        // Barra de Abas (9 Abas Completas igual ao Huginn_rs)
        ui.horizontal_wrapped(|ui| {
            let abas = [
                (AbaSobre::Sobre, loc.t("about_tab_about")),
                (AbaSobre::Historico, loc.t("about_tab_history")),
                (AbaSobre::Clc, loc.t("about_tab_clc")),
                (AbaSobre::Copyright, loc.t("about_tab_copyright")),
                (AbaSobre::Eula, loc.t("about_tab_eula")),
                (AbaSobre::Licencas, loc.t("about_tab_licenses")),
                (AbaSobre::Avisos, loc.t("about_tab_notices")),
                (AbaSobre::Privacidade, loc.t("about_tab_privacy")),
                (AbaSobre::Release, loc.t("about_tab_release")),
            ];

            for (aba, rotulo) in abas {
                if ui
                    .selectable_label(
                        self.aba_ativa == aba,
                        RichText::new(rotulo)
                            .color(EoloTheme::text_primary(dark))
                            .strong(),
                    )
                    .clicked()
                {
                    self.aba_ativa = aba;
                    self.current_hit = 0;
                    self.scroll_to_current_hit = true;
                }
            }
        });

        ui.separator();

        // Área de Conteúdo com Rolagem e Destaque de Busca em Amarelo (contida em região fixa sem auto-expansão Y)
        let footer_h = 38.0;
        let available_h = (ui.available_height() - footer_h).max(160.0);
        let mut hit_counter = 0usize;
        let target_hit = self.current_hit;
        let should_scroll = self.scroll_to_current_hit;

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), available_h),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ScrollArea::vertical()
                    .id_salt(("scroll_about", self.aba_ativa as u32))
                    .max_height(available_h)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.aba_ativa == AbaSobre::Licencas {
                            self.renderizar_aba_licencas(
                                ui,
                                loc,
                                dark,
                                &query_trimmed,
                                &mut hit_counter,
                                target_hit,
                                should_scroll,
                            );
                        } else {
                            Self::renderizar_texto_com_destaque(
                                ui,
                                &texto_aba,
                                &query_trimmed,
                                &mut hit_counter,
                                target_hit,
                                should_scroll,
                            );
                        }
                    });
            },
        );

        self.scroll_to_current_hit = false;

        ui.separator();
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(loc.t("close")).clicked() {
                *close_clicked = true;
            }
        });
    }

    fn contar_ocorrencias(texto: &str, query_lower: &str) -> usize {
        if query_lower.is_empty() {
            return 0;
        }
        let lower = texto.to_lowercase();
        lower.matches(query_lower).count()
    }

    fn renderizar_texto_com_destaque(
        ui: &mut Ui,
        texto: &str,
        query_lower: &str,
        hit_counter: &mut usize,
        target_hit: usize,
        should_scroll: bool,
    ) {
        if query_lower.is_empty() {
            ui.label(RichText::new(texto).monospace().size(12.5));
            return;
        }

        let normal_color = ui.visuals().text_color();
        for linha in texto.lines() {
            let linha_lower = linha.to_lowercase();
            if !linha_lower.contains(query_lower) {
                ui.label(RichText::new(linha).monospace().size(12.5));
                continue;
            }

            let mut job = LayoutJob::default();
            let mut last_idx = 0usize;
            let mut contem_target = false;

            for (idx, matched) in linha_lower.match_indices(query_lower) {
                if idx > last_idx {
                    if let Some(prefix) = linha.get(last_idx..idx) {
                        job.append(
                            prefix,
                            0.0,
                            TextFormat {
                                font_id: FontId::monospace(12.5),
                                color: normal_color,
                                ..Default::default()
                            },
                        );
                    }
                }

                let is_active = *hit_counter == target_hit;
                if is_active {
                    contem_target = true;
                }
                let bg = if is_active {
                    Color32::from_rgb(255, 165, 0)
                } else {
                    Color32::from_rgb(255, 235, 59)
                };

                let slice_end = idx + matched.len();
                let original_slice = linha.get(idx..slice_end).unwrap_or(matched);
                job.append(
                    original_slice,
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(12.5),
                        color: Color32::BLACK,
                        background: bg,
                        ..Default::default()
                    },
                );

                *hit_counter += 1;
                last_idx = slice_end;
            }

            if last_idx < linha.len() {
                if let Some(suffix) = linha.get(last_idx..) {
                    job.append(
                        suffix,
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(12.5),
                            color: normal_color,
                            ..Default::default()
                        },
                    );
                }
            }

            let resp = ui.label(job);
            if contem_target && should_scroll {
                resp.scroll_to_me(Some(egui::Align::Center));
            }
        }
    }

    fn arquivos_licencas() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "GNU Affero General Public License v3.0 (Eolo Platform Source Code)",
                "GNU AFFERO GENERAL PUBLIC LICENSE Version3.txt",
            ),
            (
                "MIT License (egui, eframe, serde, rusqlite, rfd, chrono, image, winres)",
                "MIT License.txt",
            ),
            (
                "Apache License Version 2.0 (Rust Standard Library, wgpu, winit, windows-sys)",
                "Apache License Version 2.0.txt",
            ),
            (
                "SQLite Public Domain Dedication (Embedded SQLite Engine)",
                "SQLite Is Public Domain.txt",
            ),
            (
                "BSD 3-Clause License (tiny-skia, tiny-skia-path, moxcms, pxfm)",
                "BSD License Clause-3.txt",
            ),
            (
                "BSD 2-Clause License (arrayref, mach2)",
                "BSD License Clause-2.txt",
            ),
            ("ISC License (libloading)", "ISC LICENSE.txt"),
            (
                "Zlib License (foldhash, slotmap, zlib-rs)",
                "Zlib License.txt",
            ),
            (
                "Boost Software License 1.0 (clipboard-win, error-code)",
                "Boost Software License 1.0.txt",
            ),
            (
                "Unicode License v3 (icu_*, zerovec, yoke, tinystr, litemap, writeable)",
                "Unicode License v3.txt",
            ),
            (
                "SIL Open Font License 1.1 (Embedded Fonts — epaint_default_fonts)",
                "SIL Open Font License 1.1.txt",
            ),
            (
                "Ubuntu Font Licence 1.0 (Ubuntu Regular & Light — epaint_default_fonts)",
                "Ubuntu Font Licence 1.0.txt",
            ),
            (
                "GNU General Public License v3.0 (Reference Copy)",
                "GNU GENERAL PUBLIC LICENSE Version 3.txt",
            ),
            (
                "GNU Lesser General Public License v3.0 (Reference Copy)",
                "GNU LESSER GENERAL PUBLIC LICENSE Version 3.txt",
            ),
            (
                "GNU Lesser General Public License v2.1 (Reference Copy)",
                "GNU LESSER GENERAL PUBLIC LICENSE Version 2.1.txt",
            ),
        ]
    }

    fn carregar_licenca(filename: &str) -> String {
        let rel = format!("LICENSES/{}", filename);
        if let Some(p) = resolver_caminho_asset(&rel) {
            if let Ok(content) = std::fs::read_to_string(p) {
                return content;
            }
        }
        format!("License file: {}", filename)
    }

    #[allow(clippy::too_many_arguments)]
    fn renderizar_aba_licencas(
        &self,
        ui: &mut Ui,
        loc: &Localizer,
        dark: bool,
        query_lower: &str,
        hit_counter: &mut usize,
        target_hit: usize,
        should_scroll: bool,
    ) {
        ui.label(loc.t("about_licenses_intro"));
        ui.add_space(8.0);

        ui.label(
            RichText::new(loc.t("about_official_websites"))
                .strong()
                .size(14.0)
                .color(EoloTheme::accent_blue(dark)),
        );
        ui.add_space(4.0);
        let links = [
            ("Rust Programming Language", "https://www.rust-lang.org"),
            ("egui & eframe Immediate Mode GUI", "https://github.com/emilk/egui"),
            ("SQLite Relational Engine", "https://www.sqlite.org"),
            ("rusqlite (Rust SQLite Bindings)", "https://github.com/rusqlite/rusqlite"),
            ("Serde Serialization Framework", "https://serde.rs"),
            ("rfd (Rusty File Dialogs)", "https://github.com/PolyMeilex/rfd"),
            ("ABNT (Associação Brasileira de Normas Técnicas)", "https://www.abnt.org.br"),
            ("ABNT Catálogo (Consulta Oficial de Normas)", "https://www.abntcatalogo.com.br"),
        ];
        for (nome, url) in links {
            ui.horizontal(|ui| {
                ui.label("•");
                ui.label(
                    RichText::new(nome)
                        .color(EoloTheme::text_primary(dark))
                        .strong(),
                );
                ui.label("—");
                ui.hyperlink(url);
            });
        }

        ui.add_space(10.0);
        ui.separator();
        ui.label(
            RichText::new(loc.t("about_full_licenses"))
                .strong()
                .size(14.0)
                .color(EoloTheme::accent_blue(dark)),
        );
        ui.add_space(6.0);

        for (i, (titulo, fname)) in Self::arquivos_licencas().iter().enumerate() {
            let txt = Self::carregar_licenca(fname);
            let open_by_default =
                i == 0 || (!query_lower.is_empty() && txt.to_lowercase().contains(query_lower));
            egui::CollapsingHeader::new(
                RichText::new(*titulo)
                    .color(EoloTheme::text_primary(dark))
                    .strong(),
            )
                .default_open(open_by_default)
                .show(ui, |ui| {
                    Self::renderizar_texto_com_destaque(
                        ui,
                        &txt,
                        query_lower,
                        hit_counter,
                        target_hit,
                        should_scroll,
                    );
                });
            ui.add_space(4.0);
        }
    }

    fn carregar_texto_aba(&self, aba: AbaSobre, lang_code: &str) -> String {
        let sufixo = match lang_code {
            "en_US" => "en_US",
            _ => "pt_BR",
        };

        let candidatos_relativos: Vec<String> = match aba {
            AbaSobre::Sobre => vec![
                format!("ABOUT/ABOUT_{}.txt", sufixo),
                "ABOUT/ABOUT_pt_BR.txt".to_string(),
            ],
            AbaSobre::Historico => vec![
                format!("ABOUT/History_APP_{}.txt", sufixo),
                "ABOUT/History_APP_pt_BR.txt".to_string(),
            ],
            AbaSobre::Clc => vec![
                format!("CLC/CLC_{} - Eolo.txt", sufixo),
                "CLC/CLC_pt_BR - Eolo.txt".to_string(),
            ],
            AbaSobre::Copyright => vec![
                match sufixo {
                    "en_US" => "COPYRIGHT/COPYRIGHT AND TRADEMARK NOTICE_en_US.txt".to_string(),
                    _ => "COPYRIGHT/AVISO DE COPYRIGHT E MARCA REGISTRA_pt_BR.txt".to_string(),
                },
                "COPYRIGHT/AVISO DE COPYRIGHT E MARCA REGISTRA_pt_BR.txt".to_string(),
            ],
            AbaSobre::Eula => vec![
                format!("EULA/EULA_{} - Eolo.txt", sufixo),
                "EULA/EULA_pt_BR - Eolo.txt".to_string(),
            ],
            AbaSobre::Licencas => vec![
                "LICENSES/GNU AFFERO GENERAL PUBLIC LICENSE Version3.txt".to_string(),
                "LICENSES/MIT License.txt".to_string(),
            ],
            AbaSobre::Avisos => vec![
                format!("NOTICES/NOTICE_{}.txt", sufixo),
                "NOTICES/NOTICE_pt_BR.txt".to_string(),
            ],
            AbaSobre::Privacidade => vec![
                format!("PRIVACY_POLICY/Privacy_Policy_{}.txt", sufixo),
                "PRIVACY_POLICY/Privacy_Policy_pt_BR.txt".to_string(),
            ],
            AbaSobre::Release => vec![
                format!("RELEASE/RELEASE NOTES_{}.txt", sufixo),
                "RELEASE/RELEASE NOTES_pt_BR.txt".to_string(),
            ],
        };

        for rel in candidatos_relativos {
            if let Some(p) = resolver_caminho_asset(&rel) {
                if let Ok(conteudo) = std::fs::read_to_string(p) {
                    return conteudo;
                }
            }
        }

        // Fallback embutido em tempo de compilação caso os arquivos de asset não sejam localizados em disco
        // Mantém marcador de versão sincronizado com o utilitário version-editor:
        let _version_marker = "Versão: v2026.11.8.0";
        match (aba, sufixo) {
            (AbaSobre::Sobre, "en_US") => include_str!("../../assets/ABOUT/ABOUT_en_US.txt").to_string(),
            (AbaSobre::Sobre, _) => include_str!("../../assets/ABOUT/ABOUT_pt_BR.txt").to_string(),
            (AbaSobre::Historico, "en_US") => include_str!("../../assets/ABOUT/History_APP_en_US.txt").to_string(),
            (AbaSobre::Historico, _) => include_str!("../../assets/ABOUT/History_APP_pt_BR.txt").to_string(),
            (AbaSobre::Clc, "en_US") => include_str!("../../assets/CLC/CLC_en_US - Eolo.txt").to_string(),
            (AbaSobre::Clc, _) => include_str!("../../assets/CLC/CLC_pt_BR - Eolo.txt").to_string(),
            (AbaSobre::Copyright, "en_US") => include_str!("../../assets/COPYRIGHT/COPYRIGHT AND TRADEMARK NOTICE_en_US.txt").to_string(),
            (AbaSobre::Copyright, _) => include_str!("../../assets/COPYRIGHT/AVISO DE COPYRIGHT E MARCA REGISTRA_pt_BR.txt").to_string(),
            (AbaSobre::Eula, "en_US") => include_str!("../../assets/EULA/EULA_en_US - Eolo.txt").to_string(),
            (AbaSobre::Eula, _) => include_str!("../../assets/EULA/EULA_pt_BR - Eolo.txt").to_string(),
            (AbaSobre::Licencas, _) => include_str!("../../assets/LICENSES/GNU AFFERO GENERAL PUBLIC LICENSE Version3.txt").to_string(),
            (AbaSobre::Avisos, "en_US") => include_str!("../../assets/NOTICES/NOTICE_en_US.txt").to_string(),
            (AbaSobre::Avisos, _) => include_str!("../../assets/NOTICES/NOTICE_pt_BR.txt").to_string(),
            (AbaSobre::Privacidade, "en_US") => include_str!("../../assets/PRIVACY_POLICY/Privacy_Policy_en_US.txt").to_string(),
            (AbaSobre::Privacidade, _) => include_str!("../../assets/PRIVACY_POLICY/Privacy_Policy_pt_BR.txt").to_string(),
            (AbaSobre::Release, "en_US") => include_str!("../../assets/RELEASE/RELEASE NOTES_en_US.txt").to_string(),
            (AbaSobre::Release, _) => include_str!("../../assets/RELEASE/RELEASE NOTES_pt_BR.txt").to_string(),
        }
    }
}
