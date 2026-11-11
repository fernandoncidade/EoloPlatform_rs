use serde::{Deserialize, Serialize};

use crate::core::project::EoloProject;
use super::cpe_cpi_tables::{
    calculate_eave_coefficients, calculate_internal_pressure, calculate_roof_coefficients,
    calculate_wall_coefficients, EaveCoefficients, InternalPressureState, RoofCoefficients,
    WallCoefficients,
};
use super::wind_core::WindCalculatedState;

/// Resultados das envoltórias de carregamento estrutural
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindEnvelopeResults {
    pub wind_state: WindCalculatedState,
    pub walls: WallCoefficients,
    pub roof: RoofCoefficients,
    pub eaves: EaveCoefficients,
    pub internal_pressure: InternalPressureState,

    // Casos Críticos de Pressão Efetiva Líquida: Delta_p = (Cpe - Cpi) * q  (kN/m²)
    /// Caso 1: Sucção máxima na cobertura (arrancamento das telhas e alívio de fundações)
    pub roof_max_uplift_kn_m2: f64,
    /// Caso 2: Pressão máxima na cobertura (sobrecarga gravitacional de vento)
    pub roof_max_downward_kn_m2: f64,
    /// Sucção na água de sotavento (kN/m²)
    pub roof_leeward_kn_m2: f64,

    /// Parede de barlavento: pressão positiva efetiva líquida (kN/m²)
    pub wall_windward_kn_m2: f64,
    /// Parede de sotavento: sucção efetiva líquida (kN/m²)
    pub wall_leeward_kn_m2: f64,
    /// Parede lateral: sucção no canto frontal (kN/m²)
    pub wall_side_corner_kn_m2: f64,

    /// Arrancamento líquido total no beiral de barlavento (kN/m²)
    pub eave_max_uplift_kn_m2: f64,

    // Esforços Lineares nas Terças e Longarinas (kN/m)
    /// Carga distribuída crítica de arrancamento na terça de cobertura (kN/m)
    pub purlin_max_uplift_kn_m: f64,
    /// Carga distribuída de arrancamento na terça de beiral/aresta (kN/m)
    pub purlin_edge_uplift_kn_m: f64,
    /// Carga distribuída na longarina de fechamento lateral no canto (kN/m)
    pub girt_corner_kn_m: f64,

    // Esforços Lineares nos Pórticos Principais (kN/m)
    /// Carga linear perpendicular à viga/banzo de barlavento do pórtico (kN/m)
    pub frame_rafter_windward_kn_m: f64,
    /// Carga linear perpendicular à viga/banzo de sotavento do pórtico (kN/m)
    pub frame_rafter_leeward_kn_m: f64,
    /// Carga linear horizontal no pilar de barlavento do pórtico (kN/m)
    pub frame_column_windward_kn_m: f64,
    /// Carga linear horizontal no pilar de sotavento do pórtico (kN/m)
    pub frame_column_leeward_kn_m: f64,

    // Força Global de Arrasto e Tombamento
    /// Coeficiente global de arrasto Ca estimado para o galpão
    pub drag_coefficient_ca: f64,
    /// Força horizontal total de arrasto Fa (kN)
    pub total_drag_force_fa_kn: f64,
    /// Momento global de tombamento na base dos pilares (kN·m)
    pub overturning_moment_kn_m: f64,
}

impl WindEnvelopeResults {
    pub fn compute(project: &EoloProject) -> Self {
        let wind_state = WindCalculatedState::compute(project);
        let q = wind_state.q_kn_m2;

        let walls = calculate_wall_coefficients(&project.geometry, 90.0);
        let roof = calculate_roof_coefficients(&project.geometry);
        let eaves = calculate_eave_coefficients(&roof, &walls);
        let internal_pressure =
            calculate_internal_pressure(&project.openings, walls.windward_ce);

        let cpi_pos = internal_pressure.cpi_positive; // Sobrepressão interna máxima (+0.1 a +0.8)
        let cpi_neg = internal_pressure.cpi_negative; // Sucção interna máxima (-0.2 a -0.9)

        // Delta_p = (Cpe - Cpi) * q
        // Arrancamento máximo na cobertura: Cpe mais negativo (sucção externa) com Cpi positivo (pressão interna)
        // Delta_p_uplift = (roof.windward_slope_suction_ce - cpi_pos) * q
        let delta_cp_uplift = roof.windward_slope_suction_ce - cpi_pos;
        let roof_max_uplift_kn_m2 = delta_cp_uplift * q;

        // Sobrecarga máxima na cobertura: Cpe positivo com Cpi negativo (sucção interna puxando para baixo)
        let delta_cp_down = roof.windward_slope_pressure_ce - cpi_neg;
        let roof_max_downward_kn_m2 = delta_cp_down * q;

        let delta_cp_leeward = roof.leeward_slope_ce - cpi_pos;
        let roof_leeward_kn_m2 = delta_cp_leeward * q;

        // Parede de barlavento: Cpe = +0.7 ou +0.8 com Cpi negativo (-0.3)
        // Delta_p_windward = (walls.windward_ce - cpi_neg) * q -> máxima pressão para dentro
        let wall_windward_kn_m2 = (walls.windward_ce - cpi_neg) * q;

        // Parede de sotavento: Cpe = -0.4 a -0.6 com Cpi positivo (+0.2 ou +0.8)
        let wall_leeward_kn_m2 = (walls.leeward_ce - cpi_pos) * q;

        // Canto lateral da parede:
        let wall_side_corner_kn_m2 = (walls.corner_cpe_local - cpi_pos) * q;

        // Beiral com arrancamento:
        // A face inferior tem a pressão da parede (+0.7), o topo tem sucção (-0.8 ou -1.2)
        // Mais o efeito de Cpi interno caso haja vazamentos
        let eave_max_uplift_kn_m2 = eaves.net_uplift_cp * q;

        // Cargas lineares nas terças de cobertura (kN/m)
        let purlin_s = project.geometry.purlin_spacing;
        let purlin_max_uplift_kn_m = roof_max_uplift_kn_m2 * purlin_s;
        // Na aresta e beiral, aplica-se o coeficiente de sucção local médio
        let delta_cp_edge = roof.local_peak_suction_cpe - cpi_pos;
        let purlin_edge_uplift_kn_m = (delta_cp_edge * q) * purlin_s;

        // Carga linear na longarina de fechamento lateral no canto
        let girt_s = project.geometry.girt_spacing;
        let girt_corner_kn_m = wall_side_corner_kn_m2 * girt_s;

        // Cargas lineares nos pórticos principais (kN/m)
        let frame_s = project.geometry.frame_spacing;
        let frame_rafter_windward_kn_m = roof_max_uplift_kn_m2 * frame_s;
        let frame_rafter_leeward_kn_m = roof_leeward_kn_m2 * frame_s;
        let frame_column_windward_kn_m = wall_windward_kn_m2 * frame_s;
        let frame_column_leeward_kn_m = wall_leeward_kn_m2 * frame_s;

        // Força global de arrasto Fa = q * Ca * Ae * fv  (Item 4.3.3)
        // Ca para prisma retangular conforme Figura 4 da NBR 6123
        let ca = calculate_drag_coefficient(
            project.geometry.relative_height_h_over_b(),
            project.geometry.aspect_ratio_a_over_b(),
        );
        let ae = project.geometry.frontal_area_90deg();
        let total_drag_force_fa_kn = q * ca * ae;

        // Momento de tombamento na base (braço de alavanca ~ 0.55 * h_med)
        let arm_z = 0.55 * project.geometry.mean_height();
        let overturning_moment_kn_m = total_drag_force_fa_kn * arm_z;

        Self {
            wind_state,
            walls,
            roof,
            eaves,
            internal_pressure,
            roof_max_uplift_kn_m2: round2(roof_max_uplift_kn_m2),
            roof_max_downward_kn_m2: round2(roof_max_downward_kn_m2),
            roof_leeward_kn_m2: round2(roof_leeward_kn_m2),
            wall_windward_kn_m2: round2(wall_windward_kn_m2),
            wall_leeward_kn_m2: round2(wall_leeward_kn_m2),
            wall_side_corner_kn_m2: round2(wall_side_corner_kn_m2),
            eave_max_uplift_kn_m2: round2(eave_max_uplift_kn_m2),
            purlin_max_uplift_kn_m: round2(purlin_max_uplift_kn_m),
            purlin_edge_uplift_kn_m: round2(purlin_edge_uplift_kn_m),
            girt_corner_kn_m: round2(girt_corner_kn_m),
            frame_rafter_windward_kn_m: round2(frame_rafter_windward_kn_m),
            frame_rafter_leeward_kn_m: round2(frame_rafter_leeward_kn_m),
            frame_column_windward_kn_m: round2(frame_column_windward_kn_m),
            frame_column_leeward_kn_m: round2(frame_column_leeward_kn_m),
            drag_coefficient_ca: round2(ca),
            total_drag_force_fa_kn: round2(total_drag_force_fa_kn),
            overturning_moment_kn_m: round2(overturning_moment_kn_m),
        }
    }
}

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

/// Coeficiente de arrasto Ca conforme Figura 4 da NBR 6123:2023
fn calculate_drag_coefficient(h_over_l1: f64, l1_over_l2: f64) -> f64 {
    // Para edificações paralelepipédicas assentes no solo:
    // Ca varia tipicamente entre 0,8 e 1,4 dependendo das proporções
    let base_ca = if l1_over_l2 <= 1.0 {
        1.10
    } else if l1_over_l2 <= 2.0 {
        1.20
    } else if l1_over_l2 <= 4.0 {
        1.30
    } else {
        1.40
    };

    if h_over_l1 <= 0.5 {
        base_ca * 0.85
    } else if h_over_l1 <= 1.5 {
        base_ca * 0.95
    } else {
        base_ca * 1.05
    }
}
