# Observações Técnicas / Technical Notes — Eolo Platform

<p align="center">
  <b>Selecione o idioma / Select language:</b><br>
  <a href="#ptbr">🇧🇷 Português (BR)</a> | <a href="#enus">🇺🇸 English (US)</a>
</p>

---

## <a id="ptbr"></a>🇧🇷 Português (BR)

<details open>
<summary><b>Clique para expandir / recolher as observações técnicas em português</b></summary><br>

# Eolo Platform — Notas de Engenharia de Software, Compilação e Empacotamento

Versão documentada: `2026.11.11.0`  
**Data:** `8 de novembro de 2026`  
**Autor:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. Isolamento dos Diretórios de Compilação (`.cargo/config.toml`)
Para preservar a raiz do repositório limpa e evitar dispersão de artefatos intermediários do compilador `rustc`:
- A configuração do Cargo (`.cargo/config.toml`) redireciona o diretório de destino (`target-dir`) para a pasta `builds/eolo/`.
- Os binários executáveis otimizados em modo release são gerados em:
  - `builds/eolo/release/eolo.exe` (Aplicativo desktop principal com GUI)
  - `builds/eolo/release/eolo-cli.exe` (Utilitário autônomo de linha de comando)

---

### 2. Recursos Win32, Metadados PE e Ícones (`src/build.rs`)
- O arquivo `src/build.rs` utiliza a biblioteca `winres` para compilar o ícone oficial multi-resolução (`src/assets/icons/eolo.ico`) diretamente na tabela de recursos PE (*Portable Executable*) do Windows;
- O `build.rs` também estampa metadados formais no cabeçalho binário (Versão `2026.11.8.0`, Copyright de Fernando Nillsson Cidade, Descrição do Produto e Nome da Empresa);
- Em tempo de execução, a função `carregar_icone_janela()` (`src/utils/icons.rs`) tenta carregar o ícone oficial a partir do sistema de arquivos e conta com fallback garantido em memória através de `include_bytes!`.

---

### 3. Registro do AppUserModelID no Windows 11
- Para garantir que o Eolo Platform seja reconhecido de forma única pelo Windows Shell e pela Barra de Tarefas do Windows 11, o ponto de entrada `src/main.rs` executa a chamada Win32:
  ```rust
  SetCurrentProcessExplicitAppUserModelID("FernandoCidade.EoloPlatform.NBR6123.2026")
  ```
- Essa configuração assegura o agrupamento correto de janelas, a exibição do ícone de alta densidade e a fixação estável de atalhos.

---

### 4. Empacotamento Inno Setup 7 (`repo/EoloPlatform.iss`)
- O instalador oficial para Windows é gerado através do script `repo/EoloPlatform.iss` compilado com o Inno Setup 7 (`ISCC.exe`);
- O instalador embute o ícone oficial, associa a extensão de arquivos de projeto `.eolo`, cadastra o `AppUserModelID` nos atalhos da Área de Trabalho e do Menu Iniciar, e embute todos os textos de licenças e documentações institucionais de `src/assets/`.

---

### 5. Sincronização de Versão (`version/`)
- A pasta `version/` contém um utilitário dedicado em Rust para validação e auditoria da numeração de versão em todos os arquivos de configuração, código e documentação:
  ```powershell
  cargo run --manifest-path version/Cargo.toml -- --check .
  ```

</details>

---

## <a id="enus"></a>🇺🇸 English (US)

<details>
<summary><b>Click to expand / collapse the technical notes in English</b></summary><br>

# Eolo Platform — Software Engineering, Build, and Packaging Notes

Documented version: `2026.11.11.0`  
**Date:** `November 8, 2026`  
**Author:** Fernando Nillsson Cidade (`linceu_lighthouse@outlook.com`)  

---

### 1. Isolated Build Target Directory (`.cargo/config.toml`)
To keep the source tree clean and avoid cluttering intermediate compiler artifacts:
- Cargo configuration (`.cargo/config.toml`) redirects the `target-dir` to `builds/eolo/`.
- Optimized production release binaries are produced at:
  - `builds/eolo/release/eolo.exe` (Main desktop GUI application)
  - `builds/eolo/release/eolo-cli.exe` (Headless command-line utility)

---

### 2. Win32 Resources, PE Metadata, and Icons (`src/build.rs`)
- The `src/build.rs` build script uses `winres` to compile the official multi-resolution icon (`src/assets/icons/eolo.ico`) directly into the Windows Portable Executable (PE) resource table;
- `build.rs` embeds formal metadata into the binary header (Version `2026.11.8.0`, Copyright by Fernando Nillsson Cidade, Product Description, and Company Name);
- At runtime, `carregar_icone_janela()` (`src/utils/icons.rs`) loads the official icon from disk with a resilient in-memory fallback using `include_bytes!`.

---

### 3. Windows Shell AppUserModelID Registration
- To ensure unique identification by the Windows 11 Taskbar and Explorer Shell, the entry point in `src/main.rs` invokes the native Win32 API:
  ```rust
  SetCurrentProcessExplicitAppUserModelID("FernandoCidade.EoloPlatform.NBR6123.2026")
  ```
- This guarantees proper window grouping, crisp titlebar and taskbar icon rendering, and reliable shortcut pinning behavior.

---

### 4. Inno Setup 7 Packaging (`repo/EoloPlatform.iss`)
- The traditional Windows installer is produced from `repo/EoloPlatform.iss` using Inno Setup 7 (`ISCC.exe`);
- The installer bundles the high-resolution icon, associates `.eolo` project files, registers the `AppUserModelID` on Desktop and Start Menu shortcuts, and packages all legal and institutional texts from `src/assets/`.

---

### 5. Version Synchronization (`version/`)
- The `version/` directory contains a dedicated Rust CLI tool that audits and synchronizes version numbers across all configuration files, source code, and documentation:
  ```powershell
  cargo run --manifest-path version/Cargo.toml -- --check .
  ```

</details>

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\src`

```
├── 📁 assets
│   ├── 📁 ABOUT
│   │   ├── 📄 ABOUT_en_US.txt
│   │   ├── 📄 ABOUT_pt_BR.txt
│   │   ├── 📄 History_APP_en_US.txt
│   │   └── 📄 History_APP_pt_BR.txt
│   ├── 📁 CLC
│   │   ├── 📄 CLC_en_US - Eolo.txt
│   │   └── 📄 CLC_pt_BR - Eolo.txt
│   ├── 📁 COPYRIGHT
│   │   ├── 📄 AVISO DE COPYRIGHT E MARCA REGISTRA_pt_BR.txt
│   │   └── 📄 COPYRIGHT AND TRADEMARK NOTICE_en_US.txt
│   ├── 📁 EULA
│   │   ├── 📄 EULA_en_US - Eolo.txt
│   │   └── 📄 EULA_pt_BR - Eolo.txt
│   ├── 📁 LICENSES
│   │   ├── 📄 Apache License Version 2.0.txt
│   │   ├── 📄 BSD License Clause-2.txt
│   │   ├── 📄 BSD License Clause-3.txt
│   │   ├── 📄 Boost Software License 1.0.txt
│   │   ├── 📄 GNU AFFERO GENERAL PUBLIC LICENSE Version3.txt
│   │   ├── 📄 GNU GENERAL PUBLIC LICENSE Version 3.txt
│   │   ├── 📄 GNU LESSER GENERAL PUBLIC LICENSE Version 2.1.txt
│   │   ├── 📄 GNU LESSER GENERAL PUBLIC LICENSE Version 3.txt
│   │   ├── 📄 ISC LICENSE.txt
│   │   ├── 📄 MIT License.txt
│   │   ├── 📄 PYTHON SOFTWARE FOUNDATION LICENSE.txt
│   │   ├── 📄 SIL Open Font License 1.1.txt
│   │   ├── 📄 SQLite Is Public Domain.txt
│   │   ├── 📄 Ubuntu Font Licence 1.0.txt
│   │   ├── 📄 Unicode License v3.txt
│   │   └── 📄 Zlib License.txt
│   ├── 📁 NOTICES
│   │   ├── 📄 NOTICE_en_US.txt
│   │   └── 📄 NOTICE_pt_BR.txt
│   ├── 📁 PRIVACY_POLICY
│   │   ├── 📄 Privacy_Policy_en_US.txt
│   │   └── 📄 Privacy_Policy_pt_BR.txt
│   ├── 📁 RELEASE
│   │   ├── 📄 RELEASE NOTES_en_US.txt
│   │   └── 📄 RELEASE NOTES_pt_BR.txt
│   └── 📁 icons
│       ├── 📄 eolo.ico
│       ├── 🖼️ eolo.png
│       ├── 🖼️ eolo_1080-1080.png
│       ├── 🖼️ eolo_1440-2160.png
│       ├── 🖼️ eolo_150-150.png
│       ├── 🖼️ eolo_1920-1080.png
│       ├── 🖼️ eolo_2160-2160.png
│       ├── 🖼️ eolo_300-300.png
│       ├── 🖼️ eolo_3840-2160.png
│       ├── 🖼️ eolo_584-800.png
│       └── 🖼️ eolo_71-71.png
├── 📁 core
│   ├── 🦀 mod.rs
│   ├── 🦀 project.rs
│   ├── 🦀 scenarios.rs
│   └── 🦀 types.rs
├── 📁 data
│   ├── 🦀 database.rs
│   └── 🦀 mod.rs
├── 📁 engine
│   ├── 🦀 cpe_cpi_tables.rs
│   ├── 🦀 envelopes.rs
│   ├── 🦀 factors_s1_s2_s3.rs
│   ├── 🦀 isopleth_map.rs
│   ├── 🦀 math3d.rs
│   ├── 🦀 mod.rs
│   └── 🦀 wind_core.rs
├── 📁 locale
│   ├── 🦀 manual.rs
│   ├── 🦀 mod.rs
│   └── 🦀 translations.rs
├── 📁 reports
│   ├── 🦀 memorial_generator.rs
│   └── 🦀 mod.rs
├── 📁 ui
│   ├── 📁 dialogs
│   │   ├── 🦀 about_dialog.rs
│   │   ├── 🦀 audit_dialog.rs
│   │   ├── 🦀 manual_dialog.rs
│   │   ├── 🦀 mod.rs
│   │   ├── 🦀 modeless.rs
│   │   ├── 🦀 quick_calc_dialog.rs
│   │   ├── 🦀 settings_dialog.rs
│   │   └── 🦀 units_dialog.rs
│   ├── 📁 tabs
│   │   ├── 🦀 mod.rs
│   │   ├── 🦀 tab1_location.rs
│   │   ├── 🦀 tab2_geometry.rs
│   │   ├── 🦀 tab3_openings.rs
│   │   ├── 🦀 tab4_canvas.rs
│   │   └── 🦀 tab5_memorial.rs
│   ├── 🦀 app.rs
│   ├── 🦀 menu_bar.rs
│   ├── 🦀 mod.rs
│   ├── 🦀 theme.rs
│   └── 🦀 toolbar.rs
├── 📁 utils
│   ├── 🦀 icons.rs
│   ├── 🦀 mod.rs
│   └── 🦀 paths.rs
├── 🦀 build.rs
├── 🦀 lib.rs
└── 🦀 main.rs
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\tests`

```
├── 🦀 dialog_render_test.rs
└── 🦀 normative_tests.rs
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\version`

```
├── 📁 .cargo
│   └── ⚙️ config.toml
├── 📁 src
│   ├── 🦀 app.rs
│   ├── 🦀 engine.rs
│   └── 🦀 main.rs
├── 📄 Cargo.lock
├── ⚙️ Cargo.toml
└── 📝 README.md
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\scripts`

```
├── 📄 gerar_distribuicao_limpa.ps1
└── 🐍 verify_assets_parity.py
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\repo`

```
├── 📁 LISTAGEM
│   ├── 📁 en_US
│   │   ├── 🖼️ Captura de tela 2026-11-08 021719.png
│   │   ├── 🖼️ Captura de tela 2026-11-08 021729.png
│   │   ├── 🖼️ Captura de tela 2026-11-08 021741.png
│   │   ├── 🖼️ Captura de tela 2026-11-08 021753.png
│   │   └── 🖼️ Captura de tela 2026-11-08 021804.png
│   └── 📁 pt_BR
│       ├── 🖼️ Captura de tela 2026-11-08 021625.png
│       ├── 🖼️ Captura de tela 2026-11-08 021635.png
│       ├── 🖼️ Captura de tela 2026-11-08 021645.png
│       ├── 🖼️ Captura de tela 2026-11-08 021654.png
│       └── 🖼️ Captura de tela 2026-11-08 021705.png
└── 📄 PRINCIPAIS_RECURSOS_EOLO.txt
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\docs`

```
├── 📝 CLA.md
├── 📝 MIGRATION_REPORT.md
├── 📝 architecture.md
├── 📝 cli-reference.md
├── 📝 compliance.md
├── 📝 licensing.md
├── 📝 release-v2026.11.8.0.md
├── 📝 security-model.md
├── 📝 suggested-issues.md
└── 📝 user-guide.md
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\.github`

```
├── 📁 ISSUE_TEMPLATE
│   ├── 📝 bug_report.md
│   ├── ⚙️ config.yml
│   ├── 📝 feature_request.md
│   └── 📝 question.md
├── 📁 workflows
│   └── ⚙️ update-readme-version.yml
└── 📝 PULL_REQUEST_TEMPLATE.md
```

**Root Path:** `c:\Users\ferna\APLICATIVOS\RS\EoloPlatform_rs\.cargo`

```
└── ⚙️ config.toml
```
