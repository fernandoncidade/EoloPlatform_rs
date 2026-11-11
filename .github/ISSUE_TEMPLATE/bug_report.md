---
name: Relato de Bug / Bug Report
about: Crie um relatório para ajudar a diagnosticar e corrigir uma falha no Eolo Platform / Create a report to help diagnose and fix a bug in Eolo Platform
title: '[BUG] '
labels: ['bug', 'triage']
assignees: ''
---

<details>
<summary>Clique para expandir / recolher o formulário em Português</summary><br>

## Descrição do Problema
Uma descrição clara e concisa do comportamento inesperado ou falha encontrada no **Eolo Platform**.

## Informações do Ambiente
- **Versão do Sistema Operacional:** Windows 11 / Windows 10 (ex: Windows 11 24H2 x64)
- **Versão do Eolo Platform:** `v2026.11.8.0`
- **Binário Afetado:** [ ] Desktop GUI (`eolo.exe`)  [ ] Utilitário CLI (`eolo-cli.exe`)  [ ] Ambos
- **Tipo de Distribuição:** [ ] Microsoft Store (MSIX)  [ ] Instalador Inno Setup (`EoloPlatform_v*.exe`)  [ ] Compilado do Código-Fonte (`cargo build --release`)
- **Toolchain Rust (se compilado do fonte):** Saída de `rustc --version` e `cargo --version` (ex: `stable-x86_64-pc-windows-msvc`)
- **Adaptador Gráfico / GPU (se problema no visualizador 3D):** (ex: NVIDIA RTX / AMD Radeon / Intel Iris Xe)

## Módulo / Subsistema Relacionado
- [ ] Aba 1 — Localização, Isopletas V0 e Fatores S1, S2, S3 (ABNT NBR 6123)
- [ ] Aba 2 — Geometria da Edificação e Coeficientes Externos (Cpe/Ce — Tabelas 6, 7, 8, 10, 14)
- [ ] Aba 3 — Aberturas de Fachada, Permeabilidade e Coeficiente Interno (Cpi)
- [ ] Aba 4 — Diagrama de Cargas e Visualizador 2D/3D na GPU (`egui`/`eframe`/`wgpu`)
- [ ] Aba 5 — Memorial de Cálculo Formal e Exportação (HTML5, Markdown, JSON)
- [ ] Utilitário de Linha de Comando (`eolo-cli.exe`: `calc`, `report`, `isopleth`, `verify`)
- [ ] Bilinguismo em Tempo Real (pt-BR / en-US) ou Alternância de Tema (Claro / Escuro)
- [ ] Diálogo Sobre o Eolo Platform (9 Abas Informativas / Licenças)

## Passos para Reproduzir o Erro
1. Na Aba '...', configure os parâmetros: ...
2. Execute a ação ou clique no botão '...'
3. Observe o erro ou discrepância nos cálculos / interface.

## Comportamento Esperado
Uma descrição clara do que você esperava que acontecesse com base na ABNT NBR 6123:2023.

## Logs, Mensagens de Erro ou Capturas de Tela
Se aplicável, anexe capturas de tela ou trechos do terminal (scrub de dados confidenciais):
```text
Cole aqui mensagens de erro do terminal, backtrace (RUST_BACKTRACE=1) ou logs relevantes
```

## Informações Adicionais
Adicione qualquer outro detalhe relevante sobre o cálculo ou parâmetros da edificação.

</details>

<details>
<summary>Click to expand / collapse the form in English</summary><br>

## Description
A clear and concise description of the unexpected behavior or bug encountered in **Eolo Platform**.

## Environment Information
- **OS Version:** Windows 11 / Windows 10 (e.g., Windows 11 24H2 x64)
- **Eolo Platform Version:** `v2026.11.8.0`
- **Affected Binary:** [ ] Desktop GUI (`eolo.exe`)  [ ] Console CLI (`eolo-cli.exe`)  [ ] Both
- **Distribution:** [ ] Microsoft Store (MSIX)  [ ] Inno Setup Installer (`EoloPlatform_v*.exe`)  [ ] Built from Source (`cargo build --release`)
- **Rust Toolchain (if built from source):** Output of `rustc --version` and `cargo --version` (e.g., `stable-x86_64-pc-windows-msvc`)
- **Graphics Card / GPU (if 3D viewport issue):** (e.g., NVIDIA RTX / AMD Radeon / Intel Iris Xe)

## Affected Subsystem
- [ ] Tab 1 — Location, V0 Isopleths, and S1, S2, S3 Factors (ABNT NBR 6123)
- [ ] Tab 2 — Building Geometry and External Coefficients (Cpe/Ce — Tables 6, 7, 8, 10, 14)
- [ ] Tab 3 — Facade Openings, Permeability, and Internal Pressure (Cpi)
- [ ] Tab 4 — Load Diagrams and 2D/3D GPU Viewport (`egui`/`eframe`/`wgpu`)
- [ ] Tab 5 — Formal Calculation Report and Export (HTML5, Markdown, JSON)
- [ ] Command-Line Utility (`eolo-cli.exe`: `calc`, `report`, `isopleth`, `verify`)
- [ ] Real-Time Bilingual Switching (pt-BR / en-US) or Theme Toggle (Light / Dark)
- [ ] About Dialog (9 Information Tabs / Legal Notices)

## Steps to Reproduce
1. In Tab '...', configure parameters: ...
2. Perform action or click button '...'
3. See error or numerical discrepancy.

## Expected Behavior
A clear and concise description of what you expected to happen per ABNT NBR 6123:2023.

## Logs, Error Output, or Screenshots
If applicable, attach screenshots or terminal output (scrubbing sensitive project details):
```text
Paste terminal output, Rust backtrace (RUST_BACKTRACE=1), or relevant logs here
```

## Additional Context
Add any other context about the calculation scenario or building geometry here.

</details>