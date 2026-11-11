# Política de Segurança / Security Policy — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher a política de segurança em português</b></summary><br>

# Eolo Platform — Política Oficial de Segurança da Informação

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Responsável de Segurança:** Fernando Nillsson Cidade  
**Canal Oficial de Divulgação:** `linceu_lighthouse@outlook.com`  

---

### 1. Versões com Suporte Ativo de Segurança

O fornecedor mantém suporte ativo de correções de segurança e conformidade normativa para as seguintes versões:

| Versão / Version | Status de Suporte / Supported |
| :--- | :---: |
| `v2026.11.x` | :white_check_mark: |
| `< v2026.11` | :x: |

---

### 2. Filosofia de Segurança e Privacidade Soberana

O Eolo Platform é concebido sob o princípio da **Privacidade Soberana (100% Local-First)**:
- O software não abre sockets de rede, não possui telemetria e não envia dados analíticos a servidores externos;
- Não há integração com serviços remotos de Inteligência Artificial ou LLMs na nuvem, garantindo isolamento total dos projetos de engenharia civil;
- O aplicativo executa em modo usuário padrão no Windows, sem solicitar privilégios de Administrador.

---

### 3. Como Reportar uma Vulnerabilidade de Segurança

Caso identifique uma vulnerabilidade em dependências Rust, falha potencial de integridade de arquivos ou inconformidade algorítmica crítica:
1. **Envie um e-mail confidencial para:** `linceu_lighthouse@outlook.com`
2. **Inclua no assunto:** `[Segurança - Eolo Platform] Relato de Vulnerabilidade`
3. **No corpo da mensagem, informe:**
   - Descrição detalhada da vulnerabilidade e passos de reprodução (*PoC*);
   - Versão exata do executável (`eolo.exe` ou `eolo-cli.exe`);
   - Versão do Windows utilizada.
4. **Prazo de Resposta:** Confirmação de recebimento em até 48 (quarenta e oito) horas e plano de mitigação/atualização corretiva rápida.

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the security policy in English</b></summary><br>

# Eolo Platform — Official Information Security Policy

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Security Lead:** Fernando Nillsson Cidade  
**Official Reporting Channel:** `linceu_lighthouse@outlook.com`  

---

### 1. Supported Versions

Active security updates and normative maintenance are provided for the following releases:

| Release / Version | Support Status / Supported |
| :--- | :---: |
| `v2026.11.x` | :white_check_mark: |
| `< v2026.11` | :x: |

---

### 2. Security Philosophy & Sovereign Privacy

Eolo Platform is engineered under the principle of **Sovereign Privacy (100% Local-First)**:
- The software opens zero network sockets, contains no telemetry harnesses, and transmits zero data to remote servers;
- No remote cloud AI or LLM services are utilized, ensuring complete confidential isolation of proprietary civil engineering models;
- The application executes in standard user space on Windows without requesting Administrator elevation.

---

### 3. Reporting a Vulnerability

If you discover a potential vulnerability in Rust dependencies, file integrity discrepancy, or critical algorithmic flaw:
1. **Send a confidential e-mail to:** `linceu_lighthouse@outlook.com`
2. **Subject line:** `[Security - Eolo Platform] Vulnerability Report`
3. **Provide the following details:**
   - Detailed description and proof-of-concept (PoC) reproduction steps;
   - Exact binary version (`eolo.exe` or `eolo-cli.exe`);
   - Windows operating system version.
4. **SLA Commitment:** Acknowledgment within 48 hours and a rapid remediation release roadmap.

</details>
