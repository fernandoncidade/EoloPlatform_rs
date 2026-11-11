use eframe::App;
use egui::{CentralPanel, CornerRadius, Frame, Margin, RichText, Stroke};

use crate::core::project::EoloProject;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::locale::Localizer;
use crate::ui::dialogs::*;
use crate::ui::menu_bar::render_menu_bar;
use crate::ui::tabs::*;
use crate::ui::theme::EoloTheme;
use crate::ui::toolbar::render_toolbar;

pub struct EoloApp {
    pub project: EoloProject,
    pub envelope_results: WindEnvelopeResults,
    pub localizer: Localizer,
    pub dark_mode: bool,
    pub active_tab: usize,

    // Estados locais das abas
    pub tab1_state: LocationTabState,
    pub tab4_state: CanvasTabState,
    pub tab5_state: MemorialTabState,

    // Caixas de diálogo Modeless / flutuantes (com Z-Order)
    pub dialog_about: AboutDialog,
    pub dialog_manual: ManualDialog,
    pub dialog_audit: AuditDialog,
    pub dialog_quick_calc: QuickCalcDialog,
    pub dialog_units: UnitsDialog,
    pub dialog_settings: SettingsDialog,
}

impl Default for EoloApp {
    fn default() -> Self {
        let mut project = EoloProject::default();
        project.sync_parameters();
        let envelope_results = WindEnvelopeResults::compute(&project);

        Self {
            project,
            envelope_results,
            localizer: Localizer::default(),
            dark_mode: true,
            active_tab: 0,
            tab1_state: LocationTabState::default(),
            tab4_state: CanvasTabState::default(),
            tab5_state: MemorialTabState::default(),
            dialog_about: AboutDialog::default(),
            dialog_manual: ManualDialog::default(),
            dialog_audit: AuditDialog::default(),
            dialog_quick_calc: QuickCalcDialog::default(),
            dialog_units: UnitsDialog::default(),
            dialog_settings: SettingsDialog::default(),
        }
    }
}

impl App for EoloApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // 1. Aplica tema visual dinâmico em tempo de execução e sincroniza o Ui raiz imediatamente
        EoloTheme::apply(&ctx, self.dark_mode);
        *ui.style_mut() = (*ctx.global_style()).clone();
        let dark = self.dark_mode;

        // Atualiza dinamicamente o título da janela principal no viewport do S.O.
        let main_title = self.localizer.tr(
            "Eolo Platform — Ações de Vento em Edificações e Coberturas (Estudo Acadêmico • Ref. ABNT NBR 6123:2023)",
            "Eolo Platform — Wind Actions on Buildings and Roofs (Academic Study • Ref. ABNT NBR 6123:2023)",
        );
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(main_title.to_string()));

        // 2. Atalhos Globais de Teclado (Modeless + Z-Order Raise)
        if ctx.input(|i| i.key_pressed(egui::Key::F1) || (i.modifiers.ctrl && i.key_pressed(egui::Key::M))) {
            self.dialog_manual.open();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::I)) {
            self.dialog_about.open();
        }

        // 3. Recalcula determinística e instantaneamente todo o modelo de vento
        self.project.sync_parameters();
        self.envelope_results = WindEnvelopeResults::compute(&self.project);

        // 3. Painel Superior: Barra de Menus + Moldura Fina da Toolbar + Abas
        egui::Panel::top("eolo_top_panel").show(ui, |ui| {
            render_menu_bar(
                ui,
                &mut self.project,
                &mut self.localizer,
                &mut self.dark_mode,
                &mut self.active_tab,
                &mut self.dialog_about,
                &mut self.dialog_manual,
                &mut self.dialog_audit,
                &mut self.dialog_quick_calc,
                &mut self.dialog_units,
                &mut self.dialog_settings,
            );

            ui.add_space(3.0);

            // Todos os widgets abaixo da barra de menus ficam dentro de moldura retangular fina
            render_toolbar(
                ui,
                &mut self.project,
                &self.envelope_results,
                &mut self.dark_mode,
                &mut self.localizer,
            );

            ui.add_space(3.0);

            // Barra de Abas Principais de Engenharia dentro de moldura fina responsiva
            let tab_frame = Frame::NONE
                .fill(ui.visuals().widgets.noninteractive.bg_fill)
                .stroke(Stroke::new(
                    1.0,
                    ui.visuals().widgets.noninteractive.bg_stroke.color,
                ))
                .corner_radius(CornerRadius::same(4))
                .inner_margin(Margin::symmetric(8, 4));

            tab_frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    let tab_keys = [
                        ("📍", "tab1_title"),
                        ("🏗️", "tab2_title"),
                        ("🚪", "tab3_title"),
                        ("🌐", "tab4_title"),
                        ("📋", "tab5_title"),
                    ];

                    for (idx, (icon, key)) in tab_keys.iter().enumerate() {
                        let is_selected = self.active_tab == idx;
                        let label = format!("{} {}", icon, self.localizer.t(key));
                        let text = if is_selected {
                            RichText::new(label)
                                .strong()
                                .size(13.5)
                                .color(EoloTheme::text_primary(dark))
                        } else {
                            RichText::new(label)
                                .size(13.0)
                                .color(EoloTheme::text_primary(dark))
                        };
                        if ui.selectable_label(is_selected, text).clicked() {
                            self.active_tab = idx;
                        }
                        if idx < 4 {
                            ui.separator();
                        }
                    }
                });
            });
            ui.add_space(2.0);
        });

        // 4. Barra de Status Inferior (Telemetria Normativa e Conformidade)
        egui::Panel::bottom("eolo_status_bar").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(format!(
                        "📌 {}: {}",
                        self.localizer.tr("Projeto", "Project"),
                        self.project.metadata.title
                    ))
                    .size(11.5)
                    .color(EoloTheme::text_primary(dark)),
                );
                ui.separator();
                ui.label(
                    RichText::new(format!(
                        "🌍 {}: {} / {} (V0 = {:.0} m/s)",
                        self.localizer.tr("Local", "Location"),
                        self.project.metadata.location_city,
                        self.project.metadata.location_state,
                        self.project.wind.v0
                    ))
                    .size(11.5)
                    .color(EoloTheme::text_primary(dark)),
                );
                ui.separator();
                ui.label(
                    RichText::new(format!(
                        "🌪️ {}: {:.1} kN | {}: {:.1} kN·m",
                        self.localizer.tr("Arraste Global Fa", "Global Drag Fa"),
                        self.envelope_results.total_drag_force_fa_kn,
                        self.localizer
                            .tr("Momento Tombamento", "Overturning Moment"),
                        self.envelope_results.overturning_moment_kn_m
                    ))
                    .size(11.5)
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();
                ui.label(
                    RichText::new(self.localizer.tr(
                        "🔒 100% Offline • Estudo Acadêmico (Ref. NBR 6123:2023) • Conferir Manualmente",
                        "🔒 100% Offline • Academic Study Only (Ref. NBR 6123:2023) • Verify Manually",
                    ))
                    .size(11.5)
                    .color(EoloTheme::success_green(dark))
                    .strong(),
                );

                // Gerenciador Modeless Z-Order: Barra de Tarefas de Janelas Secundárias Ativas
                let any_open = self.dialog_manual.is_open
                    || self.dialog_about.is_open
                    || self.dialog_audit.is_open
                    || self.dialog_quick_calc.is_open
                    || self.dialog_units.is_open
                    || self.dialog_settings.is_open;

                if any_open {
                    ui.separator();
                    ui.label(
                        RichText::new(
                            self.localizer
                                .tr("🪟 Janelas (Z-Order):", "🪟 Windows (Z-Order):"),
                        )
                        .size(11.5)
                        .strong()
                        .color(EoloTheme::text_secondary(dark)),
                    );

                    let hover_z = self.localizer.tr(
                        "Clique para elevar o Z-Order (trazer esta janela Modeless para frente)",
                        "Click to raise Z-Order (bring this Modeless window to the front)",
                    );

                    if self.dialog_manual.is_open
                        && ui
                            .small_button(self.localizer.tr("📖 Manual", "📖 Manual"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_manual.open();
                    }
                    if self.dialog_about.is_open
                        && ui
                            .small_button(self.localizer.tr("ℹ️ Sobre", "ℹ️ About"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_about.open();
                    }
                    if self.dialog_audit.is_open
                        && ui
                            .small_button(self.localizer.tr("✅ Auditoria", "✅ Audit"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_audit.open();
                    }
                    if self.dialog_quick_calc.is_open
                        && ui
                            .small_button(self.localizer.tr("🧮 Calc Vk/q", "🧮 Calc Vk/q"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_quick_calc.open();
                    }
                    if self.dialog_units.is_open
                        && ui
                            .small_button(self.localizer.tr("⚖️ Unidades", "⚖️ Units"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_units.open();
                    }
                    if self.dialog_settings.is_open
                        && ui
                            .small_button(self.localizer.tr("⚙️ Config.", "⚙️ Settings"))
                            .on_hover_text(hover_z)
                            .clicked()
                    {
                        self.dialog_settings.open();
                    }
                }
            });
        });

        // 5. Painel Central da Aba Ativa
        CentralPanel::default().show(ui, |ui| match self.active_tab {
            0 => render_tab_location(
                ui,
                &mut self.project,
                &self.envelope_results,
                &mut self.tab1_state,
                &self.localizer,
            ),
            1 => render_tab_geometry(
                ui,
                &mut self.project,
                &self.envelope_results,
                &self.localizer,
            ),
            2 => render_tab_openings(
                ui,
                &mut self.project,
                &self.envelope_results,
                &self.localizer,
            ),
            3 => render_tab_canvas(
                ui,
                &mut self.project,
                &self.envelope_results,
                &mut self.tab4_state,
                &self.localizer,
            ),
            4 => render_tab_memorial(
                ui,
                &self.project,
                &self.envelope_results,
                &mut self.tab5_state,
                &self.localizer,
            ),
            _ => {}
        });

        // 6. Renderização das Janelas Modeless Independentes / Utilitários (com controle de Z-Order e janela Pai = ROOT)
        self.dialog_about.show(&ctx, &self.localizer, self.dark_mode);
        self.dialog_manual.show(&ctx, &self.localizer, self.dark_mode);
        self.dialog_audit.show(&ctx, &self.localizer, self.dark_mode);
        self.dialog_quick_calc
            .show(&ctx, &self.localizer, self.dark_mode);
        self.dialog_units.show(&ctx, &self.localizer, self.dark_mode);
        self.dialog_settings.show(
            &ctx,
            &mut self.project,
            &mut self.dark_mode,
            &mut self.localizer.current_lang,
        );
    }
}
