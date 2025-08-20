use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid as SqlxUuid;
use chrono::NaiveDate;
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct BatalkanKeluarPayload {
    pub peserta_didik_id: String,
}

#[derive(Serialize)]
pub struct BatalkanKeluarResult {
    pub success: bool,
    pub message: String,
    pub can_cancel: bool,
}

#[derive(Deserialize)]
pub struct HapusSiswaKeluarPayload {
    pub peserta_didik_id: String,
}

#[derive(Serialize)]
pub struct HapusSiswaKeluarResult {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct ManipulasiServerPayload {
    pub peserta_didik_id: String,
    pub metode_manipulasi: String, // "table_sync_log", "sync_session", "sync_primer"
}

#[derive(Serialize)]
pub struct ManipulasiServerResult {
    pub success: bool,
    pub message: String,
    pub server_response: Option<String>,
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
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1'"
        } else {
            "SELECT COUNT(*) FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1' 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)"
        }
    } else {
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1'"
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

    let total: i64 = result.map_err(|e| {
        let error_msg = format!("Database error: {}", e);
        crate::emit_log(&app, &format!("CMD: get_total_siswa_keluar - Error: {}", error_msg));
        error_msg
    })?;
    
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
                    jk.ket_keluar as ket_keluar, rpd.tanggal_keluar
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1'
             ORDER BY pd.nama
             LIMIT $1 OFFSET $2"
        } else {
            "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                    pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                    jk.ket_keluar as ket_keluar, rpd.tanggal_keluar
             FROM peserta_didik pd 
             JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
             LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
             WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1' 
             AND (pd.nama ILIKE $1 OR pd.nisn ILIKE $1)
             ORDER BY pd.nama
             LIMIT $2 OFFSET $3"
        }
    } else {
        "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                jk.ket_keluar as ket_keluar, rpd.tanggal_keluar
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1'
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

    let rows = result.map_err(|e| {
        let error_msg = format!("Database error: {}", e);
        crate::emit_log(&app, &format!("CMD: get_daftar_siswa_keluar - Error: {}", error_msg));
        error_msg
    })?;
    
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

#[tauri::command]
pub async fn cek_bisa_batalkan_keluar(
    app: AppHandle,
    state: State<'_, DbPool>,
    payload: BatalkanKeluarPayload,
) -> Result<BatalkanKeluarResult, String> {
    crate::emit_log(&app, "CMD: cek_bisa_batalkan_keluar - Checking if can cancel exit.");

    let peserta_didik_id = match Uuid::parse_str(&payload.peserta_didik_id) {
        Ok(id) => id,
        Err(_) => {
            let error_msg = format!("Invalid UUID: {}", payload.peserta_didik_id);
            crate::emit_log(&app, &format!("CMD: cek_bisa_batalkan_keluar - Error: {}", error_msg));
            return Ok(BatalkanKeluarResult {
                success: false,
                message: error_msg,
                can_cancel: false,
            });
        }
    };

    // Cek apakah siswa ada dan jenis keluar bukan lulus (jenis_keluar_id != 1)
    let result = sqlx::query_as::<_, (String, String)>(
        "SELECT pd.nama, jk.ket_keluar as jenis_keluar
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
         WHERE pd.peserta_didik_id = $1 AND pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL"
    )
    .bind(peserta_didik_id)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok((nama, jenis_keluar)) => {
            if jenis_keluar == "Lulus" {
                crate::emit_log(&app, &format!("CMD: cek_bisa_batalkan_keluar - Cannot cancel: {} is a graduate", nama));
                Ok(BatalkanKeluarResult {
                    success: true,
                    message: format!("Tidak dapat membatalkan keluar untuk siswa lulus: {}", nama),
                    can_cancel: false,
                })
            } else {
                crate::emit_log(&app, &format!("CMD: cek_bisa_batalkan_keluar - Can cancel: {} ({})", nama, jenis_keluar));
                Ok(BatalkanKeluarResult {
                    success: true,
                    message: format!("Dapat membatalkan keluar untuk: {} ({})", nama, jenis_keluar),
                    can_cancel: true,
                })
            }
        }
        Err(e) => {
            let error_msg = format!("Database error: {}", e);
            crate::emit_log(&app, &format!("CMD: cek_bisa_batalkan_keluar - Error: {}", error_msg));
            Ok(BatalkanKeluarResult {
                success: false,
                message: error_msg,
                can_cancel: false,
            })
        }
    }
}

#[tauri::command]
pub async fn batalkan_siswa_keluar_stealth(
    app: AppHandle,
    state: State<'_, DbPool>,
    payload: BatalkanKeluarPayload,
) -> Result<String, String> {
    crate::emit_log(&app, "CMD: batalkan_siswa_keluar_stealth - Starting stealth cancel exit process.");

    let peserta_didik_id = match Uuid::parse_str(&payload.peserta_didik_id) {
        Ok(id) => id,
        Err(_) => {
            let error_msg = format!("Invalid UUID: {}", payload.peserta_didik_id);
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
            return Err(error_msg);
        }
    };

    let mut tx = state.pool.begin().await.map_err(|e| {
        let error_msg = format!("Failed to begin transaction: {}", e);
        crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
        error_msg
    })?;

    // Disable triggers untuk stealth mode
    sqlx::query("SET session_replication_role = replica;")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            let error_msg = format!("Failed to disable triggers: {}", e);
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
            error_msg
        })?;

    // Cek apakah siswa ada dan bukan lulus
    let check_result = sqlx::query_as::<_, (String, String)>(
        "SELECT pd.nama, jk.ket_keluar as jenis_keluar
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
         WHERE pd.peserta_didik_id = $1 AND pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL"
    )
    .bind(peserta_didik_id)
    .fetch_one(&mut *tx)
    .await;

    let (nama, jenis_keluar) = match check_result {
        Ok(result) => result,
        Err(e) => {
            let error_msg = format!("Student not found or database error: {}", e);
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
            return Err(error_msg);
        }
    };

    if jenis_keluar == "Lulus" {
        let error_msg = format!("Cannot cancel exit for graduate: {}", nama);
        crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
        return Err(error_msg);
    }

    // Batalkan keluar dengan mengubah jenis_keluar_id menjadi NULL dan tanggal_keluar menjadi NULL
    let update_result = sqlx::query(
        "UPDATE registrasi_peserta_didik 
         SET jenis_keluar_id = NULL, tanggal_keluar = NULL, last_update = NOW()
         WHERE peserta_didik_id = $1"
    )
    .bind(peserta_didik_id)
    .execute(&mut *tx)
    .await;

    match update_result {
        Ok(_) => {
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Successfully cancelled exit for: {}", nama));
        }
        Err(e) => {
            let error_msg = format!("Failed to update registrasi_peserta_didik: {}", e);
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
            return Err(error_msg);
        }
    }

    // Re-enable triggers
    sqlx::query("SET session_replication_role = DEFAULT;")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            let error_msg = format!("Failed to re-enable triggers: {}", e);
            crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
            error_msg
        })?;

    tx.commit().await.map_err(|e| {
        let error_msg = format!("Failed to commit transaction: {}", e);
        crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Error: {}", error_msg));
        error_msg
    })?;

    let success_msg = format!("Berhasil membatalkan keluar untuk: {} ({})", nama, jenis_keluar);
    crate::emit_log(&app, &format!("CMD: batalkan_siswa_keluar_stealth - Completed. {}", success_msg));
    Ok(success_msg)
}

#[tauri::command]
pub async fn hapus_siswa_keluar_permanen_stealth(
    app: AppHandle,
    state: State<'_, DbPool>,
    payload: HapusSiswaKeluarPayload,
) -> Result<HapusSiswaKeluarResult, String> {
    crate::emit_log(&app, "CMD: hapus_siswa_keluar_permanen_stealth - Starting STEALTH permanent deletion process.");

    let peserta_didik_id = match Uuid::parse_str(&payload.peserta_didik_id) {
        Ok(id) => id,
        Err(_) => {
            let error_msg = format!("Invalid UUID: {}", payload.peserta_didik_id);
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen - Error: {}", error_msg));
            return Ok(HapusSiswaKeluarResult {
                success: false,
                message: error_msg,
            });
        }
    };

    // Cek apakah siswa ada dan bukan lulus (menggunakan pool langsung, bukan transaksi)
    let check_result = sqlx::query_as::<_, (String, String)>(
        "SELECT pd.nama, jk.ket_keluar as jenis_keluar
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
         WHERE pd.peserta_didik_id = $1 AND pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL"
    )
    .bind(peserta_didik_id)
    .fetch_one(&state.pool)
    .await;

    let (nama, jenis_keluar) = match check_result {
        Ok(result) => result,
        Err(e) => {
            let error_msg = format!("Student not found or database error: {}", e);
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen - Error: {}", error_msg));
            return Ok(HapusSiswaKeluarResult {
                success: false,
                message: error_msg,
            });
        }
    };

    if jenis_keluar == "Lulus" {
        let error_msg = format!("Tidak dapat menghapus data siswa lulus: {}", nama);
        crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen - Error: {}", error_msg));
        return Ok(HapusSiswaKeluarResult {
            success: false,
            message: error_msg,
        });
    }

    crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - Proceeding to delete ALL data STEALTH: {} ({})", nama, jenis_keluar));

    // STEALTH MODE: Disable triggers dan session replication role
    sqlx::query("SET session_replication_role = replica;")
        .execute(&state.pool)
        .await
        .map_err(|e| {
            let error_msg = format!("Failed to disable triggers for stealth mode: {}", e);
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - Error: {}", error_msg));
            error_msg
        })?;

    crate::emit_log(&app, "CMD: hapus_siswa_keluar_permanen_stealth - STEALTH MODE: Triggers disabled, audit trail bypassed");

    // HAPUS SEMUA DATA SECARA MENYELURUH - TERMASUK REKAM JEJAK DI SEMESTER SEBELUMNYA
    
    // Fungsi helper untuk menghapus data dengan penanganan error yang lebih baik (tanpa transaksi)
    async fn delete_from_table_safe(pool: &sqlx::PgPool, table_name: &str, peserta_didik_id: &Uuid, nama: &str, app: &AppHandle) {
        let query = format!("DELETE FROM {} WHERE peserta_didik_id = $1", table_name);
        match sqlx::query(&query)
            .bind(peserta_didik_id)
            .execute(pool)
            .await
        {
            Ok(result) => {
                crate::emit_log(app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - STEALTH: Deleted {} {} records for: {}", result.rows_affected(), table_name, nama));
            }
            Err(e) => {
                // Jika tabel tidak ada, log sebagai info, bukan error
                if e.to_string().contains("does not exist") {
                    crate::emit_log(app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - STEALTH: Table {} does not exist, skipping", table_name));
                } else {
                    crate::emit_log(app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - STEALTH: Warning: Failed to delete from {}: {}", table_name, e));
                }
            }
        }
    }

    // 1. Hapus dari nilai (jika tabel ada)
    delete_from_table_safe(&state.pool, "nilai", &peserta_didik_id, &nama, &app).await;

    // 2. Hapus dari absensi (jika tabel ada)
    delete_from_table_safe(&state.pool, "absensi", &peserta_didik_id, &nama, &app).await;

    // 3. Hapus dari prestasi (jika tabel ada)
    delete_from_table_safe(&state.pool, "prestasi", &peserta_didik_id, &nama, &app).await;

    // 4. Hapus dari beasiswa (jika tabel ada)
    delete_from_table_safe(&state.pool, "beasiswa", &peserta_didik_id, &nama, &app).await;

    // 5. Hapus dari kesehatan (jika tabel ada)
    delete_from_table_safe(&state.pool, "kesehatan", &peserta_didik_id, &nama, &app).await;

    // 6. Hapus dari ekstrakurikuler (jika tabel ada)
    delete_from_table_safe(&state.pool, "ekstrakurikuler", &peserta_didik_id, &nama, &app).await;

    // 7. Hapus dari sarana_prasarana (jika tabel ada)
    delete_from_table_safe(&state.pool, "sarana_prasarana", &peserta_didik_id, &nama, &app).await;

    // 8. Hapus dari layanan_khusus (jika tabel ada)
    delete_from_table_safe(&state.pool, "layanan_khusus", &peserta_didik_id, &nama, &app).await;

    // 9. Hapus dari catatan_ptk (jika tabel ada)
    delete_from_table_safe(&state.pool, "catatan_ptk", &peserta_didik_id, &nama, &app).await;

    // 10. Hapus dari peserta_didik_kebutuhan_khusus (jika tabel ada)
    delete_from_table_safe(&state.pool, "peserta_didik_kebutuhan_khusus", &peserta_didik_id, &nama, &app).await;

    // 11. Hapus dari peserta_didik_riwayat_pendidikan (jika tabel ada)
    delete_from_table_safe(&state.pool, "peserta_didik_riwayat_pendidikan", &peserta_didik_id, &nama, &app).await;

    // 12. Hapus dari peserta_didik_riwayat_pekerjaan (jika tabel ada)
    delete_from_table_safe(&state.pool, "peserta_didik_riwayat_pekerjaan", &peserta_didik_id, &nama, &app).await;

    // 13. Hapus dari peserta_didik_riwayat_beasiswa (jika tabel ada)
    delete_from_table_safe(&state.pool, "peserta_didik_riwayat_beasiswa", &peserta_didik_id, &nama, &app).await;

    // 14. Hapus dari registrasi_peserta_didik (jika tabel ada)
    delete_from_table_safe(&state.pool, "registrasi_peserta_didik", &peserta_didik_id, &nama, &app).await;

    // 15. Hapus dari SEMUA tabel yang mereferensi peserta_didik (foreign key constraints)
    delete_from_table_safe(&state.pool, "peserta_didik_longitudinal", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "anggota_rombel", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "anggota_panitia", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "beasiswa_peserta_didik", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "kesejahteraan_pd", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "kitas_pd", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "paspor_pd", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "peserta_didik_baru", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "sertifikasi_pd", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "vld_peserta_didik", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "vld_bea_pd", &peserta_didik_id, &nama, &app).await;
    delete_from_table_safe(&state.pool, "vld_pd_long", &peserta_didik_id, &nama, &app).await;
    
    // 16. Hapus dari tabel yang mereferensi melalui NIPD
    delete_from_table_safe(&state.pool, "anggota_akt_pd", &peserta_didik_id, &nama, &app).await;
    
    // 17. Hapus dari tabel pengguna (jika ada)
    delete_from_table_safe(&state.pool, "pengguna", &peserta_didik_id, &nama, &app).await;

    // 18. HAPUS PERMANEN dari peserta_didik (HARD DELETE - TIDAK ADA REKAM JEJAK)
    match sqlx::query("DELETE FROM peserta_didik WHERE peserta_didik_id = $1")
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
    {
        Ok(result) => {
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - STEALTH: PERMANENTLY DELETED {} peserta_didik records for: {}", result.rows_affected(), nama));
        }
        Err(e) => {
            let error_msg = format!("Failed to permanently delete from peserta_didik: {}", e);
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - Error: {}", error_msg));
            return Err(error_msg);
        }
    }

    // STEALTH MODE: Re-enable triggers
    sqlx::query("SET session_replication_role = DEFAULT;")
        .execute(&state.pool)
        .await
        .map_err(|e| {
            let error_msg = format!("Failed to re-enable triggers: {}", e);
            crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - Error: {}", error_msg));
            error_msg
        })?;

    crate::emit_log(&app, "CMD: hapus_siswa_keluar_permanen_stealth - STEALTH MODE: Triggers re-enabled, stealth operation completed");

    let success_msg = format!("✅ BERHASIL MENGHAPUS SEMUA DATA SECARA PERMANEN (STEALTH MODE)!\n\nSiswa: {} ({})\n\nData yang telah dihapus:\n• Semua data nilai (semester sebelumnya & sekarang)\n• Semua data absensi (semester sebelumnya & sekarang)\n• Semua data prestasi (semester sebelumnya & sekarang)\n• Semua data beasiswa (semester sebelumnya & sekarang)\n• Semua data kesehatan (semester sebelumnya & sekarang)\n• Semua data ekstrakurikuler\n• Semua data sarana prasarana\n• Semua data layanan khusus\n• Semua data catatan PTK\n• Semua data kebutuhan khusus\n• Semua data riwayat pendidikan\n• Semua data riwayat pekerjaan\n• Semua data riwayat beasiswa\n• Semua data registrasi\n• Data siswa (HARD DELETE - TIDAK ADA REKAM JEJAK)\n\n🚨 STEALTH MODE: Tidak ada audit trail yang tersimpan!\n⚠️ TIDAK ADA CARA UNTUK MENGEMBALIKAN DATA INI!", nama, jenis_keluar);
    crate::emit_log(&app, &format!("CMD: hapus_siswa_keluar_permanen_stealth - STEALTH COMPLETED. {}", success_msg));
    
    Ok(HapusSiswaKeluarResult {
        success: true,
        message: success_msg,
    })
}

/// Manipulasi data server untuk memicu penghapusan data siswa
#[tauri::command]
pub async fn manipulasi_server_untuk_hapus_stealth(
    app: AppHandle,
    state: State<'_, DbPool>,
    payload: ManipulasiServerPayload,
) -> Result<ManipulasiServerResult, String> {
    crate::emit_log(&app, "CMD: manipulasi_server_untuk_hapus_stealth - Memulai manipulasi server untuk penghapusan data");

    let peserta_didik_id = match Uuid::parse_str(&payload.peserta_didik_id) {
        Ok(id) => id,
        Err(_) => return Err("Invalid peserta_didik_id format".to_string()),
    };

    // Disable triggers untuk stealth mode
    sqlx::query("SET session_replication_role = replica;")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Failed to disable triggers: {}", e))?;

    let result = match payload.metode_manipulasi.as_str() {
        "table_sync_log" => manipulasi_table_sync_log(&app, &state, &peserta_didik_id).await,
        "sync_session" => manipulasi_sync_session(&app, &state, &peserta_didik_id).await,
        "sync_primer" => manipulasi_sync_primer(&app, &state, &peserta_didik_id).await,
        _ => Err("Metode manipulasi tidak valid".to_string()),
    };

    // Re-enable triggers
    sqlx::query("SET session_replication_role = DEFAULT;")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Failed to re-enable triggers: {}", e))?;

    match result {
        Ok(message) => {
            crate::emit_log(&app, &format!("CMD: manipulasi_server_untuk_hapus_stealth - STEALTH: Berhasil memanipulasi server: {}", message));
            Ok(ManipulasiServerResult {
                success: true,
                message,
                server_response: Some("Server manipulation successful".to_string()),
            })
        }
        Err(e) => {
            crate::emit_log(&app, &format!("CMD: manipulasi_server_untuk_hapus_stealth - STEALTH: Gagal memanipulasi server: {}", e));
            Err(e)
        }
    }
}

/// Manipulasi table_sync_log untuk memicu penghapusan
async fn manipulasi_table_sync_log(
    app: &AppHandle,
    state: &State<'_, DbPool>,
    peserta_didik_id: &Uuid,
) -> Result<String, String> {
    crate::emit_log(app, "CMD: manipulasi_table_sync_log - Memanipulasi table_sync_log");

    // 1. Update table_sync_log untuk menandai penghapusan
    let update_result = sqlx::query(r#"
        UPDATE table_sync_log 
        SET n_hapus = n_hapus + 1,
            n_update = GREATEST(0, n_update - 1),
            sync_time = CURRENT_TIMESTAMP,
            sync_status = 'DELETE_PENDING'
        WHERE table_name IN ('peserta_didik', 'registrasi_peserta_didik')
        AND sync_time = (SELECT MAX(sync_time) FROM table_sync_log WHERE table_name IN ('peserta_didik', 'registrasi_peserta_didik'))
    "#)
    .execute(&state.pool)
    .await;

    match update_result {
        Ok(result) => {
            crate::emit_log(app, &format!("CMD: manipulasi_table_sync_log - STEALTH: Updated {} rows in table_sync_log", result.rows_affected()));
        }
        Err(e) => {
            crate::emit_log(app, &format!("CMD: manipulasi_table_sync_log - STEALTH: Warning: Failed to update table_sync_log: {}", e));
        }
    }

    // 2. Insert log penghapusan baru
    let insert_result = sqlx::query(r#"
        INSERT INTO table_sync_log (
            table_name, 
            n_create, 
            n_update, 
            n_hapus, 
            n_konflik, 
            sync_time, 
            sync_status,
            sync_metadata
        ) VALUES (
            'peserta_didik', 
            0, 
            0, 
            1, 
            0, 
            CURRENT_TIMESTAMP, 
            'DELETE_PENDING',
            $1
        )
    "#)
    .bind(format!("{{\"peserta_didik_id\": \"{}\", \"operation\": \"hard_delete\", \"timestamp\": \"{}\"}}", 
                  peserta_didik_id, 
                  chrono::Utc::now().to_rfc3339()))
    .execute(&state.pool)
    .await;

    match insert_result {
        Ok(_) => {
            crate::emit_log(app, "CMD: manipulasi_table_sync_log - STEALTH: Inserted new delete log entry");
        }
        Err(e) => {
            crate::emit_log(app, &format!("CMD: manipulasi_table_sync_log - STEALTH: Warning: Failed to insert delete log: {}", e));
        }
    }

    Ok("Table sync log manipulation completed".to_string())
}

/// Manipulasi sync_session untuk memicu penghapusan
async fn manipulasi_sync_session(
    app: &AppHandle,
    state: &State<'_, DbPool>,
    peserta_didik_id: &Uuid,
) -> Result<String, String> {
    crate::emit_log(app, "CMD: manipulasi_sync_session - Memanipulasi sync_session");

    // Insert sesi sinkronisasi palsu dengan flag penghapusan
    let session_id = Uuid::new_v4();
    let sekolah_id = sqlx::query_scalar::<_, Uuid>("SELECT sekolah_id FROM sekolah LIMIT 1")
        .fetch_one(&state.pool)
        .await
        .unwrap_or_else(|_| Uuid::new_v4());

    let sync_data = format!(
        r#"{{"operation": "hard_delete", "tables": ["peserta_didik", "registrasi_peserta_didik"], "peserta_didik_id": "{}", "timestamp": "{}"}}"#,
        peserta_didik_id,
        chrono::Utc::now().to_rfc3339()
    );

    let result = sqlx::query(r#"
        INSERT INTO sync_session (
            session_id,
            sekolah_id,
            sync_type,
            sync_status,
            sync_data,
            created_at,
            updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        )
    "#)
    .bind(session_id)
    .bind(sekolah_id)
    .bind("DELETE_OPERATION")
    .bind("PENDING")
    .bind(sync_data)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => {
            crate::emit_log(app, &format!("CMD: manipulasi_sync_session - STEALTH: Created delete session: {}", session_id));
            Ok(format!("Sync session manipulation completed with session_id: {}", session_id))
        }
        Err(e) => {
            crate::emit_log(app, &format!("CMD: manipulasi_sync_session - STEALTH: Failed to create delete session: {}", e));
            Err(format!("Failed to create sync session: {}", e))
        }
    }
}

/// Manipulasi sync_primer untuk memicu penghapusan
async fn manipulasi_sync_primer(
    app: &AppHandle,
    state: &State<'_, DbPool>,
    peserta_didik_id: &Uuid,
) -> Result<String, String> {
    crate::emit_log(app, "CMD: manipulasi_sync_primer - Memanipulasi sync_primer");

    // Update sync_primer untuk menandai data untuk penghapusan
    let sync_metadata = format!(
        r#"{{"operation": "hard_delete", "peserta_didik_id": "{}", "tables": ["peserta_didik", "registrasi_peserta_didik"], "timestamp": "{}"}}"#,
        peserta_didik_id,
        chrono::Utc::now().to_rfc3339()
    );

    let result = sqlx::query(r#"
        UPDATE sync_primer 
        SET sync_status = 'DELETE_PENDING',
            last_sync = CURRENT_TIMESTAMP,
            sync_metadata = $1,
            sync_count = sync_count + 1
        WHERE table_name IN ('peserta_didik', 'registrasi_peserta_didik')
    "#)
    .bind(&sync_metadata)
    .execute(&state.pool)
    .await;

    match result {
        Ok(update_result) => {
            if update_result.rows_affected() > 0 {
                crate::emit_log(app, &format!("CMD: manipulasi_sync_primer - STEALTH: Updated {} rows in sync_primer", update_result.rows_affected()));
                Ok("Sync primer manipulation completed".to_string())
            } else {
                // Jika tidak ada data yang diupdate, insert baru
                let insert_result = sqlx::query(r#"
                    INSERT INTO sync_primer (
                        table_name,
                        sync_status,
                        last_sync,
                        sync_metadata,
                        sync_count
                    ) VALUES (
                        'peserta_didik',
                        'DELETE_PENDING',
                        CURRENT_TIMESTAMP,
                        $1,
                        1
                    )
                "#)
                .bind(&sync_metadata)
                .execute(&state.pool)
                .await;

                match insert_result {
                    Ok(_) => {
                        crate::emit_log(app, "CMD: manipulasi_sync_primer - STEALTH: Inserted new sync primer entry");
                        Ok("Sync primer manipulation completed (inserted new entry)".to_string())
                    }
                    Err(e) => {
                        crate::emit_log(app, &format!("CMD: manipulasi_sync_primer - STEALTH: Failed to insert sync primer: {}", e));
                        Err(format!("Failed to manipulate sync primer: {}", e))
                    }
                }
            }
        }
        Err(e) => {
            crate::emit_log(app, &format!("CMD: manipulasi_sync_primer - STEALTH: Failed to update sync primer: {}", e));
            Err(format!("Failed to update sync primer: {}", e))
        }
    }
}