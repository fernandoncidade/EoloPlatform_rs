# Sugestões de Issues Iniciais / Starter Issues — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher as sugestões de issues em português</b></summary><br>

# Eolo Platform — Guia de Issues Recomendadas para a Comunidade

**Versão:** `2026.11.8.0`  
**Data:** `8 de novembro de 2026`  
**Público:** Desenvolvedores Rust, Engenheiros Civis, Pesquisadores de Aerodinâmica das Construções e Contribuidores Open-Source  

Este documento apresenta 6 sugestões de *issues* bem estruturadas para a versão vigente (ex: v2026.11.8.0), direcionando novas contribuições ao repositório público do Eolo Platform. Todas as contribuições submetidas via Pull Request devem observar os termos do [Acordo de Licença de Colaborador (CLA)](CLA.md) e conter a assinatura formal de conformidade com o DCO (`Signed-off-by`).

---

### Issue 1: Suporte a Coberturas Curvas Cilíndricas e Cúpulas Hemisféricas (NBR 6123 Item 6.3)
- **Título:** `[FEATURE] Support cylindrical curved roofs and hemispherical domes under ABNT NBR 6123:2023`
- **Labels:** `enhancement`, `engineering`, `rust`
- **Conteúdo da Issue:**
```markdown
### Resumo
Atualmente, o motor em `src/engine/cpe_cpi_tables.rs` contempla com precisão coberturas planas, de uma água, de duas águas com ângulo alfa contínuo e sheds industriais.

### Melhoria Proposta
Implementar a formulação completa para coberturas curvas cilíndricas (abóbadas) e cúpulas (domos hemisféricos) conforme o Item 6.3 e Tabela 7 da ABNT NBR 6123:2023:
- Cálculo dos coeficientes Cpe em função da relação f/l (flecha sobre vão) e h/l (altura sobre vão);
- Divisão das zonas aerodinâmicas de barlavento, topo central e sotavento;
- Renderização 3D paramétrica no canvas `tab4_canvas.rs`.

### Critérios de Aceite
- [ ] Funções analíticas dedicadas em `src/engine/cpe_cpi_tables.rs` com testes unitários cobrindo f/l de 0,1 a 0,5.
- [ ] Visualização 3D no canvas egui refletindo a malha da abóbada.
- [ ] Inclusão das formulações correspondentes no memorial gerado em `src/reports/`.
```

---

### Issue 2: Módulo de Forças em Estruturas Reticuladas e Treliças Metálicas (NBR 6123 Item 7.3)
- **Título:** `[FEATURE] Wind force module for plane and spatial lattice trusses under NBR 6123 Clause 7.3`
- **Labels:** `enhancement`, `aerodynamics`, `structural-steel`
- **Conteúdo da Issue:**
```markdown
### Resumo
Estruturas de suporte de esteiras industriais, pontes rolantes, torres de transmissão e vigas treliçadas espaciais necessitam de coeficientes de arrasto específicos para perfis tubulares e cantoneiras.

### Melhoria Proposta
- Implementar o cálculo do índice de área cheia (taxa de solidez phi = A_real / A_total);
- Determinar os coeficientes de arrasto Ca para treliças planas isoladas e treliças paralelas múltiplas com efeito de proteção contra o vento (Tabela 11 e 12 da norma).

### Critérios de Aceite
- [ ] Módulo analítico em `src/engine/factors_s1_s2_s3.rs` ou `src/engine/trusses.rs`.
- [ ] Suporte a perfis de cantos vivos vs. perfis circulares/tubulares com dependência do número de Reynolds.
```

---

### Issue 3: Otimização SIMD para Envoltórias de Pressão em Malhas Densas
- **Título:** `[PERF] SIMD vectorization for multi-panel wind envelope calculations`
- **Labels:** `performance`, `simd`, `rust`
- **Conteúdo da Issue:**
```markdown
### Resumo
A geração de envoltórias completas para grandes galpões com dezenas de combinações de vento (0°, 45°, 90°, 135°, 180°) e múltiplas hipóteses de Cpi realiza repetições intensivas de produtos vetoriais.

### Melhoria Proposta
Utilizar instruções vetorizadas AVX2 / NEON via `core::simd` para processar em paralelo as matrizes de pressão líquida `q * (Cpe - Cpi)` e encontrar valores máximos e mínimos de projeto instantaneamente.

### Critérios de Aceite
- [ ] Aceleração de pelo menos 3x no benchmark de envoltórias com 10.000 painéis discretizados.
- [ ] Fallback transparente para arquiteturas sem suporte a AVX2.
```

---

### Issue 4: Exportação Estruturada para Softwares de Cálculo (SAP2000, Ftool e Eberick)
- **Título:** `[FEATURE] Direct export of nodal wind loads to SAP2000 (.s2k), Ftool and JSON schemas`
- **Labels:** `interoperability`, `civil-engineering`, `json`
- **Conteúdo da Issue:**
```markdown
### Resumo
Engenheiros projetistas frequentemente necessitam transferir os esforços calculados no Eolo Platform para softwares de análise de elementos finitos ou pórticos planos.

### Melhoria Proposta
- Adicionar gerador de scripts de carregamento no formato texto `.s2k` (SAP2000) e esquemas JSON estruturados com forças nodais equivalentes nas coordenadas (x, y, z).

### Critérios de Aceite
- [ ] Diálogo de exportação integrado na Aba 5 (Memorial).
- [ ] Suporte a forças concentradas nos nós dos pilares e cargas distribuídas nas terças de cobertura.
```

---

### Issue 5: Interpolação Geoespacial de Alta Resolução por Kriging / Voronoi para Isopletas de V0
- **Título:** `[FEATURE] High-resolution geospatial interpolation for basic wind speed V0`
- **Labels:** `geospatial`, `gis`, `math`
- **Conteúdo da Issue:**
```markdown
### Resumo
Atualmente, as estações meteorológicas em `src/engine/isopleth_map.rs` utilizam mapeamento pontual por municípios e capitais.

### Melhoria Proposta
Implementar interpolação espacial ponderada por distância inversa (IDW) ou Kriging ordinário para determinar a velocidade básica V0 em qualquer coordenada geográfica (Latitude/Longitude) dentro do território nacional.

### Critérios de Aceite
- [ ] Cálculo da velocidade V0 a partir de Latitude e Longitude digitadas pelo usuário.
- [ ] Validação contra o mapa vetorial oficial de isopletas do Anexo A da NBR 6123:2023.
```

---

### Issue 6: Internacionalização de Normas Técnicas Adicionais (Eurocode 1 e ASCE 7)
- **Título:** `[FEATURE] Modular framework architecture for Eurocode 1 (EN 1991-1-4) and ASCE 7-22`
- **Labels:** `architecture`, `internationalization`, `standards`
- **Conteúdo da Issue:**
```markdown
### Resumo
A arquitetura do Eolo Platform foi desacoplada em domínios analíticos (`src/engine/`).

### Melhoria Proposta
Generalizar o trait `WindStandard` para permitir a inclusão de motores alternativos de normas estrangeiras, mantendo a interface egui e os visualizadores 3D unificados.

### Critérios de Aceite
- [ ] Definição do trait `WindCodeEngine` com métodos abstratos para velocidade básica, fatores de terreno, coeficientes de pressão e forças globais.
- [ ] Preservação de 100% dos testes e conformidade existente da ABNT NBR 6123:2023 como motor padrão.
```

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the starter issues in English</b></summary><br>

# Eolo Platform — Community Recommended Starter Issues

**Version:** `2026.11.8.0`  
**Date:** `November 8, 2026`  
**Target Audience:** Rust Developers, Civil Engineers, Wind Engineering Researchers, and Open-Source Contributors  

This document outlines 6 well-structured starter issues for the active release (e.g., v2026.11.8.0), designed to guide contributions to the public Eolo Platform repository. All contributions submitted via Pull Requests must comply with the [Contributor License Agreement (CLA)](CLA.md) and include a formal Developer Certificate of Origin (`Signed-off-by`) sign-off.

---

### Issue 1: Support Cylindrical Curved Roofs and Hemispherical Domes (NBR 6123 Clause 6.3)
- **Title:** `[FEATURE] Support cylindrical curved roofs and hemispherical domes under ABNT NBR 6123:2023`
- **Labels:** `enhancement`, `engineering`, `rust`
- **Issue Description:**
```markdown
### Summary
Currently, the analytical tables in `src/engine/cpe_cpi_tables.rs` handle flat, monopitch, gable (with continuous pitch angle alpha), and industrial sawtooth (shed) roofs.

### Proposed Improvement
Implement comprehensive formulation for cylindrical curved roofs (barrel vaults) and hemispherical domes pursuant to Clause 6.3 and Table 7 of ABNT NBR 6123:2023:
- Calculate Cpe coefficients as functions of rise-to-span (f/l) and height-to-span (h/l) ratios;
- Discretize windward, central crown, and leeward aerodynamic zones;
- Provide parametric 3D representation inside `tab4_canvas.rs`.

### Acceptance Criteria
- [ ] Dedicated analytical routines in `src/engine/cpe_cpi_tables.rs` with unit tests covering f/l from 0.1 to 0.5.
- [ ] Interactive 3D mesh rendering in the egui canvas.
- [ ] Incorporation into calculation memorials generated by `src/reports/`.
```

---

### Issue 2: Wind Force Module for Plane and Spatial Lattice Trusses (NBR 6123 Clause 7.3)
- **Title:** `[FEATURE] Wind force module for plane and spatial lattice trusses under NBR 6123 Clause 7.3`
- **Labels:** `enhancement`, `aerodynamics`, `structural-steel`
- **Issue Description:**
```markdown
### Summary
Support towers, transmission structures, and space trusses require specialized drag coefficients for angle and tubular cross-sections.

### Proposed Improvement
- Implement solidity ratio computation (phi = A_solid / A_enclosed);
- Evaluate aerodynamic drag coefficients Ca for single planar trusses and multi-bay parallel trusses accounting for shielding effects (Tables 11 & 12).

### Acceptance Criteria
- [ ] Analytical module in `src/engine/factors_s1_s2_s3.rs` or `src/engine/trusses.rs`.
- [ ] Distinction between sharp-edged members and circular/tubular sections with Reynolds number dependency.
```

---

### Issue 3: SIMD Vectorization for Multi-Panel Wind Envelope Calculations
- **Title:** `[PERF] SIMD vectorization for multi-panel wind envelope calculations`
- **Labels:** `performance`, `simd`, `rust`
- **Issue Description:**
```markdown
### Summary
Generating multi-directional load envelopes (0°, 45°, 90°, 135°, 180°) across multiple Cpi scenarios performs repetitive matrix vector operations.

### Proposed Improvement
Utilize AVX2 / NEON vector intrinsics via `core::simd` to parallelize net pressure matrices `q * (Cpe - Cpi)` and instantaneously determine peak suction and overpressure design values.

### Acceptance Criteria
- [ ] At least 3x speedup on benchmark envelope calculations with 10,000 discretized panels.
- [ ] Transparent fallback for CPUs lacking AVX2 extensions.
```

---

### Issue 4: Direct Export of Nodal Wind Loads to Structural Analysis Software (SAP2000, Ftool, JSON)
- **Title:** `[FEATURE] Direct export of nodal wind loads to SAP2000 (.s2k), Ftool and JSON schemas`
- **Labels:** `interoperability`, `civil-engineering`, `json`
- **Issue Description:**
```markdown
### Summary
Structural engineers routinely need to transfer computed wind loads from Eolo Platform to downstream finite element and frame analysis packages.

### Proposed Improvement
- Add load export drivers generating `.s2k` text scripts (SAP2000) and structured JSON schemas mapping equivalent nodal wind forces to (x, y, z) coordinates.

### Acceptance Criteria
- [ ] Export dialog integrated into Tab 5 (Memorial).
- [ ] Support for concentrated joint loads along column lines and distributed loads along roof purlins.
```

---

### Issue 5: High-Resolution Geospatial Interpolation for Basic Wind Speed V0
- **Title:** `[FEATURE] High-resolution geospatial interpolation for basic wind speed V0`
- **Labels:** `geospatial`, `gis`, `math`
- **Issue Description:**
```markdown
### Summary
Current weather stations in `src/engine/isopleth_map.rs` operate via discrete municipal lookups.

### Proposed Improvement
Implement Inverse Distance Weighting (IDW) or ordinary Kriging to compute basic wind velocity V0 at any user-specified geographic latitude and longitude across Brazil.

### Acceptance Criteria
- [ ] Calculation of V0 from user-entered coordinates.
- [ ] Numerical validation against the official isopleth contours of ABNT NBR 6123:2023 Annex A.
```

---

### Issue 6: Modular Multi-Standard Architecture (Eurocode 1 and ASCE 7)
- **Title:** `[FEATURE] Modular framework architecture for Eurocode 1 (EN 1991-1-4) and ASCE 7-22`
- **Labels:** `architecture`, `internationalization`, `standards`
- **Issue Description:**
```markdown
### Summary
Eolo Platform's modular analytical domain (`src/engine/`) allows generalized extension to foreign engineering codes.

### Proposed Improvement
Introduce a generalized `WindStandard` trait allowing pluggable implementations of international standards while reusing the shared egui UI and 3D visualizers.

### Acceptance Criteria
- [ ] Definition of a `WindCodeEngine` trait with methods for basic speed, terrain factors, shape coefficients, and global forces.
- [ ] 100% regression-free preservation of ABNT NBR 6123:2023 as the primary engine.
```

</details>
