use serde::{Deserialize, Serialize};

use crate::core::project::{BuildingGeometry, OpeningsConfig};
use crate::core::types::*;

/// Coeficientes externos Cpe e Ce para paredes conforme NBR 6123:2023 Tabela 6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallCoefficients {
    /// Barlavento (parede frontal exposta ao vento): sobrepressão
    pub windward_ce: f64,
    /// Sotavento (parede traseira oposta ao vento): sucção
    pub leeward_ce: f64,
    /// Parede lateral — Zona 1 (junto ao barlavento, sucção elevada)
    pub side_zone1_ce: f64,
    /// Parede lateral — Zona 2 (trecho intermediário)
    pub side_zone2_ce: f64,
    /// Parede lateral — Zona 3 (trecho posterior, quando a/b > 1)
    pub side_zone3_ce: f64,
    /// Coeficiente de pressão local médio cpe_médio para vedações nas arestas
    pub corner_cpe_local: f64,
    /// Extensão da zona de canto/aresta: min(0.2*b, h)
    pub corner_dimension: f64,
}

/// Coeficientes externos Cpe e Ce para telhado conforme NBR 6123:2023 Tabela 7 e 8
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoofCoefficients {
    /// Água de barlavento (vento soprando nela) — Caso 1 (Sucção máxima / Arrancamento)
    pub windward_slope_suction_ce: f64,
    /// Água de barlavento — Caso 2 (Sobrepressão máxima / Sobrecarga)
    pub windward_slope_pressure_ce: f64,
    /// Água de sotavento (lado oposto da cumeeira)
    pub leeward_slope_ce: f64,
    /// Coeficiente para vento longitudinal (0°) no trecho inicial frontal (E/G)
    pub longitudinal_front_ce: f64,
    /// Coeficiente para vento longitudinal (0°) no trecho posterior (F/H)
    pub longitudinal_rear_ce: f64,
    /// Coeficiente local médio cpe_médio nos beirais e cumeeira para telhas/terças
    pub local_peak_suction_cpe: f64,
    /// Coeficiente local no canto da cobertura (zona de maior turbulência)
    pub corner_peak_suction_cpe: f64,
}

/// Coeficientes de pressão atuantes nos beirais (efeito de arrancamento)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EaveCoefficients {
    /// Coeficiente na face superior do beiral (sucção do telhado)
    pub top_cpe: f64,
    /// Coeficiente na face inferior do beiral (pressão da parede adjacente)
    pub bottom_cpe: f64,
    /// Coeficiente resultante efetivo Delta_Cp = Cpe_top - Cpe_bottom
    pub net_uplift_cp: f64,
}

/// Coeficiente de pressão interna Cpi conforme NBR 6123:2023 Item 6.3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalPressureState {
    pub dominance: OpeningDominance,
    /// Cpi para o caso crítico de sobrepressão interna máxima (empurra paredes/telhado para fora)
    pub cpi_positive: f64,
    /// Cpi para o caso crítico de sucção interna máxima (puxa paredes/telhado para dentro)
    pub cpi_negative: f64,
    /// Cpi adotado para o caso de carga ativo
    pub cpi_adopted: f64,
    pub explanation_pt: String,
}

/// Avaliação dos coeficientes de paredes segundo NBR 6123:2023 Tabela 6
pub fn calculate_wall_coefficients(geom: &BuildingGeometry, angle_deg: f64) -> WallCoefficients {
    let b = geom.minor_dimension();
    let a = geom.major_dimension();
    let h = geom.eave_height_h;

    let h_over_b = if b > 0.0 { h / b } else { 0.5 };
    let a_over_b = if b > 0.0 { a / b } else { 1.0 };

    let corner_dim = (0.2 * b).min(h);

    let is_transverse = (angle_deg - 90.0).abs() < 45.0;

    if is_transverse {
        // Vento transversal (90°): Sopra perpendicularmente à maior fachada a
        let windward_ce = if h_over_b > 1.5 { 0.8 } else { 0.7 };
        let leeward_ce = if h_over_b <= 0.5 {
            if a_over_b <= 1.5 { -0.4 } else { -0.5 }
        } else if h_over_b <= 1.5 {
            if a_over_b <= 1.5 { -0.5 } else { -0.6 }
        } else {
            -0.6
        };

        let side1 = if h_over_b <= 0.5 { -0.8 } else if h_over_b <= 1.5 { -0.9 } else { -1.0 };
        let side2 = if h_over_b <= 0.5 { -0.4 } else if h_over_b <= 1.5 { -0.5 } else { -0.6 };
        let corner = if h_over_b <= 0.5 { -0.9 } else if h_over_b <= 1.5 { -1.1 } else { -1.2 };

        WallCoefficients {
            windward_ce,
            leeward_ce,
            side_zone1_ce: side1,
            side_zone2_ce: side2,
            side_zone3_ce: -0.2,
            corner_cpe_local: corner,
            corner_dimension: corner_dim,
        }
    } else {
        // Vento longitudinal (0°): Sopra perpendicularmente ao oitão b
        let windward_ce = if h_over_b > 1.5 { 0.8 } else { 0.7 };
        let leeward_ce = if h_over_b <= 0.5 {
            if a_over_b <= 1.5 { -0.4 } else if a_over_b >= 2.0 { -0.3 } else {
                -0.4 + (a_over_b - 1.5) * (-0.3 - (-0.4)) / 0.5
            }
        } else if h_over_b <= 1.5 {
            if a_over_b <= 1.5 { -0.5 } else if a_over_b >= 2.0 { -0.3 } else {
                -0.5 + (a_over_b - 1.5) * (-0.3 - (-0.5)) / 0.5
            }
        } else {
            if a_over_b <= 1.5 { -0.6 } else if a_over_b >= 2.0 { -0.3 } else {
                -0.6 + (a_over_b - 1.5) * (-0.3 - (-0.6)) / 0.5
            }
        };

        let side1 = if h_over_b <= 0.5 { -0.8 } else if h_over_b <= 1.5 { -0.9 } else { -1.0 };
        let side2 = if h_over_b <= 1.5 { -0.5 } else { -0.6 };
        // Nota 3 da Tabela 6 para A3 e B3:
        let side3 = if a_over_b <= 1.0 {
            side2
        } else if a_over_b >= 2.0 {
            -0.2
        } else {
            side2 + (a_over_b - 1.0) * (-0.2 - side2)
        };

        let corner = if h_over_b <= 0.5 { -0.9 } else if h_over_b <= 1.5 { -1.1 } else { -1.2 };

        WallCoefficients {
            windward_ce,
            leeward_ce,
            side_zone1_ce: side1,
            side_zone2_ce: side2,
            side_zone3_ce: side3,
            corner_cpe_local: corner,
            corner_dimension: corner_dim,
        }
    }
}

/// Avaliação dos coeficientes de telhado segundo NBR 6123:2023 Tabela 7 (Duas Águas) e Tabela 8 (Uma Água)
pub fn calculate_roof_coefficients(geom: &BuildingGeometry) -> RoofCoefficients {
    let theta = geom.pitch_angle_deg.clamp(0.0, 60.0);

    match geom.typology {
        RoofTypology::DuoPitch | RoofTypology::ShedIndustrial => {
            // Tabela 7 da NBR 6123:2023
            // Interpolação analítica dos coeficientes em função do ângulo theta (graus)
            let (suct_ce, press_ce, leeward_ce, local_peak, corner_peak): (f64, f64, f64, f64, f64) = if theta <= 5.0 {
                let t = theta / 5.0;
                let s = -0.8 + t * (-0.9 - (-0.8));
                let l = -0.4;
                let lp = -2.0 + t * (-1.4 - (-2.0));
                let cp = -2.0;
                (s, s, l, lp, cp)
            } else if theta <= 10.0 {
                let t = (theta - 5.0) / 5.0;
                let s = -0.9 + t * (-1.2 - (-0.9));
                let l = -0.4 + t * (-0.6 - (-0.4));
                let lp = -1.4;
                let cp = -2.0;
                (s, s, l, lp, cp)
            } else if theta <= 15.0 {
                let t = (theta - 10.0) / 5.0;
                let s = -1.2 + t * (-1.0 - (-1.2));
                let l = -0.6;
                let lp = -1.4 + t * (-1.2 - (-1.4));
                let cp = -1.8;
                (s, s, l, lp, cp)
            } else if theta <= 20.0 {
                let t = (theta - 15.0) / 5.0;
                let s = -1.0 + t * (-0.4 - (-1.0));
                let p = -0.4 + t * (0.2 - (-0.4)); // Início de sobrepressão positiva
                let l = -0.6;
                let lp = -1.2;
                let cp = -1.5;
                (s, p, l, lp, cp)
            } else if theta <= 30.0 {
                let t = (theta - 20.0) / 10.0;
                let s = -0.4 + t * (0.0 - (-0.4));
                let p = 0.2 + t * (0.3 - 0.2);
                let l = -0.6;
                let lp = -1.0;
                let cp = -1.5;
                (s, p, l, lp, cp)
            } else if theta <= 45.0 {
                let t = (theta - 30.0) / 15.0;
                let s = 0.0 + t * (0.3 - 0.0);
                let p = 0.3 + t * (0.7 - 0.3);
                let l = -0.6;
                let lp = -0.8;
                let cp = -1.1;
                (s, p, l, lp, cp)
            } else {
                // theta > 45°
                let s = 0.3 + (theta - 45.0) / 15.0 * (0.7 - 0.3);
                let p = 0.7;
                let l = -0.6;
                (s, p, l, -0.8, -1.0)
            };

            RoofCoefficients {
                windward_slope_suction_ce: (suct_ce * 100.0).round() / 100.0,
                windward_slope_pressure_ce: (press_ce * 100.0).round() / 100.0,
                leeward_slope_ce: (leeward_ce * 100.0).round() / 100.0,
                longitudinal_front_ce: -0.8,
                longitudinal_rear_ce: -0.4,
                local_peak_suction_cpe: (local_peak * 100.0).round() / 100.0,
                corner_peak_suction_cpe: (corner_peak * 100.0).round() / 100.0,
            }
        }

        RoofTypology::MonoPitch => {
            // Tabela 8 da NBR 6123:2023 (Uma Água)
            let suct = if theta <= 10.0 { -1.0 } else if theta <= 20.0 { -0.8 } else { -0.5 };
            let leeward = -0.5;
            RoofCoefficients {
                windward_slope_suction_ce: suct,
                windward_slope_pressure_ce: suct.max(-0.2),
                leeward_slope_ce: leeward,
                longitudinal_front_ce: -0.9,
                longitudinal_rear_ce: -0.5,
                local_peak_suction_cpe: -1.8,
                corner_peak_suction_cpe: -2.0,
            }
        }

        RoofTypology::CurvedArch => {
            // Cobertura em arco (Figuras 7 e 8)
            // Sucção intensa no topo da abóbada (Ce = -1.2 a -1.4)
            RoofCoefficients {
                windward_slope_suction_ce: -0.9,
                windward_slope_pressure_ce: 0.2, // Sobrepressão no terço inferior
                leeward_slope_ce: -0.5,
                longitudinal_front_ce: -0.8,
                longitudinal_rear_ce: -0.4,
                local_peak_suction_cpe: -1.6,
                corner_peak_suction_cpe: -2.0,
            }
        }

        RoofTypology::CanopyIsolated => {
            // Cobertura isolada sem paredes (marquise de posto)
            // Sujeita a fortes momentos e coeficientes de força de arrasto/sustentação
            RoofCoefficients {
                windward_slope_suction_ce: -1.3,
                windward_slope_pressure_ce: 0.6,
                leeward_slope_ce: -0.6,
                longitudinal_front_ce: -1.0,
                longitudinal_rear_ce: -0.6,
                local_peak_suction_cpe: -2.0,
                corner_peak_suction_cpe: -2.4,
            }
        }
    }
}

/// Cálculo dos coeficientes para beirais salientes da cobertura (Item 6.1 e Nota 1 da Tabela 7)
/// NOTA 1: "O coeficiente de forma Ce na face inferior do beiral é igual ao da parede correspondente."
pub fn calculate_eave_coefficients(
    roof_coef: &RoofCoefficients,
    wall_coef: &WallCoefficients,
) -> EaveCoefficients {
    // Face superior do beiral no barlavento sofre a sucção crítica do telhado
    let top_cpe = roof_coef.windward_slope_suction_ce;
    // Face inferior do beiral no barlavento recebe a sobrepressão positiva que sobe pela parede
    let bottom_cpe = wall_coef.windward_ce;
    // Força resultante líquida atuando para cima (arrancamento):
    // Delta_Cp = Cpe_top - Cpe_bottom
    // Ex: -0.8 - (+0.7) = -1.50 (força ascensional intensa!)
    let net_uplift = top_cpe - bottom_cpe;

    EaveCoefficients {
        top_cpe,
        bottom_cpe,
        net_uplift_cp: (net_uplift * 100.0).round() / 100.0,
    }
}

/// Determinação rigorosa da pressão interna Cpi segundo ABNT NBR 6123:2023 (Item 6.3)
pub fn calculate_internal_pressure(
    openings: &OpeningsConfig,
    windward_wall_ce: f64,
) -> InternalPressureState {
    match openings.dominance {
        OpeningDominance::Impermeable => InternalPressureState {
            dominance: OpeningDominance::Impermeable,
            cpi_positive: 0.0,
            cpi_negative: -0.2,
            cpi_adopted: -0.2,
            explanation_pt: "Item 6.3.2.2: Edificações estanques (sem aberturas significativas). Adota-se Cpi = -0,20 ou 0,0.".to_string(),
        },

        OpeningDominance::UniformPermeability => InternalPressureState {
            dominance: OpeningDominance::UniformPermeability,
            cpi_positive: 0.0,
            cpi_negative: -0.3,
            cpi_adopted: -0.3,
            explanation_pt: "Item 6.3.2.1-b: Quatro faces igualmente permeáveis. Adota-se Cpi = -0,30 ou 0,0 (o mais nocivo).".to_string(),
        },

        OpeningDominance::DominantWindward => {
            // Item 6.3.2.1-c: Abertura dominante na face de barlavento
            // Proporção entre área na face de barlavento e demais aberturas
            let ratio = openings.opening_ratio();
            let cpi = if ratio >= 6.0 {
                0.80
            } else if ratio >= 3.0 {
                0.60 + (ratio - 3.0) / 3.0 * (0.80 - 0.60)
            } else if ratio >= 2.0 {
                0.50 + (ratio - 2.0) * (0.60 - 0.50)
            } else if ratio >= 1.5 {
                0.30 + (ratio - 1.5) / 0.5 * (0.50 - 0.30)
            } else if ratio >= 1.0 {
                0.10 + (ratio - 1.0) / 0.5 * (0.30 - 0.10)
            } else {
                0.10 * ratio
            };
            let cpi_rounded = (cpi.min(windward_wall_ce) * 100.0).round() / 100.0;

            InternalPressureState {
                dominance: OpeningDominance::DominantWindward,
                cpi_positive: cpi_rounded,
                cpi_negative: 0.0,
                cpi_adopted: cpi_rounded,
                explanation_pt: format!(
                    "Item 6.3.2.1-c: Abertura dominante a barlavento (razão de áreas = {:.2}). Cpi = +{:.2} gerando sobrepressão interna máxima.",
                    ratio, cpi_rounded
                ),
            }
        }

        OpeningDominance::DominantLeeward => {
            // Abertura dominante na face de sotavento
            let ratio = openings.opening_ratio();
            let cpi = if ratio >= 3.0 {
                -0.90
            } else if ratio >= 1.5 {
                -0.80 + (ratio - 1.5) / 1.5 * (-0.90 - (-0.80))
            } else if ratio >= 1.0 {
                -0.70 + (ratio - 1.0) / 0.5 * (-0.80 - (-0.70))
            } else if ratio >= 0.75 {
                -0.60 + (ratio - 0.75) / 0.25 * (-0.70 - (-0.60))
            } else if ratio >= 0.50 {
                -0.50 + (ratio - 0.50) / 0.25 * (-0.60 - (-0.50))
            } else {
                -0.40
            };
            let cpi_rounded = (cpi * 100.0).round() / 100.0;

            InternalPressureState {
                dominance: OpeningDominance::DominantLeeward,
                cpi_positive: 0.0,
                cpi_negative: cpi_rounded,
                cpi_adopted: cpi_rounded,
                explanation_pt: format!(
                    "Item 6.3.2.1-c: Abertura dominante a sotavento (razão de áreas = {:.2}). Cpi = {:.2} gerando sucção interna elevada.",
                    ratio, cpi_rounded
                ),
            }
        }

        OpeningDominance::DominantParallel => {
            // Abertura dominante em face lateral (paralela ao vento)
            let ratio = openings.opening_ratio();
            let cpi = if ratio >= 2.0 { -0.70 } else { -0.50 };
            InternalPressureState {
                dominance: OpeningDominance::DominantParallel,
                cpi_positive: 0.0,
                cpi_negative: cpi,
                cpi_adopted: cpi,
                explanation_pt: format!(
                    "Item 6.3.2.1-c: Abertura dominante em face lateral paralela ao vento. Cpi = {:.2} (sucção interna).",
                    cpi
                ),
            }
        }

        OpeningDominance::CustomCpi => {
            let val = openings.custom_cpi_value;
            InternalPressureState {
                dominance: OpeningDominance::CustomCpi,
                cpi_positive: val.max(0.0),
                cpi_negative: val.min(0.0),
                cpi_adopted: val,
                explanation_pt: format!("Valor de Cpi fixado manualmente pelo usuário: {:.2}.", val),
            }
        }
    }
}
