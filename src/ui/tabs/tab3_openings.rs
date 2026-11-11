use egui::{RichText, ScrollArea, Ui};

use crate::core::project::EoloProject;
use crate::core::types::OpeningDominance;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::locale::Localizer;
use crate::ui::theme::EoloTheme;

pub fn render_tab_openings(
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
                "3. Aberturas Dominantes, Permeabilidade e Coeficiente de Pressão Interna (Cpi — Cláusula 6.3)",
                "3. Dominant Openings, Permeability & Internal Pressure Coefficient (Cpi — Clause 6.3)",
            ))
            .color(EoloTheme::text_primary(dark))
            .strong(),
        );
        ui.add_space(4.0);

        ui.columns(2, |cols| {
            // =========================================================================
            // COLUNA ESQUERDA: Configuração de Aberturas e Áreas
            // =========================================================================
            cols[0].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🚪 Configuração de Permeabilidade e Portões (Cláusula 6.3)",
                        "🚪 Permeability & Gate Configuration (Clause 6.3)",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.label(
                    RichText::new(localizer.tr(
                        "Em galpões industriais e hangares, o rompimento ou abertura de um portão em barlavento infla o interior da edificação (Cpi até +0,80), duplicando o esforço de arrancamento da cobertura!",
                        "In industrial warehouses and hangars, an open or breached windward gate pressurizes the interior (Cpi up to +0.80), nearly doubling roof uplift forces!",
                    ))
                    .size(11.5)
                    .color(sec_color),
                );
                ui.separator();

                for dom in [
                    OpeningDominance::Impermeable,
                    OpeningDominance::UniformPermeability,
                    OpeningDominance::DominantWindward,
                    OpeningDominance::DominantLeeward,
                    OpeningDominance::DominantParallel,
                    OpeningDominance::CustomCpi,
                ] {
                    ui.radio_value(&mut project.openings.dominance, dom, dom.name_i18n(lang));
                }

                ui.add_space(8.0);
                ui.separator();

                match project.openings.dominance {
                    OpeningDominance::DominantWindward
                    | OpeningDominance::DominantLeeward
                    | OpeningDominance::DominantParallel => {
                        ui.label(
                            RichText::new(localizer.tr(
                                "📏 Áreas de Aberturas para Interpolação da Tabela 6.3.2:",
                                "📏 Opening Areas for Table 6.3.2 Interpolation:",
                            ))
                            .color(txt_color)
                            .strong(),
                        );

                        egui::Grid::new("openings_area_grid").num_columns(2).spacing([14.0, 8.0]).show(ui, |ui| {
                            ui.label(localizer.tr(
                                "Área da Abertura Dominante Adom (m²):",
                                "Dominant Opening Area Adom (m²):",
                            ));
                            ui.add(egui::Slider::new(&mut project.openings.dominant_area, 0.5..=200.0).suffix(" m²"));
                            ui.end_row();

                            ui.label(localizer.tr(
                                "Soma das Demais Aberturas Aoutros (m²):",
                                "Sum of Remaining Openings Aothers (m²):",
                            ));
                            ui.add(egui::Slider::new(&mut project.openings.other_openings_area, 0.1..=100.0).suffix(" m²"));
                            ui.end_row();
                        });

                        let ratio = project.openings.opening_ratio();
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(format!(
                                "{} {:.2} : 1",
                                localizer.tr(
                                    "Razão de Áreas (Adom / Aoutros):",
                                    "Opening Area Ratio (Adom / Aothers):"
                                ),
                                ratio
                            ))
                            .color(EoloTheme::accent_blue(dark))
                            .strong(),
                        );
                    }
                    OpeningDominance::CustomCpi => {
                        ui.label(
                            RichText::new(localizer.tr(
                                "Defina manualmente o valor de Cpi personalizado:",
                                "Manually specify custom Cpi value:",
                            ))
                            .color(txt_color)
                            .strong(),
                        );
                        egui::Grid::new("custom_cpi_grid").num_columns(2).spacing([14.0, 8.0]).show(ui, |ui| {
                            ui.label(localizer.tr(
                                "Valor de Cpi (-0,90 a +0,80):",
                                "Custom Cpi Value (-0.90 to +0.80):",
                            ));
                            ui.add(egui::Slider::new(&mut project.openings.custom_cpi_value, -0.90..=0.80));
                            ui.end_row();
                        });
                    }
                    _ => {
                        ui.label(
                            RichText::new(localizer.tr(
                                "Os coeficientes Cpi são determinados diretamente pelos casos normativos padrão da Cláusula 6.3.1.",
                                "Coefficients Cpi are determined directly from standard normative cases in Clause 6.3.1.",
                            ))
                            .italics()
                            .color(sec_color),
                        );
                    }
                }
            });

            cols[0].add_space(8.0);

            cols[0].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🔍 Coeficientes de Pressão Interna Resultantes (Cpi)",
                        "🔍 Resulting Internal Pressure Coefficients (Cpi)",
                    ))
                    .color(EoloTheme::success_green(dark))
                    .strong(),
                );
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(localizer.tr(
                        "Sobrepressão Interna Crítica (Cpi,+):",
                        "Critical Internal Overpressure (Cpi,+):",
                    ));
                    ui.label(
                        RichText::new(format!("+{:.2}", env.internal_pressure.cpi_positive))
                            .size(16.0)
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(localizer.tr(
                        "Sucção Interna Crítica (Cpi,-):",
                        "Critical Internal Suction (Cpi,-):",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2}", env.internal_pressure.cpi_negative))
                            .size(16.0)
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                });

                ui.add_space(4.0);
                let cpi_explanation = if localizer.is_pt() {
                    &env.internal_pressure.explanation_pt
                } else {
                    match project.openings.dominance {
                        OpeningDominance::Impermeable => "Clause 6.3.1: Effectively sealed building (Cpi = -0.20 or 0.00, adopting the most unfavorable).",
                        OpeningDominance::UniformPermeability => "Clause 6.3.1: Four walls with uniform permeability (Cpi = -0.30 or 0.00).",
                        OpeningDominance::DominantWindward => "Table 6.3.2: Dominant windward opening pressurizes the building interior (+Cpi), maximizing roof uplift!",
                        OpeningDominance::DominantLeeward => "Table 6.3.2: Dominant leeward opening induces internal suction (-Cpi) equal to external leeward wall Ce.",
                        OpeningDominance::DominantParallel => "Table 6.3.2: Dominant opening on side wall parallel to wind induces strong internal suction (-Cpi).",
                        OpeningDominance::CustomCpi => "User-defined custom internal pressure coefficients Cpi.",
                    }
                };
                ui.label(RichText::new(cpi_explanation).size(12.0));
            });

            // =========================================================================
            // COLUNA DIREITA: Envoltória de Pressões Efetivas e Cargas nas Barras
            // =========================================================================
            let unit = project.display_unit;
            let sym = unit.symbol();

            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🌪️ Envoltória de Pressões Efetivas: Δp = (Cpe - Cpi) · q",
                        "🌪️ Effective Net Pressure Envelope: Δp = (Cpe - Cpi) · q",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                egui::Grid::new("envelopes_pressure_grid").striped(true).num_columns(3).spacing([14.0, 7.0]).show(ui, |ui| {
                    ui.label(RichText::new(localizer.tr("Região da Edificação", "Building Region")).color(txt_color).strong());
                    ui.label(RichText::new(format!("{} ({})", localizer.tr("Pressão Δp", "Net Pressure Δp"), sym)).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Condição Governante", "Governing Case")).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Telhado — Arrancamento Máximo:",
                        "Roof — Maximum Uplift:",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2} {}", unit.from_pa(env.roof_max_uplift_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sucção Externa + Cpi(+)", "Ext. Suction + Cpi(+)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Telhado — Sobrepressão Máxima:",
                        "Roof — Maximum Downward Push:",
                    ));
                    ui.label(
                        RichText::new(format!("+{:.2} {}", unit.from_pa(env.roof_max_downward_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Pressão Externa + Cpi(-)", "Ext. Push + Cpi(-)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Beiral — Arrancamento Líquido:",
                        "Eave Overhang — Net Uplift:",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2} {}", unit.from_pa(env.eave_max_uplift_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::eave_magenta(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Nota 1 da Tabela 7", "Table 7 Note 1"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Parede Barlavento (Empuxo Frontal):",
                        "Windward Wall (Frontal Push):",
                    ));
                    ui.label(
                        RichText::new(format!("+{:.2} {}", unit.from_pa(env.wall_windward_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sobrepressão + Cpi(-)", "Overpressure + Cpi(-)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Parede Sotavento (Sucção Traseira):",
                        "Leeward Wall (Rear Suction):",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2} {}", unit.from_pa(env.wall_leeward_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Sucção + Cpi(+)", "Suction + Cpi(+)"));
                    ui.end_row();

                    ui.label(localizer.tr(
                        "Parede Lateral — Borda de Alta Sucção:",
                        "Side Wall — High Suction Leading Edge:",
                    ));
                    ui.label(
                        RichText::new(format!("{:.2} {}", unit.from_pa(env.wall_side_corner_kn_m2 * 1000.0), sym))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.label(localizer.tr("Zona cpe_médio + Cpi(+)", "Zone cpe_mean + Cpi(+)"));
                    ui.end_row();
                });
            });

            cols[1].add_space(8.0);

            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🏗️ Cargas Distribuídas Lineares para Pórtico e Terças (kN/m)",
                        "🏗️ Distributed Linear Loads for Portal Frame & Purlins (kN/m)",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.label(
                    RichText::new(localizer.tr(
                        "Valores prontos para inserção direta em softwares de análise matricial (Ftool, SAP2000, CYPE, Eberick):",
                        "Values ready for direct input into structural analysis software (Ftool, SAP2000, Robot, STAAD):",
                    ))
                    .size(11.5)
                    .color(sec_color),
                );
                ui.separator();

                egui::Grid::new("linear_loads_grid").striped(true).num_columns(3).spacing([14.0, 6.0]).show(ui, |ui| {
                    ui.label(RichText::new(localizer.tr("Elemento Estrutural", "Structural Member")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Largura Influência", "Tributary Width")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Carga Linear (w)", "Linear Load (w)")).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr("Terça de Cobertura (Zona Corrente):", "Roof Purlin (Interior Zone):"));
                    ui.label(format!("{:.2} m", project.geometry.purlin_spacing));
                    ui.label(
                        RichText::new(format!("{:.2} kN/m", env.purlin_max_uplift_kn_m))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Terça de Borda / Beiral (Pico Local):", "Edge / Eave Purlin (Local Peak):"));
                    ui.label(format!("{:.2} m", project.geometry.purlin_spacing));
                    ui.label(
                        RichText::new(format!("{:.2} kN/m", env.purlin_edge_uplift_kn_m))
                            .color(EoloTheme::eave_magenta(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Pórtico — Pilar de Barlavento:", "Portal Frame — Windward Column:"));
                    ui.label(format!("{:.2} m", project.geometry.frame_spacing));
                    ui.label(
                        RichText::new(format!("+{:.2} kN/m", env.frame_column_windward_kn_m))
                            .color(EoloTheme::overpressure_red(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Pórtico — Pilar de Sotavento:", "Portal Frame — Leeward Column:"));
                    ui.label(format!("{:.2} m", project.geometry.frame_spacing));
                    ui.label(
                        RichText::new(format!("{:.2} kN/m", env.frame_column_leeward_kn_m))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Pórtico — Viga Telhado Barlavento:", "Portal Frame — Windward Rafter:"));
                    ui.label(format!("{:.2} m", project.geometry.frame_spacing));
                    ui.label(
                        RichText::new(format!("{:.2} kN/m", env.frame_rafter_windward_kn_m))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.end_row();

                    ui.label(localizer.tr("Pórtico — Viga Telhado Sotavento:", "Portal Frame — Leeward Rafter:"));
                    ui.label(format!("{:.2} m", project.geometry.frame_spacing));
                    ui.label(
                        RichText::new(format!("{:.2} kN/m", env.frame_rafter_leeward_kn_m))
                            .color(EoloTheme::suction_cyan(dark))
                            .strong(),
                    );
                    ui.end_row();
                });
            });
        });
    });
}
