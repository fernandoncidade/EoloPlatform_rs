use serde::{Deserialize, Serialize};

use super::types::*;

/// Metadados formais do projeto para o Memorial de Cálculo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub title: String,
    pub engineer: String,
    pub crea_art: String,
    pub client: String,
    pub location_city: String,
    pub location_state: String,
    pub date: String,
    pub notes: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            title: "Galpão Industrial em Estrutura Metálica — Ações de Vento".to_string(),
            engineer: "Eng. Civil Fernando Nillsson Cidade".to_string(),
            crea_art: "CREA-PR / ART nº 2026.11.001".to_string(),
            client: "Indústria & Logística Brasileira S/A".to_string(),
            location_city: "Curitiba".to_string(),
            location_state: "PR".to_string(),
            date: chrono::Local::now().format("%d/%m/%Y").to_string(),
            notes: "Dimensionamento e verificação de forças devidas ao vento conforme ABNT NBR 6123:2023. Análise de envoltórias críticas para pórticos e terças de cobertura.".to_string(),
        }
    }
}

/// Parâmetros topográficos para determinação de S1 (Item 5.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopographyParams {
    pub topo_type: TopographyType,
    /// Diferença de nível dt (m) entre a base e o topo do talude ou morro
    pub dt: f64,
    /// Inclinação média do talude ou encosta theta_t (graus)
    pub theta_t_deg: f64,
    /// Posição do ponto na encosta (0.0 = base A, 1.0 = topo/crista B, > 1.0 = sotavento C)
    pub relative_position: f64,
    /// Altura z (m) medida a partir da superfície do terreno no ponto considerado
    pub z_point: f64,
}

impl Default for TopographyParams {
    fn default() -> Self {
        Self {
            topo_type: TopographyType::Flat,
            dt: 20.0,
            theta_t_deg: 10.0,
            relative_position: 1.0, // Topo/crista
            z_point: 10.0,
        }
    }
}

/// Parâmetros fundamentais de vento (NBR 6123:2023 Capítulos 4 e 5)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindParameters {
    /// Velocidade básica do vento V0 (m/s) das isopletas da NBR 6123:2023
    pub v0: f64,
    /// Parâmetros para cálculo do fator topográfico S1
    pub topography: TopographyParams,
    /// Categoria de rugosidade do terreno (I a V)
    pub roughness: TerrainRoughness,
    /// Classe dimensional da edificação (A, B ou C)
    pub building_class: BuildingClass,
    /// Se true, a classe é determinada automaticamente pela maior dimensão max(a, b, h)
    pub auto_class: bool,
    /// Fator estatístico S3 baseado na ocupação e vida útil
    pub statistical_group: StatisticalGroup,
    /// Aplica redução de 0,92 * S3 para o dimensionamento exclusivo de vedações (Nota da Tabela 4)
    pub apply_cladding_s3_reduction: bool,
    /// Altura z (m) de referência para cálculo da velocidade característica e pressão dinâmica
    pub reference_height_z: f64,
}

impl Default for WindParameters {
    fn default() -> Self {
        Self {
            v0: 45.0, // Típico Região Sul / Curitiba
            topography: TopographyParams::default(),
            roughness: TerrainRoughness::CategoryIV, // Área urbana/industrial densa
            building_class: BuildingClass::ClassB,  // Galpão de 40m
            auto_class: true,
            statistical_group: StatisticalGroup::Group3, // Indústria corrente (S3 = 1.00)
            apply_cladding_s3_reduction: false,
            reference_height_z: 7.5, // Altura média/cumeeira
        }
    }
}

/// Geometria tridimensional completa da edificação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingGeometry {
    /// Tipologia da cobertura
    pub typology: RoofTypology,
    /// Dimensão em planta: vão ou largura b (m), medida transversalmente aos pórticos
    pub span_b: f64,
    /// Dimensão em planta: comprimento total a (m), medido ao longo da cumeeira
    pub length_a: f64,
    /// Pé-direito livre na calha / beiral h (m)
    pub eave_height_h: f64,
    /// Inclinação da cobertura theta (graus). Ex: 5.71° = 10%
    pub pitch_angle_deg: f64,
    /// Beiral lateral ex (m) — balanço da cobertura além das paredes laterais
    pub eave_overhang_x: f64,
    /// Beiral frontal ey (m) — balanço da cobertura nos oitões
    pub eave_overhang_y: f64,
    /// Espaçamento longitudinal entre pórticos principais (m)
    pub frame_spacing: f64,
    /// Espaçamento entre terças de cobertura (m)
    pub purlin_spacing: f64,
    /// Espaçamento entre longarinas de fechamento lateral (m)
    pub girt_spacing: f64,
    /// Largura do lanternim ou abertura zenital na cumeeira (m) — 0 se não houver
    pub ridge_monitor_width: f64,
    /// Altura do lanternim (m)
    pub ridge_monitor_height: f64,
}

impl Default for BuildingGeometry {
    fn default() -> Self {
        Self {
            typology: RoofTypology::DuoPitch,
            span_b: 20.0,
            length_a: 40.0,
            eave_height_h: 6.0,
            pitch_angle_deg: 8.53, // ~15% de inclinação
            eave_overhang_x: 0.80, // 80 cm de beiral
            eave_overhang_y: 0.50, // 50 cm no oitão
            frame_spacing: 6.0,    // Pórticos a cada 6 m
            purlin_spacing: 1.50,   // Terças a cada 1,5 m
            girt_spacing: 1.50,     // Longarinas a cada 1,5 m
            ridge_monitor_width: 2.0,
            ridge_monitor_height: 0.8,
        }
    }
}

impl BuildingGeometry {
    /// Altura total da edificação na cumeeira (m)
    pub fn total_ridge_height(&self) -> f64 {
        match self.typology {
            RoofTypology::DuoPitch => {
                let half_span = self.span_b / 2.0;
                let rise = half_span * (self.pitch_angle_deg.to_radians()).tan();
                self.eave_height_h + rise
            }
            RoofTypology::MonoPitch => {
                let rise = self.span_b * (self.pitch_angle_deg.to_radians()).tan();
                self.eave_height_h + rise
            }
            RoofTypology::CurvedArch => {
                // Flecha padrão de arco ~ 15% do vão
                self.eave_height_h + (self.span_b * 0.15)
            }
            RoofTypology::CanopyIsolated => self.eave_height_h,
            RoofTypology::ShedIndustrial => {
                let rise = (self.span_b / 2.0) * (self.pitch_angle_deg.to_radians()).tan();
                self.eave_height_h + rise + self.ridge_monitor_height
            }
        }
    }

    /// Altura média da cobertura h_med (m) para aplicação normativa
    pub fn mean_height(&self) -> f64 {
        (self.eave_height_h + self.total_ridge_height()) / 2.0
    }

    /// Menor dimensão em planta b_min
    pub fn minor_dimension(&self) -> f64 {
        self.span_b.min(self.length_a)
    }

    /// Maior dimensão em planta a_max
    pub fn major_dimension(&self) -> f64 {
        self.span_b.max(self.length_a)
    }

    /// Relação de aspecto em planta a/b
    pub fn aspect_ratio_a_over_b(&self) -> f64 {
        let b = self.minor_dimension();
        if b > 0.0 {
            self.major_dimension() / b
        } else {
            1.0
        }
    }

    /// Relação de altura relativa h/b
    pub fn relative_height_h_over_b(&self) -> f64 {
        let b = self.minor_dimension();
        if b > 0.0 {
            self.total_ridge_height() / b
        } else {
            0.5
        }
    }

    /// Maior dimensão global para determinação da Classe da edificação
    pub fn max_global_dimension(&self) -> f64 {
        self.length_a.max(self.span_b).max(self.total_ridge_height())
    }

    /// Área de projeção em planta (m²)
    pub fn footprint_area(&self) -> f64 {
        self.span_b * self.length_a
    }

    /// Área frontal efetiva Ae para vento a 90° (vento transversal na fachada longa ou frontal)
    pub fn frontal_area_90deg(&self) -> f64 {
        self.length_a * self.mean_height()
    }

    /// Área frontal efetiva Ae para vento a 0° (vento longitudinal no oitão)
    pub fn frontal_area_0deg(&self) -> f64 {
        let wall_area = self.span_b * self.eave_height_h;
        let gable_area = 0.5 * self.span_b * (self.total_ridge_height() - self.eave_height_h);
        wall_area + gable_area
    }
}

/// Configuração de aberturas e permeabilidade para pressão interna Cpi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningsConfig {
    pub dominance: OpeningDominance,
    /// Área da abertura dominante (m²) — ex: portão industrial aberto
    pub dominant_area: f64,
    /// Área total das demais aberturas (m²)
    pub other_openings_area: f64,
    /// Índice de permeabilidade disseminada geral (%) — ex: 0.1% a 1.0%
    pub permeability_percent: f64,
    /// Coeficiente Cpi fixo quando dominance == CustomCpi
    pub custom_cpi_value: f64,
}

impl Default for OpeningsConfig {
    fn default() -> Self {
        Self {
            dominance: OpeningDominance::UniformPermeability,
            dominant_area: 12.0, // Portão de 3 x 4 m
            other_openings_area: 8.0,
            permeability_percent: 0.5,
            custom_cpi_value: -0.3,
        }
    }
}

impl OpeningsConfig {
    /// Razão entre área da abertura dominante e total das outras aberturas
    pub fn opening_ratio(&self) -> f64 {
        if self.other_openings_area > 0.0 {
            self.dominant_area / self.other_openings_area
        } else {
            10.0
        }
    }
}

/// Estrutura central do projeto Eolo Platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EoloProject {
    pub metadata: ProjectMetadata,
    pub wind: WindParameters,
    pub geometry: BuildingGeometry,
    pub openings: OpeningsConfig,
    pub wind_angle_analysis: WindAnglePreset,
    pub display_unit: PressureUnit,
}

impl Default for EoloProject {
    fn default() -> Self {
        let geometry = BuildingGeometry::default();
        let mut wind = WindParameters::default();
        if wind.auto_class {
            wind.building_class = BuildingClass::from_dimension(geometry.max_global_dimension());
        }
        wind.reference_height_z = geometry.total_ridge_height();

        Self {
            metadata: ProjectMetadata::default(),
            wind,
            geometry,
            openings: OpeningsConfig::default(),
            wind_angle_analysis: WindAnglePreset::EnvelopeCritical,
            display_unit: PressureUnit::KiloNewtonPerSquareMeter,
        }
    }
}

impl EoloProject {
    /// Atualiza classes e parâmetros automáticos após mudança de geometria
    pub fn sync_parameters(&mut self) {
        if self.wind.auto_class {
            self.wind.building_class =
                BuildingClass::from_dimension(self.geometry.max_global_dimension());
        }
        self.wind.reference_height_z = self.geometry.total_ridge_height();
    }
}
