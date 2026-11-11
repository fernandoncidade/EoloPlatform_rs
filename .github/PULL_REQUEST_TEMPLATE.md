# Modelo de Pull Request / Pull Request Template

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

> [!NOTE]
> **Aviso ao Colaborador / Contributor Notice:**  
> 🇧🇷 *Este repositório é um projeto pessoal, científico e educacional mantido pelo autor em ritmo próprio de pesquisa. Pull Requests são bem-vindos, mas a análise técnica e incorporação de código ocorrem de forma voluntária e sem compromisso de prazo. Caso não haja manifestação ou merge imediato, sinta-se plenamente livre para manter, evoluir e distribuir suas melhorias em seu próprio fork sob os termos da licença **GNU AGPLv3**.*  
> 🇺🇸 *This repository is a personal, scientific, and educational project maintained by the author at their own research pace. Pull Requests are welcome, but technical review and merging are carried out voluntarily without any guaranteed timeframe. If there is no immediate response or merge, please feel free to maintain, evolve, and distribute your improvements in your own fork under the **GNU AGPLv3** license.*

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o template em português</b></summary><br>

### Descrição
Descreva brevemente a finalidade deste Pull Request no **Eolo Platform** e qual problema ou melhoria ele entrega.

### Tipo de Alteração
- [ ] Correção de bug (alteração compatível que corrige uma falha analítica ou de UI)
- [ ] Nova funcionalidade no Núcleo de Engenharia (`src/engine/` — ABNT NBR 6123:2023)
- [ ] Melhoria na Interface Gráfica (`egui` / 5 Abas / Canvas 3D / Diálogos Modeless)
- [ ] Melhoria no Utilitário CLI (`eolo-cli.exe` em `src/bin/main_cli.rs`)
- [ ] Aprimoramento no Gerador de Memoriais Técnicos (`src/reports/` — HTML/MD/JSON)
- [ ] Otimização de desempenho / Refatoração em Rust
- [ ] Atualização de documentação (`README.md`, `docs/`, `src/assets/`)
- [ ] Atualização de internacionalização / tradução (`src/locale/translations.rs`)

### Testes e Validação Realizados
Descreva as verificações executadas localmente antes de submeter o PR:
- [ ] Verificação estática sem erros (`cargo check --all-targets --all-features`)
- [ ] Suíte de testes automatizados 100% aprovada (`cargo test`)
- [ ] Análise com clippy sem avisos (`cargo clippy --all-targets`)
- [ ] Binários de produção compilados em MSVC x64 (`cargo build --release --bin eolo --bin eolo-cli`)
- [ ] Testes manuais realizados no Windows 10 / Windows 11:
  - Detalhes dos testes: 

### Conformidade Arquitetural, Normativa e Legal
- [ ] Os módulos analíticos de cálculo (`src/engine/`) permanecem desacoplados da camada visual (`egui`) para suportar tanto a GUI quanto o utilitário headless `eolo-cli.exe`.
- [ ] Qualquer fórmula ou tabela implementada cita formalmente a cláusula correspondente da **ABNT NBR 6123:2023**.
- [ ] Nenhuma conexão externa de rede, telemetria ou dependência de modelos de IA/LLM externos foi introduzida (100% Local-First / Offline).
- [ ] Todas as novas strings de interface foram adicionadas tanto em Português (`pt_BR`) quanto em Inglês (`en_US`).
- [ ] Todos os commits contêm a assinatura DCO (`Signed-off-by`) e estou de acordo com o [Acordo de Licença de Colaborador (CLA)](docs/CLA.md).
- [ ] As alterações respeitam integralmente os termos da licença **GNU Affero General Public License v3 (GNU AGPLv3)**.

### Issues Relacionadas
Corrige #(número da issue)

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the template in English</b></summary><br>

### Description
Briefly describe the purpose of this Pull Request in **Eolo Platform** and what bug fix or enhancement it provides.

### Type of Change
- [ ] Bug fix (non-breaking change which fixes an analytical or UI defect)
- [ ] New feature in Engineering Core (`src/engine/` — ABNT NBR 6123:2023)
- [ ] Graphical interface enhancement (`egui` / 5 Tabs / 3D Canvas / Modeless Dialogs)
- [ ] CLI utility improvement (`eolo-cli.exe` in `src/bin/main_cli.rs`)
- [ ] Calculation memorial generator enhancement (`src/reports/` — HTML/MD/JSON)
- [ ] Performance optimization / Rust refactoring
- [ ] Documentation update (`README.md`, `docs/`, `src/assets/`)
- [ ] Localization / translation update (`src/locale/translations.rs`)

### Tests and Verification Performed
Describe the verification steps executed locally prior to submission:
- [ ] Static check without errors (`cargo check --all-targets --all-features`)
- [ ] Automated test suite 100% passing (`cargo test`)
- [ ] Clippy analysis with zero warnings (`cargo clippy --all-targets`)
- [ ] Production release binaries compiled for MSVC x64 (`cargo build --release --bin eolo --bin eolo-cli`)
- [ ] Manual verification performed on Windows 10 / Windows 11:
  - Verification details: 

### Architectural, Normative, and Legal Compliance
- [ ] Analytical engineering modules (`src/engine/`) remain decoupled from the presentation layer (`egui`) to support both the GUI and headless `eolo-cli.exe`.
- [ ] Any formula or table implemented formally cites the corresponding clause of **ABNT NBR 6123:2023**.
- [ ] Zero external network calls, telemetry modules, or cloud AI/LLM API dependencies were introduced (100% Local-First / Offline).
- [ ] All new user interface strings are added in both Portuguese (`pt_BR`) and English (`en_US`).
- [ ] All commits include a DCO sign-off (`Signed-off-by`) and I agree to the [Contributor License Agreement (CLA)](docs/CLA.md).
- [ ] All changes strictly adhere to the terms of the **GNU Affero General Public License v3 (GNU AGPLv3)**.

### Related Issues
Fixes #(issue number)

</details>
