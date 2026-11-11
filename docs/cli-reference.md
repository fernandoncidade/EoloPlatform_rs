# Referência da Linha de Comando / CLI Reference — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher a referência da CLI em português</b></summary><br>

# Eolo Platform CLI (`eolo-cli.exe`) — Manual de Linha de Comando

**Versão:** `2026.11.11.0`  
**Data:** `11 de novembro de 2026`  
**Executável:** `builds/eolo/release/eolo-cli.exe`  
**Autor:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Licenciamento:** GNU AGPLv3 (Código Aberto) / Licença Comercial (Microsoft Store)  

---

### 1. Visão Geral do Utilitário

O **`eolo-cli`** é a interface de linha de comando autônoma e de alto desempenho do Eolo Platform. Projetada para execução em modo *headless* (sem inicialização de subsistema gráfico), a ferramenta permite:
- Cálculo automatizado e instantâneo de parâmetros de vento da **ABNT NBR 6123:2023**;
- Geração em lote de memoriais de cálculo em formatos abertos (HTML, Markdown e JSON);
- Consulta automatizada da base de dados de isopletas e estações meteorológicas da Força Aérea Brasileira (FAB);
- Integração contínua (CI/CD) e auditoria normativa de modelos estruturais em scripts PowerShell, Python ou Bash.

---

### 2. Sintaxe Geral e Comandos Disponíveis

```bash
eolo-cli [COMANDO] [OPÇÕES]
```

| Comando | Descrição |
|:---|:---|
| `calc` | Realiza o cálculo expedito da velocidade característica $V_k$ e da pressão dinâmica básica $q$. |
| `report` | Gera o memorial formal de cálculo nos formatos HTML, Markdown ou JSON. |
| `isopleth` | Consulta velocidades básicas de vento ($V_0$) por Estado (UF) ou estações da FAB. |
| `scenarios` | Lista os 5 cenários pré-configurados de referência da engenharia civil. |
| `verify` | Executa a auditoria e validação normativa do projeto frente à NBR 6123:2023. |
| `test` | Roda a esteira de testes analíticos e verificação determinística de algoritmos. |
| `help`, `-h` | Exibe o menu de instruções e opções de sintaxe. |

---

### 3. Detalhamento dos Comandos e Opções

#### 3.1. Comando `calc`
Calcula $V_k$ e $q$ a partir de parâmetros ambientais e da cota vertical $z$.

**Parâmetros:**
- `--v0 <valor>`: Velocidade básica do vento em $m/s$ (padrão: `45.0`);
- `--s1 <valor>`: Fator topográfico (padrão: `1.00`);
- `--cat <1-5>`: Categoria de rugosidade do terreno (1 a 5, padrão: `4`);
- `--class <a|b|c>`: Classe dimensional da edificação (`a`, `b` ou `c`, padrão: `b`);
- `--s3 <valor>`: Fator estatístico de ocupação (padrão: `1.00`);
- `-z <metros>`: Altura da cota de análise em metros (padrão: `7.5`).

**Exemplo de Execução:**
```bash
eolo-cli calc --v0 45.0 --s1 1.0 --cat 4 --class b --s3 1.0 -z 15.0
```

**Saída Típica no Terminal:**
```text
=== EOLO PLATFORM CLI — CÁLCULO ANALÍTICO NBR 6123:2023 ===
Velocidade Básica (V0):       45.00 m/s (162.00 km/h)
Fator Topográfico (S1):       1.00
Rugosidade / Classe (S2):     0.88 (Cat. IV, Classe B em z = 15.0 m)
Fator Estatístico (S3):       1.00 (Grupo 2 - Edifícios habitacionais)
------------------------------------------------------------
Velocidade Característica Vk: 39.60 m/s (142.56 km/h)
Pressão Dinâmica q:           961.42 N/m² (0.96 kN/m²)
============================================================
```

---

#### 3.2. Comando `report`
Gera o Memorial Descritivo de cálculo em arquivo externo.

**Parâmetros:**
- `--scenario <1-5>`: Índice do cenário padronizado pré-configurado;
- `--format <html|md|json>`: Formato de saída do arquivo (padrão: `html`);
- `--output <caminho>`: Nome ou caminho de destino do arquivo gerado.

**Exemplo de Execução:**
```bash
eolo-cli report --scenario 1 --format html --output memorial_galpao.html
eolo-cli report --scenario 2 --format json --output resultado_torre.json
```

---

#### 3.3. Comando `isopleth`
Consulta a velocidade básica oficial e estações da FAB por Estado brasileiro.

**Parâmetros:**
- `--uf <SIGLA>`: Sigla da Unidade Federativa (ex: `PR`, `SP`, `SC`, `RJ`, `MG`, `RS`);
- *(Sem parâmetros)*: Lista todos os Estados e estações com as respectivas velocidades médias de isopletas.

**Exemplo de Execução:**
```bash
eolo-cli isopleth --uf PR
```

---

#### 3.4. Comando `verify`
Audita e valida um cenário ou arquivo de projeto estrutural em conformidade com as regras da ABNT NBR 6123:2023.

**Parâmetros:**
- `--scenario <1-5>`: Índice do cenário a ser verificado.

**Exemplo de Execução:**
```bash
eolo-cli verify --scenario 1
```

**Saída Típica de Verificação:**
```text
Auditando Cenário 1: Galpão Industrial Padrão...
[OK] Parâmetro V0 = 45.0 m/s dentro do mapa de isopletas.
[OK] Relação h/b = 0.187 enquadrada na Tabela 4 da NBR 6123:2023.
[OK] Inclinação alfa = 15.0° enquadrada na Tabela 5 (Duas Águas).
[OK] Coeficientes Cpe calculados com sucesso.
Status Geral: VERIFICAÇÃO ACADÊMICA CONCLUÍDA (Ref. ABNT NBR 6123:2023 — Conferir manualmente).
```

---

### 4. Códigos de Retorno do Processo (Exit Codes)

| Código | Significado |
|:---:|:---|
| `0` | Execução concluída com sucesso; projeto em conformidade. |
| `1` | Parâmetros de comando inválidos ou desconhecidos. |
| `2` | Erro de E/S de arquivo (permissão negada, caminho inexistente). |

---

### 5. Exemplo de Automação via Script PowerShell

```powershell
# Script de automação para cálculo em lote de 5 alturas
$alturas = @(5.0, 10.0, 15.0, 20.0, 30.0)
foreach ($z in $alturas) {
    Write-Host "Processando altura z = $z metros..." -ForegroundColor Cyan
    & ".\eolo-cli.exe" calc --v0 45.0 --cat 4 --class b --s3 1.0 -z $z
}
```

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the CLI reference in English</b></summary><br>

# Eolo Platform CLI (`eolo-cli.exe`) — Command-Line Interface Manual

**Version:** `2026.11.11.0`  
**Date:** `November 11, 2026`  
**Executable:** `builds/eolo/release/eolo-cli.exe`  
**Author:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  
**Licensing:** GNU AGPLv3 (Open Source) / Commercial License (Microsoft Store)  

---

### 1. Tool Overview

**`eolo-cli`** is the standalone, high-performance command-line utility for Eolo Platform. Designed for headless execution without launching a graphical window, it enables:
- Instant, automated calculation of wind load variables under **ABNT NBR 6123:2023**;
- Batch generation of engineering calculation memorials in open formats (HTML, Markdown, and JSON);
- Direct querying of the official isopleth database and Brazilian Air Force (FAB) weather stations;
- CI/CD integration and automated normative structural compliance verification using PowerShell, Python, or Bash scripts.

---

### 2. General Syntax & Available Commands

```bash
eolo-cli [COMMAND] [OPTIONS]
```

| Command | Description |
|:---|:---|
| `calc` | Computes characteristic wind velocity $V_k$ and basic dynamic stagnation pressure $q$. |
| `report` | Generates a formal engineering calculation memorial in HTML, Markdown, or JSON. |
| `isopleth` | Queries basic wind velocities ($V_0$) by Brazilian State (UF) or FAB meteorological stations. |
| `scenarios` | Lists the 5 pre-configured civil engineering reference presets. |
| `verify` | Audits and validates structural wind project parameters against NBR 6123:2023. |
| `test` | Runs the internal deterministic analytical test pipeline. |
| `help`, `-h` | Prints command syntax, flags, and usage examples. |

---

### 3. Detailed Commands & Options

#### 3.1. `calc` Command
Computes $V_k$ and $q$ from environmental factors and evaluation elevation $z$.

**Flags:**
- `--v0 <val>`: Basic wind velocity in $m/s$ (default: `45.0`);
- `--s1 <val>`: Topographic factor (default: `1.00`);
- `--cat <1-5>`: Terrain roughness category (1 to 5, default: `4`);
- `--class <a|b|c>`: Building size class (`a`, `b`, or `c`, default: `b`);
- `--s3 <val>`: Statistical occupancy factor (default: `1.00`);
- `-z <meters>`: Evaluation elevation in meters (default: `7.5`).

**Command Example:**
```bash
eolo-cli calc --v0 45.0 --s1 1.0 --cat 4 --class b --s3 1.0 -z 15.0
```

**Terminal Output:**
```text
=== EOLO PLATFORM CLI — NBR 6123:2023 ANALYTICAL CALCULATION ===
Basic Wind Velocity (V0):     45.00 m/s (162.00 km/h)
Topographic Factor (S1):      1.00
Roughness / Class (S2):       0.88 (Cat. IV, Class B at z = 15.0 m)
Statistical Factor (S3):      1.00 (Group 2 - Residential/Commercial)
------------------------------------------------------------
Characteristic Wind Speed Vk: 39.60 m/s (142.56 km/h)
Dynamic Pressure q:           961.42 N/m² (0.96 kN/m²)
============================================================
```

---

#### 3.2. `report` Command
Generates an auditable structural calculation memorial to a target file.

**Flags:**
- `--scenario <1-5>`: Index of pre-configured engineering scenario;
- `--format <html|md|json>`: Output format (default: `html`);
- `--output <path>`: Target output file path.

**Command Example:**
```bash
eolo-cli report --scenario 1 --format html --output warehouse_memorial.html
eolo-cli report --scenario 2 --format json --output residential_tower.json
```

---

#### 3.3. `isopleth` Command
Queries basic wind velocities and FAB stations across Brazilian states.

**Flags:**
- `--uf <CODE>`: Two-letter Brazilian state code (e.g., `PR`, `SP`, `SC`, `RJ`, `MG`, `RS`);
- *(No flags)*: Displays all states and meteorological stations with normative velocities.

**Command Example:**
```bash
eolo-cli isopleth --uf PR
```

---

#### 3.4. `verify` Command
Audits and validates an engineering scenario against normative constraints of ABNT NBR 6123:2023.

**Flags:**
- `--scenario <1-5>`: Index of the preset scenario to audit.

**Command Example:**
```bash
eolo-cli verify --scenario 1
```

**Verification Output:**
```text
Auditing Scenario 1: Standard Industrial Warehouse...
[OK] Parameter V0 = 45.0 m/s verified against isopleth boundaries.
[OK] Aspect ratio h/b = 0.187 matches NBR 6123:2023 Table 4.
[OK] Roof pitch alpha = 15.0° matches Table 5 (Gable Roof).
[OK] External pressure coefficients Cpe successfully evaluated.
Overall Status: ACADEMIC VERIFICATION COMPLETED (Ref. ABNT NBR 6123:2023 — Verify manually).
```

---

### 4. Process Exit Codes

| Exit Code | Meaning |
|:---:|:---|
| `0` | Execution completed successfully; project fully compliant. |
| `1` | Invalid arguments or unrecognized command flag. |
| `2` | File I/O error (access denied, path not found). |

---

### 5. Automated Batch Script Example (PowerShell)

```powershell
# Automated batch script computing dynamic pressures across 5 elevations
$elevations = @(5.0, 10.0, 15.0, 20.0, 30.0)
foreach ($z in $elevations) {
    Write-Host "Processing elevation z = $z meters..." -ForegroundColor Cyan
    & ".\eolo-cli.exe" calc --v0 45.0 --cat 4 --class b --s3 1.0 -z $z
}
```

</details>
