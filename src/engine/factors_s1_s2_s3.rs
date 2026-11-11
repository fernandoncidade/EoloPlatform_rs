use crate::core::types::*;
use crate::core::project::TopographyParams;

/// Cálculo analítico rigoroso do Fator Topográfico S1 segundo ABNT NBR 6123:2023 (Item 5.2)
pub fn calculate_s1(params: &TopographyParams) -> f64 {
    match params.topo_type {
        TopographyType::Flat => 1.0,
        TopographyType::DeepValley => 0.90,
        TopographyType::Slope | TopographyType::Hill => {
            let theta = params.theta_t_deg;
            let dt = params.dt.max(1.0);
            let z = params.z_point.max(0.0);

            // Cálculo do S1 no topo / crista (Ponto B)
            let s1_b = calculate_s1_at_crest(theta, dt, z);

            // Posição relativa: 0.0 = base A (S1 = 1.0), 1.0 = crista B (S1 = s1_b)
            // Se for talude ou morro, interpola linearmente entre A e B
            if params.relative_position <= 0.0 {
                1.0
            } else if params.relative_position >= 1.0 {
                // Se for ponto na crista ou sotavento
                if params.relative_position == 1.0 {
                    s1_b
                } else if params.topo_type == TopographyType::Slope {
                    // No talude, após a crista B o terreno continua no patamar superior
                    // NBR 6123 indica retorno gradual a 1.0 no ponto C
                    let pos_past_crest = params.relative_position - 1.0;
                    if pos_past_crest >= 1.0 {
                        1.0
                    } else {
                        s1_b + (1.0 - s1_b) * pos_past_crest
                    }
                } else {
                    // No morro, desce a encosta de sotavento até a base
                    let pos_down = params.relative_position - 1.0;
                    if pos_down >= 1.0 {
                        1.0
                    } else {
                        s1_b + (1.0 - s1_b) * pos_down
                    }
                }
            } else {
                // Interpolação linear entre Base A e Crista B
                1.0 + (s1_b - 1.0) * params.relative_position
            }
        }
    }
}

/// Cálculo do fator S1(z) exatamente no ponto B (crista/topo)
fn calculate_s1_at_crest(theta_deg: f64, dt: f64, z: f64) -> f64 {
    let z_ratio = z / dt;
    let factor_height = (2.5 - z_ratio).max(0.0);

    if theta_deg <= 3.0 {
        1.0
    } else if theta_deg >= 45.0 {
        // Equação da NBR 6123:2023: S1(z) = 1.0 + (2.5 - z/dt) * 0.31 >= 1.0
        (1.0 + factor_height * 0.31).max(1.0)
    } else if (6.0..=17.0).contains(&theta_deg) {
        // S1(z) = 1.0 + (2.5 - z/dt) * tg(theta_t - 3°) >= 1.0
        let delta_angle_rad = (theta_deg - 3.0).to_radians();
        (1.0 + factor_height * delta_angle_rad.tan()).max(1.0)
    } else if theta_deg > 3.0 && theta_deg < 6.0 {
        // Interpolação linear entre 3° (S1 = 1.0) e 6°
        let s1_at_6 = {
            let delta_6_rad = (6.0 - 3.0f64).to_radians();
            (1.0 + factor_height * delta_6_rad.tan()).max(1.0)
        };
        let t = (theta_deg - 3.0) / (6.0 - 3.0);
        1.0 + (s1_at_6 - 1.0) * t
    } else {
        // 17° < theta_deg < 45°: Interpolação linear entre 17° e 45°
        let s1_at_17 = {
            let delta_17_rad = (17.0 - 3.0f64).to_radians();
            (1.0 + factor_height * delta_17_rad.tan()).max(1.0)
        };
        let s1_at_45 = (1.0 + factor_height * 0.31).max(1.0);
        let t = (theta_deg - 17.0) / (45.0 - 17.0);
        s1_at_17 + (s1_at_45 - s1_at_17) * t
    }
}

/// Cálculo analítico rigoroso do Fator S2 segundo ABNT NBR 6123:2023 (Item 5.3)
/// S2 = bm * Fr * (z/10)^p
pub fn calculate_s2(roughness: TerrainRoughness, class: BuildingClass, height_z: f64) -> f64 {
    let zg = roughness.zg();
    let (bm, p) = roughness.parameters(class);
    let fr = class.fr();

    // Regra da NBR 6123:2023 para parte inferior da edificação:
    // - Para z < 5 m, adota-se o valor correspondente a z = 5 m (Tabela 3)
    // - Para Categoria V, o fator S2 é considerado constante até 10 m de altura
    let effective_z = if roughness == TerrainRoughness::CategoryV {
        height_z.max(10.0)
    } else {
        height_z.max(5.0)
    };

    // Até a altura zg (camada limite)
    let capped_z = effective_z.min(zg);

    let s2 = bm * fr * (capped_z / 10.0).powf(p);
    (s2 * 1000.0).round() / 1000.0 // Arredondamento normalizado a 3 casas decimais
}

/// Fator Estatístico S3 segundo NBR 6123:2023 (Item 5.4 e Tabela 4)
pub fn calculate_s3(group: StatisticalGroup, is_cladding_reduction: bool) -> f64 {
    let base_s3 = group.s3();
    if is_cladding_reduction {
        // Nota da Tabela 4: Exclusivamente para vedações, pode-se usar (0,92 * S3)
        (0.92 * base_s3 * 1000.0).round() / 1000.0
    } else {
        base_s3
    }
}

/// Cálculo da transição de rugosidade de terreno segundo NBR 6123:2023 (Item 5.5)
/// Retorna as alturas zx e zi da camada interna e externa para a distância x da transição
pub fn calculate_roughness_transition(
    z01: f64,
    z02: f64,
    distance_x: f64,
) -> (f64, f64) {
    if distance_x <= 0.0 || z01 <= 0.0 || z02 <= 0.0 {
        return (0.0, 0.0);
    }
    // A = 0,63 - 0,03 * ln(z02 / z01)
    let a = 0.63 - 0.03 * (z02 / z01).ln();
    // zx = A * z02 * (x / z02)^0.8
    let zx = a * z02 * (distance_x / z02).powf(0.8);
    // zi = 0,36 * z02 * (x / z02)^0.75
    let zi = 0.36 * z02 * (distance_x / z02).powf(0.75);
    (zx, zi)
}
