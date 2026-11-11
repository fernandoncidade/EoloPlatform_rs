use crate::locale::manual::{obter_secoes_manual, ManualSection};
use crate::locale::Localizer;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;
use egui::{text::LayoutJob, Color32, FontId, RichText, ScrollArea, TextFormat, Ui};

#[derive(Default)]
pub struct ManualDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub secao_selecionada: usize,
    pub termo_busca: String,
    pub current_hit: usize,
    pub total_hits: usize,
    scroll_to_current_hit: bool,
}

impl ManualDialog {
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
            "eolo_dialog_manual",
            loc.t("manual_window_title"),
            [920.0, 620.0],
            [680.0, 480.0],
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
        let secoes = obter_secoes_manual(lang_code);

        if self.secao_selecionada >= secoes.len() {
            self.secao_selecionada = 0;
        }

        let query_lower = self.termo_busca.trim().to_lowercase();

        // Contagem de ocorrências na seção selecionada
        let hits_secao_atual = if !query_lower.is_empty() && !secoes.is_empty() {
            Self::contar_hits_secao(&secoes[self.secao_selecionada], &query_lower)
        } else {
            0
        };
        self.total_hits = hits_secao_atual;
        if self.total_hits == 0 || self.current_hit >= self.total_hits {
            self.current_hit = 0;
        }

        // Cabeçalho e Barra de Pesquisa
        ui.horizontal_wrapped(|ui| {
            ui.heading(
                RichText::new(loc.t("manual_header_title"))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
            );
            ui.add_space(12.0);
            ui.label("🔍");
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.termo_busca)
                    .hint_text(loc.t("manual_search_placeholder"))
                    .desired_width(260.0),
            );
            if resp.changed() {
                self.current_hit = 0;
                self.scroll_to_current_hit = true;
                if !self.termo_busca.trim().is_empty() {
                    let q = self.termo_busca.trim().to_lowercase();
                    if Self::contar_hits_secao(&secoes[self.secao_selecionada], &q) == 0 {
                        if let Some((idx, _)) = secoes
                            .iter()
                            .enumerate()
                            .find(|(_, s)| Self::contar_hits_secao(s, &q) > 0)
                        {
                            self.secao_selecionada = idx;
                        }
                    }
                }
            }
            if !self.termo_busca.is_empty() && ui.button("✖").clicked() {
                self.termo_busca.clear();
                self.current_hit = 0;
            }

            if !query_lower.is_empty() {
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
        ui.separator();

        let available_h = (ui.available_height() - 42.0).max(160.0);
        let mut hit_counter = 0usize;
        let target_hit = self.current_hit;
        let should_scroll = self.scroll_to_current_hit;

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), available_h),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                // Medição dinâmica em tempo de execução da largura máxima necessária para a Coluna 1
                let title_font = FontId::proportional(13.0);
                let item_font = FontId::proportional(12.5);
                let mut max_str_width: f32 = ui
                    .painter()
                    .layout_no_wrap(
                        loc.t("manual_sections_title").to_string(),
                        title_font,
                        Color32::WHITE,
                    )
                    .size()
                    .x;

                for sec in secoes {
                    let hits_na_sec = if !query_lower.is_empty() {
                        Self::contar_hits_secao(sec, &query_lower)
                    } else {
                        0
                    };
                    let label_text = if hits_na_sec > 0 {
                        format!("{} [{}]", sec.title, hits_na_sec)
                    } else {
                        sec.title.to_string()
                    };
                    let w = ui
                        .painter()
                        .layout_no_wrap(label_text, item_font.clone(), Color32::WHITE)
                        .size()
                        .x;
                    if w > max_str_width {
                        max_str_width = w;
                    }
                }

                let total_w = ui.available_width();
                let col1_w = (max_str_width + 48.0).clamp(210.0, (total_w * 0.45).max(210.0));
                let col2_w = (total_w - col1_w - 14.0).max(250.0);

                ui.horizontal(|ui| {
                    // Coluna Esquerda: Lista de Tópicos com Badges de Ocorrências (Largura Dinâmica)
                    ui.allocate_ui_with_layout(
                        egui::vec2(col1_w, available_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(col1_w);
                            ui.label(
                                RichText::new(loc.t("manual_sections_title"))
                                    .strong()
                                    .color(EoloTheme::accent_blue(dark)),
                            );
                            ui.add_space(4.0);

                            ScrollArea::vertical()
                                .id_salt("manual_sidebar_scroll")
                                .max_height(available_h - 24.0)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    for (idx, sec) in secoes.iter().enumerate() {
                                        let hits_na_sec = if !query_lower.is_empty() {
                                            Self::contar_hits_secao(sec, &query_lower)
                                        } else {
                                            0
                                        };

                                        if !query_lower.is_empty()
                                            && hits_na_sec == 0
                                            && self.secao_selecionada != idx
                                        {
                                            continue;
                                        }

                                        let label_text = if hits_na_sec > 0 {
                                            format!("{} [{}]", sec.title, hits_na_sec)
                                        } else {
                                            sec.title.to_string()
                                        };

                                        let selecionado = self.secao_selecionada == idx;
                                        if ui.selectable_label(selecionado, label_text).clicked() {
                                            self.secao_selecionada = idx;
                                            self.current_hit = 0;
                                            self.scroll_to_current_hit = true;
                                        }
                                    }
                                });
                        },
                    );

                    ui.separator();

                    // Coluna Direita: Conteúdo Detalhado da Seção Selecionada (Expansão Integral Dinâmica)
                    ui.allocate_ui_with_layout(
                        egui::vec2(col2_w, available_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(col2_w);
                            ScrollArea::vertical()
                                .id_salt(("manual_content_scroll", self.secao_selecionada))
                                .max_height(available_h)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                            if let Some(sec) = secoes.get(self.secao_selecionada) {
                                Self::renderizar_texto_destacado(
                                    ui,
                                    sec.title,
                                    &query_lower,
                                    17.0,
                                    true,
                                    &mut hit_counter,
                                    target_hit,
                                    should_scroll,
                                );
                                ui.separator();
                                ui.add_space(6.0);

                                for p in sec.paragraphs {
                                    Self::renderizar_texto_destacado(
                                        ui,
                                        p,
                                        &query_lower,
                                        13.5,
                                        false,
                                        &mut hit_counter,
                                        target_hit,
                                        should_scroll,
                                    );
                                    ui.add_space(8.0);
                                }

                                if !sec.bullets.is_empty() {
                                    ui.add_space(4.0);
                                    ui.label(
                                        RichText::new(loc.t("manual_key_points"))
                                            .strong()
                                            .size(14.0)
                                            .color(EoloTheme::accent_blue(dark)),
                                    );
                                    ui.add_space(4.0);
                                    for b in sec.bullets {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(
                                                RichText::new("•")
                                                    .strong()
                                                    .color(EoloTheme::accent_blue(dark)),
                                            );
                                            Self::renderizar_texto_destacado(
                                                ui,
                                                b,
                                                &query_lower,
                                                13.0,
                                                false,
                                                &mut hit_counter,
                                                target_hit,
                                                should_scroll,
                                            );
                                        });
                                        ui.add_space(3.0);
                                    }
                                }

                                if let Some(det) = &sec.details {
                                    ui.add_space(10.0);
                                    egui::Frame::group(ui.style()).show(ui, |ui| {
                                        Self::renderizar_texto_destacado(
                                            ui,
                                            det.title,
                                            &query_lower,
                                            13.5,
                                            true,
                                            &mut hit_counter,
                                            target_hit,
                                            should_scroll,
                                        );
                                        ui.separator();
                                        for item in det.items {
                                            ui.horizontal_wrapped(|ui| {
                                                ui.label(
                                                    RichText::new("▸")
                                                        .color(EoloTheme::success_green(dark)),
                                                );
                                                Self::renderizar_texto_destacado(
                                                    ui,
                                                    item,
                                                    &query_lower,
                                                    12.5,
                                                    false,
                                                    &mut hit_counter,
                                                    target_hit,
                                                    should_scroll,
                                                );
                                            });
                                        }
                                    });
                                }
                            }
                        });
                    },
                );
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

    fn contar_hits_texto(txt: &str, query_lower: &str) -> usize {
        if query_lower.is_empty() {
            return 0;
        }
        txt.to_lowercase().matches(query_lower).count()
    }

    fn contar_hits_secao(sec: &ManualSection, query_lower: &str) -> usize {
        if query_lower.is_empty() {
            return 0;
        }
        let mut total = Self::contar_hits_texto(sec.title, query_lower);
        for p in sec.paragraphs {
            total += Self::contar_hits_texto(p, query_lower);
        }
        for b in sec.bullets {
            total += Self::contar_hits_texto(b, query_lower);
        }
        if let Some(det) = &sec.details {
            total += Self::contar_hits_texto(det.title, query_lower);
            for it in det.items {
                total += Self::contar_hits_texto(it, query_lower);
            }
        }
        total
    }

    #[allow(clippy::too_many_arguments)]
    fn renderizar_texto_destacado(
        ui: &mut Ui,
        texto: &str,
        query_lower: &str,
        font_size: f32,
        bold: bool,
        hit_counter: &mut usize,
        target_hit: usize,
        should_scroll: bool,
    ) {
        let dark = ui.visuals().dark_mode;
        if query_lower.is_empty() {
            let mut rt = RichText::new(texto)
                .size(font_size)
                .color(EoloTheme::text_primary(dark));
            if bold {
                rt = rt.strong();
            }
            ui.label(rt);
            return;
        }

        let texto_lower = texto.to_lowercase();
        if !texto_lower.contains(query_lower) {
            let mut rt = RichText::new(texto)
                .size(font_size)
                .color(EoloTheme::text_primary(dark));
            if bold {
                rt = rt.strong();
            }
            ui.label(rt);
            return;
        }

        let normal_color = ui.visuals().text_color();
        let mut job = LayoutJob::default();
        let mut last_idx = 0usize;
        let mut contem_target = false;

        for (idx, matched) in texto_lower.match_indices(query_lower) {
            if idx > last_idx {
                if let Some(prefix) = texto.get(last_idx..idx) {
                    job.append(
                        prefix,
                        0.0,
                        TextFormat {
                            font_id: FontId::proportional(font_size),
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
            let orig = texto.get(idx..slice_end).unwrap_or(matched);
            job.append(
                orig,
                0.0,
                TextFormat {
                    font_id: FontId::proportional(font_size),
                    color: Color32::BLACK,
                    background: bg,
                    ..Default::default()
                },
            );

            *hit_counter += 1;
            last_idx = slice_end;
        }

        if last_idx < texto.len() {
            if let Some(suffix) = texto.get(last_idx..) {
                job.append(
                    suffix,
                    0.0,
                    TextFormat {
                        font_id: FontId::proportional(font_size),
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
