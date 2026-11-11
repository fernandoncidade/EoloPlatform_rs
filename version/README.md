# Editor de Versões e Datas Multi-idioma (version-editor)
**Eolo Platform — Módulo Version & Ferramentas de Desenvolvimento**

Este módulo é o utilitário nativo em **Rust** e **egui** para auditoria e sincronização de versões do projeto **Eolo Platform**.

---

## 🎯 Objetivo e Funcionalidades

O **`version-editor`** automatiza a auditoria, verificação e atualização sincronizada de **versões** e **datas** nos idiomas suportados (**Português `pt-BR`** e **Inglês `en-US`**) em todos os arquivos do repositório:
- **Documentação e Textos Institucionais / Legais**: `.txt` (`PRIVACY_POLICY`, `EULA`, `CLC`, `ABOUT`, `NOTICES`)
- **Documentação Técnica Modular**: `.md` (`architecture.md`, `cli-reference.md`, `compliance.md`, `licensing.md`, `security-model.md`, `user-guide.md`, `release-v*.md`, `MIGRATION_REPORT.md`)
- **Documentação de Raiz**: `README.md`, `SECURITY.md`, `MANUAL.md`, `OBSERVACAO.md`
- **Código Fonte do Projeto**: `.rs` (`src/ui/dialogs/about_dialog.rs`, `src/bin/main_cli.rs`, `src/locale/manual.rs`)
- **Manifestos e Travas de Dependência**: `.toml` (`Cargo.toml`) e `.lock` (`Cargo.lock`)
- **Scripts de Instalação e Empacotamento**: `.iss` (`repo/Eolo*.iss`)

---

## 🏗️ Compilação Separada em `builds/version`

Conforme a especificação, este código possui uma compilação separada com seus próprios artefatos e binários isolados na pasta:
```text
.\builds\version\
```

### 1. Compilação via Cargo

Para compilar o binário `version-editor` em modo debug:
```powershell
cargo build --manifest-path version/Cargo.toml --target-dir builds/version
```

Para compilar em modo otimizado (release):
```powershell
cargo build --release --manifest-path version/Cargo.toml --target-dir builds/version
```

O binário final será gerado em:
- Debug: `builds\version\debug\version-editor.exe`
- Release: `builds\version\release\version-editor.exe`

---

## 💻 Modos de Uso

### 1. Interface Gráfica Interativa (GUI com `egui`)
Basta executar o binário sem argumentos:
```powershell
.\builds\version\release\version-editor.exe
```

Recursos da GUI:
- Seleção de diretório raiz com navegação de pastas nativa.
- Cartões de edição para os 2 idiomas oficiais (🇧🇷 `pt-BR` e 🇺🇸 `en-US`).
- Sincronização bidirecional entre campos de Data (Dia, Mês localizado, Ano) e Versão (`YYYY.M.D.rev`).
- Botão **"🔍 Verificar Status dos Arquivos"**: audita existência e conformidade de padrões.
- Botão **"🔄 Recarregar Dados"**: extrai versões e datas gravadas nos arquivos de disco.
- Botão **"💾 Salvar Alterações em Todos"**: grava atualizações em lote e recarrega os dados.
- Botão **"🔗 Sincronizar Versão PT"**: propaga versão e data em português para os demais idiomas.
- Botão **"📅 Hoje para Todos"**: configura a data do sistema em todos os idiomas.
- Abas organizadas por grupo: `📊 Geral`, `🔒 PRIVACY_POLICY`, `📜 EULA`, `💼 CLC`, `ℹ️ ABOUT`, `📢 NOTICES`, `📚 DOCS`, `📁 RAIZ`, `🦀 SRC_RUST`, `⚙️ MANIFESTS` e `📦 REPO`.

### 2. Linha de Comando (CLI Headless)

#### Exibir Ajuda:
```powershell
.\builds\version\release\version-editor.exe --help
```

#### Verificar Integridade e Padrões dos Arquivos:
```powershell
.\builds\version\release\version-editor.exe --check [caminho_raiz]
```

#### Aplicar Atualizações em Lote:
```powershell
.\builds\version\release\version-editor.exe --apply [caminho_raiz] [versão_opcional]
```
Se a versão não for especificada, a versão atual do sistema será gerada automaticamente (`YYYY.M.D.0`).
