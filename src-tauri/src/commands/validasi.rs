use crate::{DbPool, emit_log};
use sqlx::types::Uuid as SqlxUuid;
use tauri::{AppHandle, State};
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use rand::seq::SliceRandom;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    peserta_didik_id: String,
    nama: String,
    field_error: String,
    error_type: String,
    error_message: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationSummary {
    total_checked: i64,
    total_errors: i64,
    errors_by_type: Vec<ErrorTypeCount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorTypeCount {
    error_type: String,
    count: i64,
    field_name: String,
}

/// Auto-fix hobby yang bernilai -1 atau NULL dengan nilai random dari tabel ref.jenis_hobby
#[tauri::command]
pub async fn auto_fix_hobby_minus_one_stealth(
    app: AppHandle,
    state: State<'_, DbPool>
) -> Result<String, String> {
    emit_log(&app, "CMD: auto_fix_hobby_minus_one_stealth - Memperbaiki id_hobby yang bernilai -1 atau NULL (STEALTH MODE)");
    
    // LANGKAH 1: Ambil semua id_hobby yang tersedia dari tabel ref.jenis_hobby
    let hobby_ids: Vec<(BigDecimal,)> = sqlx::query_as("SELECT id_hobby FROM ref.jenis_hobby WHERE id_hobby > 0")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengambil data hobby: {}", e))?;
    
    if hobby_ids.is_empty() {
        return Err("Tidak ada data hobby yang tersedia untuk dipilih secara acak.".to_string());
    }
    
    // LANGKAH 2: Hitung berapa banyak siswa yang memiliki id_hobby = -1 atau NULL
    let count_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM registrasi_peserta_didik WHERE id_hobby IS NULL OR id_hobby = -1")
        .fetch_one(&state.pool)
    .await
        .map_err(|e| format!("Gagal menghitung siswa dengan id_hobby = -1 atau NULL: {}", e))?;
    
    if count_result == 0 {
        return Ok("Tidak ada siswa dengan id_hobby = -1 atau NULL yang perlu diperbaiki.".to_string());
    }
    
    // LANGKAH 3: Ambil semua siswa yang memiliki id_hobby = -1 atau NULL
    let students_with_invalid_hobby: Vec<(SqlxUuid,)> = sqlx::query_as("SELECT peserta_didik_id FROM registrasi_peserta_didik WHERE id_hobby IS NULL OR id_hobby = -1")
        .fetch_all(&state.pool)
    .await
        .map_err(|e| format!("Gagal mengambil siswa dengan id_hobby = -1 atau NULL: {}", e))?;
    
    let mut updated_count = 0;
    
    // LANGKAH 4: Update setiap siswa dengan id_hobby random
    for (peserta_didik_id,) in &students_with_invalid_hobby {
        // Pilih hobby random
        let random_hobby = hobby_ids.choose(&mut rand::thread_rng()).unwrap().0.clone();
        
        // Update id_hobby
        sqlx::query("UPDATE registrasi_peserta_didik SET id_hobby = $1 WHERE peserta_didik_id = $2")
            .bind(&random_hobby)
            .bind(peserta_didik_id)
            .execute(&state.pool)
    .await
            .map_err(|e| format!("Gagal update id_hobby untuk siswa {}: {}", peserta_didik_id, e))?;
        
        updated_count += 1;
        
        emit_log(&app, &format!("CMD: auto_fix_hobby_minus_one_stealth - Updated siswa {} dengan id_hobby: {}", peserta_didik_id, random_hobby));
    }
    
    // LANGKAH 5: Hapus log validasi yang terkait dengan hobby
    let mut total_logs_deleted = 0;
    for (peserta_didik_id,) in &students_with_invalid_hobby {
        let deleted_logs = sqlx::query("DELETE FROM vld_peserta_didik WHERE peserta_didik_id = $1")
        .bind(peserta_didik_id)
            .execute(&state.pool)
        .await
            .map_err(|e| format!("Gagal menghapus log validasi untuk siswa {}: {}", peserta_didik_id, e))?;
        
        total_logs_deleted += deleted_logs.rows_affected();
    }
    
    emit_log(&app, &format!("CMD: auto_fix_hobby_minus_one_stealth - Berhasil memperbaiki {} siswa dan menghapus {} log validasi (STEALTH MODE)", updated_count, total_logs_deleted));
    
    Ok(format!("Berhasil memperbaiki id_hobby untuk {} siswa (tanpa jejak audit).", updated_count))
}

/// Memperbaiki id_cita yang bernilai NULL atau -1 dengan nilai random dari tabel ref.jenis_cita
#[tauri::command]
pub async fn auto_fix_cita_null_zero_stealth(
    app: AppHandle,
    state: State<'_, DbPool>
) -> Result<String, String> {
    emit_log(&app, "CMD: auto_fix_cita_null_zero_stealth - Memperbaiki id_cita yang bernilai NULL atau -1 (STEALTH MODE)");
    
    // LANGKAH 1: Ambil semua id_cita yang tersedia dari tabel ref.jenis_cita
    let cita_ids: Vec<(BigDecimal,)> = sqlx::query_as("SELECT id_cita FROM ref.jenis_cita WHERE id_cita > 0")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengambil data cita-cita: {}", e))?;
    
    if cita_ids.is_empty() {
        return Err("Tidak ada data cita-cita yang tersedia untuk dipilih secara acak.".to_string());
    }
    
    // LANGKAH 2: Hitung berapa banyak siswa yang memiliki id_cita = NULL atau -1
    let count_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM registrasi_peserta_didik WHERE id_cita IS NULL OR id_cita = -1")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| format!("Gagal menghitung siswa dengan id_cita NULL/-1: {}", e))?;
    
    if count_result == 0 {
        return Ok("Tidak ada siswa dengan id_cita NULL atau -1 yang perlu diperbaiki.".to_string());
    }
    
    // LANGKAH 3: Ambil semua siswa yang memiliki id_cita = NULL atau -1
    let students_with_invalid_cita: Vec<(SqlxUuid,)> = sqlx::query_as("SELECT peserta_didik_id FROM registrasi_peserta_didik WHERE id_cita IS NULL OR id_cita = -1")
        .fetch_all(&state.pool)
    .await
        .map_err(|e| format!("Gagal mengambil siswa dengan id_cita NULL/-1: {}", e))?;
    
    let mut updated_count = 0;
    
    // LANGKAH 4: Update setiap siswa dengan id_cita random
    for (peserta_didik_id,) in &students_with_invalid_cita {
        // Pilih cita-cita random
        let random_cita = cita_ids.choose(&mut rand::thread_rng()).unwrap().0.clone();
        
        // Update id_cita
        sqlx::query("UPDATE registrasi_peserta_didik SET id_cita = $1 WHERE peserta_didik_id = $2")
            .bind(&random_cita)
        .bind(peserta_didik_id)
            .execute(&state.pool)
        .await
            .map_err(|e| format!("Gagal update id_cita untuk siswa {}: {}", peserta_didik_id, e))?;
        
        updated_count += 1;
        
        emit_log(&app, &format!("CMD: auto_fix_cita_null_zero_stealth - Updated siswa {} dengan id_cita: {}", peserta_didik_id, random_cita));
    }
    
    // LANGKAH 5: Hapus log validasi yang terkait dengan cita-cita
    let mut total_logs_deleted = 0;
    for (peserta_didik_id,) in &students_with_invalid_cita {
        let deleted_logs = sqlx::query("DELETE FROM vld_peserta_didik WHERE peserta_didik_id = $1")
        .bind(peserta_didik_id)
            .execute(&state.pool)
        .await
            .map_err(|e| format!("Gagal menghapus log validasi untuk siswa {}: {}", peserta_didik_id, e))?;
        
        total_logs_deleted += deleted_logs.rows_affected();
    }
    
    emit_log(&app, &format!("CMD: auto_fix_cita_null_zero_stealth - Berhasil memperbaiki {} siswa dan menghapus {} log validasi (STEALTH MODE)", updated_count, total_logs_deleted));
    
    Ok(format!("Berhasil memperbaiki id_cita untuk {} siswa (tanpa jejak audit).", updated_count))
} 

/// Auto-fix NIK ayah tidak valid (spasi, dummy, dll) menjadi NULL
#[tauri::command]
pub async fn auto_fix_nik_ayah_invalid_stealth(
    app: AppHandle,
    state: State<'_, DbPool>
) -> Result<String, String> {
    emit_log(&app, "CMD: auto_fix_nik_ayah_invalid_stealth - Auto-fix NIK ayah tidak valid menjadi NULL (STEALTH MODE)");

    // LANGKAH 1: Nonaktifkan trigger audit
    sqlx::query("ALTER TABLE peserta_didik DISABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal menonaktifkan trigger audit: {}", e))?;

    // LANGKAH 2: Update NIK ayah yang tidak valid menjadi NULL
    let result = sqlx::query(r#"
        UPDATE peserta_didik SET nik_ayah = NULL 
        WHERE nik_ayah = '                ' 
           OR nik_ayah = '0000000000000000' 
           OR nik_ayah = '1111111111111111'
           OR nik_ayah ~ '^[[:space:]]+$'
           OR nik_ayah = '9999999999999999'
           OR nik_ayah = '1234567890123456'
           OR nik_ayah = ''
           OR nik_ayah = 'NULL'
    "#)
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal update NIK ayah tidak valid: {}", e))?;

    // LANGKAH 3: Hapus log validasi Dapodik untuk NIK ayah yang sudah diperbaiki
    let delete_result = sqlx::query(r#"
        DELETE FROM vld_peserta_didik 
        WHERE field_name = 'nik_ayah'
    "#)
        .execute(&state.pool)
        .await;
    
    let logs_deleted = match delete_result {
        Ok(result) => result.rows_affected(),
        Err(_) => 0
    };
    
    emit_log(&app, &format!("CMD: auto_fix_nik_ayah_invalid_stealth - Berhasil menghapus {} log validasi NIK ayah", logs_deleted));

    // LANGKAH 4: Aktifkan kembali trigger audit
    sqlx::query("ALTER TABLE peserta_didik ENABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal mengaktifkan trigger audit: {}", e))?;

    let rows_affected = result.rows_affected();
    emit_log(&app, &format!("CMD: auto_fix_nik_ayah_invalid_stealth - Berhasil memperbaiki {} NIK ayah tidak valid (STEALTH MODE)", rows_affected));
    
    Ok(format!("Berhasil memperbaiki {} NIK ayah tidak valid menjadi NULL dan menghapus log validasi Dapodik (tanpa jejak audit).", rows_affected))
} 

/// Auto-fix NIK ibu tidak valid (spasi, dummy, dll) menjadi NULL
#[tauri::command]
pub async fn auto_fix_nik_ibu_invalid_stealth(
    app: AppHandle,
    state: State<'_, DbPool>
) -> Result<String, String> {
    emit_log(&app, "CMD: auto_fix_nik_ibu_invalid_stealth - Auto-fix NIK ibu tidak valid menjadi NULL (STEALTH MODE)");

    // LANGKAH 1: Nonaktifkan trigger audit
    sqlx::query("ALTER TABLE peserta_didik DISABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal menonaktifkan trigger audit: {}", e))?;

    // LANGKAH 2: Update NIK ibu yang tidak valid menjadi NULL
    let result = sqlx::query(r#"
        UPDATE peserta_didik SET nik_ibu = NULL 
        WHERE nik_ibu = '                ' 
           OR nik_ibu = '0000000000000000' 
           OR nik_ibu = '1111111111111111'
           OR nik_ibu ~ '^[[:space:]]+$'
           OR nik_ibu = '9999999999999999'
           OR nik_ibu = '1234567890123456'
           OR nik_ibu = ''
           OR nik_ibu = 'NULL'
    "#)
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal update NIK ibu tidak valid: {}", e))?;

    // LANGKAH 3: Hapus log validasi Dapodik untuk NIK ibu yang sudah diperbaiki
    let delete_result = sqlx::query(r#"
        DELETE FROM vld_peserta_didik 
        WHERE field_name = 'nik_ibu'
    "#)
        .execute(&state.pool)
        .await;
    
    let logs_deleted = match delete_result {
        Ok(result) => result.rows_affected(),
        Err(_) => 0
    };
    
    emit_log(&app, &format!("CMD: auto_fix_nik_ibu_invalid_stealth - Berhasil menghapus {} log validasi NIK ibu", logs_deleted));

    // LANGKAH 4: Aktifkan kembali trigger audit
    sqlx::query("ALTER TABLE peserta_didik ENABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
    .await
        .map_err(|e| format!("Gagal mengaktifkan trigger audit: {}", e))?;

    let rows_affected = result.rows_affected();
    emit_log(&app, &format!("CMD: auto_fix_nik_ibu_invalid_stealth - Berhasil memperbaiki {} NIK ibu tidak valid (STEALTH MODE)", rows_affected));
    
    Ok(format!("Berhasil memperbaiki {} NIK ibu tidak valid menjadi NULL dan menghapus log validasi Dapodik (tanpa jejak audit).", rows_affected))
} 