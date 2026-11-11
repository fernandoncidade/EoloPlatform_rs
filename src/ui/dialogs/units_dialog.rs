use egui::RichText;
use crate::locale::Localizer;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;

pub struct UnitsDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub input_value: f64,
    pub selected_source: usize,
}

impl Default for UnitsDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            raise_z_order: false,
            maximized: false,
            minimized: false,
            input_value: 1.0,
            selected_source: 2, // kN/m²
        }
    }
}

impl UnitsDialog {
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

        let mut val = self.input_value;
        let mut src = self.selected_source;

        show_modeless_dialog(
            ctx,
            "eolo_dialog_units",
            loc.tr(
                "⚖️ Conversor de Unidades de Pressão e Tensão",
                "⚖️ Pressure & Stress Unit Converter",
            ),
            [480.0, 410.0],
            [400.0, 350.0],
            dark_mode,
            &mut state,
            |ui, close_clicked| {
                let dark = dark_mode;
                ui.label(loc.tr(
                    "Conversão exata entre unidades usuais de engenharia de estruturas e vento:",
                    "Exact conversion across standard structural and wind engineering pressure units:",
                ));
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(loc.tr("Valor de Entrada:", "Input Value:"));
                    ui.add(egui::DragValue::new(&mut val).speed(0.1));

                    egui::ComboBox::from_label(loc.tr("Unidade", "Unit"))
                        .selected_text(match src {
                            0 => "Pa (N/m²)",
                            1 => "kPa",
                            2 => "kN/m²",
                            3 => "kgf/m²",
                            4 => "tf/m²",
                            5 => "mmH₂O",
                            6 => "psi (lbf/in²)",
                            _ => "kN/m²",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut src, 0, "Pa (N/m²)");
                            ui.selectable_value(&mut src, 1, "kPa");
                            ui.selectable_value(&mut src, 2, "kN/m²");
                            ui.selectable_value(&mut src, 3, "kgf/m²");
                            ui.selectable_value(&mut src, 4, "tf/m²");
                            ui.selectable_value(&mut src, 5, "mmH₂O");
                            ui.selectable_value(&mut src, 6, "psi (lbf/in²)");
                        });
                });

                // Converte tudo para Pascal
                let val_pa = match src {
                    0 => val,
                    1 => val * 1_000.0,
                    2 => val * 1_000.0,
                    3 => val * 9.80665,
                    4 => val * 9_806.65,
                    5 => val * 9.80665,
                    6 => val * 6_894.757,
                    _ => val * 1_000.0,
                };

                ui.separator();
                ui.heading(
                    RichText::new(loc.tr("Equivalências Convertidas:", "Converted Equivalents:"))
                        .color(EoloTheme::accent_blue(dark))
                        .strong(),
                );

                let txt_color = EoloTheme::text_primary(dark);
                egui::Grid::new("units_results_grid")
                    .striped(true)
                    .spacing([20.0, 6.0])
                    .show(ui, |ui| {
                        ui.label("Pascal (Pa = N/m²):");
                        ui.label(RichText::new(format!("{:.2} Pa", val_pa)).color(txt_color).strong());
                        ui.end_row();

                        ui.label("KiloPascal (kPa):");
                        ui.label(RichText::new(format!("{:.4} kPa", val_pa / 1000.0)).color(txt_color).strong());
                        ui.end_row();

                        ui.label(loc.tr(
                            "Quilonewton por metro quadrado (kN/m²):",
                            "Kilonewton per square meter (kN/m²):",
                        ));
                        ui.label(RichText::new(format!("{:.4} kN/m²", val_pa / 1000.0)).color(txt_color).strong());
                        ui.end_row();

                        ui.label(loc.tr(
                            "Quilograma-força por metro quadrado (kgf/m²):",
                            "Kilogram-force per square meter (kgf/m²):",
                        ));
                        ui.label(RichText::new(format!("{:.2} kgf/m²", val_pa / 9.80665)).color(txt_color).strong());
                        ui.end_row();

                        ui.label(loc.tr(
                            "Tonelada-força por metro quadrado (tf/m²):",
                            "Metric ton-force per square meter (tf/m²):",
                        ));
                        ui.label(RichText::new(format!("{:.5} tf/m²", val_pa / 9806.65)).color(txt_color).strong());
                        ui.end_row();

                        ui.label(loc.tr(
                            "Milímetros de Coluna d'Água (mmH₂O):",
                            "Millimeters of Water Column (mmH₂O):",
                        ));
                        ui.label(RichText::new(format!("{:.2} mmH₂O", val_pa / 9.80665)).color(txt_color).strong());
                        ui.end_row();

                        ui.label(loc.tr(
                            "Libras por polegada quadrada (psi):",
                            "Pounds per square inch (psi):",
                        ));
                        ui.label(RichText::new(format!("{:.5} psi", val_pa / 6894.757)).color(txt_color).strong());
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(loc.t("close")).clicked() {
                        *close_clicked = true;
                    }
                });
            },
        );

        self.input_value = val;
        self.selected_source = src;
        self.is_open = state.is_open;
        self.raise_z_order = state.raise_z_order;
        self.maximized = state.maximized;
        self.minimized = state.minimized;
    }
}
