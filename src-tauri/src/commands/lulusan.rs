use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid as SqlxUuid;
use chrono::NaiveDate;
use uuid::Uuid;
use bigdecimal::BigDecimal;

#[derive(Serialize)]
pub struct SiswaLulus {
    pub peserta_didik_id: SqlxUuid,
    pub nama: String,
    pub nisn: String,
    pub tanggal_lahir: NaiveDate,
    pub nama_ayah: Option<String>,
    pub nama_ibu_kandung: String,
    pub jenis_ijazah_id: Option<BigDecimal>,
    pub nama_ijazah: Option<String>,
    pub nomor: Option<String>,
    pub penandatangan: Option<String>,
    pub tanggal_tanda_tangan: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct UpdateIjazahPayload {
    pub peserta_didik_id: String,
    pub jenis_ijazah_id: Option<String>,
    pub nomor: Option<String>,
    pub penandatangan: Option<String>,
    pub tanggal_tanda_tangan: Option<String>,
}

#[derive(Deserialize)]
pub struct BulkUpdateIjazahPayload {
    pub updates: Vec<UpdateIjazahPayload>,
}

#[tauri::command]
pub async fn get_total_siswa_lulus(
    app: AppHandle,
    state: State<'_, DbPool>,
    search: Option<String>,
) -> Result<i64, String> {
    crate::emit_log(&app, "CMD: get_total_siswa_lulus - Fetching total count.");

    let query = if let Some(ref search_term) = search {
        if search_term.is_empty() {
            "SELECT COUNT(*) FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1'"
        } else {
            "SELECT COUNT(*) FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1' 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)"
        }
    } else {
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1'"
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
    crate::emit_log(&app, &format!("CMD: get_total_siswa_lulus - Total: {}", total));
    Ok(total)
}

#[tauri::command]
pub async fn get_daftar_siswa_lulus(
    app: AppHandle,
    state: State<'_, DbPool>,
    page: i64,
    page_size: i64,
    search: Option<String>,
) -> Result<Vec<SiswaLulus>, String> {
    crate::emit_log(&app, "CMD: get_daftar_siswa_lulus - Fetching data.");

    let offset = (page - 1) * page_size;
    let query = if let Some(ref search_term) = search {
        if search_term.is_empty() {
            "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.tanggal_lahir,
                    pd.nama_ayah, pd.nama_ibu_kandung, ip.jenis_ijazah_id,
                    ji.nama as nama_ijazah, ip.nomor, ip.penandatangan,
                    ip.tanggal_ttd
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ijazah_pd ip ON rpd.registrasi_id = ip.registrasi_id
             LEFT JOIN ref.jenis_ijazah ji ON ip.jenis_ijazah_id = ji.jenis_ijazah_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1'
             ORDER BY pd.nama
             LIMIT $1 OFFSET $2"
        } else {
            "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.tanggal_lahir,
                    pd.nama_ayah, pd.nama_ibu_kandung, ip.jenis_ijazah_id,
                    ji.nama as nama_ijazah, ip.nomor, ip.penandatangan,
                    ip.tanggal_ttd
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ijazah_pd ip ON rpd.registrasi_id = ip.registrasi_id
             LEFT JOIN ref.jenis_ijazah ji ON ip.jenis_ijazah_id = ji.jenis_ijazah_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1' 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)
             ORDER BY pd.nama
             LIMIT $2 OFFSET $3"
        }
    } else {
        "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.tanggal_lahir,
                pd.nama_ayah, pd.nama_ibu_kandung, ip.jenis_ijazah_id,
                ji.nama as nama_ijazah, ip.nomor, ip.penandatangan,
                ip.tanggal_ttd
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ijazah_pd ip ON rpd.registrasi_id = ip.registrasi_id
         LEFT JOIN ref.jenis_ijazah ji ON ip.jenis_ijazah_id = ji.jenis_ijazah_id
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1'
         ORDER BY pd.nama
         LIMIT $1 OFFSET $2"
    };

    let result = if let Some(ref search_term) = search {
        if !search_term.is_empty() {
            let search_pattern = format!("%{}%", search_term);
            sqlx::query_as::<_, (SqlxUuid, String, String, NaiveDate, Option<String>, String, Option<BigDecimal>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>)>(query)
                .bind(search_pattern)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&state.pool)
                .await
        } else {
            sqlx::query_as::<_, (SqlxUuid, String, String, NaiveDate, Option<String>, String, Option<BigDecimal>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>)>(query)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&state.pool)
                .await
        }
    } else {
        sqlx::query_as::<_, (SqlxUuid, String, String, NaiveDate, Option<String>, String, Option<BigDecimal>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>)>(query)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&state.pool)
            .await
    };

    let rows = result.map_err(|e| e.to_string())?;
    
    let siswa_lulus: Vec<SiswaLulus> = rows
        .into_iter()
        .map(|row| SiswaLulus {
            peserta_didik_id: row.0,
            nama: row.1,
            nisn: row.2,
            tanggal_lahir: row.3,
            nama_ayah: row.4,
            nama_ibu_kandung: row.5,
            jenis_ijazah_id: row.6,
            nama_ijazah: row.7,
            nomor: row.8,
            penandatangan: row.9,
            tanggal_tanda_tangan: row.10,
        })
        .collect();

    crate::emit_log(&app, &format!("CMD: get_daftar_siswa_lulus - Fetched {} records", siswa_lulus.len()));
    Ok(siswa_lulus)
}

#[tauri::command]
pub async fn update_bulk_ijazah(
    app: AppHandle,
    state: State<'_, DbPool>,
    payload: BulkUpdateIjazahPayload,
) -> Result<String, String> {
    crate::emit_log(&app, "CMD: update_bulk_ijazah - Starting bulk update process.");

    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;
    let mut success_count = 0;

    for update in payload.updates {
        let peserta_didik_id = match Uuid::parse_str(&update.peserta_didik_id) {
            Ok(id) => id,
            Err(_) => {
                crate::emit_log(&app, &format!("CMD: update_bulk_ijazah - Invalid UUID: {}", update.peserta_didik_id));
                continue;
            }
        };

        let tanggal_tanda_tangan = if let Some(tgl_str) = &update.tanggal_tanda_tangan {
            match NaiveDate::parse_from_str(tgl_str, "%Y-%m-%d") {
                Ok(tgl) => Some(tgl),
                Err(_) => {
                    crate::emit_log(&app, &format!("CMD: update_bulk_ijazah - Invalid date format: {}", tgl_str));
                    continue;
                }
            }
        } else {
            None
        };

        let result = sqlx::query(
            "UPDATE ijazah_pd 
             SET jenis_ijazah_id = $1, nomor = $2, penandatangan = $3, tanggal_ttd = $4, last_update = NOW()
             WHERE registrasi_id = (SELECT registrasi_id FROM registrasi_peserta_didik WHERE peserta_didik_id = $5)"
        )
        .bind(&update.jenis_ijazah_id)
        .bind(&update.nomor)
        .bind(&update.penandatangan)
        .bind(tanggal_tanda_tangan)
        .bind(peserta_didik_id)
        .execute(&mut *tx)
        .await;

        match result {
            Ok(_) => {
                success_count += 1;
                crate::emit_log(&app, &format!("CMD: update_bulk_ijazah - Successfully updated peserta_didik_id: {}", peserta_didik_id));
            }
            Err(e) => {
                crate::emit_log(&app, &format!("CMD: update_bulk_ijazah - Error updating peserta_didik_id {}: {}", peserta_didik_id, e));
            }
        }
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    crate::emit_log(&app, &format!("CMD: update_bulk_ijazah - Completed. Successfully updated {} records", success_count));
    Ok(format!("Berhasil mengupdate {} data ijazah", success_count))
}

#[derive(Serialize)]
pub struct JenisIjazah {
    pub jenis_ijazah_id: BigDecimal,
    pub nama: String,
}

#[tauri::command]
pub async fn get_all_jenis_ijazah(
    app: AppHandle,
    state: State<'_, DbPool>,
) -> Result<Vec<JenisIjazah>, String> {
    crate::emit_log(&app, "CMD: get_all_jenis_ijazah - Fetching all jenis ijazah.");

    let rows = sqlx::query_as::<_, (BigDecimal, String)>(
        "SELECT jenis_ijazah_id, nama FROM ref.jenis_ijazah ORDER BY nama"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let jenis_ijazah: Vec<JenisIjazah> = rows
        .into_iter()
        .map(|row| JenisIjazah {
            jenis_ijazah_id: row.0,
            nama: row.1,
        })
        .collect();

    crate::emit_log(&app, &format!("CMD: get_all_jenis_ijazah - Found {} jenis ijazah", jenis_ijazah.len()));
    Ok(jenis_ijazah)
}