# Manual do Usuário / User Guide — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o manual do usuário em português</b></summary><br>

# Eolo Platform — Manual de Operação e Guia do Usuário

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Público-Alvo:** Engenheiros Civis, Engenheiros Estruturais, Engenheiros Mecânicos, Projetistas e Estudantes de Engenharia  
**Norma Base:** ABNT NBR 6123:2023 (*Forças devidas ao vento em edificações*)  
**Autor:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. Introdução e Primeiros Passos

O **Eolo Platform** é um software desktop de engenharia desenvolvido para automatizar, com alto rigor analítico e interface visual imediata acelerada por GPU, a modelagem das ações do vento em edificações e galpões conforme a **ABNT NBR 6123:2023**.

#### 1.1. Requisitos de Sistema
- **Sistema Operacional:** Windows 11 ou Windows 10 (versão 19041 ou superior, 64-bit);
- **Processador:** Arquitetura x86_64 dual-core de 1.6 GHz ou superior;
- **Memória RAM:** Mínimo de 2 GB (recomendado 4 GB);
- **Placa de Vídeo:** Compatível com DirectX 11 / 12, Vulkan ou suporte genérico a WGPU;
- **Armazenamento:** 80 MB de espaço livre em disco;
- **Conectividade:** Nenhuma conexão com a Internet é necessária (o aplicativo opera 100% offline).

#### 1.2. Instalação e Inicialização
- **Versão Oficial Microsoft Store:** Instalação assistida em um clique via Store com atualizações automáticas e sandbox seguro MSIX;
- **Instalador Tradicional (.exe):** Execute o assistente de instalação `EoloPlatform_Setup.exe` gerado pelo Inno Setup, criando atalhos na Área de Trabalho e no Menu Iniciar vinculados ao `AppUserModelID` oficial.

---

### 2. Guia de Operação da Interface Gráfica

A tela principal do Eolo Platform organiza o fluxo de trabalho da engenharia em uma barra de ferramentas superior e **5 abas sequenciais**:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA DE FERRAMENTAS                            │
│  [Novo] [Abrir] [Salvar] | [Cenários] [Calc Rápida] | [PT/EN] [Unidades]│
├────────────────────────────────────────────────────────────────────────┤
│  [1. Localização] [2. Geometria] [3. Aberturas] [4. 3D Canvas] [5. Mem.]│
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│                      ÁREA DE TRABALHO DA ABA ATIVA                     │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

---

#### 2.1. Aba 1 — Localização & Parâmetros Básicos de Vento
Esta aba define as variáveis meteorológicas, geográficas e de vizinhança da edificação:

1. **Seleção de Localização (Isopletas de $V_0$):**
   - Escolha o **Estado (UF)** e selecione a cidade ou estação meteorológica mais próxima no menu suspenso;
   - O campo **Velocidade Básica ($V_0$)** será automaticamente preenchido com o valor oficial do mapa de isopletas da NBR 6123:2023 (em $m/s$). O valor pode ser ajustado manualmente pelo engenheiro se houver medições anemométricas locais com período de retorno superior.
2. **Fator Topográfico ($S_1$):**
   - **Terreno Plano ou Levemente Ondulado:** $S_1 = 1,00$;
   - **Talude ou Encosta:** Insira a inclinação do talude ($\theta$), a cota de base e a altura do topo; o sistema calcula analiticamente o coeficiente $S_1$ na cota da edificação;
   - **Morro ou Crista:** Configuração para morros com perfil tridimensional ou cristas alongadas.
3. **Rugosidade do Terreno e Dimensões da Edificação ($S_2$):**
   - **Categoria de Rugosidade (I a V):** De superfícies lisas com grandes espelhos de água (Cat. I) até centros de grandes metrópoles com edifícios altos contíguos (Cat. V); a maioria das zonas industriais e cidades médias enquadra-se nas Categorias III ou IV;
   - **Classe da Edificação (A, B ou C):** Seleção segundo a maior dimensão da superfície frontal exposta ao vento (Classe A $\le 20m$; Classe B entre $20m$ e $50m$; Classe C $> 50m$).
4. **Fator Estatístico de Ocupação ($S_3$):**
   - Selecione o grupo da edificação (de instalações agrícolas temporárias $S_3 = 0,83$ a edifícios residenciais/comerciais convencionais $S_3 = 1,00$, ou hospitais e centrais de emergência $S_3 = 1,10$).
5. **Resultados em Tempo Real:**
   - O painel lateral atualiza instantaneamente a **Velocidade Característica ($V_k$)** e a **Pressão Dinâmica Básica ($q = 0,613 \cdot V_k^2$)** em $N/m^2$ e $kN/m^2$.

---

#### 2.2. Aba 2 — Geometria & Envoltória da Edificação
Modela a geometria física tridimensional do volume construído:

1. **Dimensões em Planta e Elevação:**
   - **Largura Frontal ($a$):** Dimensão da fachada voltada para a direção de referência (0°);
   - **Profundidade ($b$):** Comprimento longitudinal da edificação (vento a 90°);
   - **Altura da Calha / Pé-direito ($h$):** Altura livre até a linha de apoio da cobertura.
2. **Tipologia de Cobertura:**
   - **Duas Águas (Gable Roof):** Define o ângulo de inclinação $\alpha$ ou a altura da cumeeira;
   - **Uma Água (Single Slope):** Especifica a inclinação contínua em uma única direção;
   - **Cobertura Plana:** Para lajes impermeabilizadas ou platibandas;
   - **Shed Industrial:** Dentes de serra para iluminação e ventilação fabril;
   - **Cobertura Curva (Arco):** Raio de curvatura e flecha central.
3. **Cálculo Automático das Razões de Forma:**
   - O sistema calcula e exibe em tempo real a relação de aspecto $h/b$ e $a/b$, determinando os intervalos de busca nas tabelas de coeficientes de forma da norma.

---

#### 2.3. Aba 3 — Aberturas de Fachada & Permeabilidade ($C_{pi}$)
Configura o comportamento da pressão interna do ar gerada por portas, janelas, venezianas e portões:

1. **Permeabilidade das Paredes:**
   - Informe a área efetiva de aberturas ($m^2$) em cada uma das 4 fachadas (Frontal, Posterior, Lateral Esquerda, Lateral Direita);
2. **Aberturas Dominantes:**
   - Indique se há presença de portões industriais abertos, docas de carga ou quebra potencial de grandes panos de vidro;
   - O motor calcula a relação entre a abertura dominante e a permeabilidade residual das demais paredes (de 1 a 6 vezes), determinando o coeficiente $C_{pi}$ normativo;
3. **Casos de Carregamento Interno:**
   - Avaliação simultânea para sobrepressão interna máxima ($C_{pi} > 0$) e sucção interna máxima ($C_{pi} < 0$), combinando-as com as pressões externas.

---

#### 2.4. Aba 4 — Visualizador 3D Interativo & Diagramas de Pressão
Permite a inspeção visual tridimensional detalhada do modelo estrutural e das forças geradas:

1. **Navegação no Canvas 3D:**
   - **Girar / Orbitar:** Clique e arraste com o botão esquerdo do mouse;
   - **Transladar / Pan:** Clique e arraste com o botão direito do mouse;
   - **Zoom:** Role a roda do mouse (*scroll*).
2. **Alternância de Ângulos de Vento:**
   - Alterne instantaneamente entre **Vento a 0°** (perpendicular à fachada frontal) e **Vento a 90°** (perpendicular às paredes laterais);
3. **Vetores e Mapas de Cores:**
   - Vetores azuis apontando para a superfície indicam **sobrepressão** (vento empurrando);
   - Vetores vermelhos apontando para fora da superfície indicam **sucção** (vento puxando/arrancando);
4. **Painel de Esforços Globais:**
   - Apresenta a força horizontal total de arrasto ($F_x$ ou $F_y$), a força de levantamento total no telhado e o momento de tombamento basal na fundação.

---

#### 2.5. Aba 5 — Memorial de Cálculo & Exportação Técnica
Gera o documento técnico auditável pronto para anexação a projetos de prefeitura, laudos periciais e memoriais descritivos:

1. **Exibição do Memorial Completo:**
   - Apresenta o memorial estruturado contendo introdução normativa, dados do projeto, fórmulas matemáticas aplicadas com valores substituídos, tabelas de coeficientes $C_{pe}$ e $C_{pi}$, pressões líquidas de projeto e nota formal de responsabilidade técnica (ART/CREA);
2. **Opções de Exportação:**
   - **Salvar em HTML:** Arquivo visualmente estilizado, ideal para impressão direta em papel ou geração de PDF através da função "Imprimir" do navegador;
   - **Salvar em Markdown (.md):** Arquivo leve e compatível com repositórios e documentações Git;
   - **Salvar em JSON (.json):** Estrutura numérica completa de nós, pressões e esforços para importação em softwares de cálculo estrutural em concreto e aço.

---

### 3. Cenários de Engenharia Pré-Configurados (Presets)

Para agilizar o dimensionamento de obras recorrentes, utilize o menu `Arquivo` -> `Carregar Cenário Predefinido`:
1. **Galpão Industrial Padrão:** Galpão de $20m \times 40m$, pé-direito de $7,5m$, telhado de duas águas a 15°, Categoria de Rugosidade IV, Classe B;
2. **Edifício Residencial Urbano:** Torre retangular de $15m \times 25m$ com $30m$ de altura em centro urbano denso;
3. **Cobertura Shed Industrial:** Pavilhão de $18m \times 36m$ com telhado em dentes de serra para ventilação natural;
4. **Silo Graneleiro / Estrutura Cilíndrica:** Estrutura cilíndrica vertical com diâmetro de $12m$ e altura de $24m$;
5. **Residência Unifamiliar Litorânea:** Edificação residencial térrea de $10m \times 14m$ em região de costa marítima sujeita a ventos de alta intensidade.

---

### 4. Diálogos Auxiliares e Ferramentas

- **Calculadora Rápida de Rajada:** Permite calcular rapidamente a velocidade e a pressão do vento para uma altura específica sem alterar os parâmetros do projeto aberto;
- **Conversor de Unidades de Engenharia:** Converte instantaneamente entre $m/s, km/h, N/m^2, kN/m^2, kgf/m^2$ e $psi$;
- **Manual Interativo (`F1` / `Ctrl+M`):** Guia didático de operação e estudo com pesquisa destacada em Português e Inglês;
- **Rastreabilidade Acadêmica:** Lista as referências da ABNT NBR 6123:2023 implementadas no estudo;
- **Sobre o Eolo Platform (`Ctrl+I`):** Exibe documentação jurídica, licenças e, na aba **Avisos**, as **referências bibliográficas das normas ABNT utilizadas no projeto** (sem reprodução de conteúdo normativo protegido pela ABNT).

---

### 5. Perguntas Frequentes (FAQ)

**P: O Eolo Platform necessita de conexão com a Internet para validar licença?**  
R: Não. O aplicativo é 100% *Local-First* e opera de forma totalmente autônoma e offline.

**P: O aplicativo pode ser utilizado para projetos definitivos em produção sem conferência manual?**  
R: De forma alguma. O Eolo Platform destina-se **exclusivamente ao estudo acadêmico e educacional**, pode conter erros e **não serve como software definitivo para tomadas de decisão em produção ou ambientes de trabalho**. Todos os resultados devem ser obrigatoriamente conferidos manualmente (com exemplar oficial das normas ABNT) ou em ferramentas consolidadas de mercado, sob responsabilidade e ART de Engenheiro habilitado perante o CREA.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the user guide in English</b></summary><br>

# Eolo Platform — Operating Manual & User Guide

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Target Audience:** Civil Engineers, Structural Engineers, Mechanical Engineers, Drafters, and Engineering Students  
**Governing Standard:** ABNT NBR 6123:2023 (*Wind forces on buildings*)  
**Author:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. Introduction & Getting Started

**Eolo Platform** is a specialized civil engineering desktop application created to automate, with rigorous mathematical fidelity and immediate GPU-accelerated graphics, the aerodynamic modeling of wind loads on buildings and industrial shelters under Brazilian standard **ABNT NBR 6123:2023**.

#### 1.1. System Requirements
- **Operating System:** Windows 11 or Windows 10 (version 19041 or higher, 64-bit);
- **Processor:** x86_64 dual-core architecture running at 1.6 GHz or faster;
- **RAM:** Minimum 2 GB (4 GB recommended);
- **Graphics Card:** Compatible with DirectX 11 / 12, Vulkan, or generic WGPU support;
- **Storage:** 80 MB free disk space;
- **Network Connectivity:** None required (the application is 100% offline).

#### 1.2. Installation & Launch
- **Official Microsoft Store Edition:** Clean one-click installation through the Microsoft Store featuring automated background updates and secure MSIX sandboxing;
- **Traditional Installer (.exe):** Run the `EoloPlatform_Setup.exe` setup wizard created with Inno Setup, generating official Desktop and Start Menu shortcuts registered with the application's official `AppUserModelID`.

---

### 2. Graphical Interface Operation Guide

The main window of Eolo Platform organizes the structural wind design process through a top toolbar and **five sequential workspace tabs**:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                              TOP TOOLBAR                               │
│  [New] [Open] [Save] | [Presets] [Quick Calc] | [Language] [Units]     │
├────────────────────────────────────────────────────────────────────────┤
│  [1. Location] [2. Geometry] [3. Openings] [4. 3D Canvas] [5. Memorial]│
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│                      ACTIVE TAB WORKSPACE AREA                         │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

---

#### 2.1. Tab 1 — Location & Basic Wind Parameters
Configures regional meteorological data, terrain topography, and building surroundings:

1. **Location Selection (Isopleth Mapping of $V_0$):**
   - Select the Brazilian **State (UF)** and choose the closest municipality or meteorological station from the dropdown list;
   - The **Basic Wind Velocity ($V_0$)** field is automatically populated with the official normative isopleth value from NBR 6123:2023 (in $m/s$). This value can be manually refined if local site anemometric data is available.
2. **Topographic Factor ($S_1$):**
   - **Flat or Gently Undulating Ground:** $S_1 = 1.00$;
   - **Slope or Escarpment:** Enter the inclination angle ($\theta$), base elevation, and crest elevation; the engine calculates the analytical $S_1$ factor at building height;
   - **Hills and Ridges:** Configures symmetric or elongated topographic elevations.
3. **Terrain Roughness & Building Dimensions ($S_2$):**
   - **Roughness Category (I to V):** Spans smooth water surfaces (Category I) to city centers with tall buildings (Category V); typical industrial parks and suburban areas correspond to Categories III or IV;
   - **Building Class (A, B, or C):** Classified by the largest horizontal or vertical dimension of the windward face (Class A $\le 20m$; Class B between $20m$ and $50m$; Class C $> 50m$).
4. **Statistical Occupancy Factor ($S_3$):**
   - Select the building occupancy group (ranging from temporary agricultural facilities $S_3 = 0.83$ to standard residential/commercial buildings $S_3 = 1.00$, or hospitals and emergency centers $S_3 = 1.10$).
5. **Real-Time Analytical Feedback:**
   - The side inspector instantaneously displays the **Characteristic Wind Speed ($V_k$)** and **Basic Dynamic Pressure ($q = 0.613 \cdot V_k^2$)** in $N/m^2$ and $kN/m^2$.

---

#### 2.2. Tab 2 — Building Geometry & Envelope
Defines the three-dimensional volumetric geometry of the structure:

1. **Plan & Elevation Dimensions:**
   - **Frontal Width ($a$):** Dimension of the facade facing the 0° wind direction;
   - **Longitudinal Depth ($b$):** Dimension along the 90° wind direction;
   - **Eaves Height ($h$):** Clear vertical height to the roof line or parapet.
2. **Roof Typology:**
   - **Gable Roof (Double-pitch):** Sets the roof pitch angle $\alpha$ or ridge height;
   - **Single-Pitch (Monopitch):** Defines a continuous slope in one direction;
   - **Flat Roof:** For waterproofed concrete slabs or parapeted roofs;
   - **Sawtooth (Shed):** Multi-pitch industrial shed geometry;
   - **Curved Roof:** Central rise and radius of curvature.
3. **Aspect Ratios:**
   - The engine automatically computes $h/b$ and $a/b$ ratios, driving continuous interpolation across normative external pressure coefficient tables.

---

#### 2.3. Tab 3 — Wall Openings & Internal Permeability ($C_{pi}$)
Models internal building air pressures generated by doors, windows, louvers, and overhead gates:

1. **Facade Porosity:**
   - Enter the total opening area ($m^2$) across each of the 4 walls (Front, Rear, Left, Right);
2. **Dominant Openings:**
   - Flag whether large cargo doors or storm-susceptible glass facades create dominant openings;
   - The calculation engine determines the dominant-to-residual opening ratio (from 1 to 6 times), assigning the normative $C_{pi}$ coefficient;
3. **Load Combinations:**
   - Evaluates simultaneous cases for peak internal overpressure ($C_{pi} > 0$) and peak internal suction ($C_{pi} < 0$) combined with exterior pressures.

---

#### 2.4. Tab 4 — Interactive 3D Canvas & Pressure Diagrams
Provides an interactive 3D inspection environment for envelope forces and aerodynamic vectors:

1. **3D Canvas Navigation:**
   - **Rotate / Orbit:** Click and drag with the left mouse button;
   - **Pan:** Click and drag with the right mouse button;
   - **Zoom:** Roll the mouse scroll wheel.
2. **Wind Angle Toggle:**
   - Switch instantaneously between **Wind at 0°** (normal to frontal facade) and **Wind at 90°** (normal to sidewalls);
3. **Vector Visualization:**
   - Blue arrows pointing toward the surface represent **overpressure** (inward force);
   - Red arrows pointing away from the surface represent **suction** (uplift / outward force);
4. **Global Reactions Panel:**
   - Displays total horizontal base shear forces ($F_x, F_y$), total roof uplift, and overturning base moments.

---

#### 2.5. Tab 5 — Calculation Memorial & Technical Export
Generates auditable calculation documents ready for municipal approval, peer review, and structural calculation books:

1. **Interactive Memorial View:**
   - Displays the formatted calculation book with mathematical equations, substituted project values, $C_{pe}$ and $C_{pi}$ tables, net design pressures, and formal engineering responsibility notices (CREA / ART);
2. **Export Actions:**
   - **Export to HTML:** Fully styled report, optimized for clean printing to PDF via any web browser;
   - **Export to Markdown (.md):** Lightweight format suitable for Git repositories;
   - **Export to JSON (.json):** Machine-readable structured dataset enabling integration with downstream steel and concrete design packages.

---

### 3. Pre-Configured Engineering Scenarios (Presets)

To accelerate common structural verifications, access `File` -> `Load Preset Scenario`:
1. **Standard Industrial Warehouse:** $20m \times 40m$ warehouse, eaves height $7.5m$, gable roof at 15°, Roughness Category IV, Class B;
2. **Multi-Story Residential Building:** $15m \times 25m$ tower, $30m$ height in an urban city center;
3. **Sawtooth Industrial Canopy:** $18m \times 36m$ factory shed with repeating sawtooth roof;
4. **Grain Storage Silo / Cylindrical Structure:** Vertical cylindrical container with a $12m$ diameter and $24m$ height;
5. **Coastal Single-Family Residence:** Single-story $10m \times 14m$ home situated along a high-velocity coastal wind belt.

---

### 4. Utility Dialogs & Tools

- **Quick Gust Calculator:** Compute wind speed and stagnation pressure at any specific elevation without modifying the active project;
- **Engineering Unit Converter:** Real-time conversion across $m/s, km/h, N/m^2, kN/m^2, kgf/m^2$, and $psi$;
- **Interactive Study Manual (`F1` / `Ctrl+M`):** Bilingual didactic guide covering application workflows and study equations with highlighted keyword search;
- **Academic Traceability Matrix:** Lists reference clauses from ABNT NBR 6123:2023 implemented for study purposes;
- **About Eolo Platform (`Ctrl+I`):** Displays legal notices, licenses, and, under the **Notices** tab, the **official bibliographic references of the ABNT standards used in the project** (with no reproduction of copyrighted ABNT text).

---

### 5. Frequently Asked Questions (FAQ)

**Q: Does Eolo Platform require an internet connection for license checks?**  
A: No. The application is strictly 100% Local-First and operates fully offline without any remote license pings.

**Q: Can the application be used directly for definitive production engineering decisions without manual checking?**  
A: Absolutely not. Eolo Platform is intended **exclusively for academic study and educational purposes**, may contain errors, and **does NOT serve as definitive software for decision-making in production or workplace environments**. Every result must be manually checked (against officially purchased ABNT standards) or verified with consolidated market software under the supervision and official ART registration of a licensed Engineer registered with CREA.

</details>
