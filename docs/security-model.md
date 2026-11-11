# Modelo de Segurança e Privacidade / Security & Privacy Model — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher o modelo de segurança em português</b></summary><br>

# Eolo Platform — Modelo de Segurança, Privacidade e Análise de Ameaças

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Nível de Segurança:** 100% Local-First | Zero Telemetria | Engenharia Analítica Determinística (Sem IA Externa)  
**Ambiente Operacional:** Windows 11 / Windows 10 / Windows Server 2022+ (x64)  
**Autor e Responsável de Segurança:** Fernando Nillsson Cidade  
**Canal de Contato de Segurança:** `linceu_lighthouse@outlook.com`  

---

### 1. Filosofia de Segurança e Privacidade Soberana

O **Eolo Platform** foi concebido sob o paradigma da **Privacidade Soberana (*Sovereign Privacy*)** e arquitetura **100% Local-First**. Em projetos de engenharia civil e infraestrutura estratégica (edifícios de grande porte, pontes, galpões logísticos, usinas e hospitais), as dimensões estruturais, dados cadastrais de clientes, memórias descritivas e relatórios técnicos constituem propriedade intelectual crítica e segredos de negócio que não devem trafegar por redes externas ou servidores de terceiros.

#### 1.1. Garantias Centrais de Arquitetura
1. **Zero Comunicação de Rede (Offline por Definição):**
   - O executável não realiza requisições HTTP, HTTPS, WebSockets ou DNS para servidores externos.
   - Não há módulos de telemetria, rastreamento analítico, medição de uso, rastreadores publicitários ou envio automatizado de relatórios de falhas (*crash reports*).
   - O aplicativo opera com perfeição em estações de trabalho de engenharia totalmente desconectadas da Internet (*air-gapped environments*).
2. **Determinismo sem Modelos de Linguagem (Sem IA na Nuvem):**
   - O software não consome APIs remotas de Inteligência Artificial (LLMs, OpenAI, Claude, Gemini ou serviços em nuvem).
   - Todos os cálculos estruturais, fatores normativos e diagramas de envoltória são derivados de formulações matemáticas analíticas estritas da **ABNT NBR 6123:2023**, garantindo reprodutibilidade aritmética imune a alucinações ou injeções de prompt (*prompt injection*).
3. **Execução em Modo Usuário Padrão (Menor Privilégio):**
   - O software executa estritamente no contexto de permissões do usuário comum no Windows. Não requer privilégios de Administrador, elevação de UAC (*User Account Control*) nem instalação de drivers de sistema ou serviços em segundo plano.

---

### 2. Ciclo de Vida dos Dados e Persistência Local

Toda a persistência de informações gerada pelo Eolo Platform ocorre exclusivamente no sistema de arquivos local do usuário:

```text
┌────────────────────────────────────────────────────────────────────────┐
│               FLUXO SEGURO DE DADOS LOCAIS - EOLO PLATFORM             │
└────────────────────────────────────────────────────────────────────────┘
                                     │
           [Dados de Entrada do Engenheiro: Geometria, Terreno, V0]
                                     │
                                     ▼
                [Motor Matemático em Rust: Tipagem Estrita f64]
           (Validação contra limites físicos e regras da NBR 6123)
                                     │
                                     ▼
             [Memória Segura: Ausência de Vazamentos e Zero GC]
                                     │
                     ┌───────────────┴───────────────┐
                     ▼                               ▼
       [Projetos Locais (.json)]       [Memoriais Auditáveis (.html, .md)]
     %LOCALAPPDATA%\EoloPlatform        Pasta selecionada pelo Engenheiro
```

#### 2.1. Diretórios Utilizados no Sistema Operacional
- **Configurações e Banco de Parâmetros Locais:**  
  `%LOCALAPPDATA%\EoloPlatform\` (ou pasta equivalente do perfil de usuário). Armazena preferências de unidades (SI vs. Técnico), idioma selecionado e o banco de dados SQLite embutido em modo somente leitura contendo estações meteorológicas oficiais do Brasil.
- **Exportação de Projetos e Relatórios:**  
  Os arquivos de projeto (`.json`) e memórias descritivas (`.html`, `.md`, `.json`) são gravados exclusivamente no diretório de trabalho explicitamente escolhido pelo engenheiro na caixa de diálogo do Windows Explorer.

#### 2.2. Validação Estrita de Arquivos de Projeto e Prevenção de Injeção
- O subsistema de carregamento de projetos (`src/core/project.rs`) utiliza o motor de serialização segura `serde_json` com parsing estrito e validação de invariantes numéricos:
  - Dimensões geométricas ($a, b, h$) devem ser estritamente positivas e finitas (`f64::is_finite() && val > 0.0`);
  - Ângulos de telhado ($\alpha$) são limitados entre 0° e 89,9°;
  - Velocidades básicas de vento ($V_0$) são validadas na faixa física plausível (20 a 70 m/s);
  - Coeficientes de pressão e permeabilidade são filtrados contra valores anômalos (`NaN` ou infinito).
- Essa validação em nível de tipo em Rust previne corrupção de estado, divisões por zero e potenciais ataques de negação de serviço via arquivos maliciosamente construídos.

---

### 3. Conformidade com Leis de Proteção de Dados (LGPD, Marco Civil e GDPR)

O Eolo Platform cumpre integralmente os requisitos da **Lei Geral de Proteção de Dados Pessoais (Lei nº 13.709/2018 — LGPD)**, do **Marco Civil da Internet (Lei nº 12.965/2014)** e do **Regulamento Europeu (GDPR)**:

| Princípio Legal (LGPD Art. 6º) | Implementação no Eolo Platform |
|:---|:---|
| **Finalidade e Adequação** | O aplicativo serve exclusivamente para cálculo estrutural e geração de memoriais de vento. Nenhum dado é tratado para outra finalidade. |
| **Necessidade (Minimização)** | O software **não coleta nenhum dado pessoal**, cadastro, número de documento, IP ou e-mail do usuário durante seu funcionamento. |
| **Livre Acesso e Transparência** | Todo dado gerado é visível diretamente na interface ou exportável em formatos abertos (JSON, Markdown, HTML). |
| **Segurança e Prevenção (Art. 46)** | Processamento isolado na memória local; impossibilidade de vazamento de dados por interceptação de rede (*sniffing* ou *man-in-the-middle*). |
| **Não-Discriminação** | Funcionamento 100% autônomo, sem perfis comportamentais ou decisões automatizadas discriminatórias. |

#### Política de Privacidade da Microsoft Store (Política 10.5.1)
Em atendimento à Política 10.5.1 da Microsoft Store:
- A declaração oficial de privacidade está publicada de forma permanente e pública em URL segura (HTTPS):  
  `https://github.com/fernandoncidade/EoloPlatform_rs/blob/main/docs/security-model.md`
- O texto integral é acessível nativamente pelo usuário dentro do próprio aplicativo desktop através do menu `Ajuda` -> `Sobre o Eolo Platform...` -> Aba `Privacidade`.

---

### 4. Análise de Ameaças Estruturada (Modelo STRIDE)

| Categoria STRIDE | Risco Potencial | Mitigação Implementada no Eolo Platform |
|:---|:---|:---|
| **Spoofing (Falsificação de Identidade)** | Falsificação de binários do aplicativo ou de bibliotecas por terceiros. | O pacote comercial MSIX é assinado digitalmente com certificado Authenticode. Na versão livre, o código-fonte auditável e tags assinadas garantem a proveniência. |
| **Tampering (Adulteração de Dados)** | Modificação de arquivos `.json` de projeto para induzir erros de cálculo. | Validação estrita de tipos em Rust (`serde`), rejeição de valores `NaN`, infinitos ou dimensões negativas antes de qualquer cálculo. |
| **Repudiation (Repúdio)** | Incerteza sobre autoria ou parâmetros de cálculo de um memorial emitido. | O gerador de memorial registra no documento carimbo de data/hora local, versão exata do software (`2026.11.8.0`) e rastreabilidade das fórmulas da NBR 6123:2023. |
| **Information Disclosure (Vazamento de Informações)** | Interceptação de dados confidenciais de projetos de engenharia civil. | Mitigado por design: zero sockets de rede, zero telemetria e gravação restrita ao disco rígido local do usuário. |
| **Denial of Service (Negação de Serviço)** | Congelamento da interface gráfica ou travamento por entrada geométrica extrema. | Operações de renderização e cálculo desacopladas; validação de entradas previne loops infinitos ou exaustão de pilha. |
| **Elevation of Privilege (Elevação de Privilégios)** | Execução de comandos com permissões administrativas indesejadas. | O aplicativo roda no nível de privilégio padrão do usuário e não invoca processos de sistema nem acessa chaves restritas do registro do Windows. |

---

### 5. Segurança da Linguagem Rust e Gestão de Memória

O Eolo Platform é desenvolvido em **Rust (Edição 2021)**, usufruindo das garantias formais de segurança de memória em tempo de compilação:
- **Ausência de Erros de Memória (Memory Safety):** Proteção estrita contra *buffer overflows*, acessos fora dos limites de arrays, uso após liberação (*use-after-free*) e desreferenciamento de ponteiros nulos (*null pointer dereferences*);
- **Segurança contra Condições de Corrida (*Data Races*):** O sistema de tipagem e o verificador de empréstimos (*borrow checker*) do compilador Rust garantem segurança concorrente entre threads de interface e cálculo;
- **Zero Injeção de Código Externo:** Não são utilizados interpretadores dinâmicos, scripts em runtime ou mecanismos inseguros de reflexão.

---

### 6. Comunicação Responsável de Vulnerabilidades

Caso identifique qualquer potencial fragilidade de segurança, inconformidade de privacidade ou vulnerabilidade em dependências:
- **E-mail de Contato:** `linceu_lighthouse@outlook.com`
- **Assunto:** `[Segurança - Eolo Platform] Relatório de Vulnerabilidade`
- **Compromisso de Resposta:** Confirmação de recebimento em até 48 (quarenta e oito) horas e plano de mitigação/atualização corretiva em consonância com o ciclo de manutenção da versão `2026.11.8.0`.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the security model in English</b></summary><br>

# Eolo Platform — Security Model, Privacy Guarantees, and Threat Analysis

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Security Level:** 100% Local-First | Zero Telemetry | Deterministic Analytical Engineering (No External Cloud AI)  
**Operating Environment:** Windows 11 / Windows 10 / Windows Server 2022+ (x64)  
**Author & Security Lead:** Fernando Nillsson Cidade  
**Security Contact:** `linceu_lighthouse@outlook.com`  

---

### 1. Security Philosophy & Sovereign Privacy

**Eolo Platform** is architected under the core tenets of **Sovereign Privacy** and a **100% Local-First** operational paradigm. In civil and structural engineering projects for strategic infrastructure (high-rise towers, industrial warehouses, power stations, bridges, and medical complexes), building dimensions, structural framing configurations, proprietary calculation books, and client details represent highly confidential intellectual property and business trade secrets that must never be transmitted across external networks or third-party servers.

#### 1.1. Core Architectural Guarantees
1. **Zero Network Communication (Offline by Design):**
   - The application binary executes zero HTTP, HTTPS, WebSocket, or DNS requests to external hosts.
   - It contains no telemetry harnesses, web analytics trackers, usage metering modules, advertising beacons, or automated crash reporting services.
   - It performs flawlessly inside completely isolated, air-gapped engineering workstations and secured intranet environments.
2. **Deterministic Calculations without External AI (No Cloud LLMs):**
   - The software does not connect to external AI services or cloud-hosted Large Language Models (LLMs, OpenAI, Claude, Gemini).
   - All wind actions, aerodynamic coefficients, and load envelope graphs are computed strictly from the deterministic mathematical formulas of **ABNT NBR 6123:2023**, eliminating any possibility of prompt injection, context leakage, or stochastic calculation drift.
3. **Standard User Privilege Execution (Least Privilege):**
   - The software executes strictly within standard Windows user token privileges. It requires no Administrator rights, triggers no UAC elevation prompts, and installs no background Windows services or kernel drivers.

---

### 2. Data Lifecycle & Local Storage Architecture

All data generated and processed by Eolo Platform resides exclusively on the user's local file system:

```text
┌────────────────────────────────────────────────────────────────────────┐
│               SECURE LOCAL DATA LIFECYCLE - EOLO PLATFORM              │
└────────────────────────────────────────────────────────────────────────┘
                                     │
           [Engineer Inputs: Building Geometry, Terrain, V0]
                                     │
                                     ▼
                  [Rust Analytical Engine: Strict f64 Types]
             (Validated against physical limits and NBR 6123)
                                     │
                                     ▼
                [Safe Memory Execution: Zero Leaks, No GC]
                                     │
                     ┌───────────────┴───────────────┐
                     ▼                               ▼
       [Local Project Files (.json)]   [Auditable Memorials (.html, .md)]
     %LOCALAPPDATA%\EoloPlatform        User-selected workspace directory
```

#### 2.1. File System Paths Utilized
- **Preferences & Local Wind Parameter Database:**  
  `%LOCALAPPDATA%\EoloPlatform\` (or the operating system user profile equivalent). Houses UI unit preferences (SI vs. Technical), active localization settings, and the local embedded SQLite database containing official Brazilian meteorological stations.
- **Project Export & Engineering Reports:**  
  Structural project models (`.json`) and engineering calculation memorials (`.html`, `.md`, `.json`) are saved exclusively to the local directory explicitly chosen by the engineer through standard Windows Explorer file picker dialogs.

#### 2.2. Strict Project File Validation & Injection Defense
- The project deserializer (`src/core/project.rs`) utilizes the robust `serde_json` engine with strict schema enforcement and invariant checks:
  - Geometric dimensions ($a, b, h$) must be strictly positive and finite numbers (`f64::is_finite() && val > 0.0`);
  - Roof pitch angles ($\alpha$) are bounded between 0° and 89.9°;
  - Basic wind velocities ($V_0$) are validated against realistic physical bounds (20 to 70 m/s);
  - Pressure and permeability coefficients are screened to reject `NaN` or infinite values.
- These type-level guarantees in Rust prevent state corruption, divide-by-zero crashes, and denial-of-service vulnerabilities via malformed project files.

---

### 3. Data Protection Compliance (LGPD, Marco Civil, and GDPR)

Eolo Platform satisfies all principles and requirements set forth by the **Brazilian General Data Protection Law (Law No. 13,709/2018 — LGPD)**, the **Brazilian Internet Civil Rights Framework (Law No. 12,965/2014)**, and the **European GDPR**:

| Legal Principle (LGPD Art. 6) | Implementation in Eolo Platform |
|:---|:---|
| **Purpose & Suitability** | Dedicated strictly to structural wind load modeling and report generation. No secondary data processing takes place. |
| **Necessity (Data Minimization)** | The application **collects zero personal data**, requiring no registration, user credentials, IP address logging, or e-mail capture. |
| **Transparency & Open Access** | All generated calculation data is rendered on screen or exported to open, interoperable formats (JSON, Markdown, HTML). |
| **Security & Prevention (Art. 46)** | Complete memory isolation on the local machine; zero exposure to network eavesdropping, sniffing, or man-in-the-middle exploits. |
| **Non-Discrimination** | 100% deterministic local computation with zero behavioral profiling or automated profiling algorithms. |

#### Microsoft Store Privacy Policy Compliance (Policy 10.5.1)
In strict compliance with Microsoft Store Policy 10.5.1:
- The formal privacy policy statement is permanently hosted and publicly accessible over a secure HTTPS endpoint:  
  `https://github.com/fernandoncidade/EoloPlatform_rs/blob/main/docs/security-model.md`
- The complete privacy policy text is integrated directly into the desktop user interface, accessible via `Help` -> `About Eolo Platform...` -> `Privacy` tab.

---

### 4. Structured Threat Analysis (STRIDE Model)

| STRIDE Category | Potential Risk | Mitigation in Eolo Platform |
|:---|:---|:---|
| **Spoofing** | Forged application binaries or malicious dependency substitution. | Commercial MSIX packages are signed with official Authenticode certificates. Open-source source code is verifiable on GitHub via signed commits and tags. |
| **Tampering** | Malicious tampering with `.json` project files to induce calculation errors. | Strict type deserialization in Rust (`serde`), explicit rejection of `NaN`, infinite values, or negative dimensions prior to execution. |
| **Repudiation** | Ambiguity regarding the origin or calculation basis of an exported memorial. | The report generator includes local timestamps, exact engine release version (`2026.11.8.0`), and explicit formula citations from NBR 6123:2023. |
| **Information Disclosure** | Interception of proprietary civil engineering models or building blueprints. | Mitigated by design: zero network sockets, zero telemetry, and storage confined to the engineer's local drive. |
| **Denial of Service** | Application hang or freeze due to extreme or out-of-range geometric inputs. | Asynchronous UI rendering decoupled from computation; rigorous mathematical range checks prevent infinite loops and stack overflows. |
| **Elevation of Privilege** | Code execution with unneeded administrative privileges. | The application runs strictly with standard user privileges, invoking no administrative sub-processes and avoiding sensitive Windows Registry paths. |

---

### 5. Memory Safety & Rust Engineering Guarantees

Eolo Platform is compiled with **Rust (Edition 2021)**, providing compile-time memory safety guarantees:
- **Compile-Time Memory Safety:** Complete elimination of buffer overflows, out-of-bounds indexing, use-after-free conditions, and null pointer dereferences;
- **Thread Safety & Data Race Prevention:** Rust's strong static type system and borrow checker guarantee fearless concurrency between the GUI rendering loop and background calculation threads;
- **Zero Dynamic Code Injection:** Free from dynamic script interpreters, eval engines, or unsafe runtime reflection mechanisms.

---

### 6. Vulnerability Disclosure & Security Point of Contact

If you discover a suspected security vulnerability, privacy discrepancy, or dependency flaw:
- **Security Contact E-mail:** `linceu_lighthouse@outlook.com`
- **E-mail Subject:** `[Security - Eolo Platform] Vulnerability Report`
- **SLA Commitment:** Acknowledgment within 48 hours and a rapid remediation release adhering to the maintenance lifecycle of version `2026.11.8.0`.

</details>
