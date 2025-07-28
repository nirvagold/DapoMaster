use crate::app_state::DbPool;
use sqlx::types::Uuid as SqlxUuid;
use tauri::{AppHandle, State};

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Pengguna {
    pub pengguna_id: SqlxUuid,
    pub username: String,
    pub sekolah_id: SqlxUuid,
}

#[tauri::command]
pub async fn ambil_semua_pengguna(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<Pengguna>, String> {
    crate::emit_log(&app, "CMD: ambil_semua_pengguna - Fetching all users.");
    let result = sqlx::query_as::<_, Pengguna>("SELECT username, mp.pengguna_id, mp.sekolah_id FROM man_akses.pengguna mp, man_akses.role_pengguna mrp WHERE mrp.peran_id=10 AND mp.pengguna_id=mrp.pengguna_id")
        .fetch_all(&state.pool)
        .await;
    
    match result {
        Ok(pengguna) => {
            crate::emit_log(&app, &format!("CMD: ambil_semua_pengguna - Success, found {} users.", pengguna.len()));
            Ok(pengguna)
        },
        Err(e) => {
            let err_msg = format!("CMD: ambil_semua_pengguna - ERROR: {}", e);
            crate::emit_log(&app, &err_msg);
            Err(err_msg)
        },
    }
} 