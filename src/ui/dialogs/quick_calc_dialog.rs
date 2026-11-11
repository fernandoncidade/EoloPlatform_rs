use egui::RichText;
use crate::locale::Localizer;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;

pub struct QuickCalcDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub input_v0: f64,
    pub input_s1: f64,
    pub input_s2: f64,
    pub input_s3: f64,
}

impl Default for QuickCalcDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            raise_z_order: false,
            maximized: false,
            minimized: false,
            input_v0: 45.0,
            input_s1: 1.0,
            input_s2: 1.0,
            input_s3: 1.0,
        }
    }
}

impl QuickCalcDialog {
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

        let mut v0 = self.input_v0;
        let mut s1 = self.input_s1;
        let mut s2 = self.input_s2;
        let mut s3 = self.input_s3;

        show_modeless_dialog(
            ctx,
            "eolo_dialog_quick_calc",
            loc.tr(
                "🧮 Calculadora Rápida de Vk e Pressão Dinâmica q",
                "🧮 Quick Calculator for Vk & Dynamic Pressure q",
            ),
            [450.0, 350.0],
            [380.0, 310.0],
            dark_mode,
            &mut state,
            |ui, close_clicked| {
                let dark = dark_mode;
                ui.label(loc.tr(
                    "Cálculo instantâneo segundo a equação q = 0,613 · Vk² da NBR 6123:2023:",
                    "Instant calculation according to equation q = 0.613 · Vk² of NBR 6123:2023:",
                ));
                ui.separator();

                egui::Grid::new("quick_calc_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(loc.tr(
                            "Velocidade Básica V0 (m/s):",
                            "Basic Wind Speed V0 (m/s):",
                        ));
                        ui.add(egui::DragValue::new(&mut v0).speed(0.5).range(20.0..=60.0));
                        ui.end_row();

                        ui.label(loc.tr(
                            "Fator Topográfico S1:",
                            "Topographic Factor S1:",
                        ));
                        ui.add(egui::DragValue::new(&mut s1).speed(0.01).range(0.8..=2.5));
                        ui.end_row();

                        ui.label(loc.tr(
                            "Fator Rugosidade S2:",
                            "Roughness Factor S2:",
                        ));
                        ui.add(egui::DragValue::new(&mut s2).speed(0.01).range(0.5..=2.0));
                        ui.end_row();

                        ui.label(loc.tr(
                            "Fator Estatístico S3:",
                            "Statistical Factor S3:",
                        ));
                        ui.add(egui::DragValue::new(&mut s3).speed(0.01).range(0.8..=1.2));
                        ui.end_row();
                    });

                ui.separator();

                let vk = v0 * s1 * s2 * s3;
                let vk_kmh = vk * 3.6;
                let q_pa = 0.613 * vk * vk;
                let q_kn_m2 = q_pa / 1000.0;
                let q_kgf_m2 = q_pa / 9.80665;

                let txt_color = EoloTheme::text_primary(dark);
                ui.group(|ui| {
                    ui.label(
                        RichText::new(format!("Vk = {:.2} m/s  ({:.1} km/h)", vk, vk_kmh))
                            .color(txt_color)
                            .strong()
                            .size(15.0),
                    );
                    ui.label(
                        RichText::new(format!("q = {:.1} Pa", q_pa))
                            .color(EoloTheme::accent_blue(dark))
                            .strong(),
                    );
                    ui.label(
                        RichText::new(format!("q = {:.3} kN/m² (kPa)", q_kn_m2))
                            .color(txt_color)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(format!("q = {:.2} kgf/m²", q_kgf_m2))
                            .color(txt_color)
                            .strong(),
                    );
                });

                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(loc.t("close")).clicked() {
                        *close_clicked = true;
                    }
                });
            },
        );

        self.input_v0 = v0;
        self.input_s1 = s1;
        self.input_s2 = s2;
        self.input_s3 = s3;
        self.is_open = state.is_open;
        self.raise_z_order = state.raise_z_order;
        self.maximized = state.maximized;
        self.minimized = state.minimized;
    }
}
