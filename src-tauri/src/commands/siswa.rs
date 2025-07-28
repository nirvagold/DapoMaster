use crate::app_state::DbPool;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use sqlx::types::Uuid as SqlxUuid;
use tauri::{AppHandle, State};
use uuid::Uuid;
use rand::seq::SliceRandom;
use chrono;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct PesertaDidik {
    pub peserta_didik_id: SqlxUuid,
    pub nama: String,
    pub jenis_kelamin: String,
    pub nisn: String,
    pub nik: Option<String>,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: chrono::NaiveDate,
    pub agama_id: i16,
}

#[derive(Deserialize)]
pub struct RegistrasiSiswaPayload {
    pub nama: String,
    pub nisn: Option<String>,
    pub jenis_kelamin: String,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: String,
    pub agama_id: i16,
    pub nipd: Option<String>,
    pub tanggal_masuk_sekolah: String,
    pub jenis_pendaftaran_id: BigDecimal,
    pub id_hobby: BigDecimal,
    pub id_cita: Option<BigDecimal>,
    pub a_pernah_paud: bool,
    pub a_pernah_tk: bool,
    pub sekolah_asal: Option<String>,
    pub alamat_jalan: String,
    pub desa_kelurahan: String,
    pub kode_wilayah: String,
    pub nama_ibu_kandung: String,
    pub kewarganegaraan: String,
    pub sekolah_id: String,
}

#[tauri::command]
pub async fn get_total_siswa(app: AppHandle, search: Option<String>, rombel_id: Option<SqlxUuid>, state: State<'_, DbPool>) -> Result<i64, String> {
    crate::emit_log(&app, &format!("CMD: get_total_siswa - Counting with search: {:?}, rombel: {:?}", search, rombel_id));
    let search_term = format!("%{}%", search.unwrap_or_default());
    let base_query = if rombel_id.is_some() { "SELECT COUNT(pd.*) FROM peserta_didik pd JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) AND ar.rombongan_belajar_id = $2" } else { "SELECT COUNT(*) FROM peserta_didik pd WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)" };
    let mut query = sqlx::query_scalar(base_query).bind(&search_term);
    if let Some(id) = rombel_id { query = query.bind(id); }
    query.fetch_one(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_daftar_siswa(app: AppHandle, page: usize, page_size: usize, search: Option<String>, rombel_id: Option<SqlxUuid>, state: State<'_, DbPool>) -> Result<Vec<PesertaDidik>, String> {
    crate::emit_log(&app, &format!("CMD: get_daftar_siswa - Fetching page {} with search: {:?}, rombel: {:?}", page, search, rombel_id));
    let offset = (page - 1) * page_size;
    let search_term = format!("%{}%", search.unwrap_or_default());
    let (query_str, use_rombel_filter) = if let Some(_id) = rombel_id { ("SELECT pd.* FROM peserta_didik pd JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) AND ar.rombongan_belajar_id = $4 ORDER BY pd.nama LIMIT $2 OFFSET $3".to_string(), true) } else { ("SELECT * FROM peserta_didik WHERE (nama ILIKE $1 OR nisn ILIKE $1) ORDER BY nama LIMIT $2 OFFSET $3".to_string(), false) };
    let mut query = sqlx::query_as::<_, PesertaDidik>(&query_str).bind(&search_term).bind(page_size as i64).bind(offset as i64);
    if use_rombel_filter { if let Some(id) = rombel_id { query = query.bind(id); } }
    query.fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn registrasi_siswa_baru(app: AppHandle, payload: RegistrasiSiswaPayload, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, "CMD: registrasi_siswa_baru - Starting registration process.");
    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Gagal mulai transaksi: {}", e));
            return Err(e.to_string());
        }
    };
    let peserta_didik_id = Uuid::new_v4();
    let tanggal_lahir_naive = match chrono::NaiveDate::parse_from_str(&payload.tanggal_lahir, "%Y-%m-%d") {
        Ok(tgl) => tgl,
        Err(e) => {
            crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Format tanggal_lahir salah: {}", e));
            return Err(format!("Format tanggal salah: {}", e));
        }
    };
    let id_cita_final = match payload.id_cita {
        Some(id) => id,
        None => {
            let all_citas: Vec<(BigDecimal,)> = match sqlx::query_as("SELECT id_cita FROM ref.jenis_cita").fetch_all(&mut *tx).await {
                Ok(citas) => citas,
                Err(e) => {
                    crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Gagal mengambil data cita-cita: {}", e));
                    return Err(e.to_string());
                }
            };
            if all_citas.is_empty() {
                crate::emit_log(&app, "CMD: registrasi_siswa_baru - ERROR: Tidak ada data referensi cita-cita untuk dipilih secara acak.");
                return Err("Tidak ada data referensi cita-cita untuk dipilih secara acak.".to_string());
            }
            all_citas.choose(&mut rand::thread_rng()).unwrap().0.clone()
        }
    };
    let insert_pd_result = sqlx::query("INSERT INTO peserta_didik (peserta_didik_id, nama, jenis_kelamin, tanggal_lahir, agama_id, kebutuhan_khusus_id, alamat_jalan, desa_kelurahan, kode_wilayah, penerima_kps, layak_pip, penerima_kip, kebutuhan_khusus_id_ayah, nama_ibu_kandung, kebutuhan_khusus_id_ibu, kewarganegaraan, create_date, last_update, soft_delete, updater_id, nisn, tempat_lahir) VALUES ($1, $2, $3, $4, $5, 0, $6, $7, $8, 0, 0, 0, 0, $9, 0, $10, NOW(), NOW(), 0, $11, $12, $13)")
        .bind(peserta_didik_id)
        .bind(&payload.nama)
        .bind(&payload.jenis_kelamin)
        .bind(tanggal_lahir_naive)
        .bind(payload.agama_id)
        .bind(&payload.alamat_jalan)
        .bind(&payload.desa_kelurahan)
        .bind(&payload.kode_wilayah)
        .bind(&payload.nama_ibu_kandung)
        .bind(&payload.kewarganegaraan)
        .bind(peserta_didik_id)
        .bind(&payload.nisn)
        .bind(&payload.tempat_lahir)
        .execute(&mut *tx)
        .await;
    if let Err(e) = insert_pd_result {
        crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Gagal insert ke peserta_didik: {}", e));
        tx.rollback().await.ok();
        return Err(format!("Gagal insert ke peserta_didik: {}", e));
    }
    let sekolah_id = Uuid::parse_str(&payload.sekolah_id).unwrap();
    let registrasi_id = Uuid::new_v4();
    let insert_reg_pd_result = sqlx::query("INSERT INTO registrasi_peserta_didik (registrasi_id, peserta_didik_id, sekolah_id, jenis_pendaftaran_id, nipd, tanggal_masuk_sekolah, sekolah_asal, id_hobby, id_cita, a_pernah_paud, a_pernah_tk, create_date, last_update, soft_delete, updater_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW(), 0, $12)")
        .bind(registrasi_id)
        .bind(peserta_didik_id)
        .bind(sekolah_id)
        .bind(&payload.jenis_pendaftaran_id)
        .bind(&payload.nipd)
        .bind(chrono::NaiveDate::parse_from_str(&payload.tanggal_masuk_sekolah, "%Y-%m-%d").unwrap())
        .bind(&payload.sekolah_asal)
        .bind(&payload.id_hobby)
        .bind(id_cita_final)
        .bind(payload.a_pernah_paud)
        .bind(payload.a_pernah_tk)
        .bind(peserta_didik_id)
        .execute(&mut *tx)
        .await;
    if let Err(e) = insert_reg_pd_result {
        crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Gagal insert ke registrasi_peserta_didik: {}", e));
        tx.rollback().await.ok();
        return Err(format!("Gagal insert ke registrasi_peserta_didik: {}", e));
    }
    if let Err(e) = tx.commit().await {
        crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - ERROR: Gagal commit transaksi: {}", e));
        return Err(e.to_string());
    }
    crate::emit_log(&app, &format!("CMD: registrasi_siswa_baru - Successfully registered student: {}", payload.nama));
    Ok(format!("Siswa {} berhasil diregistrasi.", payload.nama))
}

#[tauri::command]
pub async fn get_siswa_by_id(app: AppHandle, peserta_didik_id: SqlxUuid, state: State<'_, DbPool>) -> Result<PesertaDidik, String> {
    crate::emit_log(&app, &format!("CMD: get_siswa_by_id - Fetching student with ID: {}", peserta_didik_id));
    sqlx::query_as("SELECT * FROM peserta_didik WHERE peserta_didik_id = $1").bind(peserta_didik_id).fetch_one(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_siswa(app: AppHandle, peserta_didik_id: SqlxUuid, payload: RegistrasiSiswaPayload, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: update_siswa - Updating student with ID: {}", peserta_didik_id));
    let tanggal_lahir_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_lahir, "%Y-%m-%d").map_err(|e| format!("Format tanggal salah: {}", e))?;
    sqlx::query("UPDATE peserta_didik SET nama = $1, jenis_kelamin = $2, nisn = $3, tempat_lahir = $4, tanggal_lahir = $5, agama_id = $6, last_update = NOW() WHERE peserta_didik_id = $7").bind(&payload.nama).bind(&payload.jenis_kelamin).bind(&payload.nisn).bind(&payload.tempat_lahir).bind(tanggal_lahir_naive).bind(payload.agama_id).bind(peserta_didik_id).execute(&state.pool).await.map_err(|e| e.to_string())?;
    crate::emit_log(&app, &format!("CMD: update_siswa - Successfully updated student: {}", payload.nama));
    Ok(format!("Data siswa {} berhasil diperbarui.", payload.nama))
}

#[tauri::command]
pub async fn delete_siswa(app: AppHandle, peserta_didik_id: SqlxUuid, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: delete_siswa - Deleting student with ID: {}", peserta_didik_id));
    sqlx::query("DELETE FROM anggota_rombel WHERE peserta_didik_id = $1").bind(peserta_didik_id).execute(&state.pool).await.map_err(|e| format!("Gagal menghapus dari anggota_rombel: {}", e))?;
    sqlx::query("DELETE FROM registrasi_peserta_didik WHERE peserta_didik_id = $1").bind(peserta_didik_id).execute(&state.pool).await.map_err(|e| format!("Gagal menghapus dari registrasi_peserta_didik: {}", e))?;
    sqlx::query("DELETE FROM peserta_didik WHERE peserta_didik_id = $1").bind(peserta_didik_id).execute(&state.pool).await.map_err(|e| e.to_string())?;
    crate::emit_log(&app, &format!("CMD: delete_siswa - Successfully deleted student: {}", peserta_didik_id));
    Ok("Data siswa berhasil dihapus.".to_string())
} 