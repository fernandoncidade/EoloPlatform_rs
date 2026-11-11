use egui::{RichText, ScrollArea};
use crate::locale::Localizer;
use crate::ui::dialogs::modeless::{show_modeless_dialog, ModelessState};
use crate::ui::theme::EoloTheme;

#[derive(Default)]
pub struct AuditDialog {
    pub is_open: bool,
    pub raise_z_order: bool,
    pub maximized: bool,
    pub minimized: bool,
}

impl AuditDialog {
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
            "eolo_dialog_audit",
            loc.tr(
                "✅ Rastreabilidade Acadêmica — Referência ABNT NBR 6123:2023",
                "✅ Academic Traceability — ABNT NBR 6123:2023 Reference",
            ),
            [780.0, 560.0],
            [580.0, 420.0],
            dark_mode,
            &mut state,
            |ui, close_clicked| {
                let dark = dark_mode;
                ui.heading(
                    RichText::new(loc.tr(
                        "Matriz de Rastreabilidade Analítica para Estudo Acadêmico",
                        "Analytical Traceability Matrix for Academic Study",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.label(loc.tr(
                    "Mapeamento referencial das formulações estudadas com base na ABNT NBR 6123:2023 implementadas no código-fonte para fins didáticos e acadêmicos.",
                    "Reference mapping of studied formulations based on ABNT NBR 6123:2023 implemented in the source code for educational and academic study purposes.",
                ));

                ui.add_space(4.0);
                let warn_bg = if dark {
                    egui::Color32::from_rgb(65, 42, 16)
                } else {
                    egui::Color32::from_rgb(255, 243, 205)
                };
                egui::Frame::NONE
                    .fill(warn_bg)
                    .stroke(egui::Stroke::new(1.0, EoloTheme::overpressure_red(dark)))
                    .corner_radius(4.0)
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(loc.tr(
                                "⚠️ AVISO LEGAL: Aplicação exclusiva para estudo acadêmico. Pode conter erros numéricos, simplificações ou divergências. NÃO substitui conferência manual nem ferramentas consolidadas de mercado e NÃO deve ser usada para decisões definitivas em produção.",
                                "⚠️ LEGAL NOTICE: Application strictly for academic study only. May contain numerical errors, simplifications, or discrepancies. Does NOT replace manual checking or consolidated industry tools and MUST NOT be used for definitive production decisions.",
                            ))
                            .size(11.5)
                            .color(EoloTheme::text_primary(dark))
                            .strong(),
                        );
                    });

                ui.separator();

                let available_h = (ui.available_height() - 40.0).max(160.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), available_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ScrollArea::vertical()
                            .id_salt("audit_matrix_scroll")
                            .max_height(available_h)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let items = if loc.is_pt() {
                                    [
                                        ("Ref. 4.2", "Pressão Dinâmica Básica", "q = 0,613 · Vk² (pressão em Pa com Vk em m/s). Implementado em `engine/wind_core.rs`."),
                                        ("Ref. 4.3.1 e 4.3.2", "Coeficientes de Pressão e Forma", "Diferenciação entre Cpe e Ce, e cálculo da pressão efetiva Δp = (Cpe - Cpi) · q."),
                                        ("Ref. 5.1 e Fig. 1", "Velocidade Básica V0", "Valores de referência das isopletas do Brasil (5 m/s) e lista das 49 estações da FAB (Ref. Tabela C.1)."),
                                        ("Ref. 5.2", "Fator Topográfico S1", "Formulação analítica de morros e taludes em função de θt e z/d, com interpolação contínua e limites referenciais."),
                                        ("Ref. 5.3 e Tab. 1/2/3", "Fator S2 (Rugosidade e Dimensões)", "Equação S2 = bm · Fr · (z/10)^p para Categorias I a V e Classes A a C (patamar constante até 10m na Cat. V e 5m nas demais)."),
                                        ("Ref. 5.4 e Tab. 4", "Fator Estatístico S3", "Grupos 1 a 5 (15 a 100 anos), contemplando o fator redutor referencial de 0,92 · S3 para vedações."),
                                        ("Ref. 5.5", "Transição de Rugosidade", "Cálculo das alturas zx e zi e perfil de velocidade na mudança de terreno."),
                                        ("Ref. 6.1 e Tab. 6", "Paredes de Edificações Retangulares", "Coeficientes Ce para vento a 0° e 90° em função de h/b e a/b, com zona de alta sucção cpe_médio em min(0,2b, h)."),
                                        ("Ref. 6.1 e Tab. 7", "Telhados a Duas Águas Simétricos", "Interpolação de Ce para inclinação θ de 0° a 60°, incluindo casos de sucção e sobrepressão para θ entre 15° e 30°."),
                                        ("Ref. Tab. 7 — Nota 1", "Efeito de Arrancamento em Beirais", "Face inferior do beiral com coeficiente da parede (+0,70 ou +0,80) e topo com sucção (-0,80 a -1,20)."),
                                        ("Ref. 6.3 e Tab. 6.3.2", "Pressões Internas Cpi", "Estimativa para aberturas dominantes em barlavento, sotavento e faces laterais por interpolação de razão de áreas."),
                                        ("Ref. 4.3.3 e Fig. 4", "Força Global de Arrasto Fa", "Cálculo de Fa = q · Ca · Ae e momento global de tombamento na base."),
                                    ]
                                } else {
                                    [
                                        ("Ref. 4.2", "Basic Dynamic Wind Pressure", "q = 0.613 · Vk² (pressure in Pa with Vk in m/s). Implemented in `engine/wind_core.rs`."),
                                        ("Ref. 4.3.1 & 4.3.2", "Pressure & Shape Coefficients", "Differentiation between Cpe and Ce, and calculation of net effective pressure Δp = (Cpe - Cpi) · q."),
                                        ("Ref. 5.1 & Fig. 1", "Basic Wind Speed V0", "Reference values for Brazilian isopleths (5 m/s) plus the 49 FAB meteorological stations (Ref. Table C.1)."),
                                        ("Ref. 5.2", "Topographic Factor S1", "Analytical hill and escarpment formulation as a function of θt and z/d, with continuous interpolation."),
                                        ("Ref. 5.3 & Tab. 1/2/3", "Factor S2 (Roughness & Size)", "Equation S2 = bm · Fr · (z/10)^p across Categories I to V and Classes A to C (constant S2 up to 10m in Cat. V and 5m in others)."),
                                        ("Ref. 5.4 & Tab. 4", "Statistical Factor S3", "Occupancy Groups 1 to 5 (15- to 100-year return periods), including the 0.92 · S3 cladding reference reduction."),
                                        ("Ref. 5.5", "Roughness Transition", "Calculation of transition heights zx and zi and wind velocity profile across terrain boundaries."),
                                        ("Ref. 6.1 & Tab. 6", "Rectangular Building Walls", "Coefficients Ce for 0° and 90° wind incidence as a function of h/b and a/b, with high suction zone cpe_mean over min(0.2b, h)."),
                                        ("Ref. 6.1 & Tab. 7", "Symmetric Duo-Pitch Roofs", "Continuous Ce interpolation for pitch θ from 0° to 60°, including dual suction and overpressure cases for θ between 15° and 30°."),
                                        ("Ref. Tab. 7 — Note 1", "Eave Overhang Uplift Effect", "Underside of eave takes windward wall coefficient (+0.70 or +0.80) while top surface experiences suction (-0.80 to -1.20)."),
                                        ("Ref. 6.3 & Tab. 6.3.2", "Internal Pressures Cpi", "Estimation for dominant openings on windward, leeward, and side faces via area ratio interpolation."),
                                        ("Ref. 4.3.3 & Fig. 4", "Global Drag Force Fa", "Calculation of Fa = q · Ca · Ae and global overturning moment at the foundation base."),
                                    ]
                                };

                                for (clause, title, desc) in items {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format!("[{}]", clause))
                                                    .color(EoloTheme::accent_blue(dark))
                                                    .strong(),
                                            );
                                            ui.label(
                                                RichText::new(title)
                                                    .color(EoloTheme::text_primary(dark))
                                                    .strong(),
                                            );
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(
                                                            loc.tr("ESTUDO (CONFERIR)", "STUDY (VERIFY)"),
                                                        )
                                                        .color(EoloTheme::accent_blue(dark))
                                                        .strong(),
                                                    );
                                                },
                                            );
                                        });
                                        ui.label(
                                            RichText::new(desc)
                                                .size(11.5)
                                                .color(EoloTheme::text_secondary(dark)),
                                        );
                                    });
                                }
                            });
                    },
                );

                ui.separator();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(loc.t("close")).clicked() {
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
