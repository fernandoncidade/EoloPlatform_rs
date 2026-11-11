use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

use crate::core::project::EoloProject;

#[derive(Debug, Clone)]
pub struct ProjectRecord {
    pub id: i64,
    pub title: String,
    pub engineer: String,
    pub location: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct CalculationHistoryRecord {
    pub id: i64,
    pub project_name: String,
    pub vk_ms: f64,
    pub q_kn_m2: f64,
    pub roof_uplift_kn_m2: f64,
    pub timestamp: String,
}

pub struct EoloDatabase {
    conn: Connection,
}

impl EoloDatabase {
    /// Inicializa a base SQLite local no caminho padrão do sistema ou em memória
    pub fn open_default() -> Result<Self> {
        let db_path = Self::default_db_path();
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(&db_path)?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    /// Inicializa uma base SQLite em memória para testes ou modo sem persistência em disco
    pub fn open_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn default_db_path() -> PathBuf {
        let fallback = dirs_fallback();
        let legacy_db = fallback.join("Eolo_rs").join("eolo_data.db");
        let new_dir = fallback.join("EoloPlatform");
        let new_db = new_dir.join("eolo_data.db");

        // Migração transparente caso a base antiga exista e a nova ainda não
        if !new_db.exists() && legacy_db.exists() {
            let _ = std::fs::create_dir_all(&new_dir);
            let _ = std::fs::copy(&legacy_db, &new_db);
        }

        new_db
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                engineer TEXT NOT NULL,
                city TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS calculation_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_name TEXT NOT NULL,
                vk_ms REAL NOT NULL,
                q_kn_m2 REAL NOT NULL,
                roof_uplift_kn_m2 REAL NOT NULL,
                timestamp TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn save_project(&self, project: &EoloProject) -> Result<i64> {
        let json_str = serde_json::to_string_pretty(project).unwrap_or_default();
        let now = chrono::Local::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO projects (title, engineer, city, state, created_at, updated_at, data_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                project.metadata.title,
                project.metadata.engineer,
                project.metadata.location_city,
                project.metadata.location_state,
                now,
                now,
                json_str
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_projects(&self) -> Result<Vec<ProjectRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, engineer, city || '/' || state as loc, updated_at
             FROM projects ORDER BY id DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ProjectRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                engineer: row.get(2)?,
                location: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn load_project(&self, id: i64) -> Result<Option<EoloProject>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data_json FROM projects WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let json_str: String = row.get(0)?;
            if let Ok(proj) = serde_json::from_str(&json_str) {
                return Ok(Some(proj));
            }
        }
        Ok(None)
    }

    pub fn delete_project(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn log_calculation(
        &self,
        name: &str,
        vk: f64,
        q: f64,
        roof_uplift: f64,
    ) -> Result<()> {
        let now = chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO calculation_history (project_name, vk_ms, q_kn_m2, roof_uplift_kn_m2, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, vk, q, roof_uplift, now],
        )?;
        Ok(())
    }
}

fn dirs_fallback() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata)
    } else {
        PathBuf::from(".")
    }
}
