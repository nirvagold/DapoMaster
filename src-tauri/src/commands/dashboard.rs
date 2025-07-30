use crate::app_state::DbPool;
use tauri::{AppHandle, State};

#[derive(serde::Serialize)]
pub struct DashboardStats {
    pub total_siswa: i64,
    pub total_ptk: i64,
    pub total_rombel: i64,
    pub total_jurusan: i64,
}

#[tauri::command]
pub async fn get_dashboard_stats(app: AppHandle, state: State<'_, DbPool>) -> Result<DashboardStats, String> {
    crate::emit_log(&app, "CMD: get_dashboard_stats - Fetching stats.");

    let total_siswa: (i64,) = sqlx::query_as("SELECT COUNT(pd.*) FROM peserta_didik pd JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NULL")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_ptk: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT pt.ptk_id) FROM ptk pt JOIN ptk_terdaftar ptt ON pt.ptk_id = ptt.ptk_id WHERE pt.soft_delete = 0 AND ptt.jenis_keluar_id IS NULL")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_rombel: (i64,) = sqlx::query_as("SELECT COUNT(rb.*) FROM rombongan_belajar rb JOIN ref.semester s ON rb.semester_id = s.semester_id WHERE rb.soft_delete = 0 AND s.periode_aktif = 1")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_jurusan: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jurusan_sp WHERE soft_delete = 0")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;

    let stats = DashboardStats {
        total_siswa: total_siswa.0,
        total_ptk: total_ptk.0,
        total_rombel: total_rombel.0,
        total_jurusan: total_jurusan.0,
    };
    
    crate::emit_log(&app, "CMD: get_dashboard_stats - Successfully fetched all stats.");
    Ok(stats)
} 