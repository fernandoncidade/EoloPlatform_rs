// Versão: 2026.11.11.0 | Data: 11 de novembro de 2026
// Version: 2026.11.11.0 | Date: November 11, 2026

#[derive(Debug, Clone)]
pub struct ManualDetails {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct ManualSection {
    pub id: &'static str,
    pub title: &'static str,
    pub paragraphs: &'static [&'static str],
    pub bullets: &'static [&'static str],
    pub details: Option<ManualDetails>,
}

pub fn obter_secoes_manual(lang: &str) -> &'static [ManualSection] {
    match lang {
        "en_US" => SECOES_EN_US,
        _ => SECOES_PT_BR,
    }
}

const SECOES_PT_BR: &[ManualSection] = &[
    ManualSection {
        id: "visao_geral",
        title: "1. Visão Geral, Natureza Acadêmica & Filosofia Local-First",
        paragraphs: &[
            "O Eolo Platform é uma aplicação desktop desenvolvida nativamente em Rust (Edição 2021) com interface gráfica imediata acelerada na GPU via egui/eframe e banco de dados local SQLite, destinada EXCLUSIVAMENTE a fins de estudo acadêmico, pesquisa educacional e experimentação didática.",
            "Utilizando como referência bibliográfica a norma ABNT NBR 6123:2023 (Forças devidas ao vento em edificações), o software implementa rotinas computacionais experimentais para estudo das velocidades características (Vk), pressões dinâmicas (q), coeficientes de pressão externa (Cpe/Ce), coeficientes de pressão interna (Cpi), efeitos em beirais e envoltórias estimadas.",
            "A versão 2026.11.11.0 consolida a adaptação dinâmica em tempo real aos temas Claro e Escuro do Windows 11, a tradução instantânea entre Português (BR) e Inglês (US), a barra de ferramentas responsiva em moldura retangular fina e o modelo de duplo licenciamento (GNU AGPLv3 + Licença Comercial Microsoft Store). ATENÇÃO: esta ferramenta pode conter erros de programação, simplificações ou imprecisões numéricas, NÃO servindo para decisões definitivas em produção ou projetos reais — confira sempre todos os resultados manualmente ou com ferramentas consolidadas de mercado.",
        ],
        bullets: &[
            "Finalidade Exclusivamente Acadêmica e Didática: Destinada ao aprendizado e simulação educacional; não possui homologação oficial nem substitui software comercial consolidado ou cálculo manual.",
            "Privacidade Soberana (100% Local-First e Offline): Todos os dados simulados permanecem exclusivamente na sua estação de trabalho, sem nuvem, sem telemetria e sem dependências externas.",
            "Referências Bibliográficas sem Reprodução de Normas: As normas técnicas da ABNT são protegidas por direitos autorais (todos os direitos reservados); as referências bibliográficas oficiais estão listadas em 'Ajuda > Sobre o Eolo Platform... > Avisos'.",
            "Dupla Distribuição Nativa: Aplicativo gráfico desktop (eolo.exe) e utilitário autônomo de linha de comando (eolo-cli.exe) para simulações acadêmicas.",
        ],
        details: Some(ManualDetails {
            title: "Fluxo de Dimensionamento em 5 Abas",
            items: &[
                "Aba 1 — Localização & V0: Seleção de UF/Estação Meteorológica FAB, velocidade básica V0 e fatores S1, S2 e S3.",
                "Aba 2 — Geometria da Edificação: Vão livre (b), comprimento (a), pé-direito (h), inclinação (θ), beirais e espaçamentos.",
                "Aba 3 — Aberturas & Cpi: Simulação de edificação estanque, permeabilidade uniforme ou abertura dominante (portões/docas).",
                "Aba 4 — Diagrama de Cargas & 3D GPU: Pórtico transversal 2D e wireframe 3D orbital com vetores de sobrepressão e sucção.",
                "Aba 5 — Memorial de Cálculo ABNT: Geração e exportação do laudo completo em HTML5/PDF (A4), Markdown (.md) e JSON.",
            ],
        }),
    },
    ManualSection {
        id: "localizacao_v0_fatores",
        title: "2. Velocidade Básica V0 e Fatores S1, S2 e S3 (Aba 1)",
        paragraphs: &[
            "Na Aba 1, o engenheiro define a implantação geográfica da obra e os parâmetros meteorológicos, topográficos e estatísticos que transformam a velocidade básica V0 (rajada de 3s a 10m de altura em terreno aberto, recorrência de 50 anos) na velocidade característica Vk.",
            "O banco de dados interno contempla as 27 Unidades Federativas brasileiras e as 49 estações meteorológicas oficiais da Força Aérea Brasileira (FAB) constantes na Tabela C.1 e Figura 1 da ABNT NBR 6123:2023.",
        ],
        bullets: &[
            "Equação Fundamental da Velocidade Característica (Item 4.2): Vk = V0 · S1 · S2 · S3 (em m/s).",
            "Equação Fundamental da Pressão Dinâmica (Item 4.2): q = 0,613 · Vk² (em Pa, com Vk em m/s).",
            "Fator Topográfico S1 (Item 5.2): Terreno plano (S1 = 1,00), vales profundos protegidos (S1 = 0,90) ou interpolação analítica contínua para taludes e morros em função da inclinação θt, altura d e cota z.",
            "Fator de Rugosidade e Dimensões S2 (Item 5.3): Calculado por S2 = bm · Fr · (z/10)^p para as Categorias I a V e Classes A (≤ 20m), B (20m a 50m) e C (> 50m), respeitando o patamar constante até 10m na Categoria V e 5m nas demais.",
            "Fator Estatístico S3 (Item 5.4 e Tabela 4): Grupos 1 (S3 = 1,11) a 5 (S3 = 0,83), incluindo a redução normativa de 0,92 · S3 para elementos de vedação (telhas, terças e painéis de fachada).",
        ],
        details: Some(ManualDetails {
            title: "Dica de Engenharia — Seleção de V0",
            items: &[
                "Utilize o filtro de busca na tabela de estações da FAB para localizar rapidamente a cidade ou digite manualmente o valor interpolado das curvas isopléticas (30 m/s a 50 m/s).",
                "O painel lateral direito exibe em tempo real a memória dos parâmetros bm, Fr, expoente p, altura da camada limite zg e rugosidade z0.",
            ],
        }),
    },
    ManualSection {
        id: "geometria_cpe",
        title: "3. Geometria da Edificação e Coeficientes Externos Cpe/Ce (Aba 2)",
        paragraphs: &[
            "Na Aba 2, definem-se as dimensões globais em planta e elevação, a tipologia da cobertura e os espaçamentos estruturais dos pórticos e terças/longarinas.",
            "O motor analítico avalia automaticamente as relações adimensionais h/b (altura/vão) e a/b (comprimento/vão) para interpolar os coeficientes aerodinâmicos das Tabelas 6, 7, 8, 10, 13 e 14 da ABNT NBR 6123:2023.",
        ],
        bullets: &[
            "Tipologias Suportadas: Galpão a Duas Águas Simétrico (Tabela 7), Uma Água (Tabela 8), Arco Curvo / Abóbada (Figuras 7/8), Cobertura Isolada / Marquise / Posto (Tabela 14) e Shed Industrial (Tabela 10).",
            "Paredes Retangulares (Tabela 6): Determinação automática dos coeficientes de barlavento (+0,70 / +0,80), sotavento (-0,30 a -0,50), paredes laterais (-0,80 a -0,90) e zona de alta sucção cpe_médio na faixa min(0,2·b, h).",
            "Coberturas a Duas Águas (Tabela 7): Interpolação contínua para qualquer ângulo θ entre 0° e 60°, contemplando a inversão crítica entre sucção e sobrepressão na água de barlavento para inclinações entre 15° e 30°.",
            "Arrancamento Crítico em Beirais (Nota 1 da Tabela 7): Combina a sobrepressão da parede inferior (+0,70 ou +0,80) na face inferior do beiral com a forte sucção no topo (-0,80 a -1,20), resultando em ΔCp de até -2,00.",
        ],
        details: Some(ManualDetails {
            title: "Sincronização Automática de Classe Dimensional",
            items: &[
                "Quando a opção 'Determinar Classe (A, B ou C) automaticamente' está ativa, o Eolo Platform compara max(a, b, h_cumeeira) contra os limites de 20 m e 50 m e atualiza instantaneamente o fator S2.",
            ],
        }),
    },
    ManualSection {
        id: "aberturas_cpi_envoltorias",
        title: "4. Aberturas Dominantes, Pressão Interna Cpi e Envoltórias (Aba 3)",
        paragraphs: &[
            "Grandes aberturas em galpões industriais e hangares (como portões de doca rompidos ou abertos durante tempestades) alteram drasticamente o campo de pressão interna Cpi, sendo a principal causa de colapso de coberturas metálicas por arrancamento.",
            "A Aba 3 implementa integralmente o Item 6.3 e a Tabela 6.3.2 da ABNT NBR 6123:2023, combinando os casos extremos de Cpi positivo e negativo com as pressões externas para obter a envoltória crítica Δp = (Cpe - Cpi) · q.",
        ],
        bullets: &[
            "Edificação Estanque: Cpi entre -0,20 e 0,00.",
            "Quatro Faces Igualmente Permeáveis: Cpi = -0,30 ou 0,00 (adotando-se o mais nocivo para cada verificação).",
            "Abertura Dominante em Barlavento: Quando um portão frontal fica aberto para o vento, o ar infla o galpão com Cpi até +0,80, somando-se à sucção externa do telhado e amplificando o arrancamento.",
            "Abertura Dominante em Sotavento ou Face Paralela: Gera sucção interna severa (Cpi até -0,90), puxando as paredes e o telhado para dentro.",
            "Cargas Lineares Prontas para Pórtico e Terças: Conversão direta das pressões superficiais (kN/m²) em cargas distribuídas lineares (kN/m) multiplicadas pela largura de influência dos pórticos e das terças.",
        ],
        details: None,
    },
    ManualSection {
        id: "canvas_3d_memorial_cli",
        title: "5. Visualização 2D/3D na GPU, Memorial ABNT e Linha de Comando",
        paragraphs: &[
            "A Aba 4 apresenta o visualizador interativo acelerado por GPU em modo Pórtico Transversal 2D cotado ou Wireframe Tridimensional 3D orbital com vetores de pressão e sucção em escala real.",
            "A Aba 5 consolida o Memorial de Cálculo Estrutural completo com todas as cláusulas da NBR 6123:2023, permitindo filtragem instantânea e exportação formal em HTML5/PDF (página A4 estilizada pronta para impressão), Markdown (.md) e arquivo de projeto (.json).",
        ],
        bullets: &[
            "Controles da Câmera 3D (Aba 4): Arraste com o botão esquerdo para orbitar (Yaw/Pitch), arraste com o botão direito para transladar (Pan) e use a roda do mouse para Zoom.",
            "Adaptação Dinâmica de Tema: O fundo do canvas, a grade do piso, os perfis metálicos e as cotas ajustam seu contraste automaticamente entre o Modo Escuro e o Modo Claro.",
            "Cenários Pré-Configurados (#1 a #5): Carregue com um clique galpões industriais, hangares portuários, postos de combustíveis, ginásios em arco e armazéns graneleiros.",
            "Utilitário Headless (eolo-cli.exe): Execute cálculos rápidos (`eolo-cli calc`), gere memoriais (`eolo-cli report`), consulte isopletas (`eolo-cli isopleth`) e audite normas (`eolo-cli verify`) via terminal.",
        ],
        details: Some(ManualDetails {
            title: "Suporte Oficial e Contato",
            items: &[
                "Para suporte técnico, dúvidas normativas ou relato de incidentes, contate o canal oficial: linceu_lighthouse@outlook.com.",
            ],
        }),
    },
    ManualSection {
        id: "licenciamento_lgpd_responsabilidade",
        title: "6. Duplo Licenciamento, Privacidade (LGPD), Estudo Acadêmico & Isenção de Responsabilidade",
        paragraphs: &[
            "O Eolo Platform é uma obra intelectual de titularidade exclusiva de Fernando Nillsson Cidade (pessoa física), distribuída sob modelo de Duplo Licenciamento: GNU AGPLv3 para o código-fonte público no GitHub e Licença Comercial de Usuário Final (EULA/CLC) para os binários oficiais disponibilizados na Microsoft Store.",
            "AVISO DE NATUREZA ACADÊMICA, RISCO DE ERROS E USO NÃO PRODUTIVO: Esta aplicação foi desenvolvida EXCLUSIVAMENTE PARA FINS DE ESTUDOS ACADÊMICOS E EDUCACIONAIS. Ela pode conter erros de implementação, imprecisões numéricas, simplificações ou falhas de software, e NÃO serve como ferramenta definitiva para tomadas de decisão em ambientes de trabalho, projetos executivos, obras civis ou produção comercial. É obrigatório conferir todos os resultados manualmente (com base em exemplar oficial da norma) ou por meio de ferramentas mais consolidadas no mercado.",
            "Nos termos do Código Civil Brasileiro (arts. 186, 393, 421, 422, 927 e 945), do CDC (arts. 6º, III, 12 §3º III, 14 §3º II e 31, preservando-se os direitos irrenunciáveis dos arts. 24, 25, 49 e 51, I e da Lei nº 9.609/1998, arts. 7º e 8º) e das convenções internacionais (fornecimento 'as-is' e 'with all faults' sem garantia de adequação para produção), o autor fica isento de responsabilidade por quaisquer danos decorrentes do uso real e definitivo em produção ou fora do escopo estritamente acadêmico declarado. Em conformidade com a LGPD (Lei nº 13.709/2018) e Política 10.5.1 da Microsoft Store, o app opera 100% offline e sem telemetria própria.",
        ],
        bullets: &[
            "Aplicação Apenas para Estudo Acadêmico: Não utilizar resultados diretamente em obras, licitações, laudos profissionais ou ambientes produtivos.",
            "Obrigatoriedade de Conferência Manual e Ferramentas Consolidadas: Todo cálculo deve ser auditado manualmente e validado em softwares consagrados e por Engenheiro habilitado (CREA/CONFEA, Leis nº 5.194/1966 e nº 6.496/1977).",
            "Direitos Autorais das Normas ABNT (Lei nº 9.610/1998 e Convenção de Berna): Nenhuma norma ABNT é reproduzida no aplicativo; as referências bibliográficas oficiais constam em 'Ajuda > Sobre o Eolo Platform... > Avisos'.",
            "Prazo de Validade Técnica e Garantias Legais (Lei nº 9.609/1998, arts. 7º e 8º; CDC): Mantidos 12 meses a contar de 8 de novembro de 2026 com suporte por e-mail (linceu_lighthouse@outlook.com) e ressalva expressa dos direitos irrenunciáveis do consumidor.",
        ],
        details: Some(ManualDetails {
            title: "Documentação Legal e Referências Bibliográficas Integradas",
            items: &[
                "Consulte as referências bibliográficas das normas ABNT utilizadas na aba 'Avisos' e os textos integrais de EULA, CLC, Privacidade, Copyright/Marca ('Eolo Platform'™) e Licenças na janela 'Ajuda > Sobre o Eolo Platform...'.",
            ],
        }),
    },
];

const SECOES_EN_US: &[ManualSection] = &[
    ManualSection {
        id: "overview",
        title: "1. Overview, Academic Purpose & Local-First Philosophy",
        paragraphs: &[
            "Eolo Platform is a desktop application built natively in Rust (2021 Edition) with a GPU-accelerated immediate-mode graphical interface powered by egui/eframe and local SQLite persistence, intended EXCLUSIVELY for academic study, educational learning, and didactic simulation.",
            "Using the Brazilian wind standard ABNT NBR 6123:2023 (Wind forces on buildings) as a bibliographic reference, the software implements experimental computational routines to study characteristic wind speeds (Vk), dynamic pressures (q), external pressure coefficients (Cpe/Ce), internal pressure coefficients (Cpi), eave uplift effects, and estimated load envelopes.",
            "Version 2026.11.11.0 consolidates live runtime adaptation to Windows 11 Light and Dark themes, instant runtime translation between Portuguese (BR) and English (US), a responsive toolbar housed inside a thin rectangular frame, and the dual-licensing model (GNU AGPLv3 + Microsoft Store Commercial License). WARNING: this software may contain programming bugs, simplifications, or numerical inaccuracies, and is NOT suitable for definitive decision-making in production or workplace engineering — always verify all results manually or using established industry software.",
        ],
        bullets: &[
            "Strictly Academic & Educational Purpose: Designed for study and educational exploration; it holds no regulatory certification and does not replace established commercial engineering software or manual calculations.",
            "Sovereign Privacy (100% Local-First & Offline): All study data stays strictly on your workstation, with zero cloud dependency and zero telemetry.",
            "Bibliographic References Without Standard Reproduction: ABNT standards are protected by copyright (all rights reserved); official bibliographic references are provided in 'Help > About Eolo Platform... > Notices'.",
            "Dual Native Distribution: Desktop GUI executable (eolo.exe) and standalone headless CLI utility (eolo-cli.exe) for academic study simulations.",
        ],
        details: Some(ManualDetails {
            title: "5-Tab Structural Engineering Workflow",
            items: &[
                "Tab 1 — Location & V0: Select State/FAB Meteorological Station, basic wind speed V0, and factors S1, S2, and S3.",
                "Tab 2 — Building Geometry: Clear span (b), length (a), eave height (h), roof pitch (θ), overhangs, and structural spacings.",
                "Tab 3 — Openings & Cpi: Simulate impermeable enclosures, uniform permeability, or dominant openings (dock doors/hangars).",
                "Tab 4 — Load Diagrams & 3D GPU: Dimensioned 2D portal cross-section and interactive 3D orbital wireframe with pressure/suction vectors.",
                "Tab 5 — ABNT Calculation Report: Generate and export the formal calculation report in HTML5/PDF (A4), Markdown (.md), and JSON.",
            ],
        }),
    },
    ManualSection {
        id: "location_v0_factors",
        title: "2. Basic Wind Speed V0 and Factors S1, S2, S3 (Tab 1)",
        paragraphs: &[
            "In Tab 1, the engineer specifies the geographic site of the structure along with the meteorological, topographic, and statistical factors that transform the basic wind speed V0 (3-second gust at 10m height over open terrain, 50-year return period) into the characteristic wind speed Vk.",
            "The built-in database covers all 27 Brazilian states and the 49 official Brazilian Air Force (FAB) meteorological stations listed in Table C.1 and Figure 1 of ABNT NBR 6123:2023.",
        ],
        bullets: &[
            "Characteristic Wind Speed Equation (Clause 4.2): Vk = V0 · S1 · S2 · S3 (in m/s).",
            "Dynamic Wind Pressure Equation (Clause 4.2): q = 0.613 · Vk² (in Pa, with Vk in m/s).",
            "Topographic Factor S1 (Clause 5.2): Flat terrain (S1 = 1.00), sheltered deep valleys (S1 = 0.90), or continuous analytical interpolation for slopes and hills as a function of slope angle θt, height d, and elevation z.",
            "Roughness & Building Size Factor S2 (Clause 5.3): Computed via S2 = bm · Fr · (z/10)^p across Categories I to V and Classes A (≤ 20m), B (20m to 50m), and C (> 50m), enforcing constant S2 up to 10m in Category V and 5m in others.",
            "Statistical Factor S3 (Clause 5.4 & Table 4): Occupancy Groups 1 (S3 = 1.11) through 5 (S3 = 0.83), including the 0.92 · S3 code reduction for cladding and secondary elements.",
        ],
        details: Some(ManualDetails {
            title: "Engineering Tip — Selecting V0",
            items: &[
                "Use the search box in the FAB station table to quickly find a city or manually enter an interpolated isopleth value (30 m/s to 50 m/s).",
                "The right-hand summary panel displays live intermediate parameters bm, Fr, exponent p, gradient height zg, and roughness length z0.",
            ],
        }),
    },
    ManualSection {
        id: "geometry_cpe",
        title: "3. Building Geometry & External Coefficients Cpe/Ce (Tab 2)",
        paragraphs: &[
            "In Tab 2, you define the plan and elevation dimensions, roof typology, and structural spacings for portal frames and purlins/girts.",
            "The analytical engine automatically evaluates the dimensionless ratios h/b (height/span) and a/b (length/span) to interpolate aerodynamic coefficients from Tables 6, 7, 8, 10, 13, and 14 of ABNT NBR 6123:2023.",
        ],
        bullets: &[
            "Supported Typologies: Symmetric Duo-Pitch Warehouse (Table 7), Mono-Pitch Roof (Table 8), Curved Arch / Vault (Figures 7/8), Isolated Canopy / Gas Station (Table 14), and Industrial Shed Roof (Table 10).",
            "Rectangular Walls (Table 6): Automatic calculation of windward (+0.70 / +0.80), leeward (-0.30 to -0.50), side wall (-0.80 to -0.90), and high corner suction zone cpe_mean over min(0.2·b, h).",
            "Duo-Pitch Roofs (Table 7): Continuous interpolation for any pitch angle θ from 0° to 60°, capturing the dual suction/overpressure cases on the windward slope between 15° and 30°.",
            "Critical Eave Overhang Uplift (Table 7 Note 1): Combines windward wall overpressure (+0.70 or +0.80) on the underside of the overhang with upper roof suction (-0.80 to -1.20), producing net ΔCp up to -2.00.",
        ],
        details: Some(ManualDetails {
            title: "Automatic Dimensional Class Synchronization",
            items: &[
                "When 'Automatically determine Class (A, B, or C)' is enabled, Eolo Platform compares max(a, b, h_ridge) against the 20 m and 50 m thresholds and updates factor S2 in real time.",
            ],
        }),
    },
    ManualSection {
        id: "openings_cpi_envelopes",
        title: "4. Dominant Openings, Internal Pressure Cpi & Envelopes (Tab 3)",
        paragraphs: &[
            "Large openings in industrial warehouses and hangars (such as loading dock doors left open or breached during windstorms) dramatically alter internal pressure Cpi and are the leading cause of steel roof uplift failures.",
            "Tab 3 implements Clause 6.3 and Table 6.3.2 of ABNT NBR 6123:2023, combining extreme positive and negative Cpi states with external pressures to compute the governing net pressure envelope Δp = (Cpe - Cpi) · q.",
        ],
        bullets: &[
            "Impermeable Building: Cpi between -0.20 and 0.00.",
            "Four Equally Permeable Walls: Cpi = -0.30 or 0.00 (taking the most unfavorable value for each zone).",
            "Dominant Windward Opening: When a windward gate is open, wind pressurizes the interior up to Cpi = +0.80, adding directly to external roof suction and maximizing roof uplift.",
            "Dominant Leeward or Side Opening: Produces severe internal suction (Cpi down to -0.90), pulling walls and roof slopes inward.",
            "Ready-to-Use Linear Frame & Purlin Loads: Directly converts surface pressures (kN/m²) into distributed linear loads (kN/m) using frame and purlin tributary spacings.",
        ],
        details: None,
    },
    ManualSection {
        id: "canvas_3d_report_cli",
        title: "5. 2D/3D GPU Visualization, ABNT Report & Command Line",
        paragraphs: &[
            "Tab 4 features the GPU-accelerated interactive viewer with both a dimensioned 2D Portal Cross-Section mode and an orbital 3D Wireframe mode displaying real-time overpressure and suction vectors.",
            "Tab 5 generates the comprehensive Structural Calculation Report citing every applicable NBR 6123:2023 clause, with instant section filtering and export to HTML5/PDF (print-ready A4 layout), Markdown (.md), and JSON project files.",
        ],
        bullets: &[
            "3D Camera Controls (Tab 4): Left-click and drag to orbit (Yaw/Pitch), right-click and drag to pan, and scroll the mouse wheel to zoom.",
            "Dynamic Theme Adaptation: Canvas background, ground grid, steel members, and dimension labels automatically adjust contrast between Dark and Light themes.",
            "Preset Engineering Scenarios (#1 to #5): One-click loading for industrial warehouses, port hangars, gas station canopies, arched sports arenas, and agricultural grain silos.",
            "Headless CLI Utility (eolo-cli.exe): Run instant calculations (`eolo-cli calc`), export reports (`eolo-cli report`), query isopleths (`eolo-cli isopleth`), and verify compliance (`eolo-cli verify`) from the terminal.",
        ],
        details: Some(ManualDetails {
            title: "Official Technical Support",
            items: &[
                "For technical support, normative inquiries, or bug reports, contact our official support channel: linceu_lighthouse@outlook.com.",
            ],
        }),
    },
    ManualSection {
        id: "licensing_privacy_responsibility",
        title: "6. Dual Licensing, Privacy (LGPD), Academic Study & Liability Exemption",
        paragraphs: &[
            "Eolo Platform is an intellectual work wholly owned by Fernando Nillsson Cidade (natural person), distributed under a Dual-Licensing model: GNU AGPLv3 for the public source code on GitHub and a Commercial End-User License Agreement (EULA/CLC) for official binaries distributed via the Microsoft Store.",
            "ACADEMIC STUDY PURPOSE, ERROR RISK & NON-PRODUCTION NOTICE: This application was developed EXCLUSIVELY FOR ACADEMIC STUDY AND EDUCATIONAL RESEARCH. It may contain programming bugs, numerical inaccuracies, simplifications, or discrepancies, and DOES NOT serve as definitive software for decision-making in production environments, construction, permitting, or workplace projects. All results MUST be manually verified (against officially acquired standards) and/or cross-checked with consolidated, industry-validated commercial tools.",
            "Pursuant to the Brazilian Civil Code (Arts. 186, 393, 421, 422, 927, and 945), the Brazilian Consumer Defense Code (CDC Arts. 6(III), 12(§3)(III), 14(§3)(II), and 31, preserving non-waivable consumer rights under Arts. 24, 25, 49, and 51(I) and Law No. 9,609/1998 Arts. 7 & 8), and international conventions ('AS IS' and 'WITH ALL FAULTS' disclaimer without warranty of fitness for production), the author is exempt from legal liability arising from real-world production use outside its declared academic study scope. In compliance with LGPD (Law No. 13,709/2018) and Microsoft Store Policy 10.5.1, the app runs 100% offline with zero telemetry.",
        ],
        bullets: &[
            "Academic Study Tool Only: Do not rely on software outputs directly for real construction, workplace decisions, commercial permitting, or definitive engineering design.",
            "Mandatory Manual & Consolidated Software Verification: Every output must be independently checked by hand and validated with established market tools and a licensed engineer (CREA/CONFEA Laws No. 5,194/1966 & No. 6,496/1977).",
            "ABNT Standards Copyright (Law No. 9,610/1998 & Berne Convention): No ABNT standard text is reproduced in the software; official bibliographic references are provided in 'Help > About Eolo Platform... > Notices'.",
            "Technical Validity & Statutory Rights (Law No. 9,609/1998, Arts. 7 & 8; CDC): 12-month technical validity from November 8, 2026 with best-effort e-mail support (linceu_lighthouse@outlook.com) and full saving clause for non-waivable consumer protections.",
        ],
        details: Some(ManualDetails {
            title: "Built-In Legal Documentation & Bibliographic References",
            items: &[
                "Review the bibliographic references of technical standards in the 'Notices' tab and the full texts of EULA, CLC, Privacy Policy, Copyright/Trademark Notice ('Eolo Platform'™), and Licenses inside 'Help > About Eolo Platform...'.",
            ],
        }),
    },
];
