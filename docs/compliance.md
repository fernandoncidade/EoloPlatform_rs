# Conformidade Legal e Normativa / Legal & Normative Compliance — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o documento de conformidade em português</b></summary><br>

# Eolo Platform — Mapeamento de Conformidade Legal, Normativa e Regulatória

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Produto:** Eolo Platform (`eolo.exe`, `eolo-cli.exe`)  
**Autor e Detentor dos Direitos:** Fernando Nillsson Cidade (Engenheiro Civil / Desenvolvedor)  
**Contato Institucional:** `linceu_lighthouse@outlook.com`  
**Repositório Oficial:** `https://github.com/fernandoncidade`  

---

### 1. Visão Geral de Conformidade

O **Eolo Platform** é uma plataforma de engenharia estrutural de alta precisão desenvolvida em Rust (Edição 2021) com interface gráfica imediata acelerada por hardware em `egui`/`eframe`/`wgpu` e interface de linha de comando (`eolo-cli`). A ferramenta foi projetada para realizar a modelagem matemática determinística das ações e pressões causadas pelo vento em edificações e estruturas civis.

Este documento formaliza o enquadramento de conformidade do software perante:
1. As normas técnicas de engenharia civil da Associação Brasileira de Normas Técnicas (ABNT), em especial a **ABNT NBR 6123:2023**;
2. A legislação brasileira de proteção ao software (**Lei nº 9.609/1998**);
3. O Código de Proteção e Defesa do Consumidor (**Lei nº 8.078/1990 — CDC**);
4. A Lei Geral de Proteção de Dados Pessoais (**Lei nº 13.709/2018 — LGPD**) e o Marco Civil da Internet (**Lei nº 12.965/2014**);
5. O arcabouço profissional da Engenharia e exigências de ART (**Lei nº 5.194/1966** e **Lei nº 6.496/1977** — CONFEA/CREA);
6. As políticas comerciais e técnicas da **Microsoft Store** (incluindo a Política de Privacidade 10.5.1 e diretrizes MSIX).

---

### 2. Conformidade com Normas Técnicas Brasileiras (ABNT)

#### 2.1. ABNT NBR 6123:2023 — Forças Devidas ao Vento em Edificações
O núcleo de cálculo (`src/engine/`) do Eolo Platform implementa com fidelidade matemática e rastreabilidade algorítmica integral as formulações prescritas na edição vigente da **ABNT NBR 6123:2023**:

- **Velocidade Básica do Vento ($V_0$ — Item 5.1 e Anexo A):**
  - Mapeamento das velocidades básicas a partir do mapa oficial de isopletas do território nacional para rajadas de 3 segundos com período de recorrência de 50 anos a 10 metros de altura em terreno plano aberto.
  - Base de dados estruturada com as coordenadas geográficas, altitudes e velocidades normativas de todas as estações meteorológicas e capitais brasileiras (`src/engine/isopleth_map.rs`).
- **Fator Topográfico ($S_1$ — Item 5.2):**
  - Implementação exata das condições de terreno plano ou fracamente ondulado ($S_1 = 1,00$);
  - Taludes e encostas íngremes com cálculo analítico em função da cota local e do ângulo de inclinação $\theta$ ($S_1$ variável até $1,0 + (S_1' - 1) \cdot (1 - z/d)$);
  - Morros e cômoros com perfil bidimensional simétrico e assimétrico.
- **Fator de Rugosidade, Dimensões da Edificação e Altura ($S_2$ — Item 5.3):**
  - Categorias de rugosidade de terreno de I a V (de superfícies lisas com água limpa até centros de grandes metrópoles com obstáculos densos e altos);
  - Classes de edificação A, B e C segundo a maior dimensão horizontal ou vertical da superfície exposta;
  - Cálculo analítico de $S_2(z) = b \cdot F_r \cdot (z / 10)^p$ com limitação inferior de altura mínima ($z_{min}$) e parâmetros normativos tabelados de $b$, $F_r$ e $p$ conforme a Tabela 2 da norma.
- **Fator Estatístico ($S_3$ — Item 5.4):**
  - Fator estatístico de segurança e probabilidade de superação em função do tipo de edificação e do nível de ocupação humana (Grupos 1 a 5, cobrindo de instalações agrícolas provisórias $S_3 = 0,83$ a hospitais, centrais de comunicação e centrais de socorro $S_3 = 1,10$).
- **Velocidade Característica do Vento ($V_k$ — Item 5.5):**
  - $V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3$ em metros por segundo ($m/s$).
- **Pressão Dinâmica ($q$ — Item 5.6):**
  - $q = 0,613 \cdot V_k^2$ em Newtons por metro quadrado ($N/m^2$ ou $Pa$), convertível para $kN/m^2$ ou $kgf/m^2$.
- **Coeficientes de Pressão Externa ($C_{pe}$) e Coeficientes de Forma ($C_e$ — Itens 6 e 7):**
  - Paredes de edificações paralelepipédicas retangulares para vento a 0° e 90° (superfícies de barlavento, sotavento e paredes laterais segundo a relação $h/b$ e $a/b$ da Tabela 4);
  - Coberturas de uma, duas ou múltiplas águas com determinação contínua de sucção e sobrepressão em função da inclinação $\alpha$ do telhado (Tabelas 5 e 6);
  - Coberturas curvas, sheds e coberturas isoladas (marquises, hangares abertos);
  - Forças de arrasto global ($C_a$), força resultante de tombamento e excentricidade da carga horizontal.
- **Coeficientes de Pressão Interna ($C_{pi}$ — Item 6.2.5):**
  - Edificações com permeabilidade distribuída nas quatro paredes;
  - Edificações com aberturas dominantes em uma ou duas faces opostas (relação de área de abertura dominante sobre a área total de permeabilidade residual de 1 a 6 vezes);
  - Edificações hermeticamente estanques com avaliação de abertura acidental por quebra de esquadrias durante vendavais.

#### 2.2. Interligação com ABNT NBR 8800:2024 e ABNT NBR 6118:2023
Os valores de cargas e esforços característicos calculados pelo Eolo Platform são exportados em formato estruturado (JSON, Markdown e HTML) para compor as ações variáveis diretas das combinações de ações no Estado Limite Último (ELU) e Estado Limite de Serviço (ELS) estipuladas na **ABNT NBR 8800:2024** (*Projeto de estruturas de aço e mistas*) e na **ABNT NBR 6118:2023** (*Projeto de estruturas de concreto*).

---

### 3. Conformidade com a Legislação Brasileira de Software (Lei nº 9.609/1998)

O regime jurídico aplicável à comercialização, proteção e suporte técnico do Eolo Platform observa estritamente as regras federais da Lei nº 9.609/1998:

- **Art. 2º (Regime de Proteção dos Direitos do Autor):**
  - O código-fonte, arquitetura lógica, diagramas, textos normativos e telas do programa são obras intelectuais de engenharia protegidas pelo direito autoral. O código-fonte aberto no GitHub é regido pela licença GNU AGPLv3, preservando a autoria originária de Fernando Nillsson Cidade.
- **Art. 7º (Prazo de Validade Técnica das Versões Comercializadas):**
  - Nos termos do Art. 7º da Lei nº 9.609/1998, o fornecedor declara formalmente que a versão comercial **`2026.11.8.0`** possui **prazo de validade técnica garantido de 12 (doze) meses**, contados a partir da data de seu lançamento oficial (8 de novembro de 2026), ou até a publicação de nova versão estável subsequente.
- **Art. 8º (Prestação de Serviços Técnicos Complementares no Brasil):**
  - Conforme preconizado pelo Art. 8º da Lei nº 9.609/1998, durante o prazo de validade técnica, o desenvolvedor assegura a disponibilização, em território nacional, de serviços técnicos complementares consistentes em:
    1. Canal formal de suporte técnico por correio eletrônico (`linceu_lighthouse@outlook.com`) para esclarecimento de dúvidas de instalação e operação;
    2. Correção de eventuais inconformidades algorítmicas, falhas de execução ou divergências aritméticas detectadas frente às formulações da NBR 6123:2023.

---

### 4. Conformidade com o Código de Defesa do Consumidor (Lei nº 8.078/1990 — CDC)

A disponibilização do software em canal de distribuição pago (como a Microsoft Store) estabelece relação de fornecimento de produto digital para destinatários finais, submetendo-se aos postulados protetivos de ordem pública do CDC:

- **Arts. 24 e 25 (Garantia Legal e Inadmissibilidade de Cláusula de Exoneração):**
  - É legalmente nula de pleno direito qualquer cláusula que exonere, atenue ou afaste a responsabilidade do fornecedor pela garantia legal de adequação e funcionamento do software perante o consumidor. Quaisquer termos de licença comercial (EULA) contêm ressalva expressa de salvaguarda dos direitos irrenunciáveis previstos no CDC.
- **Art. 49 (Direito de Arrependimento em Contratações Digitais):**
  - Ao adquirir o software através de contratação digital fora do estabelecimento comercial (loja online de aplicativos), o consumidor tem o direito potestativo de desistir da contratação no prazo de até 7 (sete) dias corridos a contar da data da compra ou da liberação do download, com devolução integral dos valores pagos, operada diretamente através da plataforma da Microsoft Store.
- **Art. 51, Inciso I (Nulidade de Disposições Abusivas):**
  - Nenhuma disposição do EULA ou do CLC limita a indenização devida ao consumidor em caso de dolo ou violação deliberada de direitos por parte do fornecedor, respeitando a eficácia plena das garantias legais.
- **Art. 101, Inciso I (Competência do Foro do Consumidor):**
  - Em ações judiciais relativas à responsabilidade civil do fornecedor propostas por consumidores no Brasil, prevalece o foro do domicílio do consumidor.

---

### 5. Conformidade com Privacidade e Proteção de Dados (LGPD, Marco Civil e GDPR)

O Eolo Platform adota a filosofia de engenharia **Sovereign Privacy (Privacidade Soberana) e Local-First**, assegurando cumprimento nativo da **Lei nº 13.709/2018 (LGPD)**, do **Marco Civil da Internet (Lei nº 12.965/2014)** e do **Regulamento Europeu (GDPR)**:

- **100% Offline e Execução Estritamente Local:**
  - O aplicativo não efetua nenhuma requisição de rede externa (*inbound* ou *outbound*). Não há envio de telemetria, medição de audiência, identificadores de publicidade, relatórios automáticos de telemetria de erros ou comunicação com servidores de inteligência artificial na nuvem.
- **Não Coleta de Dados Pessoais (LGPD Art. 6º — Necessidade e Finalidade):**
  - Todos os arquivos de projetos de cálculo (`.json`), memórias descritivas (`.html`, `.md`) e bancos de dados locais de parâmetros de vento residem exclusivamente no armazenamento local da máquina do engenheiro (`%LOCALAPPDATA%\EoloPlatform` ou pasta escolhida pelo usuário).
  - O software não tem acesso a dados pessoais de terceiros, não realiza profiling nem transfere dados para fora do país.
- **Segurança da Estação de Trabalho (LGPD Art. 46):**
  - A segurança dos arquivos de projeto compete ao controle de acesso físico e lógico da estação de trabalho gerenciada pelo próprio usuário ou por sua organização.

---

### 6. Responsabilidade Profissional da Engenharia (Leis nº 5.194/1966 e nº 6.496/1977)

A utilização de ferramentas computacionais na Engenharia Civil e Mecânica é disciplinada pelo Conselho Federal de Engenharia e Agronomia (CONFEA) e pelos Conselhos Regionais (CREA):

- **Caráter Instrumental e Auxiliar do Software:**
  - O **Eolo Platform** é uma ferramenta tecnológica de modelagem matemática, automação analítica e geração de relatórios de cálculo estrutural. Ele tem por finalidade otimizar o tempo de elaboração de projetos e reduzir a propensão a erros manuais na aplicação das tabelas e interpolações da ABNT NBR 6123:2023.
- **Indispensabilidade da Responsabilidade Técnica e ART:**
  - O software **não substitui, em nenhuma hipótese, o julgamento técnico, a análise crítica, a validação de condições de contorno e a responsabilidade profissional indelegável do Engenheiro Civil ou Mecânico legalmente habilitado**.
  - Todo projeto, obra, estrutura de suporte, edificação ou laudo pericial calculado com auxílio do Eolo Platform deve obrigatoriamente contar com o recolhimento da respectiva **Anotação de Responsabilidade Técnica (ART)** junto ao CREA competente, sob responsabilidade do profissional responsável técnico pela obra.
  - O desenvolvedor do software não assume responsabilidade técnica pelas obras físicas, detalhamentos executivos, acidentes estruturais ou patologias decorrentes de dados de entrada incorretos, simplificações indevidas de geometria ou interpretação equivocada de resultados pelo usuário.

---

### 7. Conformidade com as Diretrizes da Microsoft Store e Formato MSIX

Para distribuição e publicação oficial na Microsoft Store, o Eolo Platform atende a todas as exigências estipuladas no **Microsoft Store App Submission Guide**:

- **Política de Privacidade 10.5.1:**
  - A política de privacidade do aplicativo é pública, permanente e acessível via HTTPS em `https://github.com/fernandoncidade/EoloPlatform_rs/blob/main/docs/security-model.md` e espelhada no arquivo `src/assets/PRIVACY_POLICY/Privacy_Policy_pt_BR.txt`.
  - O aplicativo exibe internamente o texto integral da política de privacidade por meio da caixa de diálogo "Sobre o Eolo Platform" -> Aba "Privacidade".
- **Isolamento de Execução e Permissões Declaradas:**
  - O pacote MSIX declara apenas a capacidade necessária para execução de aplicativo desktop nativo de alta performance (`runFullTrust`), sem solicitar acesso indevido à câmera, microfone, geolocalização por hardware ou catálogo de contatos.
- **AppUserModelID e Integração com o Windows Shell:**
  - O executável registra seu AppUserModelID padronizado (`FernandoCidade.EoloPlatform.NBR6123.2026`), garantindo agrupamento correto na barra de tarefas do Windows 11, persistência de atalhos e exibição do ícone oficial em todas as resoluções de tela.
- **Windows App Certification Kit (WACK):**
  - O pacote binário é submetido aos testes automatizados de segurança, estabilidade de memória, integridade de recursos e assinatura digital Authenticode.

---

### 8. Direitos Autorais da ABNT, Referências Bibliográficas e Estudo Acadêmico

- **Associação Brasileira de Normas Técnicas (ABNT):**
  - As normas técnicas **ABNT NBR 6123:2023**, **ABNT NBR 8800:2024** e **ABNT NBR 6118:2023** são obras intelectuais protegidas por direitos autorais de titularidade exclusiva da Associação Brasileira de Normas Técnicas (*Todos os direitos reservados. Nenhuma parte destas publicações pode ser reproduzida ou utilizada por qualquer meio sem permissão por escrito da ABNT*).
  - Nenhuma norma técnica da ABNT é reproduzida no software; disponibilizam-se exclusivamente suas referências bibliográficas oficiais no menu `Ajuda > Sobre o Eolo Platform... > Aba Avisos` (`src/assets/NOTICES/`), nos termos do art. 46, III e VIII da Lei nº 9.610/1998 e art. 10 da Convenção de Berna.
  - O **Eolo Platform** e seu autor não possuem nenhuma espécie de afiliação, patrocínio, chancela, homologação ou parceria formal com a ABNT.
- **Componentes e Bibliotecas de Terceiros (Open-Source):**
  - Todos os crates e bibliotecas de código aberto incorporados na compilação do software (como `egui`, `eframe`, `wgpu`, `rusqlite`, `serde`, fontes `Ubuntu` e `Noto Sans`) são devidamente atribuídos em `src/assets/NOTICES/NOTICE_pt_BR.txt` e acompanhados de seus respectivos textos de licença em `src/assets/LICENSES/`.

---

### 9. Natureza Exclusivamente Acadêmica, Risco de Erros e Isenção de Responsabilidade em Produção

Nos termos do Código Civil Brasileiro (arts. 186, 393, 421, 421-A, 422, 927 e 945), do CDC (arts. 6º, III, 12 §3º III, 14 §3º II e 31, preservados os direitos legais irrenunciáveis dos arts. 24, 25, 49 e 51, I e da Lei nº 9.609/1998, arts. 7º e 8º) e das convenções internacionais ("AS IS" / "WITH ALL FAULTS", UCC § 2-316 e Diretiva UE 2019/770):
1. O software é fornecido "no estado em que se encontra" (*as is*) **exclusivamente para fins de estudos acadêmicos e educacionais**, podendo conter erros de programação, imprecisões ou simplificações;
2. O aplicativo **não serve como software definitivo para tomadas de decisão em produção, obras civis ou ambientes de trabalho**;
3. É obrigação exclusiva do usuário conferir manualmente todos os resultados (em exemplar oficial da norma) ou validá-los através de ferramentas consolidadas no mercado, sob supervisão e ART de engenheiro habilitado (Leis nº 5.194/1966 e nº 6.496/1977), eximindo-se o autor de responsabilidade jurídica pelo uso real e definitivo em produção.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the compliance document in English</b></summary><br>

# Eolo Platform — Legal, Normative, and Regulatory Compliance Mapping

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Product:** Eolo Platform (`eolo.exe`, `eolo-cli.exe`)  
**Author & Copyright Holder:** Fernando Nillsson Cidade (Civil Engineer / Software Developer)  
**Institutional Contact:** `linceu_lighthouse@outlook.com`  
**Official Repository:** `https://github.com/fernandoncidade`  

---

### 1. Compliance Overview

**Eolo Platform** is a high-precision structural engineering desktop application written in pure Rust (Edition 2021) featuring a hardware-accelerated immediate-mode GUI powered by `egui`/`eframe`/`wgpu`, complemented by a headless command-line interface (`eolo-cli`). The application is engineered to perform deterministic mathematical modeling of wind actions and pressures on buildings and civil engineering structures.

This document establishes the official regulatory compliance framework of the software regarding:
1. Civil engineering technical standards published by the Brazilian Association of Technical Standards (ABNT), with primary focus on **ABNT NBR 6123:2023**;
2. Brazilian Software Protection Law (**Federal Law No. 9,609/1998**);
3. Brazilian Consumer Defense Code (**Federal Law No. 8,078/1990 — CDC**);
4. Brazilian General Data Protection Law (**Federal Law No. 13,709/2018 — LGPD**) and the Brazilian Civil Rights Framework for the Internet (**Federal Law No. 12,965/2014**);
5. Legal frameworks governing professional engineering practice and mandatory Technical Responsibility Records (**Federal Law No. 5,194/1966** and **Federal Law No. 6,496/1977** — CONFEA/CREA);
6. Commercial, privacy, and technical policies of the **Microsoft Store** (including Privacy Policy 10.5.1 and MSIX packaging standards).

---

### 2. Brazilian Technical Standards Compliance (ABNT)

#### 2.1. ABNT NBR 6123:2023 — Wind Forces on Buildings
The core computation engine (`src/engine/`) of Eolo Platform strictly complies with the mathematical formulations and tables established by the current revision of **ABNT NBR 6123:2023**:

- **Basic Wind Velocity ($V_0$ — Clause 5.1 & Annex A):**
  - Continuous mapping of basic wind velocity from the official Brazilian isopleth map for 3-second gusts with a 50-year recurrence interval at 10 meters above flat open terrain.
  - High-precision geographic lookup incorporating coordinates, elevations, and normative velocities for all official FAB meteorological stations and Brazilian state capitals (`src/engine/isopleth_map.rs`).
- **Topographic Factor ($S_1$ — Clause 5.2):**
  - Precise implementation for flat or gently undulating terrain ($S_1 = 1.00$);
  - Slopes and escarpments with analytical calculation based on local elevation and slope angle $\theta$ ($S_1$ varying up to $1.0 + (S_1' - 1) \cdot (1 - z/d)$);
  - Symmetric and asymmetric ridges and hills.
- **Roughness, Dimensions, and Height Factor ($S_2$ — Clause 5.3):**
  - Five terrain roughness categories (I to V, spanning open water surfaces to dense metropolises with tall buildings);
  - Three building size classes (A, B, C based on the maximum horizontal or vertical dimension of the exposed surface);
  - Analytical computation of $S_2(z) = b \cdot F_r \cdot (z / 10)^p$ with normative lower-height thresholding ($z_{min}$) and official parameters $b$, $F_r$, and $p$ from Table 2.
- **Statistical Factor ($S_3$ — Clause 5.4):**
  - Probability-based statistical factor categorized by building occupancy, criticality, and design lifespan (Groups 1 to 5, ranging from temporary agricultural shelters $S_3 = 0.83$ to vital hospitals, communication facilities, and emergency shelters $S_3 = 1.10$).
- **Characteristic Wind Speed ($V_k$ — Clause 5.5):**
  - $V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3$ in meters per second ($m/s$).
- **Dynamic Wind Pressure ($q$ — Clause 5.6):**
  - $q = 0.613 \cdot V_k^2$ in Newtons per square meter ($N/m^2$ or $Pa$), convertible to $kN/m^2$ or $kgf/m^2$.
- **External Pressure Coefficients ($C_{pe}$) and Shape Factors ($C_e$ — Clauses 6 & 7):**
  - Rectangular parallelepiped walls subjected to wind at 0° and 90° (windward, leeward, and sidewall surfaces based on aspect ratios $h/b$ and $a/b$ from Table 4);
  - Single-pitch, gable (double-pitch), and multi-pitch roofs with continuous pressure and suction determination across varying roof pitches $\alpha$ (Tables 5 and 6);
  - Curved roofs, sawtooth (shed) industrial roofs, and isolated open canopies;
  - Global aerodynamic drag forces ($C_a$), base shear, overturning moment, and load eccentricity.
- **Internal Pressure Coefficients ($C_{pi}$ — Clause 6.2.5):**
  - Permeable buildings with distributed envelope porosity across four walls;
  - Envelopes with dominant openings on one or two opposite facades (dominant opening ratio relative to total residual area between 1 and 6);
  - Impermeable enclosures evaluating accidental fenestration breakage during severe storm events.

#### 2.2. Interoperability with ABNT NBR 8800:2024 and ABNT NBR 6118:2023
Calculated characteristic wind forces are structured and exported in standardized formats (JSON, Markdown, and HTML) ready to be integrated into Ultimate Limit State (ULS) and Serviceability Limit State (SLS) load combination models as prescribed by **ABNT NBR 8800:2024** (*Design of steel and composite structures*) and **ABNT NBR 6118:2023** (*Design of concrete structures*).

---

### 3. Compliance with Brazilian Software Law (Federal Law No. 9,609/1998)

The commercialization, intellectual property protection, and technical maintenance of Eolo Platform strictly follow Federal Law No. 9,609/1998:

- **Article 2 (Copyright Protection Regime):**
  - The source code, analytical algorithms, diagrams, normative databases, and user interface are protected intellectual property. The open-source code on GitHub is licensed under the GNU AGPLv3, preserving original copyright authorship by Fernando Nillsson Cidade.
- **Article 7 (Guaranteed Technical Validity Period):**
  - Pursuant to Article 7 of Federal Law No. 9,609/1998, the developer formally states that commercial release **`2026.11.8.0`** carries a **guaranteed technical validity period of 12 (twelve) months**, commencing on its official release date (November 8, 2026), or until superseded by a subsequent stable release.
- **Article 8 (Mandatory Complementary Technical Services in Brazil):**
  - As required by Article 8 of Federal Law No. 9,609/1998, throughout the technical validity term, the vendor ensures the provision of complementary technical services within Brazilian territory consisting of:
    1. Dedicated e-mail technical support (`linceu_lighthouse@outlook.com`) for installation and operating guidance;
    2. Prompt corrective releases addressing confirmed algorithmic inaccuracies, runtime defects, or discrepancies against NBR 6123:2023 formulations.

---

### 4. Compliance with Brazilian Consumer Defense Code (Federal Law No. 8,078/1990 — CDC)

Distribution through paid commercial channels (such as the Microsoft Store) constitutes a digital product transaction with final consumers, governed by non-waivable public policy provisions of the CDC:

- **Articles 24 and 25 (Statutory Warranty and Prohibition of Disclaimers):**
  - Any contractual provision attempting to waive, attenuate, or disclaim statutory warranty protections for product fitness and functionality in consumer relationships is null and void under Brazilian law. All commercial End User License Agreements (EULA) include an express saving clause preserving CDC consumer rights.
- **Article 49 (Seven-Day Right of Withdrawal for Digital Purchases):**
  - For digital purchases executed remotely through an online app store, Brazilian consumers have the non-waivable right to withdraw from the contract within 7 (seven) calendar days following purchase or download delivery, receiving a full refund processed via the Microsoft Store platform.
- **Article 51, Subsection I (Nullity of Abusive Clauses):**
  - No provision in the EULA or CLC limits indemnity liability in cases of intentional misconduct or gross negligence by the vendor, ensuring full efficacy of consumer protections.
- **Article 101, Subsection I (Consumer Domicile Jurisdiction):**
  - In legal actions concerning vendor civil liability initiated by consumers residing in Brazil, jurisdiction lies in the forum of the consumer's domicile.

---

### 5. Compliance with Privacy and Data Protection Laws (LGPD, Marco Civil, and GDPR)

Eolo Platform implements an uncompromising **Sovereign Privacy (Local-First)** engineering model, guaranteeing native compliance with **Federal Law No. 13,709/2018 (LGPD)**, the **Brazilian Internet Civil Rights Framework (Federal Law No. 12,965/2014)**, and the **European GDPR**:

- **100% Offline and Exclusively Local Execution:**
  - The software makes zero outbound or inbound network connections. It contains no telemetry SDKs, no web analytics, no advertising identifiers, no automated crash telemetry reporting, and no interaction with remote cloud AI endpoints.
- **Zero Personal Data Collection (LGPD Article 6 — Necessity & Purpose):**
  - All calculation project files (`.json`), technical calculation memorials (`.html`, `.md`), and local SQLite parameter databases reside exclusively on the engineer's local storage (`%LOCALAPPDATA%\EoloPlatform` or a user-selected path).
  - The software does not harvest personal user data, does not perform behavioral profiling, and does not perform cross-border data transfers.
- **Workstation Security (LGPD Article 46):**
  - Physical and logical security of project files is the responsibility of the workstation access controls managed by the user or their enterprise organization.

---

### 6. Professional Engineering Responsibility (Federal Laws No. 5,194/1966 & No. 6,496/1977)

The application of computational tools in civil and structural engineering is regulated in Brazil by CONFEA and regional CREA councils:

- **Instrumental Nature of the Application:**
  - **Eolo Platform** is a specialized computational tool for mathematical modeling, analytical automation, and technical calculation reporting. Its sole purpose is to increase calculation productivity and reduce human error during manual table lookups and interpolation under ABNT NBR 6123:2023.
- **Indispensability of Professional Responsibility and ART:**
  - The application **does NOT, under any circumstance, replace the technical judgment, critical analysis, boundary condition verification, and non-delegable professional responsibility of a legally qualified Civil or Mechanical Engineer**.
  - All civil structures, cladding systems, industrial roofs, or engineering reports designed using Eolo Platform must be formally registered under a **Technical Responsibility Record (ART - Anotação de Responsabilidade Técnica)** with the competent CREA council, signed by the professional engineer of record.
  - The software developer accepts no civil or technical liability for physical structures, construction execution, structural collapses, or structural pathologies caused by inaccurate input parameters, oversimplified geometry, or misinterpretation of analytical results by the user.

---

### 7. Microsoft Store Policy Compliance & MSIX Standards

For commercial distribution and publication through the Microsoft Store, Eolo Platform satisfies all criteria outlined in the **Microsoft Store App Submission Guide**:

- **Store Policy 10.5.1 (Privacy Policy):**
  - The application's privacy statement is publicly and permanently accessible over HTTPS at `https://github.com/fernandoncidade/EoloPlatform_rs/blob/main/docs/security-model.md` and mirrored locally in `src/assets/PRIVACY_POLICY/Privacy_Policy_en_US.txt`.
  - The application displays the full text of its privacy policy directly in-app via "About Eolo Platform" -> "Privacy" tab.
- **Sandboxed Execution and Minimal Permissions:**
  - The MSIX application package declares only the capability required for high-performance desktop execution (`runFullTrust`), requesting no access to cameras, microphones, hardware geolocation, or address book resources.
- **AppUserModelID & Windows Shell Integration:**
  - The executable registers a dedicated AppUserModelID (`FernandoCidade.EoloPlatform.NBR6123.2026`), ensuring taskbar grouping on Windows 11, clean shortcut persistence, and official high-resolution icon rendering.
- **Windows App Certification Kit (WACK):**
  - Binary packages pass automated WACK testing covering security, memory stability, resource integrity, and Authenticode digital signatures.

---

### 8. ABNT Copyrights, Bibliographic References & Academic Purpose

- **Brazilian Association of Technical Standards (ABNT):**
  - Technical standards **ABNT NBR 6123:2023**, **ABNT NBR 8800:2024**, and **ABNT NBR 6118:2023** are protected intellectual property exclusively owned by Associação Brasileira de Normas Técnicas (*All rights reserved. No part of these publications may be reproduced or utilized by any means without written permission from ABNT*).
  - No ABNT standard text is reproduced in the software; only official bibliographic references are provided under `Help > About Eolo Platform... > Notices` (`src/assets/NOTICES/`), pursuant to Art. 46, III & VIII of Brazilian Law No. 9,610/1998 and Berne Convention Art. 10.
  - **Eolo Platform** and its author have no affiliation, sponsorship, endorsement, certification, or partnership with ABNT.
- **Open-Source Third-Party Dependencies:**
  - All open-source crates and libraries incorporated into the binary (e.g., `egui`, `eframe`, `wgpu`, `rusqlite`, `serde`, `Ubuntu` and `Noto Sans` fonts) are credited in `src/assets/NOTICES/NOTICE_en_US.txt` and accompanied by their license texts in `src/assets/LICENSES/`.

---

### 9. Exclusively Academic Purpose, Risk of Errors & Production Liability Exemption

Pursuant to the Brazilian Civil Code (Arts. 186, 393, 421, 421-A, 422, 927, and 945), the Brazilian CDC (Arts. 6(III), 12(§3)(III), 14(§3)(II), and 31, preserving non-waivable statutory consumer rights under Arts. 24, 25, 49, and 51(I) and Law No. 9,609/1998 Arts. 7 & 8), and international conventions ("AS IS" / "WITH ALL FAULTS", UCC § 2-316, and EU Directive 2019/770):
1. The software is provided "as is" **exclusively for academic study and educational purposes**, and may contain programming bugs, inaccuracies, or simplifications;
2. The application **does NOT serve as definitive software for decision-making in production, civil construction, or workplace environments**;
3. It is the user's sole obligation to manually verify every output (against an officially acquired standard copy) or cross-check via established, consolidated market tools under a licensed engineer's supervision and ART (Brazilian Laws No. 5,194/1966 & No. 6,496/1977), exempting the author from legal liability for real-world production use.

</details>
