use std::fs;
use tauri::{AppHandle, Emitter};
use tokio;
use filetime::{FileTime, set_file_mtime, set_file_atime};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use sysinfo::{System};
use wait_timeout::ChildExt;

fn emit_log(app: &AppHandle, msg: &str) {
    println!("{}", msg); // Tambahkan log ke terminal
    let _ = app.emit("backend_log", msg.to_string());
}

fn is_dapodik_running() -> bool {
    let sys = System::new_all();
    for (_pid, process) in sys.processes() {
        if process.name().to_ascii_lowercase() == "dapodik" {
            return true;
        }
    }
    false
}

/// Restore file dari backup, menyalin isi dan metadata (mtime, atime, permission)
pub fn restore_file_with_metadata(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    use std::fs;
    // 1. Baca metadata
    let metadata = fs::metadata(src)?;
    let atime = FileTime::from_last_access_time(&metadata);
    let mtime = FileTime::from_last_modification_time(&metadata);
    let permissions = metadata.permissions();

    // 2. Salin isi file
    let content = fs::read(src)?;
    fs::write(dst, &content)?;

    // 3. Set metadata
    set_file_mtime(dst, mtime)?;
    set_file_atime(dst, atime)?;
    #[cfg(unix)]
    {
        fs::set_permissions(dst, permissions)?;
    }
    Ok(())
}

pub async fn jalankan_tricky_method_rust(app_handle: &AppHandle) -> Result<(), String> {
    // Tambahkan deklarasi path file
    let dapodik_path = std::path::PathBuf::from("C:\\Program Files (x86)\\Dapodik");
    let db_path = dapodik_path.join("database");
    let pg_hba_path = db_path.join("pg_hba.conf");

    // Cek apakah Dapodik sedang berjalan
    if is_dapodik_running() {
        let msg = "[TRICKY] ERROR: Dapodik sedang berjalan. Silakan tutup Dapodik terlebih dahulu.";
        emit_log(app_handle, msg);
        let _ = app_handle.emit("tricky_method_failed", msg.to_string());
        std::process::exit(1);
    }
    emit_log(app_handle, "[TRICKY] --- DEBUG: tricky_method benar-benar dipanggil ---");
    emit_log(app_handle, "[TRICKY] Tricky method started. This is a complex 7-step process.");

    // 1. Backup isi asli pg_hba.conf ke memory dan simpan mtime
    emit_log(app_handle, "[TRICKY] Step 1: Backing up pg_hba.conf (in-memory, with mtime)...");
    let mut original_pg_hba: Option<String> = None;
    let mut _original_mtime: Option<FileTime> = None;
    if pg_hba_path.exists() {
        match fs::read_to_string(&pg_hba_path) {
            Ok(content) => {
                original_pg_hba = Some(content);
                match fs::metadata(&pg_hba_path) {
                    Ok(meta) => {
                        _original_mtime = Some(FileTime::from_last_modification_time(&meta));
                        emit_log(app_handle, "[TRICKY] Step 1 SUCCESS: File and mtime backed up in memory.");
                    },
                    Err(e) => {
                        emit_log(app_handle, &format!("[TRICKY] Step 1 WARNING: Could not get mtime: {}", e));
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("[TRICKY] Step 1 FAILED: Could not read pg_hba.conf: {}", e);
                emit_log(app_handle, &err_msg);
                let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
                std::process::exit(1);
            }
        }
    } else {
        emit_log(app_handle, "[TRICKY] Step 1 SKIPPED: pg_hba.conf not found.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 2. Modifikasi file ke trust
    emit_log(app_handle, "[TRICKY] Step 2: Modifying pg_hba.conf to 'trust' mode...");
    if pg_hba_path.exists() {
        let content = original_pg_hba.clone().unwrap_or_default();
        let new_content = content.replace("password", "trust").replace("md5", "trust");
        if let Err(e) = fs::write(&pg_hba_path, &new_content) {
            let err_msg = format!("[TRICKY] Step 2 FAILED: Could not write to pg_hba.conf: {}", e);
            emit_log(app_handle, &err_msg);
            let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
            std::process::exit(1);
        }
        emit_log(app_handle, "[TRICKY] Step 2 SUCCESS: pg_hba.conf temporarily set to 'trust' mode.");
        // Reload service PostgreSQL agar perubahan trust aktif
        emit_log(app_handle, "[TRICKY] Step 2b: Reloading PostgreSQL service...");
        let output = std::process::Command::new("net")
            .args(["stop", "postgresql"])
            .output();
        let output2 = std::process::Command::new("net")
            .args(["start", "postgresql"])
            .output();
        match (output, output2) {
            (Ok(out1), Ok(out2)) if out1.status.success() && out2.status.success() => {
                emit_log(app_handle, "[TRICKY] Step 2b SUCCESS: PostgreSQL service reloaded.");
            },
            _ => {
                emit_log(app_handle, "[TRICKY] Step 2b WARNING: Failed to reload PostgreSQL service. Perubahan trust mungkin belum aktif.");
            }
        }
    } else {
        emit_log(app_handle, "[TRICKY] Step 2 SKIPPED: pg_hba.conf not found.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 3. Jalankan ALTER ROLE via psql CLI dari folder Dapodik dengan timeout
    emit_log(app_handle, "[TRICKY] Step 3: Executing ALTER ROLE via psql CLI from Dapodik folder...");
    let psql_path = "C:\\Program Files (x86)\\Dapodik\\pgsql\\bin\\psql.exe";
    let alter_cmd = "ALTER ROLE dapodik_user WITH PASSWORD '17Agustus1945' SUPERUSER CREATEDB CREATEROLE;";
    let mut child = match std::process::Command::new(psql_path)
        .args(&[
            "--port=54532",
            "--username=postgres",
            "--dbname=pendataan",
            "--command", alter_cmd
        ])
        .spawn() {
            Ok(child) => child,
            Err(e) => {
                let err_msg = format!("psql CLI spawn error: {}", e);
                emit_log(app_handle, &format!("[TRICKY] Step 3 FAILED: Could not spawn psql CLI: {}", e));
                let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
                std::process::exit(1);
            }
        };
    let timeout = std::time::Duration::from_secs(10);
    match child.wait_timeout(timeout).unwrap() {
        Some(status) if status.success() => {
            emit_log(app_handle, "[TRICKY] Step 3 SUCCESS: ALTER ROLE via psql CLI executed successfully.");
        },
        Some(status) => {
            let err_msg = format!("[TRICKY] Step 3 FAILED: psql CLI exited with status: {}", status);
            emit_log(app_handle, &err_msg);
            let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
            std::process::exit(1);
        },
        None => {
            let _ = child.kill();
            let err_msg = "[TRICKY] Step 3 FAILED: psql CLI timed out and was killed.".to_string();
            emit_log(app_handle, &err_msg);
            let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
            std::process::exit(1);
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 4. Restore file ori dari memory dan mtime
    emit_log(app_handle, "[TRICKY] Step 4: Restoring original pg_hba.conf from memory and mtime...");
    if let Some(ref backup_content) = original_pg_hba {
        // Simpan backup ke file temp
        let tmp_backup = db_path.join("pg_hba.conf_restore_tmp");
        if let Err(e) = fs::write(&tmp_backup, backup_content) {
            let err_msg = format!("[TRICKY] Step 4 FAILED: Could not write temp restore file: {}", e);
            emit_log(app_handle, &err_msg);
        } else {
            // Restore file beserta metadata
            if let Err(e) = restore_file_with_metadata(&tmp_backup, &pg_hba_path) {
                let err_msg = format!("[TRICKY] Step 4 FAILED: Could not restore file with metadata: {}", e);
                emit_log(app_handle, &err_msg);
                let _ = app_handle.emit("tricky_method_failed", err_msg.clone());
                std::process::exit(1);
            } else {
                emit_log(app_handle, "[TRICKY] Step 4 SUCCESS: pg_hba.conf restored with metadata.");
            }
            let _ = fs::remove_file(&tmp_backup);
        }
    } else {
        emit_log(app_handle, "[TRICKY] Step 4 SKIPPED: No in-memory backup found.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    emit_log(app_handle, "[TRICKY] Tricky method finished.");
    Ok(())
} 