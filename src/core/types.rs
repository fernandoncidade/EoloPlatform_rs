use serde::{Deserialize, Serialize};
use crate::locale::Language;

/// Categoria de rugosidade do terreno segundo NBR 6123:2023 (Tabela 1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainRoughness {
    /// Categoria I: Superfícies lisas de grandes dimensões (mar calmo, lagos, pântanos sem vegetação)
    CategoryI,
    /// Categoria II: Terrenos abertos com poucos obstáculos (pastagens, campos de aviação, litoral desabrigado)
    CategoryII,
    /// Categoria III: Terrenos com obstáculos baixos e dispersos (granjas, subúrbios distantes, parques industriais)
    CategoryIII,
    /// Categoria IV: Terrenos com obstáculos numerosos e pouco espaçados (zonas urbanas densas, florestas, distritos industriais)
    CategoryIV,
    /// Categoria V: Terrenos cobertos por obstáculos grandes, altos e muito pouco espaçados (centros de grandes cidades)
    CategoryV,
}

impl TerrainRoughness {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::CategoryI => "Categoria I — Superfícies lisas de grandes dimensões (mar calmo, lagos)",
            Self::CategoryII => "Categoria II — Terrenos abertos com poucos obstáculos (pastagens, campos abertos)",
            Self::CategoryIII => "Categoria III — Obstáculos baixos e dispersos (granjas, subúrbios)",
            Self::CategoryIV => "Categoria IV — Obstáculos numerosos e pouco espaçados (cidades, zonas industriais)",
            Self::CategoryV => "Categoria V — Obstáculos grandes, altos e densos (centros de metrópoles)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::CategoryI => "Category I — Large smooth surfaces (calm sea, lakes, marshes)",
            Self::CategoryII => "Category II — Open terrain with few obstacles (pastures, airfields, coast)",
            Self::CategoryIII => "Category III — Low and scattered obstacles (farms, suburbs, light industrial)",
            Self::CategoryIV => "Category IV — Numerous closely spaced obstacles (cities, dense industrial)",
            Self::CategoryV => "Category V — Large, tall, and dense obstacles (metropolitan centers)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::CategoryI => "Cat. I",
            Self::CategoryII => "Cat. II",
            Self::CategoryIII => "Cat. III",
            Self::CategoryIV => "Cat. IV",
            Self::CategoryV => "Cat. V",
        }
    }

    /// Altura da camada limite atmosférica zg (m) segundo NBR 6123:2023 Tabela 1
    pub fn zg(&self) -> f64 {
        match self {
            Self::CategoryI => 250.0,
            Self::CategoryII => 300.0,
            Self::CategoryIII => 350.0,
            Self::CategoryIV => 420.0,
            Self::CategoryV => 500.0,
        }
    }

    /// Parâmetro de rugosidade z0 (m) segundo NBR 6123:2023 Tabela 5
    pub fn z0(&self) -> f64 {
        match self {
            Self::CategoryI => 0.005,
            Self::CategoryII => 0.07,
            Self::CategoryIII => 0.30,
            Self::CategoryIV => 1.00,
            Self::CategoryV => 2.50,
        }
    }

    /// Parâmetro meteorológico bm e expoente p para uma dada classe de edificação
    pub fn parameters(&self, class: BuildingClass) -> (f64, f64) {
        match (self, class) {
            (Self::CategoryI, BuildingClass::ClassA) => (1.10, 0.060),
            (Self::CategoryI, BuildingClass::ClassB) => (1.11, 0.065),
            (Self::CategoryI, BuildingClass::ClassC) => (1.12, 0.070),

            (Self::CategoryII, BuildingClass::ClassA) => (1.00, 0.085),
            (Self::CategoryII, BuildingClass::ClassB) => (1.00, 0.090),
            (Self::CategoryII, BuildingClass::ClassC) => (1.00, 0.100),

            (Self::CategoryIII, BuildingClass::ClassA) => (0.94, 0.100),
            (Self::CategoryIII, BuildingClass::ClassB) => (0.94, 0.105),
            (Self::CategoryIII, BuildingClass::ClassC) => (0.93, 0.115),

            (Self::CategoryIV, BuildingClass::ClassA) => (0.86, 0.120),
            (Self::CategoryIV, BuildingClass::ClassB) => (0.85, 0.125),
            (Self::CategoryIV, BuildingClass::ClassC) => (0.84, 0.135),

            (Self::CategoryV, BuildingClass::ClassA) => (0.74, 0.150),
            (Self::CategoryV, BuildingClass::ClassB) => (0.73, 0.160),
            (Self::CategoryV, BuildingClass::ClassC) => (0.71, 0.175),
        }
    }
}

/// Classe de dimensão da edificação ou componente segundo NBR 6123:2023 (Item 5.3.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingClass {
    /// Classe A: Maior dimensão <= 20 m (intervalo 3 s)
    ClassA,
    /// Classe B: Maior dimensão entre 20 m e 50 m (intervalo 5 s)
    ClassB,
    /// Classe C: Maior dimensão > 50 m (intervalo 10 s)
    ClassC,
}

impl BuildingClass {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::ClassA => "Classe A — Maior dimensão horizontal ou vertical ≤ 20 m (rajada de 3 s)",
            Self::ClassB => "Classe B — Maior dimensão entre 20 m e 50 m (rajada de 5 s)",
            Self::ClassC => "Classe C — Maior dimensão frontal > 50 m (rajada de 10 s)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::ClassA => "Class A — Largest horizontal or vertical dimension ≤ 20 m (3 s gust)",
            Self::ClassB => "Class B — Largest dimension between 20 m and 50 m (5 s gust)",
            Self::ClassC => "Class C — Largest frontal dimension > 50 m (10 s gust)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::ClassA => "Classe A",
            Self::ClassB => "Classe B",
            Self::ClassC => "Classe C",
        }
    }

    pub fn short_name_i18n(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::PtBr, Self::ClassA) => "Classe A",
            (Language::PtBr, Self::ClassB) => "Classe B",
            (Language::PtBr, Self::ClassC) => "Classe C",
            (Language::EnUs, Self::ClassA) => "Class A",
            (Language::EnUs, Self::ClassB) => "Class B",
            (Language::EnUs, Self::ClassC) => "Class C",
        }
    }

    /// Fator de rajada Fr segundo NBR 6123:2023 Tabela 2
    pub fn fr(&self) -> f64 {
        match self {
            Self::ClassA => 1.00,
            Self::ClassB => 0.98,
            Self::ClassC => 0.95,
        }
    }

    /// Determinação automática da classe baseada na maior dimensão frontal
    pub fn from_dimension(d_max: f64) -> Self {
        if d_max <= 20.0 {
            Self::ClassA
        } else if d_max <= 50.0 {
            Self::ClassB
        } else {
            Self::ClassC
        }
    }
}

/// Fator estatístico S3 segundo NBR 6123:2023 (Tabela 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatisticalGroup {
    /// Grupo 1: Hospitais, quartéis, centrais de controle, substâncias perigosas (S3 = 1.11, Tp = 100 anos)
    Group1,
    /// Grupo 2: Aglomerações > 300 pessoas, escolas > 250, ginásios, estádios (S3 = 1.06, Tp = 75 anos)
    Group2,
    /// Grupo 3: Residências, hotéis, comércio, indústrias correntes (S3 = 1.00, Tp = 50 anos)
    Group3,
    /// Grupo 4: Depósitos sem pessoas no entorno, silos agrícolas (S3 = 0.95, Tp = 37 anos)
    Group4,
    /// Grupo 5: Edificações temporárias não reutilizáveis, obras (S3 = 0.83, Tp = 15 anos)
    Group5,
}

impl StatisticalGroup {
    pub fn s3(&self) -> f64 {
        match self {
            Self::Group1 => 1.11,
            Self::Group2 => 1.06,
            Self::Group3 => 1.00,
            Self::Group4 => 0.95,
            Self::Group5 => 0.83,
        }
    }

    pub fn recurrence_years(&self) -> u32 {
        match self {
            Self::Group1 => 100,
            Self::Group2 => 75,
            Self::Group3 => 50,
            Self::Group4 => 37,
            Self::Group5 => 15,
        }
    }

    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::Group1 => "Grupo 1 — Hospitais, quartéis, controle de emergência, substâncias perigosas (S3 = 1,11)",
            Self::Group2 => "Grupo 2 — Aglomerações > 300 pessoas, estádios, escolas > 250, creches (S3 = 1,06)",
            Self::Group3 => "Grupo 3 — Edificações correntes para residências, comércio e indústrias (S3 = 1,00)",
            Self::Group4 => "Grupo 4 — Edificações sem ocupação humana contínua, silos, depósitos rurais (S3 = 0,95)",
            Self::Group5 => "Grupo 5 — Estruturas temporárias não reutilizáveis e fase construtiva ≤ 2 anos (S3 = 0,83)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Group1 => "Group 1 — Hospitals, fire stations, emergency control, hazardous materials (S3 = 1.11)",
            Self::Group2 => "Group 2 — Assemblies > 300 people, stadiums, schools > 250, daycares (S3 = 1.06)",
            Self::Group3 => "Group 3 — Standard residential, commercial, and industrial buildings (S3 = 1.00)",
            Self::Group4 => "Group 4 — Buildings without continuous human occupancy, silos, rural storage (S3 = 0.95)",
            Self::Group5 => "Group 5 — Non-reusable temporary structures and construction phase ≤ 2 years (S3 = 0.83)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }
}

/// Tipo de relevo para o Fator Topográfico S1 (NBR 6123:2023 Item 5.2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopographyType {
    /// Terreno plano ou fracamente acidentado (S1 = 1.00)
    Flat,
    /// Talude (encosta alongada plana)
    Slope,
    /// Morro alongado (crista)
    Hill,
    /// Vale profundo protegido de ventos (S1 = 0.90)
    DeepValley,
}

impl TopographyType {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::Flat => "Terreno plano ou fracamente acidentado (S1 = 1,00)",
            Self::Slope => "Talude alongado (encosta de corte ou aterro)",
            Self::Hill => "Morro alongado (colina ou crista)",
            Self::DeepValley => "Vale profundo protegido de ventos (S1 = 0,90)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Flat => "Flat or weakly undulating terrain (S1 = 1.00)",
            Self::Slope => "Elongated escarpment / slope (cut or embankment)",
            Self::Hill => "Elongated hill or ridge crest",
            Self::DeepValley => "Deep valley sheltered from wind (S1 = 0.90)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }
}

/// Tipologia da cobertura / edificação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoofTypology {
    /// Telhado a duas águas simétrico (Tabela 7)
    DuoPitch,
    /// Telhado a uma água / monoslope (Tabela 8)
    MonoPitch,
    /// Cobertura curva / abóbada cilíndrica (Figura 7/8, Tabela 13)
    CurvedArch,
    /// Cobertura isolada / marquise / posto de combustíveis (Tabela 14)
    CanopyIsolated,
    /// Shed industrial com lanternim (Tabela 10/11)
    ShedIndustrial,
}

impl RoofTypology {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::DuoPitch => "Galpão com Telhado a Duas Águas Simétrico (NBR 6123 Tabela 7)",
            Self::MonoPitch => "Edificação com Telhado a Uma Água (NBR 6123 Tabela 8)",
            Self::CurvedArch => "Cobertura em Arco / Abóbada Cilíndrica (NBR 6123 Figuras 7 e 8)",
            Self::CanopyIsolated => "Cobertura Isolada / Marquise / Posto de Combustíveis (NBR 6123)",
            Self::ShedIndustrial => "Shed Industrial com Lanternim de Ventilação (NBR 6123 Tabela 10)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::DuoPitch => "Symmetric Duo-Pitch Gable Warehouse (NBR 6123 Table 7)",
            Self::MonoPitch => "Mono-Pitch / Shed-Slope Building (NBR 6123 Table 8)",
            Self::CurvedArch => "Curved Arch / Cylindrical Vault Roof (NBR 6123 Figures 7 & 8)",
            Self::CanopyIsolated => "Isolated Canopy / Marquee / Gas Station Roof (NBR 6123)",
            Self::ShedIndustrial => "Industrial Sawtooth Shed with Ventilation Monitor (NBR 6123 Table 10)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::DuoPitch => "Duas Águas",
            Self::MonoPitch => "Uma Água",
            Self::CurvedArch => "Arco Curvo",
            Self::CanopyIsolated => "Marquise/Posto",
            Self::ShedIndustrial => "Shed/Lanternim",
        }
    }

    pub fn short_name_i18n(&self, lang: Language) -> &'static str {
        match (lang, self) {
            (Language::PtBr, _) => self.short_name(),
            (Language::EnUs, Self::DuoPitch) => "Duo-Pitch",
            (Language::EnUs, Self::MonoPitch) => "Mono-Pitch",
            (Language::EnUs, Self::CurvedArch) => "Curved Arch",
            (Language::EnUs, Self::CanopyIsolated) => "Canopy",
            (Language::EnUs, Self::ShedIndustrial) => "Shed/Monitor",
        }
    }
}

/// Configuração de aberturas e permeabilidade para pressão interna Cpi (Item 6.3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpeningDominance {
    /// Edificação estanque (faces impermeáveis, Cpi = -0.2 ou 0.0)
    Impermeable,
    /// 4 faces com permeabilidade uniforme (Cpi = -0.3 ou 0.0)
    UniformPermeability,
    /// Abertura dominante na face de barlavento (vento soprando na abertura)
    DominantWindward,
    /// Abertura dominante na face de sotavento (lado oposto ao vento)
    DominantLeeward,
    /// Abertura dominante em face paralela ao vento (lateral)
    DominantParallel,
    /// Coeficiente de pressão interna fixo/personalizado pelo usuário
    CustomCpi,
}

impl OpeningDominance {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::Impermeable => "Edificação estanque (paredes e cobertura impermeáveis)",
            Self::UniformPermeability => "Quatro faces com permeabilidade uniforme (janelas/portas padrão)",
            Self::DominantWindward => "Abertura dominante na face de barlavento (portão/abertura frontal)",
            Self::DominantLeeward => "Abertura dominante na face de sotavento (abertura traseira)",
            Self::DominantParallel => "Abertura dominante na face paralela ao vento (fachada lateral)",
            Self::CustomCpi => "Valor de Cpi personalizado diretamente pelo usuário",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Impermeable => "Impermeable building (effectively sealed walls and roof)",
            Self::UniformPermeability => "Four faces with uniform permeability (standard windows/doors)",
            Self::DominantWindward => "Dominant opening on windward face (open front gate/dock)",
            Self::DominantLeeward => "Dominant opening on leeward face (open rear wall)",
            Self::DominantParallel => "Dominant opening on side face parallel to wind",
            Self::CustomCpi => "User-specified custom internal pressure coefficient Cpi",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }
}

/// Ângulo de incidência do vento para análise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindAnglePreset {
    /// Vento a 0° (soprando paralelamente à cumeeira / longitudinalmente ao galpão)
    Angle0Deg,
    /// Vento a 90° (soprando perpendicularmente à cumeeira / transversalmente)
    Angle90Deg,
    /// Envoltória crítica (combinação mais nociva de todas as direções)
    EnvelopeCritical,
}

impl WindAnglePreset {
    pub fn name_pt(&self) -> &'static str {
        match self {
            Self::Angle0Deg => "Vento a 0° (Longitudinal — Paralelo à cumeeira)",
            Self::Angle90Deg => "Vento a 90° (Transversal — Perpendicular à cumeeira)",
            Self::EnvelopeCritical => "Envoltória Crítica (Verificação mais severa combinada)",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Angle0Deg => "Wind at 0° (Longitudinal — Parallel to ridge)",
            Self::Angle90Deg => "Wind at 90° (Transverse — Perpendicular to ridge)",
            Self::EnvelopeCritical => "Critical Envelope (Most severe combined verification)",
        }
    }

    pub fn name_i18n(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.name_pt(),
            Language::EnUs => self.name_en(),
        }
    }
}

/// Unidade de exibição de pressões
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureUnit {
    Pascal,
    KiloPascal,
    KiloNewtonPerSquareMeter,
    KilogramForcePerSquareMeter,
}

impl PressureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Pascal => "Pa",
            Self::KiloPascal => "kPa",
            Self::KiloNewtonPerSquareMeter => "kN/m²",
            Self::KilogramForcePerSquareMeter => "kgf/m²",
        }
    }

    /// Converte um valor em Pa para a unidade desejada
    pub fn from_pa(&self, val_pa: f64) -> f64 {
        match self {
            Self::Pascal => val_pa,
            Self::KiloPascal => val_pa / 1000.0,
            Self::KiloNewtonPerSquareMeter => val_pa / 1000.0,
            Self::KilogramForcePerSquareMeter => val_pa / 9.80665,
        }
    }

    /// Converte da unidade desejada para Pa
    pub fn to_pa(&self, val: f64) -> f64 {
        match self {
            Self::Pascal => val,
            Self::KiloPascal => val * 1000.0,
            Self::KiloNewtonPerSquareMeter => val * 1000.0,
            Self::KilogramForcePerSquareMeter => val * 9.80665,
        }
    }
}
