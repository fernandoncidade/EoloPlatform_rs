# Manual Técnico de Engenharia / Structural Engineering Manual — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o manual técnico em português</b></summary><br>

# Eolo Platform — Manual Técnico de Formulações e Engenharia

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Norma Principal:** ABNT NBR 6123:2023 (*Forças devidas ao vento em edificações*)  
**Normas Correlatas:** ABNT NBR 8800:2024 (*Estruturas de Aço*) e ABNT NBR 6118:2023 (*Estruturas de Concreto*)  
**Autor e Responsável:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. Visão Geral do Sistema

O **Eolo Platform** é uma aplicação desktop corporativa de engenharia civil desenvolvida nativamente em **Rust (Edição 2021)** com interface imediata acelerada por GPU em **`egui` / `eframe` / `wgpu`**. O sistema modela de forma determinística as ações estáticas e aerodinâmicas do vento em edificações paralelepipédicas, galpões industriais e coberturas isoladas, gerando memoriais descritivos auditáveis em conformidade com as normas brasileiras da ABNT.

---

### 2. Equações e Métodos Analíticos da ABNT NBR 6123:2023

#### 2.1. Velocidade Característica do Vento ($V_k$)
$$V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3 \quad [m/s]$$
- **$V_0$ (Velocidade Básica):** Rajada de 3 segundos com período de recorrência de 50 anos a 10m de altura em terreno plano aberto (Anexo A / Mapa de Isopletas);
- **$S_1$ (Fator Topográfico):**
  - Terreno plano ou suavemente ondulado: $S_1 = 1,00$;
  - Taludes e encostas íngremes: $S_1$ calculado continuamente em função do desnível e da inclinação $\theta$;
  - Morros e cristas alongadas: interpolação conforme perfil de cumeeira;
- **$S_2$ (Fator de Rugosidade, Dimensões e Altura):**
  $$S_2(z) = b \cdot F_r \cdot \left(\frac{z}{10}\right)^p$$
  Avaliado para as Categorias de Rugosidade de I a V e Classes de Dimensão A, B e C, respeitando os limites de altura mínima $z_{min}$ da Tabela 2 da norma;
- **$S_3$ (Fator Estatístico de Ocupação e Risco):**
  Varia de $0,83$ (instalações provisórias e agrícolas) a $1,10$ (hospitais e centrais de defesa civil).

#### 2.2. Pressão Dinâmica Básica ($q$)
$$q = 0,613 \cdot V_k^2 \quad [N/m^2 \text{ ou } Pa]$$
Convertível diretamente para $kN/m^2$ ($1 \, kN/m^2 = 1000 \, Pa$) ou $kgf/m^2$ ($1 \, kgf/m^2 \approx 9,80665 \, Pa$).

#### 2.3. Pressão Efetiva Resultante ($\Delta p$)
$$\Delta p = (C_{pe} - C_{pi}) \cdot q \quad [N/m^2]$$
- **$C_{pe}$ (Coeficiente de Pressão Externa):** Determinado em função da geometria e da direção do vento (0° ou 90°) para paredes frontais, posteriores, laterais e painéis de cobertura;
- **$C_{pi}$ (Coeficiente de Pressão Interna):** Calculado com base na permeabilidade relativa das paredes ou pela presença de aberturas dominantes (Item 6.2.5).

#### 2.4. Forças Globais e Esforços de Base
- **Força Total de Arrasto ($F_x, F_y$):**
  $$F = C_a \cdot q \cdot A_e \quad [N \text{ ou } kN]$$
  onde $C_a$ é o coeficiente de arrasto global e $A_e$ é a área frontal de projeção efetiva;
- **Momento de Tombamento ($M_t$):**
  $$M_t = \int q(z) \cdot C_{pe}(z) \cdot z \, dA \quad [kN \cdot m]$$

---

### 3. As 5 Abas de Trabalho de Engenharia

1. **Aba 1 — Localização & $V_0$:** Seleção de UF, cidade ou estação meteorológica da FAB e calibração fina de $S_1$, $S_2$ e $S_3$;
2. **Aba 2 — Geometria da Edificação:** Dimensões em planta ($a \times b$), pé-direito ($h$), tipologia de cobertura (plana, uma água, duas águas com ângulo $\alpha$, shed, curva) e beirais;
3. **Aba 3 — Aberturas & $C_{pi}$:** Área efetiva de janelas, venezianas e portas, caracterização de aberturas dominantes e cálculo do $C_{pi}$;
4. **Aba 4 — Visualizador 3D Interativo na GPU:** Canvas interativo com órbita, translação e zoom, exibindo o modelo estrutural e vetores orientados de pressão (azul) e sucção (vermelho);
5. **Aba 5 — Memorial de Cálculo:** Exibição do laudo analítico estruturado e exportação formal em HTML5 (pronto para PDF), Markdown e JSON.

---

### 4. Aviso Legal de Estudo Acadêmico, Risco de Erros & Responsabilidade Técnica

O software Eolo Platform é uma ferramenta computacional destinada **exclusivamente a fins de estudos acadêmicos e educacionais**, tomando a ABNT NBR 6123:2023 apenas como referência bibliográfica (disponível em `Ajuda > Sobre o Eolo Platform... > Avisos`, sem reprodução do texto protegido da norma). O aplicativo **pode conter erros e NÃO serve como software definitivo para tomadas de decisão em produção ou ambientes de trabalho**. Todos os resultados devem ser obrigatoriamente conferidos manualmente e/ou através de outras ferramentas mais consolidadas no mercado, e qualquer aplicação prática real **exige validação e recolhimento de Anotação de Responsabilidade Técnica (ART) por Engenheiro legalmente habilitado perante o CREA** (Leis federais nº 5.194/1966 e nº 6.496/1977; Código Civil arts. 186, 393 e 927).

Versão documentada: `2026.11.11.0`

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the engineering manual in English</b></summary><br>

# Eolo Platform — Technical Formulations & Engineering Manual

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Governing Standard:** ABNT NBR 6123:2023 (*Wind forces on buildings*)  
**Correlated Standards:** ABNT NBR 8800:2024 (*Steel Structures*) and ABNT NBR 6118:2023 (*Concrete Structures*)  
**Author & Lead Engineer:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. System Overview

**Eolo Platform** is a professional desktop civil engineering platform developed natively in **Rust (Edition 2021)** featuring an immediate-mode GPU-accelerated GUI in **`egui` / `eframe` / `wgpu`**. The system models deterministic aerodynamic wind loads on rectangular buildings, industrial warehouses, and isolated roofs, producing formal auditable calculation memorials compliant with Brazilian ABNT engineering standards.

---

### 2. Analytical Formulations & Equations under ABNT NBR 6123:2023

#### 2.1. Characteristic Wind Speed ($V_k$)
$$V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3 \quad [m/s]$$
- **$V_0$ (Basic Wind Velocity):** 3-second gust with a 50-year recurrence interval at 10m height above flat open terrain (Annex A / Isopleth Map);
- **$S_1$ (Topographic Factor):**
  - Flat or gently undulating terrain: $S_1 = 1.00$;
  - Steep slopes and escarpments: analytical evaluation based on elevation and slope angle $\theta$;
  - Ridges and elongated hills: continuous interpolation along crest profiles;
- **$S_2$ (Roughness, Building Dimensions, and Height Factor):**
  $$S_2(z) = b \cdot F_r \cdot \left(\frac{z}{10}\right)^p$$
  Evaluated across Terrain Categories I–V and Building Classes A–C, subject to minimum height limits $z_{min}$ from Table 2;
- **$S_3$ (Statistical Occupancy & Risk Factor):**
  Ranges from $0.83$ (temporary agricultural structures) to $1.10$ (vital hospitals and emergency response centers).

#### 2.2. Basic Dynamic Stagnation Pressure ($q$)
$$q = 0.613 \cdot V_k^2 \quad [N/m^2 \text{ or } Pa]$$
Directly convertible to $kN/m^2$ ($1 \, kN/m^2 = 1000 \, Pa$) or $kgf/m^2$ ($1 \, kgf/m^2 \approx 9.80665 \, Pa$).

#### 2.3. Net Resultant Design Pressure ($\Delta p$)
$$\Delta p = (C_{pe} - C_{pi}) \cdot q \quad [N/m^2]$$
- **$C_{pe}$ (External Pressure Coefficient):** Evaluated as a function of geometry and wind attack angle (0° or 90°) for windward, leeward, sidewalls, and roof zones;
- **$C_{pi}$ (Internal Pressure Coefficient):** Calculated from relative envelope permeability or dominant fenestration openings (Clause 6.2.5).

#### 2.4. Global Aerodynamic Reactions & Base Moments
- **Total Drag Force ($F_x, F_y$):**
  $$F = C_a \cdot q \cdot A_e \quad [N \text{ or } kN]$$
  where $C_a$ is the global aerodynamic drag coefficient and $A_e$ is the effective frontal projected area;
- **Base Overturning Moment ($M_t$):**
  $$M_t = \int q(z) \cdot C_{pe}(z) \cdot z \, dA \quad [kN \cdot m]$$

---

### 3. The 5 Sequential Engineering Tabs

1. **Tab 1 — Location & $V_0$:** Brazilian state and FAB meteorological station selection with fine-tuning of $S_1$, $S_2$, and $S_3$;
2. **Tab 2 — Building Geometry:** Plan dimensions ($a \times b$), eaves height ($h$), roof typology (flat, monopitch, gable at angle $\alpha$, sawtooth, curved) and overhangs;
3. **Tab 3 — Openings & $C_{pi}$:** Porosity areas for doors, windows, and louvers, dominant opening analysis, and $C_{pi}$ evaluation;
4. **Tab 4 — Interactive 3D GPU Viewport:** Real-time 3D viewport with orbit, pan, and zoom controls displaying the structural wireframe and directed vectors for overpressure (blue) and suction (red);
5. **Tab 5 — Calculation Memorial:** Full analytical calculation book viewable in real time and exportable to HTML5 (ready for PDF printing), Markdown, and JSON.

---

### 4. Academic Study Purpose, Risk of Errors & Legal Disclaimer

Eolo Platform is a computational tool intended **exclusively for academic study and educational purposes**, referencing ABNT NBR 6123:2023 strictly as a bibliographic source (listed in `Help > About Eolo Platform... > Notices`, without reproducing copyrighted ABNT text). The software **may contain errors and does NOT serve as definitive software for decision-making in production or workplace environments**. All results must be verified manually and/or checked via consolidated market tools, and any real-world structural engineering use **strictly requires independent validation and an official Technical Responsibility Record (ART) by a licensed Engineer registered with CREA** (Brazilian Laws No. 5,194/1966 & No. 6,496/1977; Civil Code Arts. 186, 393 & 927).

Documented version: `2026.11.11.0`

</details>
