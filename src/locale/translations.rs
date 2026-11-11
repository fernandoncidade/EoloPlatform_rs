use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    PtBr,
    EnUs,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PtBr => "pt_BR",
            Self::EnUs => "en_US",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::PtBr => "🇧🇷 Português (Brasil)",
            Self::EnUs => "🇺🇸 English (US)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Localizer {
    pub current_lang: Language,
}

impl Default for Localizer {
    fn default() -> Self {
        Self {
            current_lang: Language::PtBr,
        }
    }
}

impl Localizer {
    pub fn new(lang: Language) -> Self {
        Self { current_lang: lang }
    }

    pub fn is_pt(&self) -> bool {
        self.current_lang == Language::PtBr
    }

    pub fn codigo_idioma(&self) -> &'static str {
        self.current_lang.code()
    }

    /// Retorna dinamicamente em tempo de execução a string em Português (BR) ou Inglês (US)
    #[inline]
    pub fn tr<'a>(&self, pt: &'a str, en: &'a str) -> &'a str {
        match self.current_lang {
            Language::PtBr => pt,
            Language::EnUs => en,
        }
    }

    pub fn t(&self, key: &str) -> &'static str {
        match (self.current_lang, key) {
            // Títulos das 5 Abas Principais
            (Language::PtBr, "tab1_title") => "1. Localização & V0 (NBR 6123)",
            (Language::EnUs, "tab1_title") => "1. Location & V0 (NBR 6123)",
            (Language::PtBr, "tab2_title") => "2. Geometria da Edificação",
            (Language::EnUs, "tab2_title") => "2. Building Geometry",
            (Language::PtBr, "tab3_title") => "3. Aberturas & Cpi",
            (Language::EnUs, "tab3_title") => "3. Openings & Cpi",
            (Language::PtBr, "tab4_title") => "4. Diagrama de Cargas & 3D GPU",
            (Language::EnUs, "tab4_title") => "4. Load Diagrams & 3D GPU",
            (Language::PtBr, "tab5_title") => "5. Memorial de Cálculo ABNT",
            (Language::EnUs, "tab5_title") => "5. ABNT Calculation Report",

            // Barra de Menus
            (Language::PtBr, "menu_file") => "Arquivo",
            (Language::EnUs, "menu_file") => "File",
            (Language::PtBr, "menu_scenarios") => "Cenários",
            (Language::EnUs, "menu_scenarios") => "Scenarios",
            (Language::PtBr, "menu_tools") => "Ferramentas",
            (Language::EnUs, "menu_tools") => "Tools",
            (Language::PtBr, "menu_settings") => "Configurações",
            (Language::EnUs, "menu_settings") => "Settings",
            (Language::PtBr, "menu_help") => "Ajuda",
            (Language::EnUs, "menu_help") => "Help",

            // Ações Rápidas
            (Language::PtBr, "btn_new") => "Novo Projeto",
            (Language::EnUs, "btn_new") => "New Project",
            (Language::PtBr, "btn_open") => "Abrir...",
            (Language::EnUs, "btn_open") => "Open...",
            (Language::PtBr, "btn_save") => "Salvar",
            (Language::EnUs, "btn_save") => "Save",
            (Language::PtBr, "btn_export") => "Exportar Memorial",
            (Language::EnUs, "btn_export") => "Export Report",
            (Language::PtBr, "close") => "Fechar",
            (Language::EnUs, "close") => "Close",

            // Status e Grandezas
            (Language::PtBr, "stat_vk") => "Velocidade Característica Vk:",
            (Language::EnUs, "stat_vk") => "Characteristic Wind Speed Vk:",
            (Language::PtBr, "stat_q") => "Pressão Dinâmica q:",
            (Language::EnUs, "stat_q") => "Dynamic Pressure q:",
            (Language::PtBr, "stat_uplift") => "Arrancamento Máx. Cobertura:",
            (Language::EnUs, "stat_uplift") => "Max. Roof Uplift:",

            // Janela Sobre (AboutDialog)
            (Language::PtBr, "about_window_title") => "Sobre o Eolo Platform — Estudo Acadêmico (Ref. ABNT NBR 6123:2023)",
            (Language::EnUs, "about_window_title") => "About Eolo Platform — Academic Study (Ref. ABNT NBR 6123:2023)",
            (Language::PtBr, "about_version_label") => "Versão:",
            (Language::EnUs, "about_version_label") => "Version:",
            (Language::PtBr, "about_authors") => "Autor: Fernando Nillsson Cidade",
            (Language::EnUs, "about_authors") => "Author: Fernando Nillsson Cidade",
            (Language::PtBr, "about_description") => "Ferramenta Computacional para Estudo Acadêmico e Simulação Didática de Ações de Vento em Edificações (Referência: ABNT NBR 6123:2023)",
            (Language::EnUs, "about_description") => "Computational Tool for Academic Study & Educational Simulation of Wind Actions on Buildings (Reference: ABNT NBR 6123:2023)",
            (Language::PtBr, "search_placeholder") => "Pesquisar no documento...",
            (Language::EnUs, "search_placeholder") => "Search in document...",
            (Language::PtBr, "about_tab_about") => "Sobre",
            (Language::EnUs, "about_tab_about") => "About",
            (Language::PtBr, "about_tab_history") => "Histórico",
            (Language::EnUs, "about_tab_history") => "History",
            (Language::PtBr, "about_tab_clc") => "CLC",
            (Language::EnUs, "about_tab_clc") => "CLC",
            (Language::PtBr, "about_tab_copyright") => "Copyright",
            (Language::EnUs, "about_tab_copyright") => "Copyright",
            (Language::PtBr, "about_tab_eula") => "EULA",
            (Language::EnUs, "about_tab_eula") => "EULA",
            (Language::PtBr, "about_tab_licenses") => "Licenças",
            (Language::EnUs, "about_tab_licenses") => "Licenses",
            (Language::PtBr, "about_tab_notices") => "Avisos",
            (Language::EnUs, "about_tab_notices") => "Notices",
            (Language::PtBr, "about_tab_privacy") => "Privacidade",
            (Language::EnUs, "about_tab_privacy") => "Privacy",
            (Language::PtBr, "about_tab_release") => "Notas de Versão",
            (Language::EnUs, "about_tab_release") => "Release Notes",
            (Language::PtBr, "about_licenses_intro") => "O Eolo Platform utiliza componentes open-source de alta confiabilidade do ecossistema Rust. Abaixo estão listados os websites oficiais e os textos integrais das licenças:",
            (Language::EnUs, "about_licenses_intro") => "Eolo Platform uses high-reliability open-source components from the Rust ecosystem. Below are the official websites and full license texts:",
            (Language::PtBr, "about_official_websites") => "Websites Oficiais dos Componentes e Normas",
            (Language::EnUs, "about_official_websites") => "Official Component & Standard Websites",
            (Language::PtBr, "about_full_licenses") => "Textos Integrais das Licenças",
            (Language::EnUs, "about_full_licenses") => "Full License Texts",

            // Janela Manual (ManualDialog)
            (Language::PtBr, "manual_window_title") => "Manual de Estudo e Operação — Eolo Platform (Ref. NBR 6123:2023)",
            (Language::EnUs, "manual_window_title") => "Study & Operations Manual — Eolo Platform (Ref. NBR 6123:2023)",
            (Language::PtBr, "manual_header_title") => "Manual Interativo de Estudo Acadêmico — Eolo Platform",
            (Language::EnUs, "manual_header_title") => "Interactive Academic Study Manual — Eolo Platform",
            (Language::PtBr, "manual_search_placeholder") => "Pesquisar tópicos, cláusulas da NBR 6123 ou equações...",
            (Language::EnUs, "manual_search_placeholder") => "Search topics, NBR 6123 clauses, or equations...",
            (Language::PtBr, "manual_sections_title") => "Tópicos do Manual",
            (Language::EnUs, "manual_sections_title") => "Manual Topics",
            (Language::PtBr, "manual_key_points") => "Pontos-Chave e Equações Normativas",
            (Language::EnUs, "manual_key_points") => "Key Points & Code Equations",
            (Language::PtBr, "manual_details") => "Detalhamento Técnico e Operacional",
            (Language::EnUs, "manual_details") => "Technical & Operational Details",

            _ => "---",
        }
    }
}
