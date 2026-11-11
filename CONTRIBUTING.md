# Guia de Contribuição / Contributing Guide — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o guia de contribuição em português</b></summary><br>

# Eolo Platform — Diretrizes para Contribuição e Governança Comunitária

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Autor e Mantenedor:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Licenciamento:** GNU AGPLv3 (Código Aberto) / Licença Comercial (Microsoft Store)  

---

### 1. Aviso Inicial ao Colaborador

> [!NOTE]
> **Aviso de Projeto Pessoal e Análise Voluntária:**  
> Este repositório é um projeto pessoal, técnico e educacional mantido pelo autor em seu ritmo próprio de pesquisa e desenvolvimento. Contribuições, relatórios de problemas e Pull Requests são muito bem-vindos, mas a análise técnica, validação e incorporação de código ocorrem de forma voluntária, sem compromisso de prazo de resposta ou obrigação de merge.  
> Caso suas melhorias não sejam incorporadas de imediato, sinta-se plenamente livre e incentivado a manter, evoluir e distribuir suas modificações em seu próprio *fork* público sob os termos da licença **GNU AGPLv3**.

---

### 2. Duplo Licenciamento e Acordo de Colaborador (CLA)

O Eolo Platform opera sob um modelo de **Duplo Licenciamento**:
1. O repositório público de código-fonte no GitHub é disponibilizado sob a **GNU Affero General Public License v3 (GNU AGPLv3)**;
2. As compilações oficiais binárias são distribuídas comercialmente através da Microsoft Store sob o **EULA Comercial** e as **Condições de Licença Comercial (CLC)**.

Para viabilizar legalmente a coexistência de ambas as formas de distribuição:
- Toda e qualquer contribuição submetida via Pull Request exige a concordância expressa com o **[Acordo de Licença de Colaborador (CLA)](docs/CLA.md)**;
- Todo commit deve conter a linha formal de certificação de origem do desenvolvedor (DCO — *Developer Certificate of Origin*), adicionada através do comando:
  ```bash
  git commit -s -m "feat(engine): implementa tabela 8 de coeficientes de vento"
  ```
  que insere automaticamente a declaração:
  ```text
  Signed-off-by: Seu Nome <seu.email@exemplo.com>
  ```

---

### 3. Padrões de Qualidade de Código e Engenharia

Para que uma contribuição seja elegível para análise e revisão:
1. **Rigor Normativo (ABNT NBR 6123:2023):**
   - Qualquer modificação nos módulos de cálculo analítico (`src/engine/`) deve obrigatoriamente citar a cláusula, fórmula ou tabela específica da **ABNT NBR 6123:2023**;
   - Toda alteração deve ser acompanhada de testes analíticos automatizados cobrindo valores extremos e limites normativos.
2. **Qualidade em Rust (Edição 2021):**
   - O código deve compilar sem nenhum aviso (*0 warnings*):
     ```powershell
     cargo check --all-targets --all-features
     cargo clippy --all-targets
     ```
   - Toda a suíte de testes deve passar com 100% de sucesso:
     ```powershell
     cargo test
     ```
3. **Privacidade Soberana (Local-First):**
   - É terminantemente vedada a introdução de dependências ou códigos que façam conexões de rede externas, telemetria, rastreamento analítico ou consumo de APIs de IA na nuvem.

---

### 4. Fluxo de Submissão de Pull Request

1. Faça um *fork* do repositório oficial no GitHub;
2. Crie uma branch temática a partir da branch `main`:
   ```bash
   git checkout -b feature/suporte-cupulas
   ```
3. Desenvolva as alterações, escreva os testes e formate o código (`cargo fmt`);
4. Realize os commits assinados com `-s` (*Signed-off-by*);
5. Abra o Pull Request preenchendo todos os itens do [Template de Pull Request](.github/PULL_REQUEST_TEMPLATE.md).

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the contributing guide in English</b></summary><br>

# Eolo Platform — Contribution Guidelines & Community Governance

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Author & Maintainer:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Licensing:** GNU AGPLv3 (Open Source) / Commercial License (Microsoft Store)  

---

### 1. Contributor Notice

> [!NOTE]
> **Personal Project Notice & Voluntary Review:**  
> This repository is a personal, educational, and scientific engineering project maintained by the author at their own research pace. Pull Requests, issue reports, and community insights are warmly welcome; however, technical review and merging are undertaken voluntarily without any guaranteed response timeline or merge obligations.  
> Should your pull request not be merged immediately, you are fully authorized and encouraged to maintain, evolve, and distribute your enhancements in your own public fork under the terms of the **GNU AGPLv3** license.

---

### 2. Dual-Licensing Architecture & Contributor License Agreement (CLA)

Eolo Platform operates under an explicit **Dual-Licensing** framework:
1. The public source code repository on GitHub is licensed under the **GNU Affero General Public License v3 (GNU AGPLv3)**;
2. Official pre-compiled binary packages are distributed commercially through the Microsoft Store under the **Commercial EULA** and **Commercial License Conditions (CLC)**.

To provide legal certainty for this dual distribution model:
- Every contribution submitted via Pull Request requires agreement to the **[Contributor License Agreement (CLA)](docs/CLA.md)**;
- Every commit must carry a Developer Certificate of Origin (DCO) sign-off line, generated using the `-s` flag:
  ```bash
  git commit -s -m "feat(engine): implement Table 8 wind coefficients"
  ```
  producing the automated signature:
  ```text
  Signed-off-by: Your Name <your.email@example.com>
  ```

---

### 3. Engineering & Code Quality Standards

To be eligible for review and integration:
1. **Normative Rigor (ABNT NBR 6123:2023):**
   - Any modification to analytical engine modules (`src/engine/`) must explicitly cite the governing clause, formula, or table of **ABNT NBR 6123:2023**;
   - All code adjustments must include automated unit tests verifying boundary and extreme values.
2. **Rust Engineering Quality (Edition 2021):**
   - The workspace must compile cleanly with zero warnings:
     ```powershell
     cargo check --all-targets --all-features
     cargo clippy --all-targets
     ```
   - All automated test suites must pass 100%:
     ```powershell
     cargo test
     ```
3. **Sovereign Privacy (Local-First):**
   - No external network connections, telemetry harnesses, tracking beacons, or cloud AI API dependencies may be introduced.

---

### 4. Pull Request Submission Workflow

1. Fork the official GitHub repository;
2. Create a feature branch branching from `main`:
   ```bash
   git checkout -b feature/dome-support
   ```
3. Implement your changes, write tests, and run formatting (`cargo fmt`);
4. Commit your changes using `-s` (*Signed-off-by*);
5. Submit the Pull Request completing all checkboxes in the [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md).

</details>
