use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::Serialize;
use sqlx::types::Uuid as SqlxUuid;
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct SiswaKeluar {
    pub peserta_didik_id: SqlxUuid,
    pub nama: String,
    pub nisn: String,
    pub nik: Option<String>,
    pub tanggal_lahir: NaiveDate,
    pub nama_ayah: Option<String>,
    pub nama_ibu_kandung: String,
    pub jenis_keluar_id: String,
    pub ket_keluar: String,
    pub tanggal_keluar: Option<NaiveDate>,
}

#[tauri::command]
pub async fn get_total_siswa_keluar(
    app: AppHandle,
    state: State<'_, DbPool>,
    search: Option<String>,
) -> Result<i64, String> {
    crate::emit_log(&app, "CMD: get_total_siswa_keluar - Fetching total count.");

    let query = if let Some(ref search_term) = search {
        if search_term.is_empty() {
            "SELECT COUNT(*) FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1"
        } else {
            "SELECT COUNT(*) FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)"
        }
    } else {
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1"
    };

    let result = if let Some(ref search_term) = search {
        if !search_term.is_empty() {
            let search_pattern = format!("%{}%", search_term);
            sqlx::query_scalar(query)
                .bind(search_pattern)
                .fetch_one(&state.pool)
                .await
        } else {
            sqlx::query_scalar(query)
                .fetch_one(&state.pool)
                .await
        }
    } else {
        sqlx::query_scalar(query)
            .fetch_one(&state.pool)
            .await
    };

    let total: i64 = result.map_err(|e| e.to_string())?;
    crate::emit_log(&app, &format!("CMD: get_total_siswa_keluar - Total: {}", total));
    Ok(total)
}

#[tauri::command]
pub async fn get_daftar_siswa_keluar(
    app: AppHandle,
    state: State<'_, DbPool>,
    page: i64,
    page_size: i64,
    search: Option<String>,
) -> Result<Vec<SiswaKeluar>, String> {
    crate::emit_log(&app, "CMD: get_daftar_siswa_keluar - Fetching data.");

    let offset = (page - 1) * page_size;
    let query = if let Some(ref search_term) = search {
        if search_term.is_empty() {
            "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                    pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                    jk.nama as ket_keluar, rpd.tanggal_keluar
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1
             ORDER BY pd.nama
             LIMIT $1 OFFSET $2"
        } else {
            "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                    pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                    jk.nama as ket_keluar, rpd.tanggal_keluar
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)
             ORDER BY pd.nama
             LIMIT $2 OFFSET $3"
        }
    } else {
        "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                jk.nama as ket_keluar, rpd.tanggal_keluar
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != 1
         ORDER BY pd.nama
         LIMIT $1 OFFSET $2"
    };

    let result = if let Some(ref search_term) = search {
        if !search_term.is_empty() {
            let search_pattern = format!("%{}%", search_term);
            sqlx::query_as::<_, (SqlxUuid, String, String, Option<String>, NaiveDate, Option<String>, String, String, Option<String>, Option<NaiveDate>)>(query)
                .bind(search_pattern)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&state.pool)
                .await
        } else {
            sqlx::query_as::<_, (SqlxUuid, String, String, Option<String>, NaiveDate, Option<String>, String, String, Option<String>, Option<NaiveDate>)>(query)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&state.pool)
                .await
        }
    } else {
        sqlx::query_as::<_, (SqlxUuid, String, String, Option<String>, NaiveDate, Option<String>, String, String, Option<String>, Option<NaiveDate>)>(query)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&state.pool)
            .await
    };

    let rows = result.map_err(|e| e.to_string())?;
    
    let siswa_keluar: Vec<SiswaKeluar> = rows
        .into_iter()
        .map(|row| SiswaKeluar {
            peserta_didik_id: row.0,
            nama: row.1,
            nisn: row.2,
            nik: row.3,
            tanggal_lahir: row.4,
            nama_ayah: row.5,
            nama_ibu_kandung: row.6,
            jenis_keluar_id: row.7,
            ket_keluar: row.8.unwrap_or_else(|| "Tidak diketahui".to_string()),
            tanggal_keluar: row.9,
        })
        .collect();

    crate::emit_log(&app, &format!("CMD: get_daftar_siswa_keluar - Fetched {} records", siswa_keluar.len()));
    Ok(siswa_keluar)
}