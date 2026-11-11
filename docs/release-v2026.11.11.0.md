# Notas de Lançamento / Release Notes — v2026.11.11.0 — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher as notas de lançamento em português</b></summary><br>

# Eolo Platform — Notas Oficiais de Lançamento da Versão `2026.11.8.0`

**Tag de Versão:** `v2026.11.11.0`  
**Data de Publicação:** 11 de novembro de 2026
**Autor e Detentor dos Direitos:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Repositório Oficial:** `https://github.com/fernandoncidade`  

---

### 1. Destaques da Versão

A versão **`2026.11.8.0`** consolida o **Eolo Platform** como a plataforma de engenharia de referência para análise e dimensionamento de ações do vento em edificações no Brasil, desenvolvida integralmente em **Rust (Edição 2021)**. Esta versão introduz o suporte completo à norma **ABNT NBR 6123:2023**, novo sistema visual de alta fidelidade para Windows 11, esteira completa de linha de comando (`eolo-cli`) e conformidade regulatória plena com a legislação brasileira e as políticas da Microsoft Store.

---

### 2. Principais Novidades e Recursos Implementados

#### 2.1. Implementação Completa da ABNT NBR 6123:2023
- **Base de Isopletas de Vento Básico ($V_0$):** Inclusão de base de dados geográfica com coordenadas e velocidades normativas de todas as capitais e estações meteorológicas da Força Aérea Brasileira (FAB);
- **Cálculo Analítico de $S_1, S_2, S_3$:** Avaliação contínua dos fatores topográfico ($S_1$), rugosidade/altura ($S_2$) e estatístico ($S_3$), com suporte às 5 categorias e 3 classes de edificação;
- **Coeficientes de Pressão ($C_{pe}$ e $C_{pi}$):** Tabelas normativas completas para paredes retangulares sob ventos a 0° e 90°, coberturas de uma água, duas águas com inclinação contínua $\alpha$, sheds, coberturas curvas e permeabilidade de aberturas dominantes;
- **Forças Globais e Esforços Basais:** Cálculo de força de arrasto ($C_a$), cortante de base ($F_x, F_y$) e momentos de tombamento ($M_{x}, M_{y}$).

#### 2.2. Integração Visual Completa com Windows 11 & AppUserModelID
- **Ícone Nativo Multi-Resolução:** Inclusão do ícone oficial `eolo.ico` (de 16x16 a 256x256 pixels) embutido no binário executável através do `winres` e no script Inno Setup (`EoloPlatform.iss`);
- **AppUserModelID Dedicado:** Registro explícito de `FernandoCidade.EoloPlatform.NBR6123.2026` via API Win32 no `main.rs`, garantindo fixação correta na barra de tarefas do Windows 11, agrupamento limpo de janelas e exibição de ícone de alta fidelidade;
- **Carregador de Ícone em Runtime:** Implementação de `carregar_icone_janela()` com carregamento do sistema de arquivos e fallback resiliente em memória via `include_bytes!`.

#### 2.3. Duplo Licenciamento Transparente & Governança de Contribuição
- **Código Aberto sob GNU AGPLv3:** Repositório público no GitHub mantido sob a licença GNU AGPLv3 para fomento ao estudo, pesquisa e desenvolvimento científico da engenharia estrutural;
- **Edição Comercial na Microsoft Store (MSIX):** Pacote pré-compilado oficial e assinado digitalmente regido pelo EULA Comercial e CLC;
- **Acordo de Contribuição (CLA) e DCO:** Estabelecimento do documento formal `docs/CLA.md` e exigência de assinatura de Developer Certificate of Origin (`Signed-off-by`) em todos os Pull Requests.

#### 2.4. Conformidade Regulatória Plena no Brasil
- **Lei do Software (Lei nº 9.609/1998):** Declaração explícita de prazo de validade técnica de 12 meses (Art. 7º) e prestação de suporte técnico e correções corretivas no Brasil (Art. 8º);
- **Código de Defesa do Consumidor (Lei nº 8.078/1990):** Preservação irrestrita da garantia legal (Arts. 24 e 25), direito de arrependimento em 7 dias na Microsoft Store (Art. 49) e cláusula de salvaguarda contra isenções abusivas (Art. 51, I);
- **Privacidade Soberana (LGPD / GDPR):** Arquitetura 100% Local-First, sem telemetria, sem cookies e sem tráfego de dados para a nuvem;
- **Estudo Acadêmico & ART (Leis 5.194/1966 e 6.496/1977; Código Civil arts. 186, 393 e 927):** Aplicação voltada exclusivamente para fins de estudo acadêmico (sujeita a possíveis erros e não definitiva para decisões em produção), exigindo conferência manual ou via softwares consolidados e ART por Engenheiro habilitado em caso de aplicação prática; referências bibliográficas das normas ABNT exibidas na aba `Avisos` sem reprodução de conteúdo protegido.

#### 2.5. Nova Interface de Linha de Comando (`eolo-cli.exe`)
- Subcomandos nativos `calc`, `report`, `isopleth`, `scenarios`, `verify`, `test` e `help` para execução headless, batch processing e integração em pipelines de projeto.

#### 2.6. Gerador Modernizado de Memoriais Técnicos
- Exportação em HTML profissional com CSS responsivo para impressão direta ou geração de PDF no navegador, além de formatos abertos em Markdown e JSON estruturado.

---

### 3. Registro de Mudanças (Changelog)

#### Adicionado (Added)
- Módulo `src/build.rs` com suporte ao `winres` para compilação de metadados PE e ícone oficial no Windows;
- Módulo `src/utils/icons.rs` com conversão de ícone ICO/PNG em runtime para `egui::IconData`;
- Chamada Win32 `SetCurrentProcessExplicitAppUserModelID` no ponto de entrada `src/main.rs`;
- 15 textos de licenças de terceiros e fontes em `src/assets/LICENSES/`;
- Textos institucionais bilíngues com paridade estrita em `src/assets/` (EULA, CLC, Privacidade, Sobre, Histórico, Avisos e Notas de Lançamento);
- Documentos de governança técnica em `docs/` (`compliance.md`, `security-model.md`, `architecture.md`, `user-guide.md`, `cli-reference.md`, `licensing.md`, `CLA.md`).

#### Modificado (Changed)
- `Cargo.toml`: Atualizada licença para `AGPL-3.0-only`, repositório oficial e dependências (`winres`, `image`);
- `memorial_generator.rs`: Versão atualizada para `v2026.11.8.0` e rodapé de responsabilidade técnica privativa (CREA/ART);
- `repo/EoloPlatform.iss`: Configuração de atalhos e parâmetros do Inno Setup 7 com `AppUserModelID`.

#### Corrigido (Fixed)
- Resolução de referências de versão estáticas que apontavam para revisões anteriores;
- Eliminação de advertências de compilação em variáveis não utilizadas nos testes analíticos.

---

### 4. Validade Técnica e Suporte

Nos termos do Art. 7º da Lei nº 9.609/1998, a versão `2026.11.8.0` possui validade técnica garantida de **12 (doze) meses** a contar de 8 de novembro de 2026, com suporte técnico via `linceu_lighthouse@outlook.com`.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the release notes in English</b></summary><br>

# Eolo Platform — Official Release Notes for Version `2026.11.8.0`

**Version Tag:** `v2026.11.11.0`  
**Publication Date:** November 11, 2026
**Author & Copyright Holder:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Official Repository:** `https://github.com/fernandoncidade`  

---

### 1. Release Highlights

Release **`2026.11.8.0`** establishes **Eolo Platform** as the benchmark engineering suite for structural wind load calculation and analysis in Brazil, developed from the ground up in **Rust (Edition 2021)**. This milestone release introduces full implementation of Brazilian standard **ABNT NBR 6123:2023**, a redesigned visual branding system for Windows 11, a comprehensive headless CLI pipeline (`eolo-cli`), and complete regulatory compliance with Brazilian law and Microsoft Store guidelines.

---

### 2. Major Features & Technical Capabilities

#### 2.1. Comprehensive Implementation of ABNT NBR 6123:2023
- **Basic Wind Velocity Isopleths ($V_0$):** Integrated geographic database covering official coordinates and design velocities for all Brazilian state capitals and Air Force (FAB) weather stations;
- **Continuous Analytical $S_1, S_2, S_3$ Pipeline:** Evaluates topographic factor ($S_1$), terrain roughness/height factor ($S_2$), and statistical factor ($S_3$) across all 5 roughness categories and 3 building classes;
- **Aerodynamic Pressure Coefficients ($C_{pe}$ and $C_{pi}$):** Complete normative matrices for rectangular walls at 0° and 90°, monopitch roofs, gable roofs with continuous roof pitch angles $\alpha$, sawtooth sheds, curved roofs, and dominant fenestration permeability;
- **Global Wind Reactions:** Automated computation of aerodynamic drag coefficients ($C_a$), base shear ($F_x, F_y$), and overturning moments ($M_x, M_y$).

#### 2.2. Seamless Windows 11 Shell Integration & AppUserModelID
- **Multi-Resolution Native Icon:** High-resolution `eolo.ico` asset (16x16 to 256x256 pixels) compiled directly into the binary resource table via `winres` and configured in the Inno Setup installer (`EoloPlatform.iss`);
- **Dedicated AppUserModelID:** Explicit registration of `FernandoCidade.EoloPlatform.NBR6123.2026` via Win32 API in `main.rs`, ensuring pristine taskbar grouping on Windows 11, reliable pin-to-taskbar behavior, and sharp titlebar icon rendering;
- **Runtime Icon Loader:** Resilient `carregar_icone_janela()` implementation with disk-based loading and embedded in-memory fallback via `include_bytes!`.

#### 2.3. Transparent Dual-Licensing & Contribution Governance
- **Open-Source Codebase under GNU AGPLv3:** Public GitHub repository maintained under the GNU AGPLv3 to advance academic, scientific, and community peer review;
- **Commercial Microsoft Store Edition (MSIX):** Digitally signed, pre-compiled distribution governed by the commercial EULA and CLC;
- **Contributor License Agreement (CLA) & DCO:** Implementation of `docs/CLA.md` requiring Developer Certificate of Origin (`Signed-off-by`) sign-offs on all pull requests.

#### 2.4. Comprehensive Brazilian Statutory Compliance
- **Software Law (Law No. 9,609/1998):** Formal declaration of a 12-month guaranteed technical validity term (Article 7) and complementary technical support in Brazil (Article 8);
- **Consumer Defense Code (Law No. 8,078/1990):** Non-waivable statutory warranties (Articles 24 & 25), 7-day right of withdrawal for Microsoft Store purchases (Article 49), and saving clause against abusive liability waivers (Article 51, I);
- **Sovereign Privacy (LGPD / GDPR):** 100% Local-First architecture, zero telemetry, zero cookies, and zero cloud data transit;
- **Academic Study Notice & ART (Laws 5,194/1966 & 6,496/1977; Civil Code Arts. 186, 393 & 927):** Software strictly intended for academic study only (subject to potential errors and unsuitable for definitive production decisions), requiring manual checking or verification with consolidated market tools and ART registration by a licensed engineer before any real-world use; ABNT bibliographic references are displayed in the `Notices` tab without reproducing protected standard text.

#### 2.5. Dedicated Command-Line Interface (`eolo-cli.exe`)
- Native subcommands `calc`, `report`, `isopleth`, `scenarios`, `verify`, `test`, and `help` for headless server execution, batch calculations, and CI/CD pipelines.

#### 2.6. Modernized Calculation Memorial Engine
- Professional HTML export with styled print-ready CSS for direct PDF generation, accompanied by open Markdown and structured JSON export formats.

---

### 3. Detailed Changelog

#### Added
- `src/build.rs` module with `winres` support for Windows PE manifest, icon, and metadata compilation;
- `src/utils/icons.rs` providing runtime ICO/PNG conversion into `egui::IconData`;
- Native Win32 `SetCurrentProcessExplicitAppUserModelID` invocation inside `src/main.rs`;
- 15 upstream open-source license texts in `src/assets/LICENSES/`;
- Institutional bilingual assets with strict structural parity in `src/assets/` (EULA, CLC, Privacy Policy, About, History, Notices, and Release Notes);
- Technical governance documentation in `docs/` (`compliance.md`, `security-model.md`, `architecture.md`, `user-guide.md`, `cli-reference.md`, `licensing.md`, `CLA.md`).

#### Changed
- `Cargo.toml`: Updated package license to `AGPL-3.0-only`, official repository link, and new build dependencies (`winres`, `image`);
- `memorial_generator.rs`: Updated engine version stamp to `v2026.11.8.0` with professional engineering responsibility notices;
- `repo/EoloPlatform.iss`: Upgraded shortcuts and metadata configurations for Inno Setup 7 with AppUserModelID.

#### Fixed
- Replaced legacy version strings across reports and UI dialogs;
- Resolved compilation warnings on unused variables in test suites.

---

### 4. Technical Validity & Support

Pursuant to Article 7 of Brazilian Federal Law No. 9,609/1998, release `2026.11.8.0` carries a guaranteed technical validity period of **12 (twelve) months** starting November 8, 2026, supported via `linceu_lighthouse@outlook.com`.

</details>
