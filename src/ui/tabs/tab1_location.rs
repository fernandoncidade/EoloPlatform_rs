use egui::{RichText, ScrollArea, Ui};

use crate::core::project::EoloProject;
use crate::core::types::*;
use crate::engine::envelopes::WindEnvelopeResults;
use crate::engine::isopleth_map::*;
use crate::locale::Localizer;
use crate::ui::theme::EoloTheme;

pub struct LocationTabState {
    pub search_station: String,
    pub selected_uf_idx: usize,
}

impl Default for LocationTabState {
    fn default() -> Self {
        Self {
            search_station: String::new(),
            selected_uf_idx: 15, // PR (Paraná) padrão
        }
    }
}

pub fn render_tab_location(
    ui: &mut Ui,
    project: &mut EoloProject,
    env: &WindEnvelopeResults,
    state: &mut LocationTabState,
    localizer: &Localizer,
) {
    let dark = ui.visuals().dark_mode;
    let lang = localizer.current_lang;
    let txt_color = EoloTheme::text_primary(dark);
    let sec_color = EoloTheme::text_secondary(dark);

    ScrollArea::vertical().show(ui, |ui| {
        ui.heading(
            RichText::new(localizer.tr(
                "1. Localização, Velocidade Básica V0 e Fatores S1, S2 e S3 (ABNT NBR 6123:2023)",
                "1. Site Location, Basic Wind Speed V0, and Factors S1, S2, S3 (ABNT NBR 6123:2023)",
            ))
            .color(EoloTheme::text_primary(dark))
            .strong(),
        );
        ui.add_space(4.0);

        ui.columns(2, |cols| {
            // =========================================================================
            // COLUNA ESQUERDA: V0 e Fator Topográfico S1
            // =========================================================================
            cols[0].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "📍 Velocidade Básica do Vento (V0) — Cláusula 5.1",
                        "📍 Basic Wind Speed (V0) — Clause 5.1",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.label(
                    RichText::new(localizer.tr(
                        "Rajada de 3s a 10m de altura em campo aberto (Cat. II), período de recorrência de 50 anos.",
                        "3-second gust at 10m height over open terrain (Cat. II), 50-year mean return period.",
                    ))
                    .size(11.5)
                    .color(sec_color),
                );
                ui.separator();

                // Seleção Rápida por Estado (UF)
                ui.horizontal(|ui| {
                    ui.label(localizer.tr(
                        "Selecionar Estado (UF):",
                        "Select Brazilian State (UF):",
                    ));
                    let curr_st = &BRAZILIAN_STATES[state.selected_uf_idx];
                    egui::ComboBox::from_id_salt("uf_selector_combo")
                        .selected_text(format!("{} — {} (V0 ref: {:.0} m/s)", curr_st.uf, curr_st.name, curr_st.v0_typical))
                        .show_ui(ui, |ui| {
                            for (idx, st) in BRAZILIAN_STATES.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        state.selected_uf_idx == idx,
                                        format!("{} — {} ({}: {:.0} m/s)", st.uf, st.name, st.capital, st.v0_typical),
                                    )
                                    .clicked()
                                {
                                    state.selected_uf_idx = idx;
                                    project.metadata.location_state = st.uf.to_string();
                                    project.metadata.location_city = st.capital.to_string();
                                    project.wind.v0 = st.v0_typical;
                                }
                            }
                        });
                });

                ui.add_space(4.0);

                // Controle manual de V0
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(localizer.tr(
                            "Velocidade Básica V0 (m/s):",
                            "Basic Wind Speed V0 (m/s):",
                        ))
                        .color(txt_color)
                        .strong(),
                    );
                    ui.add(egui::Slider::new(&mut project.wind.v0, 30.0..=55.0).step_by(1.0).suffix(" m/s"));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(localizer.tr("Curvas Isopléticas:", "Isopleth Curves:"));
                    for iso in [30.0, 35.0, 40.0, 45.0, 50.0] {
                        if ui.selectable_label((project.wind.v0 - iso).abs() < 0.1, format!("{:.0} m/s", iso)).clicked() {
                            project.wind.v0 = iso;
                        }
                    }
                });

                ui.add_space(6.0);
                ui.separator();

                // Tabela das 49 Estações Meteorológicas da NBR 6123:2023 (Anexo C)
                ui.label(
                    RichText::new(localizer.tr(
                        "✈️ Estações Meteorológicas Oficiais da FAB (NBR 6123 Tabela C.1):",
                        "✈️ Official FAB Meteorological Stations (NBR 6123 Table C.1):",
                    ))
                    .color(txt_color)
                    .strong(),
                );
                ui.horizontal(|ui| {
                    ui.label(localizer.tr("Filtrar cidade/UF:", "Filter city/UF:"));
                    ui.text_edit_singleline(&mut state.search_station);
                    if !state.search_station.is_empty() && ui.button("✖").clicked() {
                        state.search_station.clear();
                    }
                });

                ScrollArea::vertical()
                    .id_salt("stations_scroll")
                    .max_height(150.0)
                    .show(ui, |ui| {
                        egui::Grid::new("stations_grid").striped(true).num_columns(4).spacing([12.0, 4.0]).show(ui, |ui| {
                            ui.label(RichText::new(localizer.tr("Estação", "Station")).color(txt_color).strong());
                            ui.label(RichText::new("UF").color(txt_color).strong());
                            ui.label(RichText::new("V0 (NBR)").color(txt_color).strong());
                            ui.label(RichText::new(localizer.tr("Ação", "Action")).color(txt_color).strong());
                            ui.end_row();

                            let q = state.search_station.to_lowercase();
                            for st in NBR6123_STATIONS {
                                if !q.is_empty()
                                    && !st.name.to_lowercase().contains(&q)
                                    && !st.uf.to_lowercase().contains(&q)
                                {
                                    continue;
                                }
                                ui.label(st.name);
                                ui.label(st.uf);
                                ui.label(format!("{:.0} m/s", st.v0_isopleth));
                                if ui.small_button(localizer.tr("Aplicar", "Apply")).clicked() {
                                    project.wind.v0 = st.v0_isopleth;
                                    project.metadata.location_city = st.name.to_string();
                                    project.metadata.location_state = st.uf.to_string();
                                }
                                ui.end_row();
                            }
                        });
                    });
            });

            cols[0].add_space(8.0);

            // FATOR TOPOGRÁFICO S1
            cols[0].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "⛰️ Fator Topográfico (S1) — Cláusula 5.2",
                        "⛰️ Topographic Factor (S1) — Clause 5.2",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                for topo in [
                    TopographyType::Flat,
                    TopographyType::Slope,
                    TopographyType::Hill,
                    TopographyType::DeepValley,
                ] {
                    ui.radio_value(
                        &mut project.wind.topography.topo_type,
                        topo,
                        topo.name_i18n(lang),
                    );
                }

                if matches!(project.wind.topography.topo_type, TopographyType::Slope | TopographyType::Hill) {
                    ui.add_space(4.0);
                    egui::Grid::new("s1_params_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                        ui.label(localizer.tr(
                            "Desnível da encosta d (m):",
                            "Hill/escarpment height d (m):",
                        ));
                        ui.add(egui::DragValue::new(&mut project.wind.topography.dt).speed(0.5).range(1.0..=500.0));
                        ui.end_row();

                        ui.label(localizer.tr(
                            "Inclinação média θt (°):",
                            "Average slope angle θt (°):",
                        ));
                        ui.add(egui::Slider::new(&mut project.wind.topography.theta_t_deg, 1.0..=60.0).suffix("°"));
                        ui.end_row();

                        ui.label(localizer.tr(
                            "Posição relativa na encosta (0=base, 1=topo):",
                            "Relative position on slope (0=base, 1=crest):",
                        ));
                        ui.add(egui::Slider::new(&mut project.wind.topography.relative_position, 0.0..=1.0));
                        ui.end_row();
                    });
                }

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} {:.3}",
                            localizer.tr("Fator S1 Calculado:", "Calculated Factor S1:"),
                            env.wind_state.s1
                        ))
                        .color(EoloTheme::accent_blue(dark))
                        .strong(),
                    );
                });
            });

            // =========================================================================
            // COLUNA DIREITA: Fatores S2, S3 e Resumo Analítico
            // =========================================================================
            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🏙️ Fator de Rugosidade e Dimensões (S2) — Cláusula 5.3",
                        "🏙️ Roughness & Building Size Factor (S2) — Clause 5.3",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                ui.label(
                    RichText::new(localizer.tr(
                        "Categoria de Rugosidade do Terreno (Tabela 1):",
                        "Terrain Roughness Category (Table 1):",
                    ))
                    .color(txt_color)
                    .strong(),
                );
                for cat in [
                    TerrainRoughness::CategoryI,
                    TerrainRoughness::CategoryII,
                    TerrainRoughness::CategoryIII,
                    TerrainRoughness::CategoryIV,
                    TerrainRoughness::CategoryV,
                ] {
                    ui.radio_value(&mut project.wind.roughness, cat, cat.name_i18n(lang));
                }

                ui.add_space(6.0);
                ui.checkbox(
                    &mut project.wind.auto_class,
                    localizer.tr(
                        "Determinar Classe (A, B ou C) automaticamente pelas dimensões da obra",
                        "Automatically determine Class (A, B, or C) from building dimensions",
                    ),
                );

                if !project.wind.auto_class {
                    for cl in [BuildingClass::ClassA, BuildingClass::ClassB, BuildingClass::ClassC] {
                        ui.radio_value(&mut project.wind.building_class, cl, cl.name_i18n(lang));
                    }
                } else {
                    ui.label(
                        RichText::new(format!(
                            "{} {} ({}: {:.1} m)",
                            localizer.tr("Classe enquadrada:", "Assigned Class:"),
                            project.wind.building_class.name_i18n(lang),
                            localizer.tr("Maior dimensão", "Max dimension"),
                            project.geometry.max_global_dimension()
                        ))
                        .italics()
                        .color(sec_color),
                    );
                }

                let (bm, p_exp) = project.wind.roughness.parameters(project.wind.building_class);
                ui.add_space(4.0);
                ui.label(format!(
                    "{} bm = {:.2} | Fr = {:.2} | p = {:.3} | zg = {:.0} m | z0 = {:.3} m",
                    localizer.tr("Parâmetros NBR 6123:2023:", "NBR 6123:2023 Parameters:"),
                    bm,
                    project.wind.building_class.fr(),
                    p_exp,
                    project.wind.roughness.zg(),
                    project.wind.roughness.z0()
                ));
                ui.label(
                    RichText::new(format!(
                        "{} {:.2} m: {:.3}",
                        localizer.tr("Fator S2 Calculado na cota z =", "Calculated Factor S2 at z ="),
                        env.wind_state.reference_z,
                        env.wind_state.s2
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
            });

            cols[1].add_space(8.0);

            // FATOR ESTATÍSTICO S3
            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "🏥 Fator Estatístico (S3) — Cláusula 5.4 e Tabela 4",
                        "🏥 Statistical Factor (S3) — Clause 5.4 & Table 4",
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
                ui.separator();

                for grp in [
                    StatisticalGroup::Group1,
                    StatisticalGroup::Group2,
                    StatisticalGroup::Group3,
                    StatisticalGroup::Group4,
                    StatisticalGroup::Group5,
                ] {
                    ui.radio_value(
                        &mut project.wind.statistical_group,
                        grp,
                        grp.name_i18n(lang),
                    );
                }

                ui.add_space(4.0);
                ui.checkbox(
                    &mut project.wind.apply_cladding_s3_reduction,
                    localizer.tr(
                        "Aplicar redução de 0,92 · S3 para elementos de vedação (Nota da Tabela 4)",
                        "Apply 0.92 · S3 reduction for cladding & secondary elements (Table 4 Note)",
                    ),
                );

                ui.label(
                    RichText::new(format!(
                        "{} {:.2} | {} {:.3} ({} {} {})",
                        localizer.tr("S3 Estrutura Principal:", "Main Framing S3:"),
                        env.wind_state.s3,
                        localizer.tr("S3 Vedações:", "Cladding S3:"),
                        env.wind_state.s3_cladding,
                        localizer.tr("Recorrência:", "Return Period:"),
                        project.wind.statistical_group.recurrence_years(),
                        localizer.tr("anos", "years")
                    ))
                    .color(EoloTheme::accent_blue(dark))
                    .strong(),
                );
            });

            cols[1].add_space(8.0);

            // PAINEL DE RESUMO DE VELOCIDADE E PRESSÃO DINÂMICA
            cols[1].group(|ui| {
                ui.heading(
                    RichText::new(localizer.tr(
                        "⚡ Resumo: Velocidade Característica (Vk) e Pressão (q)",
                        "⚡ Summary: Characteristic Speed (Vk) & Dynamic Pressure (q)",
                    ))
                    .color(EoloTheme::success_green(dark))
                    .strong(),
                );
                ui.separator();

                egui::Grid::new("vk_q_summary_grid").striped(true).num_columns(3).spacing([16.0, 6.0]).show(ui, |ui| {
                    ui.label(RichText::new(localizer.tr("Grandeza", "Quantity")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Estrutura Principal", "Main Framing")).color(txt_color).strong());
                    ui.label(RichText::new(localizer.tr("Vedações / Terças", "Cladding / Purlins")).color(txt_color).strong());
                    ui.end_row();

                    ui.label(localizer.tr("Velocidade Vk (m/s):", "Wind Speed Vk (m/s):"));
                    ui.label(RichText::new(format!("{:.2} m/s", env.wind_state.vk_ms)).color(txt_color).strong());
                    ui.label(format!("{:.2} m/s", env.wind_state.vk_cladding_ms));
                    ui.end_row();

                    ui.label(localizer.tr("Velocidade Vk (km/h):", "Wind Speed Vk (km/h):"));
                    ui.label(format!("{:.1} km/h", env.wind_state.vk_kmh));
                    ui.label(format!("{:.1} km/h", env.wind_state.vk_cladding_ms * 3.6));
                    ui.end_row();

                    ui.label(localizer.tr("Pressão Dinâmica q (kN/m²):", "Dynamic Pressure q (kN/m²):"));
                    ui.label(
                        RichText::new(format!("{:.3} kN/m²", env.wind_state.q_kn_m2))
                            .color(EoloTheme::accent_blue(dark))
                            .strong(),
                    );
                    ui.label(format!("{:.3} kN/m²", env.wind_state.q_cladding_kn_m2));
                    ui.end_row();

                    ui.label(localizer.tr("Pressão Dinâmica q (kgf/m²):", "Dynamic Pressure q (kgf/m²):"));
                    ui.label(format!("{:.1} kgf/m²", env.wind_state.q_kgf_m2));
                    ui.label(format!("{:.1} kgf/m²", env.wind_state.q_cladding_kn_m2 * 1000.0 / 9.80665));
                    ui.end_row();
                });
            });
        });
    });
}
