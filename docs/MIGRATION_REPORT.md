# Relatório Técnico de Consolidação / Consolidation Report — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o relatório de consolidação em português</b></summary><br>

# Eolo Platform — Relatório Técnico de Consolidação Arquitetural e Conformidade Regulatória

**Versão Consolidada:** `2026.11.8.0`  
**Data de Emissão:** `8 de novembro de 2026`  
**Plataforma Base:** Rust (Edição 2021) | `egui 0.31` / `eframe 0.31` / `wgpu`  
**Autor e Engenheiro Responsável:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Repositório Oficial:** `https://github.com/fernandoncidade`  

---

### 1. Resumo Executivo da Consolidação

O projeto **Eolo Platform** alcançou sua maturidade técnica e normativa (versão 2026.11.11.0), consolidando a migração para uma arquitetura nativa em **Rust (Edição 2021)** com interface gráfica de modo imediato (*immediate-mode*) acelerada por hardware em **`egui` / `eframe`** e interface de linha de comando autônoma (**`eolo-cli`**).

O sistema fornece uma solução determinística de alto desempenho para modelagem das ações do vento em edificações e estruturas industriais conforme a **ABNT NBR 6123:2023**, com conformidade legal abrangente perante a legislação brasileira (Lei do Software, CDC, LGPD, CONFEA/CREA) e os requisitos de distribuição da Microsoft Store.

---

### 2. Mapeamento Arquitetural dos Subsistemas

| Módulo do Sistema | Responsabilidade Funcional | Arquivos em `src/` |
|:---|:---|:---|
| **Núcleo de Vento & Isopletas** | Mapeamento de $V_0$ por coordenadas, capitais e estações da FAB | `src/engine/isopleth_map.rs` |
| **Fatores Normativos ($S_1, S_2, S_3$)** | Formulação analítica de topografia, rugosidade/altura e probabilidade estatística | `src/engine/factors_s1_s2_s3.rs` |
| **Tabelas de Coeficientes ($C_{pe}, C_{pi}$)** | Matrizes aerodinâmicas de paredes, telhados de 1 e 2 águas, sheds, cúpulas e aberturas | `src/engine/cpe_cpi_tables.rs` |
| **Envoltórias e Esforços Globais** | Combinações de sobrepressão e sucção, corte basal e momentos de tombamento | `src/engine/envelopes.rs` |
| **Motor 3D & Álgebra Linear** | Projeções tridimensionais, translação, órbita e vetores de carga no canvas | `src/engine/math3d.rs` |
| **Orquestrador Central de Cálculo** | Execução sequencial da esteira completa da NBR 6123:2023 | `src/engine/wind_core.rs` |
| **Modelo de Domínio & Cenários** | Estrutura unificada de projetos (`EoloProject`) e 5 presets de engenharia civil | `src/core/project.rs`, `scenarios.rs`, `types.rs` |
| **Persistência Local SQLite** | Base embutida somente leitura para consultas de estações meteorológicas | `src/data/database.rs`, `src/data/mod.rs` |
| **Interface Gráfica (GUI)** | Shell `eframe`, menus, toolbar responsiva e 5 abas de trabalho | `src/ui/app.rs`, `toolbar.rs`, `menu_bar.rs`, `tabs/*` |
| **Diálogos Modeless & Modais** | Telas de rastreabilidade acadêmica, calculadora rápida, manual e diálogo Sobre (com referências bibliográficas em Avisos) | `src/ui/dialogs/*` |
| **Utilitário de Linha de Comando** | Ferramenta de console headless para simulações acadêmicas e scripts | `src/bin/main_cli.rs` |
| **Gerador de Relatórios** | Relatórios de estudo em HTML responsivo, Markdown e JSON | `src/reports/memorial_generator.rs` |
| **Internacionalização & Manuais** | Dicionários estáticos bilíngues e manual de estudo em Português/Inglês | `src/locale/translations.rs`, `manual.rs` |
| **Integração Win32 e Ícones** | Compilação de recursos PE (`winres`), AppUserModelID e decodificação ICO/PNG | `src/build.rs`, `src/utils/icons.rs`, `src/main.rs` |

---

### 3. Síntese de Conformidade Legal e Regulatória

1. **Normas Técnicas da ABNT:** Rastreabilidade analítica e conformidade estrita com a ABNT NBR 6123:2023, ABNT NBR 8800:2024 e ABNT NBR 6118:2023;
2. **Duplo Licenciamento:** Repositório de código-fonte livre sob **GNU AGPLv3** (`LICENSE`) e versão compilada para a Microsoft Store sob **EULA Comercial** e **CLC**;
3. **Lei do Software (Lei nº 9.609/1998):** Declaração explícita de prazo de validade técnica de 12 meses (Art. 7º) e prestação contínua de suporte técnico e correções no Brasil (Art. 8º);
4. **Código de Defesa do Consumidor (Lei nº 8.078/1990):** Garantia legal irrenunciável (Arts. 24 e 25), prazo de desistência de 7 dias para compras digitais (Art. 49) e foro do consumidor (Art. 101, I);
5. **Privacidade Soberana (LGPD / GDPR):** Processamento 100% Local-First e offline, sem coleta de dados pessoais, sem cookies e sem telemetria externa;
6. **Responsabilidade Profissional (Leis nº 5.194/1966 e nº 6.496/1977):** Inserção obrigatória de alerta formal nos memoriais de cálculo de que o software não substitui a responsabilidade técnica e o recolhimento de ART por Engenheiro Civil habilitado.

---

### 4. Integração com o Ecossistema Windows 11

- **Ícone Multi-Resolução e `winres`:** Ícone de alta densidade (`src/assets/icons/eolo.ico`) embutido no cabeçalho executável PE, garantindo exibição de alta qualidade no Windows Explorer e no Inno Setup;
- **AppUserModelID Dedicado:** Registro explícito de `FernandoCidade.EoloPlatform.NBR6123.2026` via API Win32 no `main.rs`, garantindo fixação correta na barra de tarefas do Windows 11 e isolamento de janelas;
- **Instalador Inno Setup 7:** Script `repo/EoloPlatform.iss` modernizado com caminhos canônicos de distribuição e atalhos com AppUserModelID.

---

### 5. Métricas de Desempenho Alcançadas

- **Consumo de Memória RAM:** Entre 25 MB e 40 MB em operação plena com canvas 3D ativo;
- **Tempo de Inicialização:** Inferior a 0,2 segundos em drives SSD;
- **Taxa de Quadros da Interface:** 60 FPS contínuos e sem bloqueios durante rotação 3D do modelo estrutural;
- **Tamanho do Binário Final:** ~16 MB (autônomo, sem necessidade de runtimes ou frameworks externos).

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the consolidation report in English</b></summary><br>

# Eolo Platform — Architectural Consolidation & Regulatory Compliance Technical Report

**Consolidated Version:** `2026.11.8.0`  
**Issue Date:** `November 8, 2026`  
**Technology Base:** Rust (Edition 2021) | `egui 0.31` / `eframe 0.31` / `wgpu`  
**Lead Engineer & Author:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Official Repository:** `https://github.com/fernandoncidade`  

---

### 1. Executive Summary

The **Eolo Platform** project achieves complete technical and regulatory consolidation in release **`2026.11.8.0`**, completing its migration to a native **Rust (Edition 2021)** architecture featuring a hardware-accelerated immediate-mode GUI powered by **`egui` / `eframe`** and a standalone command-line interface (**`eolo-cli`**).

The system delivers a deterministic, high-performance solution for modeling wind loads on buildings and industrial structures under Brazilian standard **ABNT NBR 6123:2023**, supported by comprehensive statutory compliance across Brazilian law (Software Law, Consumer Code, LGPD, CONFEA/CREA) and Microsoft Store commercial distribution guidelines.

---

### 2. Architectural Subsystem Mapping

| System Module | Functional Responsibility | Files in `src/` |
|:---|:---|:---|
| **Wind Core & Isopleths** | $V_0$ lookup across coordinates, state capitals, and FAB weather stations | `src/engine/isopleth_map.rs` |
| **Normative Factors ($S_1, S_2, S_3$)** | Continuous analytical formulations for topography, roughness/height, and recurrence | `src/engine/factors_s1_s2_s3.rs` |
| **Coefficient Tables ($C_{pe}, C_{pi}$)** | Aerodynamic matrices for walls, monopitch/gable/shed roofs, domes, and openings | `src/engine/cpe_cpi_tables.rs` |
| **Envelopes & Global Reactions** | Suction and overpressure combinations, base shear, and overturning moments | `src/engine/envelopes.rs` |
| **3D Engine & Linear Algebra** | Spatial projections, pan, orbit, and force vector rendering on the canvas | `src/engine/math3d.rs` |
| **Calculation Orchestrator** | Sequential pipeline execution for complete NBR 6123:2023 compliance | `src/engine/wind_core.rs` |
| **Domain Models & Presets** | Unified project model (`EoloProject`) and 5 civil engineering benchmarks | `src/core/project.rs`, `scenarios.rs`, `types.rs` |
| **Local SQLite Persistence** | Embedded read-only database for meteorological station lookups | `src/data/database.rs`, `src/data/mod.rs` |
| **Graphical Interface (GUI)** | `eframe` shell, menus, responsive toolbar, and 5 sequential workspaces | `src/ui/app.rs`, `toolbar.rs`, `menu_bar.rs`, `tabs/*` |
| **Modeless & Modal Dialogs** | Windows for academic traceability, quick calculator, manual, and About dialog (with bibliographic references in Notices) | `src/ui/dialogs/*` |
| **Command-Line Interface** | Headless console tool for academic simulations and scripting | `src/bin/main_cli.rs` |
| **Reporting Engine** | Academic study calculation memorials in responsive HTML, Markdown, and JSON | `src/reports/memorial_generator.rs` |
| **Localization & Reference Guides** | Full static bilingual dictionaries and bilingual study manual | `src/locale/translations.rs`, `manual.rs` |
| **Win32 & Icon Subsystem** | PE resource compilation (`winres`), AppUserModelID, and ICO/PNG loaders | `src/build.rs`, `src/utils/icons.rs`, `src/main.rs` |

---

### 3. Legal & Regulatory Compliance Summary

1. **ABNT Engineering Standards:** Analytical traceability and strict compliance with ABNT NBR 6123:2023, ABNT NBR 8800:2024, and ABNT NBR 6118:2023;
2. **Dual-Licensing Model:** Public open-source repository under **GNU AGPLv3** (`LICENSE`) alongside a pre-compiled commercial distribution for the Microsoft Store under **Commercial EULA** and **CLC**;
3. **Software Law (Law No. 9,609/1998):** Formal 12-month technical validity warranty (Article 7) and complementary technical support and maintenance within Brazil (Article 8);
4. **Consumer Defense Code (Law No. 8,078/1990):** Non-waivable statutory warranties (Articles 24 & 25), 7-day right of withdrawal for remote digital purchases (Article 49), and consumer forum jurisdiction (Article 101, I);
5. **Sovereign Privacy (LGPD / GDPR):** 100% Local-First offline execution with zero personal data collection, zero cookies, and zero external telemetry;
6. **Professional Engineering Notice (Laws No. 5,194/1966 & No. 6,496/1977):** Mandatory inclusion in all calculation memorials stating that the tool does not replace technical review and ART registration by a licensed Civil Engineer.

---

### 4. Windows 11 Shell Integration

- **Multi-Resolution Icon & `winres`:** High-DPI icon asset (`src/assets/icons/eolo.ico`) compiled into the Windows PE resource directory, ensuring crisp desktop rendering;
- **Dedicated AppUserModelID:** Explicit registration of `FernandoCidade.EoloPlatform.NBR6123.2026` via Win32 API in `main.rs`, guaranteeing taskbar grouping and pinning behavior;
- **Inno Setup 7 Installer:** Clean `repo/EoloPlatform.iss` script with verified target distribution paths and AppUserModelID shortcuts.

---

### 5. Benchmark Performance Metrics

- **RAM Footprint:** Between 25 MB and 40 MB during active 3D canvas interaction;
- **Launch Time:** Under 0.2 seconds on standard SSD drives;
- **GUI Framerate:** Smooth 60 FPS without frame drops during 3D model manipulation;
- **Executable Binary Size:** ~16 MB (completely self-contained, requiring no external runtimes or frameworks).

</details>
