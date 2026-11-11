use egui::{RichText, ScrollArea, Ui};

use crate::core::project::EoloProject;
use crate::core::types::RoofTypology;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::locale::Localizer;
use crate::ui::theme::EoloTheme;

pub fn render_tab_geometry(
    ui: &mut Ui,
    project: &mut EoloProject,
    env: &WindEnvelopeResults,
    localizer: &Localizer,
) {
    let dark = ui.visuals().dark_mode;
    let lang = localizer.current_lang;
    let txt_color = EoloTheme::text_primary(dark);
    let sec_color = EoloTheme::text_secondary(dark);

    ScrollArea::vertical().show(ui, |ui| {
        ui.heading(
            RichText::new(localizer.tr(
                "2. Geometria da Edificação, Tipologia da Cobertura e Coeficientes Externos (Cpe / Ce)",
                "2. Building Geometry, Roof Typology & External Coefficients (Cpe / Ce)",
            ))
            .color(EoloTheme::text_primary(dark))
            .strong(),
        );
        ui.add_space(4.0);

        ui.columns(2, |cols| {
            // =========================================================================
            // COLUNA ESQUERDA: Parâmetros Geométricos e Tipologia
            // =========================================================================
            cols[0].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🏗️ Tipologia Estrutural e Dimensões Globais",
                        "🏗️ Structural Typology & Global Dimensions",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                ui.label(
                    RichText::new(localizer.tr(
                        "Selecione a Tipologia da Cobertura:",
                        "Select Roof Typology:",
                    ))
                    .color(txt_color)
                    .strong(),
                );
                for typ in [
                    RoofTypology::DuoPitch,
                    RoofTypology::MonoPitch,
                    RoofTypology::CurvedArch,
                    RoofTypology::CanopyIsolated,
                    RoofTypology::ShedIndustrial,
                ] {
                    ui.radio_value(&mut project.geometry.typology, typ, typ.name_i18n(lang));
                }

                ui.add_space(6.0);
                ui.separator();

                egui::Grid::new("geom_inputs_grid").num_columns(2).spacing([14.0, 8.0]).show(ui, |ui| {
                    ui.label(
                        RichText::new(localizer.tr(
                            "Vão Livre Transversal b (m):",
                            "Transverse Clear Span b (m):",
                        ))
                        .color(txt_color)
                        .strong(),
                    );
                    ui.add(egui::Slider::new(&mut project.geometry.span_b, 4.0..=120.0).suffix(" m"));
                    ui.end_row();

                    ui.label(
                        RichText::new(localizer.tr(
                            "Comprimento Longitudinal a (m):",
                            "Longitudinal Length a (m):",
                        ))
                        .color(txt_color)
                        .strong(),
                    );
                    ui.add(egui::Slider::new(&mut project.geometry.length_a, 6.0..=250.0).suffix(" m"));
                    ui.end_row();

                    ui.label(
                        RichText::new(localizer.tr(
                            "Pé-Direito Livre na Calha h (m):",
                            "Eave Clear Height h (m):",
                        ))
                        .color(txt_color)
                        .strong(),
                    );
                    ui.add(egui::Slider::new(&mut project.geometry.eave_height_h, 2.5..=45.0).suffix(" m"));
                    ui.end_row();

                    ui.label(
                        RichText::new(localizer.tr(
                            "Inclinação do Telhado θ (°):",
                            "Roof Pitch Angle θ (°):",
                        ))
                        .color(txt_color)
                        .strong(),
                    );
                    ui.add(egui::Slider::new(&mut project.geometry.pitch_angle_deg, 0.0..=60.0).suffix("°"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Balanço dos Beirais Laterais ex (m):",
                        "Side Eave Overhang ex (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.eave_overhang_x, 0.0..=4.0).suffix(" m"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Balanço dos Beirais nos Oitões ey (m):",
                        "Gable End Overhang ey (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.eave_overhang_y, 0.0..=4.0).suffix(" m"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Largura do Lanternim na Cumeeira (m):",
                        "Ridge Monitor Width (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.ridge_monitor_width, 0.0..=6.0).suffix(" m"));
                    ui.end_row();
                });

                ui.add_space(6.0);
                ui.separator();
                ui.label(
                    RichText::new(localizer.tr(
                        "🔩 Espaçamento de Pórticos, Terças e Longarinas (Cargas Lineares):",
                        "🔩 Portal Frame, Purlin & Girt Spacing (Linear Loads):",
                    ))
                    .color(txt_color)
                    .strong(),
                );

                egui::Grid::new("spacing_grid").num_columns(2).spacing([14.0, 6.0]).show(ui, |ui| {
                    ui.label(localizer.tr(
                        "Espaçamento entre Pórticos (m):",
                        "Portal Frame Spacing (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.frame_spacing, 3.0..=15.0).suffix(" m"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Espaçamento entre Terças (m):",
                        "Roof Purlin Spacing (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.purlin_spacing, 0.8..=3.5).suffix(" m"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Espaçamento entre Longarinas de Parede (m):",
                        "Wall Girt Spacing (m):",
                    ));
                    ui.add(egui::Slider::new(&mut project.geometry.girt_spacing, 0.8..=4.0).suffix(" m"));
                    ui.end_row();
                });
            });

            // =========================================================================
            // COLUNA DIREITA: Relações Derivadas e Tabelas de Coeficientes Cpe/Ce
            // =========================================================================
            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "📐 Parâmetros Geométricos Derivados",
                        "📐 Derived Geometric Parameters",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                let slope_pct = project.geometry.pitch_angle_deg.to_radians().tan() * 100.0;
                let ridge_rise = project.geometry.total_ridge_height() - project.geometry.eave_height_h;
                egui::Grid::new("derived_geom_grid").striped(true).num_columns(2).spacing([20.0, 6.0]).show(ui, |ui| {
                    ui.label(localizer.tr("Desnível da Cumeeira (h_col):", "Ridge Rise (h_col):"));
                    ui.label(RichText::new(format!("{:.2} m", ridge_rise)).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr("Altura Total na Cumeeira (h_tot):", "Total Ridge Height (h_tot):"));
                    ui.label(
                        RichText::new(format!("{:.2} m", project.geometry.total_ridge_height()))
                            .color(EoloTheme::accent_blue(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Inclinação Equivalente em %:", "Equivalent Roof Slope %:"));
                    ui.label(format!("{:.1}%", slope_pct));
                    ui.end_row();

                    ui.label(localizer.tr("Relação Altura/Vão (h/b):", "Height/Span Ratio (h/b):"));
                    ui.label(RichText::new(format!("{:.3}", project.geometry.relative_height_h_over_b())).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr("Relação de Planta (a/b):", "Plan Aspect Ratio (a/b):"));
                    ui.label(RichText::new(format!("{:.3}", project.geometry.aspect_ratio_a_over_b())).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Largura da Zona de Alta Sucção min(b/5, h):",
                        "High Corner Suction Zone Width min(b/5, h):",
                    ));
                    ui.label(format!("{:.2} m", env.walls.corner_dimension));
                    ui.end_row();
                });
            });

            cols[1].add_space(8.0);

            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "📊 Coeficientes de Pressão Externa — Paredes (Tabela 6)",
                        "📊 External Pressure Coefficients — Walls (Table 6)",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                egui::Grid::new("walls_ce_grid").striped(true).num_columns(3).spacing([16.0, 6.0]).show(ui, |ui| {
                    ui.label(RichText::new(localizer.tr("Zona da Parede", "Wall Zone")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Coeficiente Ce / cpe", "Coefficient Ce / cpe")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Natureza Física", "Physical Action")).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr("Barlavento (Face Frontal):", "Windward (Front Wall):"));
                    ui.label(
                        RichText::new(format!("+{:.2}", env.walls.windward_ce))
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sobrepressão (+)", "Overpressure (+)"));
                    ui.end_row();

                    ui.label(localizer.tr("Sotavento (Face Traseira):", "Leeward (Rear Wall):"));
                    ui.label(
                        RichText::new(format!("{:.2}", env.walls.leeward_ce))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sucção (-)", "Suction (-)"));
                    ui.end_row();

                    ui.label(localizer.tr("Paredes Laterais (Zona A1/B1):", "Side Walls (Zone A1/B1):"));
                    ui.label(
                        RichText::new(format!("{:.2}", env.walls.side_zone1_ce))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sucção de Borda (-)", "Leading Edge Suction (-)"));
                    ui.end_row();

                    ui.label(localizer.tr("Paredes Laterais (Zona A2/B2):", "Side Walls (Zone A2/B2):"));
                    ui.label(format!("{:.2}", env.walls.side_zone2_ce));
                    ui.label(localizer.tr("Sucção Central (-)", "Central Suction (-)"));
                    ui.end_row();

                    ui.label(localizer.tr("Vedação no Canto (cpe_médio):", "Corner Cladding (cpe_mean):"));
                    ui.label(
                        RichText::new(format!("{:.2}", env.walls.corner_cpe_local))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Pico Local de Sucção", "Local Peak Suction"));
                    ui.end_row();
                });
            });

            cols[1].add_space(8.0);

            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🏠 Coeficientes da Cobertura e Beirais (Tabela 7 / Nota 1)",
                        "🏠 Roof & Eave Overhang Coefficients (Table 7 / Note 1)",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                egui::Grid::new("roof_ce_grid").striped(true).num_columns(3).spacing([16.0, 6.0]).show(ui, |ui| {
                    ui.label(RichText::new(localizer.tr("Zona da Cobertura", "Roof Zone")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Coeficiente", "Coefficient")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Efeito", "Effect")).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Água Barlavento — Caso 1 (Sucção):",
                        "Windward Slope — Case 1 (Suction):",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2}", env.roof.windward_slope_suction_ce))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Arrancamento (-)", "Uplift (-)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Água Barlavento — Caso 2 (Pressão):",
                        "Windward Slope — Case 2 (Pressure):",
                    ));
                    ui.label(
                        RichText::new(format!("+{:.2}", env.roof.windward_slope_pressure_ce))
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sobrepressão (+)", "Downward Push (+)"));
                    ui.end_row();

                    ui.label(localizer.tr("Água de Sotavento:", "Leeward Slope:"));
                    ui.label(
                        RichText::new(format!("{:.2}", env.roof.leeward_slope_ce))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Arrancamento (-)", "Uplift (-)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Beiral — Coeficiente Líquido (ΔCp):",
                        "Eave Overhang — Net Coeff. (ΔCp):",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2}", env.eaves.net_uplift_cp))
                            .color(EoloTheme::eave_magenta(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Arrancamento Crítico!", "Critical Uplift!"));
                    ui.end_row();
                });

                ui.add_space(4.0);
                ui.label(
                    RichText::new(localizer.tr(
                        "NBR 6123 Tabela 7 Nota 1: A face inferior do beiral recebe a sobrepressão da parede de barlavento (+0,7/+0,8) somada à sucção superior do telhado.",
                        "NBR 6123 Table 7 Note 1: Underside of eave experiences windward wall overpressure (+0.7/+0.8) combined with upper roof suction.",
                    ))
                    .size(11.0)
                    .color(sec_color),
                );
            });
        });
    });
}
