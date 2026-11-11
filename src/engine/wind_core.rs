use serde::{Deserialize, Serialize};

use crate::core::project::EoloProject;
use super::factors_s1_s2_s3::{calculate_s1, calculate_s2, calculate_s3};

/// Estado calculado das forças e velocidades de vento na cota de projeto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindCalculatedState {
    pub v0: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
    /// Velocidade característica do vento Vk (m/s)
    pub vk_ms: f64,
    /// Velocidade característica do vento Vk (km/h)
    pub vk_kmh: f64,
    /// Pressão dinâmica básica q = 0,613 * Vk^2 em Pascal (Pa = N/m²)
    pub q_pa: f64,
    /// Pressão dinâmica em kN/m² (kPa)
    pub q_kn_m2: f64,
    /// Pressão dinâmica em kgf/m²
    pub q_kgf_m2: f64,
    /// Altura de referência z (m)
    pub reference_z: f64,

    // Valores específicos para elementos de vedação (telhas/painéis/vidros) com redução 0.92 * S3
    pub s3_cladding: f64,
    pub vk_cladding_ms: f64,
    pub q_cladding_kn_m2: f64,
}

impl WindCalculatedState {
    pub fn compute(project: &EoloProject) -> Self {
        let v0 = project.wind.v0;
        let s1 = calculate_s1(&project.wind.topography);
        let s2 = calculate_s2(
            project.wind.roughness,
            project.wind.building_class,
            project.wind.reference_height_z,
        );
        let s3 = calculate_s3(project.wind.statistical_group, false);

        let vk_ms = v0 * s1 * s2 * s3;
        let vk_kmh = vk_ms * 3.6;

        // Equação 4.2 da NBR 6123:2023: q = 0,613 * Vk^2
        let q_pa = 0.613 * vk_ms * vk_ms;
        let q_kn_m2 = q_pa / 1000.0;
        let q_kgf_m2 = q_pa / 9.80665;

        // Vedações
        let s3_cladding = calculate_s3(project.wind.statistical_group, true);
        let vk_cladding_ms = v0 * s1 * s2 * s3_cladding;
        let q_cladding_pa = 0.613 * vk_cladding_ms * vk_cladding_ms;
        let q_cladding_kn_m2 = q_cladding_pa / 1000.0;

        Self {
            v0,
            s1,
            s2,
            s3,
            vk_ms,
            vk_kmh,
            q_pa,
            q_kn_m2,
            q_kgf_m2,
            reference_z: project.wind.reference_height_z,
            s3_cladding,
            vk_cladding_ms,
            q_cladding_kn_m2,
        }
    }

    /// Gera o perfil vertical de velocidades Vk(z) e pressões q(z) para gráficos na GPU
    pub fn generate_vertical_profile(
        project: &EoloProject,
        max_height: f64,
        num_points: usize,
    ) -> Vec<(f64, f64, f64)> {
        let mut profile = Vec::with_capacity(num_points);
        let step = max_height / (num_points.max(2) - 1) as f64;
        let s1 = calculate_s1(&project.wind.topography);
        let s3 = calculate_s3(project.wind.statistical_group, false);

        for i in 0..num_points {
            let z = i as f64 * step;
            let s2 = calculate_s2(
                project.wind.roughness,
                project.wind.building_class,
                z,
            );
            let vk = project.wind.v0 * s1 * s2 * s3;
            let q = 0.613 * vk * vk / 1000.0; // em kN/m²
            profile.push((z, vk, q));
        }
        profile
    }
}
