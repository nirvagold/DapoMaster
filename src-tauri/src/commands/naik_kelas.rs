use crate::app_state::DbPool;
use crate::emit_log;
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::types::Uuid as SqlxUuid;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct SiswaRombel {
    pub peserta_didik_id: SqlxUuid,
    pub nama: String,
    pub nisn: String,
    pub nama_rombel: String,
    pub tingkat_pendidikan_id: BigDecimal,
    pub tingkat_pendidikan_nama: String,
}

/// Mendapatkan daftar siswa yang akan naik kelas dari semester sebelumnya
#[tauri::command]
pub async fn get_siswa_naik_kelas(
    app: AppHandle,
    state: State<'_, DbPool>,
    semester_sebelumnya: String
) -> Result<Vec<SiswaRombel>, String> {
    emit_log(&app, &format!("CMD: get_siswa_naik_kelas - Mengambil data siswa naik kelas dari {}", semester_sebelumnya));
    
    let query = r#"
        SELECT 
            pd.peserta_didik_id,
            pd.nama,
            pd.nisn,
            rb.nama as nama_rombel,
            rb.tingkat_pendidikan_id,
            tp.nama as tingkat_pendidikan_nama
        FROM anggota_rombel ar
        JOIN rombongan_belajar rb ON ar.rombongan_belajar_id = rb.rombongan_belajar_id
        JOIN peserta_didik pd ON ar.peserta_didik_id = pd.peserta_didik_id
        JOIN ref.tingkat_pendidikan tp ON rb.tingkat_pendidikan_id = tp.tingkat_pendidikan_id
        WHERE ar.soft_delete = 0 
        AND rb.soft_delete = 0 
        AND pd.soft_delete = 0 
        AND rb.semester_id = $1
        AND rb.tingkat_pendidikan_id < 6
        ORDER BY rb.tingkat_pendidikan_id, rb.nama, pd.nama
    "#;
    
    let rows = sqlx::query(query)
        .bind(&semester_sebelumnya)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengambil data siswa naik kelas: {}", e))?;
    
    let mut siswa_list = Vec::new();
    for row in rows {
        siswa_list.push(SiswaRombel {
            peserta_didik_id: row.get("peserta_didik_id"),
            nama: row.get("nama"),
            nisn: row.get("nisn"),
            nama_rombel: row.get("nama_rombel"),
            tingkat_pendidikan_id: row.get("tingkat_pendidikan_id"),
            tingkat_pendidikan_nama: row.get("tingkat_pendidikan_nama"),
        });
    }
    
    emit_log(&app, &format!("CMD: get_siswa_naik_kelas - Berhasil mengambil {} siswa untuk naik kelas", siswa_list.len()));
    Ok(siswa_list)
}

/// Mendapatkan daftar semester yang tersedia
#[tauri::command]
pub async fn get_daftar_semester(
    app: AppHandle,
    state: State<'_, DbPool>
) -> Result<Vec<serde_json::Value>, String> {
    emit_log(&app, "CMD: get_daftar_semester - Mengambil daftar semester");
    
    let query = r#"
        SELECT 
            semester_id,
            nama,
            tahun_ajaran_id,
            semester
        FROM ref.semester 
        WHERE expired_date IS NULL OR expired_date > NOW()
        ORDER BY semester_id DESC
    "#;
    
    let rows = sqlx::query(query)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengambil daftar semester: {}", e))?;
    
    let mut semester_list = Vec::new();
    for row in rows {
        semester_list.push(serde_json::json!({
            "semester_id": row.get::<String, _>("semester_id"),
            "nama": row.get::<String, _>("nama"),
            "tahun_ajaran_id": row.get::<BigDecimal, _>("tahun_ajaran_id").to_string(),
            "semester": row.get::<BigDecimal, _>("semester").to_string()
        }));
    }
    
    emit_log(&app, &format!("CMD: get_daftar_semester - Berhasil mengambil {} semester", semester_list.len()));
    Ok(semester_list)
} 