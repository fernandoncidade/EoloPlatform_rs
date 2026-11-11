# Licenciamento e Esclarecimento Comercial / Licensing & Commercial Model — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o documento de licenciamento em Português</b></summary><br>

# Eolo Platform — Licenciamento, Termos de Código Aberto e Esclarecimento Comercial

**Produto:** Eolo Platform ("Eolo")  
**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Licença do Código-Fonte:** GNU Affero General Public License v3.0 (`AGPL-3.0-only`)  
**Licença dos Binários Comerciais Oficiais:** EULA Microsoft Store (`src/assets/EULA/`) + CLC (`src/assets/CLC/`)  
**Autor e Único Detentor dos Direitos Autorais:** Fernando Nillsson Cidade (Pessoa Física, Brasil)  
**Canal Oficial de Suporte:** `linceu_lighthouse@outlook.com`  
**Perfil GitHub Oficial:** <https://github.com/fernandoncidade>

---

## 1. Visão Geral da Matriz de Licenciamento por Componente

O **Eolo Platform** (binários `eolo.exe` e `eolo-cli.exe`) é distribuído com transparência sob um modelo de **Duplo Licenciamento** (*Dual Licensing*), possível porque **Fernando Nillsson Cidade** (pessoa física, desenvolvedor independente no Brasil) é o criador original e único detentor dos direitos autorais patrimoniais e morais da obra:

| Componente | Licença Aplicável | Direitos, Escopo e Condições |
|---|---|---|
| **Código-Fonte (Repositório Público no GitHub)** | **GNU AGPLv3** (`AGPL-3.0-only`, texto integral em [`LICENSE`](../LICENSE)) | Liberdade para estudar, auditar, executar, modificar e redistribuir o código-fonte, desde que quaisquer versões modificadas, obras derivadas ou serviços oferecidos em rede (Seção 13) mantenham a licença **GNU AGPLv3** e respeitem o termo adicional de proteção de marca da Seção 7(e). |
| **Binários Oficiais da Microsoft Store e Licenciamento Comercial Direto** | **EULA Comercial** (`src/assets/EULA/`) + **CLC** (`src/assets/CLC/`) | Aplicável aos pacotes MSIX oficiais publicados na **Microsoft Store** (submetidos no campo *"Additional license terms"* do Partner Center; na ausência de termos adicionais, aplicar-se-iam os *Standard Application License Terms — SALT* da Microsoft) e a distribuições comerciais diretas autorizadas pelo autor. |
| **Contribuições Externas ao Repositório Principal** | **GNU AGPLv3** + **[`docs/CLA.md`](CLA.md)** + **DCO (`Signed-off-by`)** | O contribuidor mantém a titularidade de seu *copyright* e concede a Fernando Nillsson Cidade licença perpétua, irrevogável, mundial, não exclusiva e gratuita para distribuir a contribuição sob a GNU AGPLv3 e sob licenças comerciais/proprietárias (EULA/CLC), nos termos dos arts. 49 e 50 da Lei nº 9.610/1998. |
| **Documentação Institucional em `src/assets/`** | **Autoria do Projeto (Uso Livre para Forks)** | Os arquivos em `src/assets/` (`ABOUT`, `CLC`, `COPYRIGHT`, `EULA`, `LICENSES`, `NOTICES`, `PRIVACY_POLICY`, `RELEASE`) destinam-se à distribuição oficial; mantenedores de *forks* sob a GNU AGPLv3 podem adaptá-los, traduzi-los ou substituí-los livremente, preservando a restrição de uso da marca **"Eolo Platform"™** nos termos da Seção 7(e) da AGPLv3. |
| **Binários Compilados por Terceiros** | **GNU AGPLv3** (`AGPL-3.0-only`) | Qualquer pessoa pode compilar e distribuir seus próprios binários a partir do código-fonte público, desde que cumpra integralmente a **GNU AGPLv3** (incluindo disponibilização do código-fonte correspondente e respeito à Seção 7(e)). |
| **Crates Rust Upstream, Fontes e SQLite** | **Licenças Permissivas e Domínio Público** | Dependências licenciadas sob `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `BSL-1.0`, `Unicode-3.0`, `Unlicense` e `0BSD`; fontes em `epaint_default_fonts` sob `SIL OFL-1.1` e `Ubuntu Font Licence 1.0`; motor SQLite dedicado ao **Domínio Público**. |

---

## 2. Direitos e Obrigações da Edição de Código Aberto (GNU AGPLv3)

Todo o código-fonte público do **Eolo Platform** — abrangendo o motor analítico determinístico da **ABNT NBR 6123:2023** ($S_1$, $S_2$, $S_3$, $V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3$, $q = 0,613 \cdot V_k^2$, coeficientes aerodinâmicos $C_{pe}/C_e$, pressão interna $C_{pi}$ e envoltórias), a visualização 2D/3D de cargas em GPU (`egui`/`eframe` 0.37 e `wgpu`), a exportação de Memoriais de Cálculo em HTML, Markdown e JSON, os 5 cenários pré-configurados e o utilitário de linha de comando `eolo-cli.exe` — é licenciado exclusivamente sob a **GNU Affero General Public License v3.0 (`AGPL-3.0-only`)**.

### 2.1. Reciprocidade (*Copyleft* Forte) e Interação em Rede (Seção 13 da AGPLv3)
- Qualquer pessoa ou organização que distribua cópias ou versões modificadas a partir do código-fonte público, ou que disponibilize o software (modificado ou não, quando obtido sob a AGPLv3) a usuários remotamente através de uma rede de computadores (como serviço web, SaaS ou API), deve disponibilizar o código-fonte completo correspondente sob a **GNU AGPLv3**.
- Usuários e mantenedores de *forks* que utilizam exclusivamente o código-fonte público sob a GNU AGPLv3 **não** estão vinculados ao EULA comercial; de igual modo, o EULA jamais restringe, limita ou revoga quaisquer direitos concedidos pela GNU AGPLv3 sobre o código-fonte público.

### 2.2. Proteção de Marca sob a Seção 7(e) da GNU AGPLv3
Nos termos da **Seção 7(e)** da GNU Affero General Public License v3.0, esta licença pública **não concede quaisquer direitos sobre a marca**, nome comercial, identidade visual ou logotipo **"Eolo Platform"™** (marca utilizada pelo autor, sem afirmação de registro perante o INPI / sem uso do símbolo ®). Versões modificadas ou *forks* distribuídos por terceiros não podem sugerir endosso oficial do autor e devem distinguir claramente sua própria identificação.

---

## 3. Mecânica do Duplo Licenciamento, Microsoft Store (MSIX) e Legislação Brasileira

### 3.1. Licenciamento do Binário Comercial e Ausência de Obrigações da AGPLv3
O binário comercial oficial distribuído na **Microsoft Store** (ou via autorização comercial direta) é licenciado diretamente pelo titular dos direitos autorais sob o **EULA** (`src/assets/EULA/`) e o **CLC** (`src/assets/CLC/`), e **não** sob a GNU AGPLv3. Consequentemente, **as obrigações de reciprocidade da GNU AGPLv3 (incluindo a Seção 13) não surgem a partir do uso do binário comercial**.

### 3.2. Submissão MSIX no Microsoft Partner Center (EULA vs. SALT)
- Conforme a documentação oficial da Microsoft (<https://learn.microsoft.com/pt-br/windows/apps/publish/publish-your-app/msix/create-app-submission>), a distribuição na Microsoft Store ocorre por meio de pacote **MSIX** (`AppUserModelID`: `FernandoCidade.EoloPlatform.NBR6123.2026`), enquanto o instalador Inno Setup (`repo/EoloPlatform.iss`) destina-se exclusivamente à distribuição direta fora da loja.
- No **Microsoft Partner Center** (*Store listings* → *Additional license terms*), o texto do **EULA** (`src/assets/EULA/`) é fornecido como contrato específico da aplicação; caso esse campo fosse deixado em branco na listagem, aplicar-se-iam supletivamente os *Standard Application License Terms (SALT)* da Microsoft.
- Pagamentos, faturamento, tributos de loja, reembolsos, conta Microsoft e validação de direito de uso (*entitlement*) são processados pela Microsoft nos termos da **Declaração de Privacidade da Microsoft** (<https://www.microsoft.com/pt-br/privacy/privacystatement>) e dos Termos de Venda da Microsoft Store.

### 3.3. Conformidade com a Lei do Software (Lei nº 9.609/1998 — Artigos 7º e 8º)
Em estrito cumprimento aos **artigos 7º e 8º da Lei nº 9.609/1998**:
- **Prazo de Validade Técnica da Versão (Art. 7º):** O prazo de validade técnica da versão `2026.11.8.0` é de **12 (doze) meses a contar da data de publicação da versão (`8 de novembro de 2026`), ou até a disponibilização de versão posterior que a substitua, o que for maior**.
- **Serviços Técnicos Complementares (Art. 8º):** Durante o prazo de validade técnica da versão, o autor presta no Brasil serviços técnicos complementares para o adequado funcionamento do programa, consistentes em manutenção corretiva por meio de atualizações de software e atendimento técnico por correio eletrônico através do canal oficial **`linceu_lighthouse@outlook.com`**. *(Nota jurídica: os artigos 7º e 8º da Lei nº 9.609/1998 estabelecem garantias de validade técnica e assistência ao usuário, jamais sendo invocados como fundamento para eximir responsabilidade legal).*

### 3.4. Compatibilidade com o Código de Defesa do Consumidor (Lei nº 8.078/1990 — CDC)
- **Direitos Inafastáveis:** Os direitos assegurados pelo **Código de Defesa do Consumidor (Lei nº 8.078/1990)** são de ordem pública e inafastáveis. Quaisquer cláusulas de limitação de responsabilidade aplicam-se **na extensão máxima permitida pela legislação aplicável** e jamais excluem, restringem ou atenuam direitos garantidos pelo CDC (em especial os **artigos 25 e 51, inciso I**).
- **Direito de Arrependimento (Art. 49 do CDC):** Nas aquisições realizadas fora do estabelecimento comercial (compras online), é assegurado ao consumidor o prazo legal de **7 (sete) dias** para exercício do direito de arrependimento e reembolso, processado através do fluxo de reembolso da Microsoft Store, sem prejuízo dos direitos previstos em lei.
- **Foro Competente:** Quando aplicável o Código de Defesa do Consumidor, fica assegurado o **foro do domicílio do consumidor** (**artigo 101, inciso I, do CDC**); nas demais relações não regidas pelo CDC, aplica-se o foro do domicílio do autor no Brasil.

---

## 4. Declaração Expressa do Titular dos Direitos Autorais

> **Declaração Expressa de Fernando Nillsson Cidade:**  
> Eu, **Fernando Nillsson Cidade** (pessoa física, desenvolvedor independente no Brasil), na qualidade de criador original e único detentor dos direitos autorais do projeto **Eolo Platform**, declaro e concedo expressamente a qualquer indivíduo, profissional de engenharia, instituição acadêmica ou organização todas as permissões previstas na licença **GNU Affero General Public License v3.0 (`AGPL-3.0-only`)** sobre o código-fonte público deste repositório, incluindo:
> - O direito de estudar, auditar, compilar e executar o código-fonte para qualquer finalidade profissional, acadêmica ou pessoal;
> - O direito de modificar o código-fonte e criar obras derivadas, respeitando a proteção da marca **"Eolo Platform"™** sob a Seção 7(e);
> - O direito de redistribuir o código-fonte e versões modificadas, desde que o código-fonte completo correspondente seja disponibilizado sob os mesmos termos da **GNU AGPLv3**.

---

## 5. Auditoria de Licenças de Terceiros e Propriedade Intelectual da ABNT

### 5.1. Tabela de Dependências de Terceiros, Eleição de Opções Permissivas e Fontes
O **Eolo Platform** utiliza exclusivamente bibliotecas de terceiros com licenças permissivas ou em domínio público. Nos casos em que um crate a montante (*upstream*) é disponibilizado sob licenciamento duplo ou múltiplo que inclua uma opção *copyleft* (por exemplo, `self_cell` sob `Apache-2.0 OR GPL-2.0-only`), **o projeto elege expressamente a opção permissiva (`Apache-2.0` ou `MIT`)**. Nenhuma dependência de terceiros impõe obrigações de *copyleft* sobre o binário comercial.

| Componente / Crate | Licença Original / Eleita | Papel no Eolo Platform |
|---|---|---|
| **`rustc` / Biblioteca Padrão Rust (`std`)** | `MIT OR Apache-2.0` | Compilador e biblioteca padrão da Edição 2021 do Rust (<https://www.rust-lang.org>). |
| **`egui` (`0.37.0`) / `eframe` (`0.37.0`) / `epaint` / `emath` / `ecolor`** | `MIT OR Apache-2.0` | Motor de interface gráfica em modo imediato e integração de janelas/GPU (<https://github.com/emilk/egui>). |
| **`wgpu` / `winit` / `glutin`** (via `eframe`) | `MIT OR Apache-2.0` | Renderização gráfica acelerada por hardware e gerenciamento de janelas nativas. |
| **`epaint_default_fonts`** (Fontes Embutidas) | `OFL-1.1` (SIL Open Font License 1.1) e `Ubuntu Font Licence 1.0` | Fontes tipográficas padrão embutidas pelo `egui`/`epaint` (Hack, Ubuntu-Light, NotoEmoji). |
| **`SQLite3` (Motor C Embarcado)** | **Domínio Público** (*Public Domain*) | Motor relacional embarcado via feature `bundled` (<https://www.sqlite.org>). |
| **`rusqlite` (`0.41.0`) / `libsqlite3-sys`** | `MIT` | Bindings seguros de SQLite para Rust (<https://github.com/rusqlite/rusqlite>). |
| **`serde` (`1.0.229`) / `serde_json` (`1.0.154`)** | `MIT OR Apache-2.0` | Serialização e desserialização tipada de projetos `.json` e relatórios (<https://serde.rs>). |
| **`chrono` (`0.4.45`)** | `MIT OR Apache-2.0` | Formatação de datas e carimbos de tempo locais. |
| **`image` (`0.25.10`)** | `MIT OR Apache-2.0` | Decodificação de ícones `.ico` e recursos `.png`. |
| **`rfd` (`0.17.2`)** | `MIT` (opção permissiva eleita) | Diálogos nativos de abertura e salvamento de arquivos (<https://github.com/PolyMeilex/rfd>). |
| **`windows-sys` (`0.61.2`)** | `MIT OR Apache-2.0` | Bindings Win32 para `FreeConsole`, `SetCurrentProcessExplicitAppUserModelID` e `GetSystemMetrics`. |
| **`winres` (`0.1`)** (Build Dependency) | `MIT` | Incorporação de ícone e metadados PE no executável Windows em tempo de compilação (`src/build.rs`). |
| **`self_cell` e Crates Transitivos da Árvore Cargo** | `Apache-2.0` / `MIT` / `BSD-2-Clause` / `BSD-3-Clause` / `ISC` / `Zlib` / `BSL-1.0` / `Unicode-3.0` / `Unlicense` / `0BSD` (opção permissiva `Apache-2.0`/`MIT` eleita onde houver alternativa GPL) | Utilitários internos de memória, texto Unicode, compressão PNG (`Zlib`) e estruturas auxiliares. |

Os textos integrais das licenças estão incluídos no diretório `src/assets/LICENSES/` e acessíveis na janela **Sobre → Licenças**: `GNU AGPLv3`, `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `BSL-1.0`, `Unicode-3.0`, `SIL OFL-1.1`, `Ubuntu Font Licence 1.0` e a dedicação ao Domínio Público do `SQLite`.

### 5.2. Direitos Autorais da ABNT, Referências Bibliográficas e Uso Exclusivamente Acadêmico
- As normas técnicas **ABNT NBR 6123:2023**, **ABNT NBR 8800:2024** e **ABNT NBR 6118:2023** são publicações protegidas por direitos autorais de titularidade exclusiva da **Associação Brasileira de Normas Técnicas (ABNT)** (<https://www.abnt.org.br> | Todos os direitos reservados; vedada qualquer reprodução sem permissão por escrito da ABNT).
- O **Eolo Platform** **não é afiliado, patrocinado, certificado nem endossado pela ABNT** e **não reproduz** o texto das normas, disponibilizando exclusivamente suas referências bibliográficas na aba **Avisos** (`src/assets/NOTICES/`), nos termos do art. 46, III e VIII da Lei nº 9.610/1998.
- O software destina-se **exclusivamente a fins de estudos acadêmicos**, pode conter erros e **não serve como software definitivo para tomadas de decisão em produção ou ambientes de trabalho**, sendo obrigatória a conferência manual (em exemplar oficial da norma) ou por ferramentas consolidadas de mercado (Código Civil arts. 186, 393 e 927; CDC arts. 6º, III, 14 §3º II e 31, preservados os direitos irrenunciáveis do consumidor; convenções internacionais `"AS IS"`).
- O conteúdo normativo de titularidade da ABNT **não** é licenciado sob a GNU AGPLv3 nem sob o EULA/CLC comercial, pois pertence exclusivamente à ABNT.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details open>
<summary><b>Click to expand / collapse the licensing document in English</b></summary><br>

# Eolo Platform — Licensing, Open-Source Terms & Commercial Clarification

**Product:** Eolo Platform ("Eolo")  
**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Source Code License:** GNU Affero General Public License v3.0 (`AGPL-3.0-only`)  
**Official Commercial Binaries License:** Microsoft Store EULA (`src/assets/EULA/`) + CLC (`src/assets/CLC/`)  
**Author & Sole Copyright Holder:** Fernando Nillsson Cidade (Natural Person, Brazil)  
**Official Support Channel:** `linceu_lighthouse@outlook.com`  
**Official GitHub Profile:** <https://github.com/fernandoncidade>

---

## 1. Component-by-Component Licensing Matrix Overview

**Eolo Platform** (binaries `eolo.exe` and `eolo-cli.exe`) is transparently distributed under a **Dual-Licensing** model, made possible because **Fernando Nillsson Cidade** (natural person, independent developer in Brazil) is the original creator and sole holder of all economic and moral copyrights in the work:

| Component | Applicable License | Rights, Scope, and Conditions |
|---|---|---|
| **Source Code (Public GitHub Repository)** | **GNU AGPLv3** (`AGPL-3.0-only`, full text in [`LICENSE`](../LICENSE)) | Freedom to study, audit, run, modify, and redistribute the source code, provided that any modified versions, derivative works, or network-hosted services (Section 13) remain under the **GNU AGPLv3** and respect the Section 7(e) trademark protection notice. |
| **Official Microsoft Store Binaries & Direct Commercial Licensing** | **Commercial EULA** (`src/assets/EULA/`) + **CLC** (`src/assets/CLC/`) | Applies to official MSIX packages published on the **Microsoft Store** (submitted in the Partner Center *"Additional license terms"* field; if left blank, Microsoft's *Standard Application License Terms — SALT* would apply instead) and to direct commercial distributions authorized by the author. |
| **External Contributions to the Main Repository** | **GNU AGPLv3** + **[`docs/CLA.md`](CLA.md)** + **DCO (`Signed-off-by`)** | The contributor retains their copyright ownership and grants Fernando Nillsson Cidade a perpetual, irrevocable, worldwide, non-exclusive, royalty-free license to distribute the contribution under both the GNU AGPLv3 and commercial/proprietary licenses (EULA/CLC), pursuant to Articles 49 and 50 of Brazilian Law No. 9,610/1998. |
| **Institutional Documentation in `src/assets/`** | **Project Authorship (Free Use for Forks)** | The files in `src/assets/` (`ABOUT`, `CLC`, `COPYRIGHT`, `EULA`, `LICENSES`, `NOTICES`, `PRIVACY_POLICY`, `RELEASE`) are intended for official distribution; maintainers of GNU AGPLv3 forks may freely adapt, translate, or replace them, while preserving the **"Eolo Platform"™** trademark restriction under AGPLv3 Section 7(e). |
| **Self-Compiled Third-Party Binaries** | **GNU AGPLv3** (`AGPL-3.0-only`) | Anyone may compile and distribute their own binaries from the public source code, provided they fully comply with the **GNU AGPLv3** (including providing corresponding source code and respecting Section 7(e)). |
| **Upstream Rust Crates, Fonts, and SQLite** | **Permissive Licenses & Public Domain** | Dependencies licensed under `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `BSL-1.0`, `Unicode-3.0`, `Unlicense`, and `0BSD`; fonts in `epaint_default_fonts` under `SIL OFL-1.1` and `Ubuntu Font Licence 1.0`; SQLite engine dedicated to the **Public Domain**. |

---

## 2. Open-Source Edition Rights and Obligations (GNU AGPLv3)

All public source code of **Eolo Platform** — covering the deterministic **ABNT NBR 6123:2023** analytical engine ($S_1$, $S_2$, $S_3$, $V_k = V_0 \cdot S_1 \cdot S_2 \cdot S_3$, $q = 0.613 \cdot V_k^2$, aerodynamic coefficients $C_{pe}/C_e$, internal pressure $C_{pi}$, and load envelopes), GPU-accelerated 2D/3D load visualization (`egui`/`eframe` 0.37 and `wgpu`), HTML, Markdown, and JSON calculation report export, the 5 preset scenarios, and the `eolo-cli.exe` command-line utility — is licensed exclusively under the **GNU Affero General Public License v3.0 (`AGPL-3.0-only`)**.

### 2.1. Reciprocity (Strong *Copyleft*) and Network Interaction (AGPLv3 Section 13)
- Any person or organization that distributes copies or modified versions derived from the public source code, or that offers the software (modified or unmodified, when obtained under the AGPLv3) to users remotely over a computer network (such as a web service, SaaS, or API), must make the complete corresponding source code available under the **GNU AGPLv3**.
- Users and fork maintainers who exclusively use the public source code under the GNU AGPLv3 are **not** bound by the commercial EULA; likewise, the EULA never restricts, limits, or revokes any rights granted by the GNU AGPLv3 over the public source code.

### 2.2. Trademark Protection under GNU AGPLv3 Section 7(e)
Pursuant to **Section 7(e)** of the GNU Affero General Public License v3.0, this public license **does not grant any rights in the trademark**, trade name, visual identity, or logo **"Eolo Platform"™** (a trademark used by the author, with no claim of registration before INPI / no use of the ® symbol). Modified versions or forks distributed by third parties must not imply official endorsement by the author and must clearly distinguish their own branding.

---

## 3. Dual-Licensing Mechanics, Microsoft Store (MSIX), and Brazilian Statutory Law

### 3.1. Commercial Binary Licensing and Absence of AGPLv3 Obligations
The official commercial binary distributed on the **Microsoft Store** (or via direct commercial authorization) is licensed directly by the copyright holder under the **EULA** (`src/assets/EULA/`) and **CLC** (`src/assets/CLC/`), and **not** under the GNU AGPLv3. Consequently, **the GNU AGPLv3 reciprocity obligations (including Section 13) do not arise from using the commercial binary**.

### 3.2. MSIX Submission in Microsoft Partner Center (EULA vs. SALT)
- In accordance with official Microsoft documentation (<https://learn.microsoft.com/pt-br/windows/apps/publish/publish-your-app/msix/create-app-submission>), Microsoft Store distribution is packaged as **MSIX** (`AppUserModelID`: `FernandoCidade.EoloPlatform.NBR6123.2026`), whereas the Inno Setup installer (`repo/EoloPlatform.iss`) is solely for direct distribution outside the Store.
- In **Microsoft Partner Center** (*Store listings* → *Additional license terms*), the **EULA** text (`src/assets/EULA/`) is submitted as the application's specific license agreement; if that field were left blank in the listing, Microsoft's *Standard Application License Terms (SALT)* would apply by default.
- Payments, billing, store taxes, refunds, Microsoft account management, and license entitlement validation are processed by Microsoft under the **Microsoft Privacy Statement** (<https://www.microsoft.com/pt-br/privacy/privacystatement>) and the Microsoft Store Terms of Sale.

### 3.3. Compliance with the Brazilian Software Law (Law No. 9,609/1998 — Articles 7 and 8)
In strict compliance with **Articles 7 and 8 of Brazilian Law No. 9,609/1998**:
- **Technical Validity Period of the Version (Article 7):** The technical validity period (*prazo de validade técnica*) of version `2026.11.8.0` is **12 (twelve) months from the version's release date (`November 8, 2026`), or until a subsequent replacement version is made available, whichever is longer**.
- **Complementary Technical Services (Article 8):** During the version's technical validity period, the author provides complementary technical services in Brazil for the proper functioning of the software, consisting of corrective maintenance through software updates and technical support via e-mail at the official channel **`linceu_lighthouse@outlook.com`**. *(Legal note: Articles 7 and 8 of Law No. 9,609/1998 establish statutory technical validity and user support obligations and are never cited as a basis to disclaim legal liability).*

### 3.4. Compatibility with the Brazilian Consumer Defense Code (Law No. 8,078/1990 — CDC)
- **Non-Waivable Rights:** Rights guaranteed by the **Brazilian Consumer Defense Code (Law No. 8,078/1990 — CDC)** are matters of public policy and cannot be waived. Any liability limitation clauses apply **to the maximum extent permitted by applicable law** and never exclude, restrict, or diminish rights protected by the CDC (particularly **Articles 25 and 51, Item I**).
- **Right of Withdrawal (CDC Article 49):** For purchases made outside a physical commercial establishment (online purchases), consumers are entitled to the statutory **7 (seven) day** right of withdrawal and refund, exercised through the Microsoft Store refund process, without prejudice to rights provided by law.
- **Governing Venue:** Whenever the Consumer Defense Code (CDC) applies, the **venue of the consumer's domicile** is guaranteed (**Article 101, Item I, of the CDC**); in all other legal relationships not governed by the CDC, the venue of the author's domicile in Brazil applies.

---

## 4. Express Rights Holder Statement

> **Express Statement by Fernando Nillsson Cidade:**  
> I, **Fernando Nillsson Cidade** (natural person, independent developer in Brazil), as the original creator and sole copyright holder of the **Eolo Platform** project, hereby expressly declare and grant to any individual, engineering professional, academic institution, or organization all permissions established in the **GNU Affero General Public License v3.0 (`AGPL-3.0-only`)** over the public source code in this repository, including:
> - The right to study, audit, compile, and run the source code for any professional, academic, or personal purpose;
> - The right to modify the source code and create derivative works, while respecting the **"Eolo Platform"™** trademark protection under Section 7(e);
> - The right to redistribute the source code and modified versions, provided that the complete corresponding source code is made available under the same **GNU AGPLv3** terms.

---

## 5. Third-Party License Audit and ABNT Intellectual Property Notice

### 5.1. Third-Party Dependency Table, Election of Permissive Options, and Fonts
**Eolo Platform** relies exclusively on third-party libraries under permissive licenses or in the public domain. Wherever an upstream crate is offered under dual or multiple licenses that include a copyleft option (for example, `self_cell` under `Apache-2.0 OR GPL-2.0-only`), **the project expressly elects the permissive option (`Apache-2.0` or `MIT`)**. No third-party dependency imposes copyleft obligations on the commercial binary.

| Component / Crate | Original / Elected License | Role in Eolo Platform |
|---|---|---|
| **`rustc` / Rust Standard Library (`std`)** | `MIT OR Apache-2.0` | Rust 2021 Edition compiler and standard library (<https://www.rust-lang.org>). |
| **`egui` (`0.37.0`) / `eframe` (`0.37.0`) / `epaint` / `emath` / `ecolor`** | `MIT OR Apache-2.0` | Immediate-mode GUI engine and windowing/GPU integration (<https://github.com/emilk/egui>). |
| **`wgpu` / `winit` / `glutin`** (via `eframe`) | `MIT OR Apache-2.0` | Hardware-accelerated graphics rendering and native window management. |
| **`epaint_default_fonts`** (Bundled Fonts) | `OFL-1.1` (SIL Open Font License 1.1) and `Ubuntu Font Licence 1.0` | Default typographic fonts bundled by `egui`/`epaint` (Hack, Ubuntu-Light, NotoEmoji). |
| **`SQLite3` (Embedded C Engine)** | **Public Domain** | Embedded relational database engine via the `bundled` feature (<https://www.sqlite.org>). |
| **`rusqlite` (`0.41.0`) / `libsqlite3-sys`** | `MIT` | Safe SQLite bindings for Rust (<https://github.com/rusqlite/rusqlite>). |
| **`serde` (`1.0.229`) / `serde_json` (`1.0.154`)** | `MIT OR Apache-2.0` | Typed serialization and deserialization of `.json` projects and reports (<https://serde.rs>). |
| **`chrono` (`0.4.45`)** | `MIT OR Apache-2.0` | Local date formatting and timestamps. |
| **`image` (`0.25.10`)** | `MIT OR Apache-2.0` | Decoding of `.ico` icons and `.png` assets. |
| **`rfd` (`0.17.2`)** | `MIT` (permissive option elected) | Native file open and save dialogs (<https://github.com/PolyMeilex/rfd>). |
| **`windows-sys` (`0.61.2`)** | `MIT OR Apache-2.0` | Win32 bindings for `FreeConsole`, `SetCurrentProcessExplicitAppUserModelID`, and `GetSystemMetrics`. |
| **`winres` (`0.1`)** (Build Dependency) | `MIT` | Compile-time embedding of Windows icon and PE version resources (`src/build.rs`). |
| **`self_cell` and Transitive Cargo Crates** | `Apache-2.0` / `MIT` / `BSD-2-Clause` / `BSD-3-Clause` / `ISC` / `Zlib` / `BSL-1.0` / `Unicode-3.0` / `Unlicense` / `0BSD` (permissive `Apache-2.0`/`MIT` option elected wherever a GPL alternative exists) | Internal memory utilities, Unicode text processing, PNG compression (`Zlib`), and helper structures. |

Full license texts are shipped in `src/assets/LICENSES/` and accessible in the **About → Licenses** tab: `GNU AGPLv3`, `MIT`, `Apache-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `ISC`, `Zlib`, `BSL-1.0`, `Unicode-3.0`, `SIL OFL-1.1`, `Ubuntu Font Licence 1.0`, and the `SQLite` Public Domain dedication notice.

### 5.2. ABNT Copyrights, Bibliographic References & Exclusively Academic Study Scope
- The technical standards **ABNT NBR 6123:2023**, **ABNT NBR 8800:2024**, and **ABNT NBR 6118:2023** are copyrighted publications owned exclusively by the **Associação Brasileira de Normas Técnicas (ABNT)** (<https://www.abnt.org.br> | All rights reserved; no part may be reproduced without written permission from ABNT).
- **Eolo Platform** is **not affiliated with, sponsored by, certified by, or endorsed by ABNT**, does **not** reproduce copyrighted standard texts, and provides solely bibliographic references in the **Notices** tab (`src/assets/NOTICES/`), pursuant to Art. 46, III & VIII of Brazilian Law No. 9,610/1998 and Berne Convention Art. 10.
- The software is intended **exclusively for academic study purposes**, may contain errors, and **does NOT serve as definitive software for decision-making in production or workplace environments**; manual checking (against an official standard copy) or verification via consolidated market tools is mandatory (Brazilian Civil Code Arts. 186, 393 & 927; CDC Arts. 6(III), 14(§3)(II) & 31, preserving non-waivable consumer rights; international `"AS IS"` conventions).
- Normative content owned by ABNT is **not** licensed under the GNU AGPLv3 or under the commercial EULA/CLC, as it belongs exclusively to ABNT.

</details>
