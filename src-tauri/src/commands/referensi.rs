use crate::app_state::DbPool;
use bigdecimal::BigDecimal;
use sqlx::types::Uuid as SqlxUuid;
use tauri::{AppHandle, State};

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct RombonganBelajar {
    pub rombongan_belajar_id: SqlxUuid,
    pub nama: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Agama {
    pub agama_id: i16,
    pub nama: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct JenisPendaftaran {
    pub jenis_pendaftaran_id: BigDecimal,
    pub nama: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Hobby {
    pub id_hobby: BigDecimal,
    pub nm_hobby: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Cita {
    pub id_cita: BigDecimal,
    pub nm_cita: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct WilayahReferensi {
    pub kode_wilayah: String,
    pub nama: String,
    pub id_level_wilayah: i16,
    pub mst_kode_wilayah: Option<String>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Semester {
    pub semester_id: String,
    pub nama: String,
    pub tahun_ajaran_id: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct TahunAjaran {
    pub tahun_ajaran_id: String,
    pub nama: String,
}

#[tauri::command]
pub async fn get_all_rombels(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<RombonganBelajar>, String> {
    crate::emit_log(&app, "CMD: get_all_rombels - Fetching...");
    sqlx::query_as("SELECT rombongan_belajar_id, nama FROM rombongan_belajar ORDER BY nama")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_agama(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<Agama>, String> {
    crate::emit_log(&app, "CMD: get_all_agama - Fetching...");
    sqlx::query_as("SELECT agama_id, nama FROM ref.agama ORDER BY agama_id")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_jenis_pendaftaran(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<JenisPendaftaran>, String> {
    crate::emit_log(&app, "CMD: get_all_jenis_pendaftaran - Fetching...");
    sqlx::query_as("SELECT jenis_pendaftaran_id, nama FROM ref.jenis_pendaftaran")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_hobby(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<Hobby>, String> {
    crate::emit_log(&app, "CMD: get_all_hobby - Fetching...");
    sqlx::query_as("SELECT id_hobby, nm_hobby FROM ref.jenis_hobby")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_cita(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<Cita>, String> {
    crate::emit_log(&app, "CMD: get_all_cita - Fetching...");
    sqlx::query_as("SELECT id_cita, nm_cita FROM ref.jenis_cita")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_wilayah_by_level_and_parent(app: AppHandle, level: i16, parent: Option<String>, state: State<'_, DbPool>) -> Result<Vec<WilayahReferensi>, String> {
    crate::emit_log(&app, &format!("CMD: get_wilayah_by_level_and_parent - level: {}, parent: {:?}", level, parent));
    let query = if let Some(parent_kode) = parent {
        sqlx::query_as::<_, WilayahReferensi>("SELECT kode_wilayah, nama, id_level_wilayah, mst_kode_wilayah FROM ref.mst_wilayah WHERE id_level_wilayah = $1 AND mst_kode_wilayah = $2 ORDER BY nama")
            .bind(level)
            .bind(parent_kode)
    } else {
        sqlx::query_as::<_, WilayahReferensi>("SELECT kode_wilayah, nama, id_level_wilayah, mst_kode_wilayah FROM ref.mst_wilayah WHERE id_level_wilayah = $1 ORDER BY nama")
            .bind(level)
    };
    let wilayah = query.fetch_all(&state.pool).await.map_err(|e| e.to_string())?;
    Ok(wilayah)
}

#[tauri::command]
pub async fn get_all_semester(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<Semester>, String> {
    crate::emit_log(&app, "CMD: get_all_semester - Fetching...");
    sqlx::query_as("SELECT semester_id, nama, tahun_ajaran_id FROM ref.semester ORDER BY tahun_ajaran_id DESC, semester_id DESC")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_tahun_ajaran(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<TahunAjaran>, String> {
    crate::emit_log(&app, "CMD: get_all_tahun_ajaran - Fetching...");
    sqlx::query_as("SELECT tahun_ajaran_id, nama FROM ref.tahun_ajaran ORDER BY nama DESC")
        .fetch_all(&state.pool).await.map_err(|e| e.to_string())
} 