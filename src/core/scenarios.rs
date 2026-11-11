use super::project::*;
use super::types::*;
use crate::locale::Language;

pub struct EngineeringScenario {
    pub id: u32,
    pub name_pt: &'static str,
    pub name_en: &'static str,
    pub description_pt: &'static str,
    pub description_en: &'static str,
    pub project: EoloProject,
}

impl EngineeringScenario {
    pub fn name(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt,
            Language::EnUs => self.name_en,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.description_pt,
            Language::EnUs => self.description_en,
        }
    }
}

pub fn get_preset_scenarios() -> Vec<EngineeringScenario> {
    vec![
        // Cenário 1: Galpão Industrial Padrão
        EngineeringScenario {
            id: 1,
            name_pt: "1. Galpão Industrial Padrão 20x40m (Curitiba/PR)",
            name_en: "1. Standard Industrial Warehouse 20x40m (Curitiba/PR)",
            description_pt: "Estrutura metálica padrão de 20 m de vão livre por 40 m de comprimento, pé-direito de 6,0 m, inclinação de 15% (8,5°), V0 = 45 m/s em terreno Categoria IV.",
            description_en: "Standard steel warehouse with 20 m clear span by 40 m length, 6.0 m eave height, 15% roof slope (8.5°), V0 = 45 m/s in Category IV terrain.",
            project: {
                let mut p = EoloProject::default();
                p.metadata.title = "Galpão Industrial Padrão 20x40m".to_string();
                p.metadata.location_city = "Curitiba".to_string();
                p.metadata.location_state = "PR".to_string();
                p.wind.v0 = 45.0;
                p.wind.roughness = TerrainRoughness::CategoryIV;
                p.wind.statistical_group = StatisticalGroup::Group3;
                p.geometry.span_b = 20.0;
                p.geometry.length_a = 40.0;
                p.geometry.eave_height_h = 6.0;
                p.geometry.pitch_angle_deg = 8.53;
                p.geometry.eave_overhang_x = 0.80;
                p.geometry.eave_overhang_y = 0.50;
                p.openings.dominance = OpeningDominance::UniformPermeability;
                p.sync_parameters();
                p
            },
        },

        // Cenário 2: Hangar Logístico Portuário
        EngineeringScenario {
            id: 2,
            name_pt: "2. Hangar Logístico Portuário 30x60m (Itajaí/SC)",
            name_en: "2. Port Logistics Hangar 30x60m (Itajaí/SC)",
            description_pt: "Centro de armazenagem portuário em campo aberto próximo ao mar (Categoria II), vão de 30 m, pé-direito de 9,0 m, beirais salientes de 1,2 m e ventos costeiros de 45 m/s.",
            description_en: "Port storage center in open coastal terrain (Category II), 30 m span, 9.0 m eave height, 1.2 m overhang eaves, and 45 m/s coastal winds.",
            project: {
                let mut p = EoloProject::default();
                p.metadata.title = "Hangar Logístico Portuário de Cargas".to_string();
                p.metadata.location_city = "Itajaí".to_string();
                p.metadata.location_state = "SC".to_string();
                p.wind.v0 = 45.0;
                p.wind.roughness = TerrainRoughness::CategoryII;
                p.wind.statistical_group = StatisticalGroup::Group3;
                p.geometry.span_b = 30.0;
                p.geometry.length_a = 60.0;
                p.geometry.eave_height_h = 9.0;
                p.geometry.pitch_angle_deg = 10.0;
                p.geometry.eave_overhang_x = 1.20;
                p.geometry.eave_overhang_y = 0.80;
                p.openings.dominance = OpeningDominance::DominantWindward;
                p.openings.dominant_area = 24.0; // Grande portão de doca
                p.openings.other_openings_area = 6.0;
                p.sync_parameters();
                p
            },
        },

        // Cenário 3: Posto de Combustíveis / Cobertura Isolada
        EngineeringScenario {
            id: 3,
            name_pt: "3. Posto de Serviços / Cobertura Isolada 15x24m (Campinas/SP)",
            name_en: "3. Gas Station / Isolated Canopy 15x24m (Campinas/SP)",
            description_pt: "Marquise metálica aberta sobre pilares sem paredes (cobertura isolada), vão de 15 m por 24 m de comprimento, pé-direito de 5,5 m, V0 = 40 m/s em Categoria III.",
            description_en: "Open steel canopy on columns without walls (isolated roof), 15 m span by 24 m length, 5.5 m clearance height, V0 = 40 m/s in Category III.",
            project: {
                let mut p = EoloProject::default();
                p.metadata.title = "Cobertura de Pista de Abastecimento — Posto de Serviços".to_string();
                p.metadata.location_city = "Campinas".to_string();
                p.metadata.location_state = "SP".to_string();
                p.wind.v0 = 40.0;
                p.wind.roughness = TerrainRoughness::CategoryIII;
                p.wind.statistical_group = StatisticalGroup::Group3;
                p.geometry.typology = RoofTypology::CanopyIsolated;
                p.geometry.span_b = 15.0;
                p.geometry.length_a = 24.0;
                p.geometry.eave_height_h = 5.5;
                p.geometry.pitch_angle_deg = 2.0; // Quase plana para drenagem
                p.geometry.eave_overhang_x = 1.50;
                p.geometry.eave_overhang_y = 1.50;
                p.openings.dominance = OpeningDominance::UniformPermeability;
                p.sync_parameters();
                p
            },
        },

        // Cenário 4: Ginásio Esportivo / Cobertura Curva
        EngineeringScenario {
            id: 4,
            name_pt: "4. Ginásio Esportivo em Arco 25x50m (Porto Alegre/RS)",
            name_en: "4. Arched Sports Gymnasium 25x50m (Porto Alegre/RS)",
            description_pt: "Cobertura curva autoportante para centro poliesportivo (Grupo 2, S3 = 1,06 para aglomeração de público), vão de 25 m, V0 = 45 m/s em Categoria IV.",
            description_en: "Self-supporting curved arch roof for multi-sport arena (Group 2, S3 = 1.06 for public assembly), 25 m span, V0 = 45 m/s in Category IV.",
            project: {
                let mut p = EoloProject::default();
                p.metadata.title = "Arena Poliesportiva Municipal — Cobertura em Arco".to_string();
                p.metadata.location_city = "Porto Alegre".to_string();
                p.metadata.location_state = "RS".to_string();
                p.wind.v0 = 45.0;
                p.wind.roughness = TerrainRoughness::CategoryIV;
                p.wind.statistical_group = StatisticalGroup::Group2; // Aglomeração pública
                p.geometry.typology = RoofTypology::CurvedArch;
                p.geometry.span_b = 25.0;
                p.geometry.length_a = 50.0;
                p.geometry.eave_height_h = 7.0;
                p.geometry.pitch_angle_deg = 15.0;
                p.geometry.eave_overhang_x = 0.60;
                p.geometry.eave_overhang_y = 0.60;
                p.openings.dominance = OpeningDominance::UniformPermeability;
                p.sync_parameters();
                p
            },
        },

        // Cenário 5: Silo e Armazém de Grãos Rural
        EngineeringScenario {
            id: 5,
            name_pt: "5. Armazém e Depósito de Grãos 18x36m (Rio Verde/GO)",
            name_en: "5. Rural Grain Storage Warehouse 18x36m (Rio Verde/GO)",
            description_pt: "Estrutura rural para depósito agrícola sem circulação contínua de pessoas (Grupo 4, S3 = 0,95), pé-direito elevado de 10,0 m, terreno plano aberto (Categoria II), V0 = 35 m/s.",
            description_en: "Rural agricultural storage facility without continuous human occupancy (Group 4, S3 = 0.95), 10.0 m tall eave height, open flat terrain (Category II), V0 = 35 m/s.",
            project: {
                let mut p = EoloProject::default();
                p.metadata.title = "Armazém Agrícola de Grãos e Fertilizantes".to_string();
                p.metadata.location_city = "Rio Verde".to_string();
                p.metadata.location_state = "GO".to_string();
                p.wind.v0 = 35.0;
                p.wind.roughness = TerrainRoughness::CategoryII;
                p.wind.statistical_group = StatisticalGroup::Group4; // Não destinado à ocupação humana
                p.geometry.span_b = 18.0;
                p.geometry.length_a = 36.0;
                p.geometry.eave_height_h = 10.0;
                p.geometry.pitch_angle_deg = 20.0; // Telhado íngreme para escoamento de grãos
                p.geometry.eave_overhang_x = 0.50;
                p.geometry.eave_overhang_y = 0.50;
                p.openings.dominance = OpeningDominance::UniformPermeability;
                p.sync_parameters();
                p
            },
        },
    ]
}
