use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use sqlx::Row;
use bigdecimal::BigDecimal;
use sqlx::types::Uuid as SqlxUuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ValidasiDetail {
    pub peserta_didik_id: String,
    pub nama: String,
    pub field: String,
    pub action: String,
    pub success: bool,
    pub message: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Serialize)]
pub struct ValidasiResult {
    pub success: bool,
    pub message: String,
    pub total_processed: i32,
    pub success_count: i32,
    pub error_count: i32,
    pub details: Vec<ValidasiDetail>,
}

#[derive(Serialize)]
pub struct ValidasiStats {
    pub total_siswa: i64,
    pub nik_ayah_invalid: i64,
    pub tanpa_hobby: i64,
    pub tanpa_cita_cita: i64,
    pub tahun_lahir_ayah_invalid: i64,
    pub nik_wali_invalid: i64,
    pub tahun_lahir_wali_invalid: i64,
    pub kps_pkh_invalid: i64,
}

#[tauri::command]
pub async fn auto_fix_validasi_errors(
    _app: AppHandle,
    pengguna_id: String,
    state: State<'_, DbPool>,
) -> Result<ValidasiResult, String> {
    println!("[VALIDASI] Memulai perbaikan otomatis data siswa aktif...");
    
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    // Fix NIK Ayah
    match fix_nik_ayah(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "nik_ayah".to_string(),
                "fix_nik_ayah".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix Hobby
    match fix_hobby(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "hobby".to_string(),
                "fix_hobby".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix Cita-cita
    match fix_cita_cita(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "cita_cita".to_string(),
                "fix_cita_cita".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix Tahun Lahir Ayah
    match fix_tahun_lahir_ayah(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "tahun_lahir_ayah".to_string(),
                "fix_tahun_lahir_ayah".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix NIK Wali
    match fix_nik_wali(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "nik_wali".to_string(),
                "fix_nik_wali".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix Tahun Lahir Wali
    match fix_tahun_lahir_wali(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "tahun_lahir_wali".to_string(),
                "fix_tahun_lahir_wali".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    // Fix KPS/PKH
    match fix_kps_pkh(&state, &pengguna_id).await {
        Ok(result) => {
            total_processed += result.total_processed;
            success_count += result.success_count;
            error_count += result.error_count;
            details.extend(result.details);
        }
        Err(e) => {
            error_count += 1;
            details.push(create_validasi_detail(
                "".to_string(),
                "".to_string(),
                "kps_pkh".to_string(),
                "fix_kps_pkh".to_string(),
                false,
                format!("Error: {}", e),
                None,
                None,
            ));
        }
    }
    
    println!("[VALIDASI] Selesai. Total: {}, Berhasil: {}, Error: {}", total_processed, success_count, error_count);
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("Validasi selesai. {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}



#[tauri::command]
pub async fn get_validasi_stats(state: State<'_, DbPool>) -> Result<ValidasiStats, String> {
    let pool = &state.pool;
    
    let total_siswa: i64 = sqlx::query_scalar(
        "SELECT COUNT(pd.*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting total siswa: {}", e))?;
    
    let nik_ayah_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_ayah IS NOT NULL AND (LENGTH(pd.nik_ayah) != 16 OR pd.nik_ayah !~ '^[0-9]+$')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid NIK ayah: {}", e))?;
    
    let tanpa_hobby: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_hobby = -1 OR rpd.id_hobby IS NULL)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting siswa tanpa hobby: {}", e))?;
    
    let tanpa_cita_cita: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_cita = -1 OR rpd.id_cita IS NULL)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting siswa tanpa cita-cita: {}", e))?;
    
    let tahun_lahir_ayah_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_ayah IS NOT NULL AND (pd.tahun_lahir_ayah < 1900 OR pd.tahun_lahir_ayah > 2024)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid tahun lahir ayah: {}", e))?;
    
    let nik_wali_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_wali IS NOT NULL AND (LENGTH(pd.nik_wali) != 16 OR pd.nik_wali !~ '^[0-9]+$')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid NIK wali: {}", e))?;
    
    let tahun_lahir_wali_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_wali IS NOT NULL AND (pd.tahun_lahir_wali < 1900 OR pd.tahun_lahir_wali > 2024)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid tahun lahir wali: {}", e))?;
    
    let kps_pkh_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.penerima_kps = 1 AND (pd.no_kps IS NULL OR pd.no_kps = '')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid KPS/PKH: {}", e))?;
    
    Ok(ValidasiStats {
        total_siswa,
        nik_ayah_invalid,
        tanpa_hobby,
        tanpa_cita_cita,
        tahun_lahir_ayah_invalid,
        nik_wali_invalid,
        tahun_lahir_wali_invalid,
        kps_pkh_invalid,
    })
}

#[tauri::command]
pub async fn validate_before_fix(state: State<'_, DbPool>) -> Result<ValidasiStats, String> {
    let pool = &state.pool;
    
    println!("[VALIDATION] Memulai validasi data sebelum perbaikan...");
    
    let total_siswa: i64 = sqlx::query_scalar(
        "SELECT COUNT(pd.*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting total siswa: {}", e))?;
    
    let nik_ayah_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_ayah IS NOT NULL AND (LENGTH(pd.nik_ayah) != 16 OR pd.nik_ayah !~ '^[0-9]+$')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid NIK ayah: {}", e))?;
    
    let tanpa_hobby: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_hobby = -1 OR rpd.id_hobby IS NULL)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting siswa tanpa hobby: {}", e))?;
    
    let tanpa_cita_cita: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_cita = -1 OR rpd.id_cita IS NULL)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting siswa tanpa cita-cita: {}", e))?;
    
    let tahun_lahir_ayah_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_ayah IS NOT NULL AND (pd.tahun_lahir_ayah < 1900 OR pd.tahun_lahir_ayah > 2024)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid tahun lahir ayah: {}", e))?;
    
    let nik_wali_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_wali IS NOT NULL AND (LENGTH(pd.nik_wali) != 16 OR pd.nik_wali !~ '^[0-9]+$')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid NIK wali: {}", e))?;
    
    let tahun_lahir_wali_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_wali IS NOT NULL AND (pd.tahun_lahir_wali < 1900 OR pd.tahun_lahir_wali > 2024)"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid tahun lahir wali: {}", e))?;
    
    let kps_pkh_invalid: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.penerima_kps = 1 AND (pd.no_kps IS NULL OR pd.no_kps = '')"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Error getting invalid KPS/PKH: {}", e))?;
    
    let total_errors = nik_ayah_invalid + tanpa_hobby + tanpa_cita_cita + tahun_lahir_ayah_invalid + 
                      nik_wali_invalid + tahun_lahir_wali_invalid + kps_pkh_invalid;
    
    println!("[VALIDATION] Total siswa: {}", total_siswa);
    println!("[VALIDATION] Total error: {}", total_errors);
    println!("[VALIDATION] NIK Ayah invalid: {}", nik_ayah_invalid);
    println!("[VALIDATION] Tanpa hobby: {}", tanpa_hobby);
    println!("[VALIDATION] Tanpa cita-cita: {}", tanpa_cita_cita);
    println!("[VALIDATION] Tahun lahir ayah invalid: {}", tahun_lahir_ayah_invalid);
    println!("[VALIDATION] NIK Wali invalid: {}", nik_wali_invalid);
    println!("[VALIDATION] Tahun lahir wali invalid: {}", tahun_lahir_wali_invalid);
    println!("[VALIDATION] KPS/PKH invalid: {}", kps_pkh_invalid);
    
    Ok(ValidasiStats {
        total_siswa,
        nik_ayah_invalid,
        tanpa_hobby,
        tanpa_cita_cita,
        tahun_lahir_ayah_invalid,
        nik_wali_invalid,
        tahun_lahir_wali_invalid,
        kps_pkh_invalid,
    })
}



// Helper functions for individual fixes
async fn fix_nik_ayah(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    let invalid_nik_ayah = sqlx::query(
        "SELECT pd.peserta_didik_id, pd.nama, pd.nik_ayah FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_ayah IS NOT NULL AND (LENGTH(pd.nik_ayah) != 16 OR pd.nik_ayah !~ '^[0-9]+$')"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying invalid NIK ayah: {}", e))?;
    
    for row in &invalid_nik_ayah {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        let old_value: Option<String> = row.try_get("nik_ayah")
            .map_err(|e| format!("Error getting nik_ayah: {}", e))?;
        
        match sqlx::query(
            "UPDATE peserta_didik SET nik_ayah = NULL, last_update = NOW(), updater_id = $2 WHERE peserta_didik_id = $1"
        )
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "nik_ayah".to_string(),
                    "Set to NULL".to_string(),
                    true,
                    "NIK ayah invalid, diatur menjadi NULL".to_string(),
                    old_value,
                    None,
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "nik_ayah".to_string(),
                    "Set to NULL".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    old_value,
                    None,
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("NIK Ayah: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_hobby(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    // Get random hobby from reference table
    let hobbies = sqlx::query("SELECT id_hobby, nm_hobby FROM ref.jenis_hobby WHERE id_hobby != -1")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error getting hobbies: {}", e))?;
    
    if hobbies.is_empty() {
        return Err("Tidak ada data hobby di tabel referensi".to_string());
    }
    
    let siswa_tanpa_hobby = sqlx::query(
        "SELECT rpd.peserta_didik_id, pd.nama FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_hobby = -1 OR rpd.id_hobby IS NULL)"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying siswa tanpa hobby: {}", e))?;
    
    for row in &siswa_tanpa_hobby {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        
        // Get random hobby
        let random_index = (Utc::now().timestamp() as usize + total_processed as usize) % hobbies.len();
        let random_hobby_row = &hobbies[random_index];
        let new_hobby_id: BigDecimal = random_hobby_row.try_get("id_hobby")
            .map_err(|e| format!("Error getting hobby id: {}", e))?;
        let new_hobby_name: String = random_hobby_row.try_get("nm_hobby")
            .map_err(|e| format!("Error getting hobby nama: {}", e))?;
        
        match sqlx::query(
            "UPDATE registrasi_peserta_didik SET id_hobby = $1, last_update = NOW(), updater_id = $3 WHERE peserta_didik_id = $2"
        )
        .bind(new_hobby_id)
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "hobby".to_string(),
                    "Fill with random".to_string(),
                    true,
                    "Hobby kosong, diisi dengan random".to_string(),
                    None,
                    Some(new_hobby_name),
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "hobby".to_string(),
                    "Fill with random".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    None,
                    Some(new_hobby_name),
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("Hobby: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_cita_cita(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    // Get random cita-cita from reference table
    let cita_cita_list = sqlx::query("SELECT id_cita, nm_cita FROM ref.jenis_cita WHERE id_cita != -1")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Error getting cita-cita: {}", e))?;
    
    if cita_cita_list.is_empty() {
        return Err("Tidak ada data cita-cita di tabel referensi".to_string());
    }
    
    let siswa_tanpa_cita_cita = sqlx::query(
        "SELECT rpd.peserta_didik_id, pd.nama FROM registrasi_peserta_didik rpd 
         JOIN peserta_didik pd ON rpd.peserta_didik_id = pd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND (rpd.id_cita = -1 OR rpd.id_cita IS NULL)"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying siswa tanpa cita-cita: {}", e))?;
    
    for row in &siswa_tanpa_cita_cita {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        
        // Get random cita-cita
        let random_index = (Utc::now().timestamp() as usize + total_processed as usize) % cita_cita_list.len();
        let random_cita_cita_row = &cita_cita_list[random_index];
        let new_cita_id: BigDecimal = random_cita_cita_row.try_get("id_cita")
            .map_err(|e| format!("Error getting cita-cita id: {}", e))?;
        let new_cita_name: String = random_cita_cita_row.try_get("nm_cita")
            .map_err(|e| format!("Error getting cita-cita nama: {}", e))?;
        
        match sqlx::query(
            "UPDATE registrasi_peserta_didik SET id_cita = $1, last_update = NOW(), updater_id = $3 WHERE peserta_didik_id = $2"
        )
        .bind(new_cita_id)
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "cita_cita".to_string(),
                    "Fill with random".to_string(),
                    true,
                    "Cita-cita kosong, diisi dengan random".to_string(),
                    None,
                    Some(new_cita_name),
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "cita_cita".to_string(),
                    "Fill with random".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    None,
                    Some(new_cita_name),
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("Cita-cita: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_tahun_lahir_ayah(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    let invalid_tahun_lahir_ayah = sqlx::query(
        "SELECT pd.peserta_didik_id, pd.nama, pd.tahun_lahir_ayah FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_ayah IS NOT NULL AND (pd.tahun_lahir_ayah < 1900 OR pd.tahun_lahir_ayah > 2024)"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying invalid tahun lahir ayah: {}", e))?;
    
    for row in &invalid_tahun_lahir_ayah {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        let old_value: Option<String> = row.try_get::<Option<BigDecimal>, _>("tahun_lahir_ayah")
            .map_err(|e| format!("Error getting tahun_lahir_ayah: {}", e))?
            .map(|v| v.to_string());
        
        match sqlx::query(
            "UPDATE peserta_didik SET tahun_lahir_ayah = NULL, last_update = NOW(), updater_id = $2 WHERE peserta_didik_id = $1"
        )
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "tahun_lahir_ayah".to_string(),
                    "Set to NULL".to_string(),
                    true,
                    "Tahun lahir ayah invalid, diatur menjadi NULL".to_string(),
                    old_value,
                    None,
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "tahun_lahir_ayah".to_string(),
                    "Set to NULL".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    old_value,
                    None,
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("Tahun Lahir Ayah: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_nik_wali(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    let invalid_nik_wali = sqlx::query(
        "SELECT pd.peserta_didik_id, pd.nama, pd.nik_wali FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.nik_wali IS NOT NULL AND (LENGTH(pd.nik_wali) != 16 OR pd.nik_wali !~ '^[0-9]+$')"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying invalid NIK wali: {}", e))?;
    
    for row in &invalid_nik_wali {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        let old_value: Option<String> = row.try_get("nik_wali")
            .map_err(|e| format!("Error getting nik_wali: {}", e))?;
        
        match sqlx::query(
            "UPDATE peserta_didik SET nik_wali = NULL, last_update = NOW(), updater_id = $2 WHERE peserta_didik_id = $1"
        )
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "nik_wali".to_string(),
                    "Set to NULL".to_string(),
                    true,
                    "NIK wali invalid, diatur menjadi NULL".to_string(),
                    old_value,
                    None,
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "nik_wali".to_string(),
                    "Set to NULL".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    old_value,
                    None,
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("NIK Wali: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_tahun_lahir_wali(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    let invalid_tahun_lahir_wali = sqlx::query(
        "SELECT pd.peserta_didik_id, pd.nama, pd.tahun_lahir_wali FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.tahun_lahir_wali IS NOT NULL AND (pd.tahun_lahir_wali < 1900 OR pd.tahun_lahir_wali > 2024)"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying invalid tahun lahir wali: {}", e))?;
    
    for row in &invalid_tahun_lahir_wali {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        let old_value: Option<String> = row.try_get::<Option<BigDecimal>, _>("tahun_lahir_wali")
            .map_err(|e| format!("Error getting tahun_lahir_wali: {}", e))?
            .map(|v| v.to_string());
        
        match sqlx::query(
            "UPDATE peserta_didik SET tahun_lahir_wali = NULL, last_update = NOW(), updater_id = $2 WHERE peserta_didik_id = $1"
        )
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "tahun_lahir_wali".to_string(),
                    "Set to NULL".to_string(),
                    true,
                    "Tahun lahir wali invalid, diatur menjadi NULL".to_string(),
                    old_value,
                    None,
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "tahun_lahir_wali".to_string(),
                    "Set to NULL".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    old_value,
                    None,
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("Tahun Lahir Wali: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}

async fn fix_kps_pkh(state: &State<'_, DbPool>, pengguna_id: &str) -> Result<ValidasiResult, String> {
    let pool = &state.pool;
    let mut details = Vec::new();
    let mut total_processed = 0;
    let mut success_count = 0;
    let mut error_count = 0;
    
    let invalid_kps_pkh = sqlx::query(
        "SELECT pd.peserta_didik_id, pd.nama, pd.no_kps FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL 
         AND pd.penerima_kps = 1 AND (pd.no_kps IS NULL OR pd.no_kps = '')"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Error querying invalid KPS/PKH: {}", e))?;
    
    for row in &invalid_kps_pkh {
        total_processed += 1;
        let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
            .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
        let nama: String = row.try_get("nama")
            .map_err(|e| format!("Error getting nama: {}", e))?;
        let old_value: Option<String> = row.try_get("no_kps")
            .map_err(|e| format!("Error getting no_kps: {}", e))?;
        
        match sqlx::query(
            "UPDATE peserta_didik SET penerima_kps = 0, last_update = NOW(), updater_id = $2 WHERE peserta_didik_id = $1"
        )
        .bind(peserta_didik_id)
        .bind(pengguna_id)
        .execute(pool)
        .await
        {
            Ok(_) => {
                success_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "kps_pkh".to_string(),
                    "Set penerima_kps to 0".to_string(),
                    true,
                    "KPS/PKH invalid, diatur menjadi tidak menerima".to_string(),
                    old_value,
                    None,
                ));
            }
            Err(e) => {
                error_count += 1;
                details.push(create_validasi_detail(
                    peserta_didik_id.to_string(),
                    nama,
                    "kps_pkh".to_string(),
                    "Set penerima_kps to 0".to_string(),
                    false,
                    format!("Error updating: {}", e),
                    old_value,
                    None,
                ));
            }
        }
    }
    
    Ok(ValidasiResult {
        success: error_count == 0,
        message: format!("KPS/PKH: {} berhasil, {} error", success_count, error_count),
        total_processed,
        success_count,
        error_count,
        details,
    })
}





fn create_validasi_detail(
    peserta_didik_id: String,
    nama: String,
    field: String,
    action: String,
    success: bool,
    message: String,
    old_value: Option<String>,
    new_value: Option<String>,
) -> ValidasiDetail {
    ValidasiDetail {
        peserta_didik_id,
        nama,
        field,
        action,
        success,
        message,
        old_value,
        new_value,
    }
} 