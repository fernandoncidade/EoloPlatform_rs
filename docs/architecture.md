# Arquitetura do Sistema / System Architecture — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher a arquitetura do sistema em português</b></summary><br>

# Eolo Platform — Arquitetura de Software e Especificações Técnicas

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Padrão Tecnológico:** Rust (Edição 2021) | `egui 0.31` / `eframe 0.31` / `wgpu` | SQLite3 Embutido | `serde` | `image 0.25`  
**Plataforma Alvo:** Windows 11 / Windows 10 / Windows Server 2022+ (x64)  
**Binários Produzidos (`builds/eolo/release/`):**  
- `eolo.exe` — Interface Gráfica Desktop com Renderização Acelerada por Hardware  
- `eolo-cli.exe` — Utilitário Autônomo de Linha de Comando para Automação e Auditoria  
**Autor e Arquiteto:** Fernando Nillsson Cidade  
**Licenciamento:** Duplo Licenciamento (GNU AGPLv3 para código-fonte no GitHub / EULA Comercial para Microsoft Store)  

---

### 1. Visão Geral e Filosofia Arquitetural

O **Eolo Platform** é uma plataforma de engenharia civil de alta precisão desenvolvida em **Rust (Edição 2021)**, voltada para a modelagem computacional determinística das ações do vento em edificações e estruturas civis conforme a norma **ABNT NBR 6123:2023** e normas correlatas (**ABNT NBR 8800:2024** e **ABNT NBR 6118:2023**).

A arquitetura do sistema é governada pelos seguintes princípios fundamentais:
1. **Privacidade Soberana e Local-First:** Processamento 100% offline, zero telemetria, zero conexão de rede e armazenamento estritamente local;
2. **Determinismo Analítico Estrito:** Ausência total de modelos de linguagem (LLMs) ou inteligência artificial em nuvem na esteira de cálculo; todos os resultados derivam de formulações analíticas matemáticas exatas;
3. **Alto Desempenho e Baixo Consumo de Recursos:** Interface em modo imediato (*immediate-mode GUI*) acelerada por GPU, consumo de memória RAM típico inferior a 40 MB e tempos de cálculo da ordem de microssegundos;
4. **Bilinguismo Nativo Rigoroso:** Suporte completo com paridade estrutural 1:1 entre Português (BR) e Inglês (US).

---

### 2. Diagrama Geral das Camadas do Sistema

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        CAMADA DE APRESENTAÇÃO                          │
│               (egui 0.31 / eframe 0.31 / wgpu + CLI Nativo)            │
│                                                                        │
│  - Barra de Menus (Arquivo, Editar, Exibir, Ferramentas, Opções)       │
│  - Barra de Ferramentas de Acesso Rápido e Seletores de Unidades/Idioma│
│  - 5 Abas de Engenharia: Localização, Geometria, Aberturas, 3D, Relat. │
│  - Diálogos Modeless (Sobre/Avisos, Rastreabilidade, Manual, Unidades)  │
│  - Utilitário Headless de Console (`eolo-cli.exe`)                     │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                      CAMADA DE RELATÓRIOS E I/O                        │
│                   (src/reports/ & src/core/project.rs)                 │
│                                                                        │
│  - Gerador de Memorial Descritivo Estruturado (HTML, Markdown, JSON)   │
│  - Serialização e Desserialização de Projetos (.json) via `serde`      │
│  - Registro Automático de Termo de Responsabilidade Técnica (ART)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     NÚCLEO DE CÁLCULO ESTRUTURAL                       │
│                           (src/engine/)                                │
│                                                                        │
│  - isopleth_map.rs: Base de Isopletas de V0, Capitais e Estações FAB   │
│  - factors_s1_s2_s3.rs: Cálculo Analítico Contínuo de S1, S2, S3 e Vk  │
│  - cpe_cpi_tables.rs: Tabelas Normativas de Cpe (Paredes/Telhados), Cpi│
│  - envelopes.rs: Envoltórias de Pressão Líquida, Força Global e Arrasto│
│  - math3d.rs: Projeções Tridimensionais e Vetores de Pressão no Canvas │
│  - wind_core.rs: Orquestrador da Pipeline Completa de Cálculo NBR 6123 │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     CAMADA DE DADOS E PERSISTÊNCIA                     │
│                        (src/data/ & src/locale/)                       │
│                                                                        │
│  - database.rs: SQLite Embutido (Somente Leitura) para Dados de Vento  │
│  - translations.rs: Dicionário Bilíngue Estático Completo (PT/EN)      │
│  - manual.rs: Manual Interativo Bilíngue de Estudo Acadêmico (PT/EN)   │
└────────────────────────────────────────────────────────────────────────┘
```

---

### 3. Descrição dos Módulos Funcionais

#### 3.1. Núcleo de Engenharia e Cálculo Analítico (`src/engine/`)
- `wind_core.rs`: Atua como a fachada central do motor de cálculo. Recebe o estado do projeto (`EoloProject`) e executa sequencialmente o cálculo da velocidade característica $V_k$, pressão de estagnação dinâmica $q$, coeficientes de pressão externa ($C_{pe}$) e interna ($C_{pi}$), pressões líquidas combinadas e forças globais de arrasto e tombamento.
- `factors_s1_s2_s3.rs`: Implementa a modelagem dos fatores meteorológicos e de terreno:
  - Fator topográfico $S_1$ para terreno plano ($1,00$), taludes e morros em função do desnível e cota;
  - Fator $S_2$ de rugosidade e dimensões conforme as 5 categorias e 3 classes da NBR 6123:2023, avaliado em qualquer cota $z$;
  - Fator estatístico $S_3$ parametrizado pelos 5 grupos de ocupação e risco.
- `cpe_cpi_tables.rs`: Contém as matrizes de coeficientes de forma aerodinâmica para paredes retangulares sob ventos a 0° e 90°, coberturas de uma água, duas águas com ângulo de inclinação contínuo $\alpha$, coberturas curvas, sheds e coberturas isoladas, além das formulações de permeabilidade para determinação de $C_{pi}$.
- `envelopes.rs`: Consolida a matriz de combinações de carregamento (sobrepressão máxima, sucção máxima e efeito combinado de vento lateral), computando forças cortantes de base ($F_x, F_y$) e momentos de tombamento ($M_{x}, M_{y}$).
- `isopleth_map.rs`: Fornece a base de dados de isopletas de velocidade básica do vento $V_0$ cobrindo todas as unidades federativas e capitais brasileiras, além de estações meteorológicas oficiais da Força Aérea Brasileira (FAB).
- `math3d.rs`: Motor matemático leve de álgebra linear para rotações de perspectiva, projeções axonométricas e renderização 3D das fachadas e vetores de carga no canvas do `egui`.

#### 3.2. Modelo de Dados e Domínio (`src/core/`)
- `project.rs`: Define a estrutura unificada `EoloProject`, encapsulando geometria da edificação, localização geográfica, parâmetros de terreno, coeficientes de abertura e metadados de autoria.
- `scenarios.rs`: Fornece 5 cenários pré-configurados representativos da prática da engenharia civil brasileira:
  1. Galpão Industrial Padrão (20m x 40m, h=7,5m, duas águas 15°);
  2. Edifício Residencial Urbano de Múltiplos Pavimentos (15m x 25m, h=30m);
  3. Cobertura Isolada / Shed Industrial (18m x 36m, h=6m);
  4. Silo Graneleiro / Estrutura Cilíndrica (diâmetro 12m, h=24m);
  5. Residência Unifamiliar Litorânea (10m x 14m, h=4,5m).
- `types.rs`: Enumerações fortemente tipadas (`TerrainRoughness`, `BuildingClass`, `TopographyType`, `RoofType`, etc.) que impedem a atribuição de estados inválidos em tempo de compilação.

#### 3.3. Interface Gráfica Desktop (`src/ui/`)
- `app.rs`: Estrutura principal `EoloApp` que gerencia o loop de atualização gráfica do `eframe`, despachando eventos e desenhando o layout dividido em abas de navegação.
- `tabs/`:
  - `tab1_location.rs`: Painel de definição de $V_0$, seleção por Estado/Cidade e parâmetros de $S_1, S_2, S_3$.
  - `tab2_geometry.rs`: Dimensionamento geométrico tridimensional do edifício e configuração do tipo e inclinação da cobertura.
  - `tab3_openings.rs`: Especificação de permeabilidade das fachadas e aberturas dominantes para cálculo de $C_{pi}$.
  - `tab4_canvas.rs`: Visualizador 3D interativo com renderização vetorial imediata das fachadas, telhados e setas orientadas de pressão dinâmica.
  - `tab5_memorial.rs`: Visualizador do memorial de cálculo formal com equações analíticas formatadas e botões de exportação.
- `dialogs/`: Diálogos modeless de rastreabilidade acadêmica (`audit_dialog.rs`), calculadora rápida (`quick_calc_dialog.rs`), conversor de unidades (`units_dialog.rs`), manual interativo (`manual_dialog.rs`), configurações (`settings_dialog.rs`) e diálogo informativo sobre autoria, referências bibliográficas na aba "Avisos", licenças, isenção de responsabilidade acadêmica e privacidade LGPD (`about_dialog.rs`).

#### 3.4. Motor de Relatórios e Exportação (`src/reports/`)
- `memorial_generator.rs`: Gera relatórios técnicos determinísticos completos em três formatos:
  - **HTML:** Memorial completo com formatação visual moderna, tabelas zebradas, caixas de destaque de fórmulas e cabeçalho de engenharia;
  - **Markdown:** Formato aberto e leve para arquivamento ou versionamento em repositórios Git;
  - **JSON:** Exportação estruturada de todos os dados de entrada e resultados numéricos calculados para interoperabilidade com softwares de dimensionamento estrutural de concreto e aço.

#### 3.5. Utilitário de Linha de Comando (`src/bin/main_cli.rs`)
- Utilitário autônomo compilado como `eolo-cli.exe`, permitindo execução em servidores, esteiras CI/CD ou integração com scripts Python/Bash sem necessidade de iniciar o subsistema de vídeo:
  - `calc`: Executa o cálculo expedito de $V_k$ e $q$;
  - `report`: Gera memoriais formais a partir de arquivos de projeto ou cenários pré-configurados;
  - `isopleth`: Consulta cidades e estações meteorológicas por UF;
  - `scenarios`: Lista os cenários padronizados;
  - `verify`: Realiza auditoria normativa automática validando o projeto frente à NBR 6123:2023;
  - `test`: Executa a bateria de testes de validação determinística.

---

### 4. Gestão de Recursos e Performance

- **Zero Garbage Collector:** A ausência de coletor de lixo em Rust garante tempo de resposta determinístico sem pausas (*frametime* consistente a 60 FPS na interface gráfica);
- **Consumo de Memória Reduzido:** O consumo de memória RAM do executável varia entre 25 MB e 40 MB em operação plena;
- **Carregamento Instantâneo:** Inicialização da aplicação em menos de 0,2 segundos em unidades SSD convencionais.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the system architecture in English</b></summary><br>

# Eolo Platform — Software Architecture & Technical Specifications

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Technology Stack:** Rust (Edition 2021) | `egui 0.31` / `eframe 0.31` / `wgpu` | Embedded SQLite3 | `serde` | `image 0.25`  
**Target Operating System:** Windows 11 / Windows 10 / Windows Server 2022+ (x64)  
**Output Binaries (`builds/eolo/release/`):**  
- `eolo.exe` — Desktop Graphical User Interface with Hardware-Accelerated Rendering  
- `eolo-cli.exe` — Headless Command-Line Interface Utility for Automation and Verification  
**Author & Lead Architect:** Fernando Nillsson Cidade  
**Licensing:** Dual Licensing (GNU AGPLv3 for source repository on GitHub / Commercial EULA for Microsoft Store)  

---

### 1. Overview & Architectural Philosophy

**Eolo Platform** is a high-precision civil engineering software suite written in **Rust (Edition 2021)**, dedicated to the deterministic computational modeling of aerodynamic wind loads on civil structures and buildings in strict compliance with **ABNT NBR 6123:2023** and related Brazilian structural standards (**ABNT NBR 8800:2024** and **ABNT NBR 6118:2023**).

The system architecture is governed by four core design pillars:
1. **Sovereign Privacy and Local-First Execution:** 100% offline execution, zero telemetry, zero external network sockets, and strictly local file persistence;
2. **Strict Analytical Determinism:** Zero dependence on cloud AI services or Large Language Models (LLMs) in the computational pipeline; all results are derived from closed-form mathematical equations;
3. **High Performance & Minimal Footprint:** Immediate-mode GPU-accelerated UI, typical RAM footprint below 40 MB, and calculation response times measured in microseconds;
4. **Strict Native Bilingualism:** Comprehensive 1:1 structural and semantic parity between Brazilian Portuguese (`pt_BR`) and American English (`en_US`).

---

### 2. General System Layer Diagram

```text
┌────────────────────────────────────────────────────────────────────────┐
│                          PRESENTATION LAYER                            │
│               (egui 0.31 / eframe 0.31 / wgpu + Native CLI)            │
│                                                                        │
│  - Persistent Menu Bar (File, Edit, View, Tools, Options)              │
│  - Quick-Access Toolbar & Unit / Language Switchers                    │
│  - 5 Engineering Tabs: Location, Geometry, Openings, 3D Canvas, Report │
│  - Modeless Dialogs (About/Notices, Traceability, Manual, Units)       │
│  - Headless Console CLI Utility (`eolo-cli.exe`)                       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                       REPORTS & DATA I/O LAYER                         │
│                   (src/reports/ & src/core/project.rs)                 │
│                                                                        │
│  - Structured Calculation Memorial Generator (HTML, Markdown, JSON)    │
│  - Safe Project Serialization & Deserialization (.json) via `serde`    │
│  - Automatic Engineering Responsibility Notice (CREA / ART) Insertion  │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                      STRUCTURAL ENGINE CORE                            │
│                           (src/engine/)                                │
│                                                                        │
│  - isopleth_map.rs: V0 Isopleth Database, Capitals & FAB Air Stations  │
│  - factors_s1_s2_s3.rs: Continuous Analytical S1, S2, S3 & Vk Pipeline │
│  - cpe_cpi_tables.rs: Normative Cpe (Walls / Roofs) & Cpi Tables       │
│  - envelopes.rs: Net Pressure Envelopes, Global Shear & Overturning    │
│  - math3d.rs: 3D Geometric Projections & Vector Rendering on Canvas   │
│  - wind_core.rs: Pipeline Orchestrator for NBR 6123 Compliance         │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                      DATA & PERSISTENCE LAYER                          │
│                        (src/data/ & src/locale/)                       │
│                                                                        │
│  - database.rs: Embedded Read-Only SQLite for Official Wind Databases  │
│  - translations.rs: Full Bilingual Static Dictionary (PT / EN)         │
│  - manual.rs: Bilingual Interactive Academic Study Manual (PT / EN)    │
└────────────────────────────────────────────────────────────────────────┘
```

---

### 3. Functional Module Breakdown

#### 3.1. Engineering Core & Analytical Mechanics (`src/engine/`)
- `wind_core.rs`: Serves as the central facade for the engineering engine. Ingests the project state (`EoloProject`) and executes the sequential calculation of characteristic wind speed $V_k$, dynamic stagnation pressure $q$, external pressure coefficients ($C_{pe}$), internal pressure coefficients ($C_{pi}$), combined net pressures, and global base shear and overturning moments.
- `factors_s1_s2_s3.rs`: Encapsulates environmental, terrain, and recurrence factor formulations:
  - Topographic factor $S_1$ for flat ground ($1.00$), slopes, ridges, and hills evaluated against elevation and slope angles;
  - Roughness and dimension factor $S_2$ spanning 5 terrain categories and 3 building classes under NBR 6123:2023, continuous for any height $z$;
  - Statistical factor $S_3$ configured across the 5 normative building occupancy groups.
- `cpe_cpi_tables.rs`: Houses aerodynamic shape coefficients for rectangular walls subjected to winds at 0° and 90°, single-pitch roofs, gable roofs with continuous roof pitch angles $\alpha$, curved roofs, sawtooth industrial sheds, and isolated open canopies, alongside porosity models for internal pressure $C_{pi}$.
- `envelopes.rs`: Aggregates the load combination matrix (maximum overpressure, maximum suction, and transverse wind loads), computing global base shear forces ($F_x, F_y$) and overturning moments ($M_x, M_y$).
- `isopleth_map.rs`: Supplies the basic wind velocity isopleth database covering all Brazilian states, capitals, and official Air Force (FAB) weather stations.
- `math3d.rs`: Lightweight 3D linear algebra library handling camera rotations, axonometric views, and interactive projection of pressure vectors on the `egui` canvas.

#### 3.2. Data Domain & Project Models (`src/core/`)
- `project.rs`: Encapsulates the core `EoloProject` structure, containing building geometry, geographic coordinates, terrain parameters, fenestration opening areas, and author metadata.
- `scenarios.rs`: Provides 5 standard civil engineering benchmark scenarios:
  1. Standard Industrial Warehouse (20m x 40m, h=7.5m, gable roof at 15°);
  2. Multi-Story Urban Residential Building (15m x 25m, h=30m);
  3. Isolated Open Canopy / Sawtooth Industrial Roof (18m x 36m, h=6m);
  4. Grain Storage Silo / Cylindrical Structure (diameter 12m, h=24m);
  5. Coastal Single-Family Residence (10m x 14m, h=4.5m).
- `types.rs`: Strongly typed enumerations (`TerrainRoughness`, `BuildingClass`, `TopographyType`, `RoofType`, etc.) that make invalid calculation states unrepresentable at compile time.

#### 3.3. Desktop Graphical User Interface (`src/ui/`)
- `app.rs`: Main application shell managing the `eframe` render loop, dispatching UI events, and rendering the five tabbed engineering workspaces.
- `tabs/`:
  - `tab1_location.rs`: Location setup, isopleth lookup, and $V_0, S_1, S_2, S_3$ factor configuration.
  - `tab2_geometry.rs`: Building 3D dimensional input and roof geometry selection.
  - `tab3_openings.rs`: Wall opening permeability and dominant fenestration ratios for $C_{pi}$ calculation.
  - `tab4_canvas.rs`: Interactive 3D structural canvas rendering facades, roofs, and dynamic pressure vectors.
  - `tab5_memorial.rs`: Real-time calculation memorial viewer with step-by-step normative equations and export actions.
- `dialogs/`: Modeless tool windows for academic traceability (`audit_dialog.rs`), quick gust calculator (`quick_calc_dialog.rs`), engineering unit converter (`units_dialog.rs`), interactive manual (`manual_dialog.rs`), preferences (`settings_dialog.rs`), and institutional about/notices/privacy dialog (`about_dialog.rs`, including bibliographic references and academic-study liability disclaimers).

#### 3.4. Reporting & Export Engine (`src/reports/`)
- `memorial_generator.rs`: Produces comprehensive, publication-grade calculation books in three formats:
  - **HTML:** Full calculation book with professional layout, styled data tables, highlighted formulas, and engineering headers;
  - **Markdown:** Clean, lightweight format suitable for version-controlled engineering repositories;
  - **JSON:** Fully structured numerical dataset enabling seamless interoperability with structural concrete and steel analysis software.

#### 3.5. Headless Command-Line Utility (`src/bin/main_cli.rs`)
- Standalone CLI executable compiled as `eolo-cli.exe` for automated workflows, CI/CD pipelines, and script integration without initializing a graphical subsystem:
  - `calc`: Fast calculation of $V_k$ and dynamic pressure $q$;
  - `report`: Automated memorial generation from project files or presets;
  - `isopleth`: Station and city isopleth query tool;
  - `scenarios`: Lists available engineering benchmark presets;
  - `verify`: Automated verification auditing project parameters against NBR 6123:2023;
  - `test`: Deterministic test suite runner.

---

### 4. Resource Efficiency & Performance

- **Zero Garbage Collection:** Rust's deterministic memory management guarantees consistent 60 FPS frametimes without runtime garbage collection pauses;
- **Minimal Memory Footprint:** Typical active RAM consumption stays between 25 MB and 40 MB during full 3D interactive rendering;
- **Instantaneous Startup:** Launches in under 0.2 seconds on standard solid-state drives.

</details>
