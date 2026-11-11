use eolo_rs::core::project::*;
use eolo_rs::core::scenarios::get_preset_scenarios;
use eolo_rs::core::types::*;
use eolo_rs::data::database::EoloDatabase;
use eolo_rs::engine::cpe_cpi_tables::*;
use eolo_rs::engine::envelopes::*;
use eolo_rs::engine::factors_s1_s2_s3::*;
use eolo_rs::engine::wind_core::*;
use eolo_rs::reports::MemorialGenerator;

#[test]
fn test_s1_topographic_factor() {
    let mut topo = TopographyParams {
        topo_type: TopographyType::Flat,
        ..Default::default()
    };
    assert_eq!(calculate_s1(&topo), 1.0);

    topo.topo_type = TopographyType::DeepValley;
    assert_eq!(calculate_s1(&topo), 0.90);

    topo.topo_type = TopographyType::Slope;
    topo.theta_t_deg = 10.0;
    topo.dt = 20.0;
    topo.z_point = 10.0;
    topo.relative_position = 1.0; // Crista B
    let s1 = calculate_s1(&topo);
    assert!(s1 > 1.0, "No topo do talude S1 deve ser maior que 1.0");
}

#[test]
fn test_s2_meteorological_parameters() {
    // Tabela 3 da NBR 6123:2023:
    // Categoria II, Classe B, z = 10m: S2 = 1.00 * 0.98 * (10/10)^0.09 = 0.980
    let s2_cat2_b_10 = calculate_s2(TerrainRoughness::CategoryII, BuildingClass::ClassB, 10.0);
    assert!((s2_cat2_b_10 - 0.98).abs() < 0.01);

    // Categoria IV, Classe B, z = 10m: bm = 0.85, Fr = 0.98 -> S2 = 0.833
    let s2_cat4_b_10 = calculate_s2(TerrainRoughness::CategoryIV, BuildingClass::ClassB, 10.0);
    assert!((s2_cat4_b_10 - 0.833).abs() < 0.01);

    // Regra da Categoria V: constante até 10m
    let s2_cat5_5m = calculate_s2(TerrainRoughness::CategoryV, BuildingClass::ClassA, 5.0);
    let s2_cat5_10m = calculate_s2(TerrainRoughness::CategoryV, BuildingClass::ClassA, 10.0);
    assert_eq!(s2_cat5_5m, s2_cat5_10m, "Na Categoria V, S2 deve ser constante até 10m");
}

#[test]
fn test_s3_statistical_factor() {
    assert_eq!(calculate_s3(StatisticalGroup::Group1, false), 1.11);
    assert_eq!(calculate_s3(StatisticalGroup::Group2, false), 1.06);
    assert_eq!(calculate_s3(StatisticalGroup::Group3, false), 1.00);
    assert_eq!(calculate_s3(StatisticalGroup::Group4, false), 0.95);
    assert_eq!(calculate_s3(StatisticalGroup::Group5, false), 0.83);

    // Redução de vedações: 0.92 * S3
    let s3_clad = calculate_s3(StatisticalGroup::Group3, true);
    assert_eq!(s3_clad, 0.92);
}

#[test]
fn test_dynamic_pressure_equation() {
    let mut p = EoloProject::default();
    p.wind.v0 = 40.0;
    p.wind.topography.topo_type = TopographyType::Flat; // S1 = 1.0
    p.wind.roughness = TerrainRoughness::CategoryII;
    p.wind.building_class = BuildingClass::ClassA; // Fr = 1.0, bm = 1.0
    p.wind.reference_height_z = 10.0; // S2 = 1.0
    p.wind.statistical_group = StatisticalGroup::Group3; // S3 = 1.0

    let st = WindCalculatedState::compute(&p);
    assert_eq!(st.vk_ms, 40.0);

    // q = 0.613 * 40^2 = 0.613 * 1600 = 980.8 Pa
    assert!((st.q_pa - 980.8).abs() < 0.5);
    assert!((st.q_kn_m2 - 0.9808).abs() < 0.001);
}

#[test]
fn test_wall_and_roof_coefficients() {
    let geom = BuildingGeometry::default();
    let walls = calculate_wall_coefficients(&geom, 90.0);
    assert_eq!(walls.windward_ce, 0.70);
    assert!(walls.leeward_ce < 0.0);

    let roof = calculate_roof_coefficients(&geom);
    assert!(roof.windward_slope_suction_ce < 0.0);

    let eaves = calculate_eave_coefficients(&roof, &walls);
    assert!(eaves.net_uplift_cp < -1.0, "Arrancamento do beiral deve ser severo");
}

#[test]
fn test_internal_pressure_dominant_windward() {
    let openings = OpeningsConfig {
        dominance: OpeningDominance::DominantWindward,
        dominant_area: 20.0,
        other_openings_area: 5.0, // Razão = 4.0
        ..Default::default()
    };

    let state = calculate_internal_pressure(&openings, 0.70);
    assert!(state.cpi_positive >= 0.60, "Abertura dominante frontal gera alta sobrepressão interna");
}

#[test]
fn test_envelopes_and_memorial() {
    let project = EoloProject::default();
    let env = WindEnvelopeResults::compute(&project);

    assert!(env.roof_max_uplift_kn_m2 < 0.0);
    assert!(env.total_drag_force_fa_kn > 0.0);

    let md = MemorialGenerator::generate_markdown(&project, &env);
    assert!(md.contains("ABNT NBR 6123:2023"));
    assert!(md.contains("MEMORIAL DE CÁLCULO ESTRUTURAL"));
    assert!(md.contains("ESTUDO ACADÊMICO"));

    let md_en = MemorialGenerator::generate_markdown_i18n(&project, &env, eolo_rs::locale::Language::EnUs);
    assert!(md_en.contains("ABNT NBR 6123:2023"));
    assert!(md_en.contains("ACADEMIC STUDY"));

    let html = MemorialGenerator::generate_html(&project, &env);
    assert!(html.contains("<!DOCTYPE html>"));
}

#[test]
fn test_sqlite_persistence() {
    let db = EoloDatabase::open_memory().expect("Banco SQLite em memória");
    let p = EoloProject::default();
    let id = db.save_project(&p).expect("Salvar projeto");
    assert!(id > 0);

    let loaded = db.load_project(id).expect("Carregar").expect("Projeto presente");
    assert_eq!(loaded.metadata.title, p.metadata.title);
}

#[test]
fn test_all_five_preset_scenarios() {
    let scenarios = get_preset_scenarios();
    assert_eq!(scenarios.len(), 5);

    for s in scenarios {
        let env = WindEnvelopeResults::compute(&s.project);
        assert!(env.wind_state.vk_ms > 0.0);
        assert!(env.wind_state.q_kn_m2 > 0.0);
        assert!(env.total_drag_force_fa_kn > 0.0);
    }
}
