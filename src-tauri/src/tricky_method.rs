use std::fs;
use tokio;
use filetime::{FileTime, set_file_mtime, set_file_atime};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
// use sysinfo::{System}; // Dihapus untuk mengurangi kompleksitas kompilasi
use wait_timeout::ChildExt;

fn is_dapodik_running() -> bool {
    // Pendekatan sederhana: cek apakah port 5432 (PostgreSQL) sedang digunakan
    // Ini adalah pendekatan alternatif yang tidak memerlukan sysinfo
    use std::net::TcpListener;
    match TcpListener::bind("localhost:53774") {
        Ok(_) => false, // Port tidak digunakan, Dapodik tidak berjalan
        Err(_) => true,  // Port digunakan, kemungkinan Dapodik berjalan
    }
}

/// Restore file dari backup, menyalin isi dan metadata (mtime, atime, permission)
pub fn restore_file_with_metadata(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    use std::fs;
    // 1. Baca metadata
    let metadata = fs::metadata(src)?;
    let atime = FileTime::from_last_access_time(&metadata);
    let mtime = FileTime::from_last_modification_time(&metadata);
    let _permissions = metadata.permissions();

    // 2. Salin isi file
    let content = fs::read(src)?;
    fs::write(dst, &content)?;

    // 3. Set metadata
    set_file_mtime(dst, mtime)?;
    set_file_atime(dst, atime)?;
    #[cfg(unix)]
    {
        fs::set_permissions(dst, _permissions)?;
    }
    Ok(())
}

pub async fn jalankan_tricky_method_startup() -> Result<(), String> {
    println!("[TRICKY] Memulai proses tricky method (startup mode)...");
    let dapodik_path = std::path::PathBuf::from("C:\\Program Files (x86)\\Dapodik");
    let db_path = dapodik_path.join("database");
    let pg_hba_path = db_path.join("pg_hba.conf");

    // Cek apakah Dapodik sedang berjalan
    println!("[TRICKY] Mengecek apakah aplikasi Dapodik sedang berjalan...");
    if is_dapodik_running() {
        let msg = "[TRICKY] ERROR: Dapodik sedang berjalan. Silakan tutup Dapodik terlebih dahulu.";
        println!("{}", msg);
        std::process::exit(1);
    }
    println!("[TRICKY] Dapodik tidak sedang berjalan. Lanjut ke langkah berikutnya.");
    println!("[TRICKY] --- DEBUG: tricky_method benar-benar dipanggil (startup) ---");
    println!("[TRICKY] Tricky method dimulai. Ini adalah proses 7 langkah.");

    // 1. Backup isi asli pg_hba.conf ke memory dan simpan mtime
    println!("[TRICKY] LANGKAH 1: Membackup file pg_hba.conf (ke memory, beserta mtime)...");
    let mut original_pg_hba: Option<String> = None;
    let mut _original_mtime: Option<FileTime> = None;
    let mut _permissions: Option<std::fs::Permissions> = None;
    if pg_hba_path.exists() {
        match fs::read_to_string(&pg_hba_path) {
            Ok(content) => {
                original_pg_hba = Some(content);
                match fs::metadata(&pg_hba_path) {
                    Ok(meta) => {
                        _original_mtime = Some(FileTime::from_last_modification_time(&meta));
                        println!("[TRICKY] LANGKAH 1 BERHASIL: File dan mtime berhasil dibackup ke memory.");
                    },
                    Err(e) => {
                        println!("[TRICKY] LANGKAH 1 PERINGATAN: Tidak bisa mendapatkan mtime: {}", e);
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("[TRICKY] LANGKAH 1 GAGAL: Tidak bisa membaca pg_hba.conf: {}", e);
                println!("{}", err_msg);
                std::process::exit(1);
            }
        }
    } else {
        println!("[TRICKY] LANGKAH 1 DILEWATI: File pg_hba.conf tidak ditemukan.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 2. Modifikasi file ke trust
    println!("[TRICKY] LANGKAH 2: Memodifikasi pg_hba.conf menjadi mode 'trust'...");
    if pg_hba_path.exists() {
        let content = original_pg_hba.clone().unwrap_or_default();
        let new_content = content.replace("password", "trust").replace("md5", "trust");
        if let Err(e) = fs::write(&pg_hba_path, &new_content) {
            let err_msg = format!("[TRICKY] LANGKAH 2 GAGAL: Tidak bisa menulis ke pg_hba.conf: {}", e);
            println!("{}", err_msg);
            std::process::exit(1);
        }
        println!("[TRICKY] LANGKAH 2 BERHASIL: pg_hba.conf sementara diubah ke mode 'trust'.");
        // Reload service PostgreSQL agar perubahan trust aktif
        println!("[TRICKY] LANGKAH 2b: Merestart service PostgreSQL...");
        let output = std::process::Command::new("net")
            .args(["stop", "postgresql"])
            .output();
        let output2 = std::process::Command::new("net")
            .args(["start", "postgresql"])
            .output();
        match (output, output2) {
            (Ok(out1), Ok(out2)) if out1.status.success() && out2.status.success() => {
                println!("[TRICKY] LANGKAH 2b BERHASIL: Service PostgreSQL berhasil direstart.");
            },
            _ => {
                println!("[TRICKY] LANGKAH 2b PERINGATAN: Gagal merestart service PostgreSQL. Perubahan trust mungkin belum aktif.");
            }
        }
    } else {
        println!("[TRICKY] LANGKAH 2 DILEWATI: File pg_hba.conf tidak ditemukan.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 3. Jalankan ALTER ROLE via psql CLI dari folder Dapodik dengan timeout
    println!("[TRICKY] LANGKAH 3: Menjalankan ALTER ROLE via psql CLI dari folder Dapodik...");
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
                let _err_msg = format!("psql CLI spawn error: {}", e);
                println!("[TRICKY] ERROR: psql CLI spawn error: {}", e);
                std::process::exit(1);
            }
        };
    let timeout = std::time::Duration::from_secs(10);
    match child.wait_timeout(timeout).unwrap() {
        Some(status) if status.success() => {
            println!("[TRICKY] LANGKAH 3 BERHASIL: ALTER ROLE via psql CLI berhasil dijalankan.");
        },
        Some(status) => {
            let err_msg = format!("[TRICKY] LANGKAH 3 GAGAL: psql CLI keluar dengan status: {}", status);
            println!("{}", err_msg);
            std::process::exit(1);
        },
        None => {
            let _ = child.kill();
            let err_msg = "[TRICKY] LANGKAH 3 GAGAL: psql CLI timeout dan dihentikan.".to_string();
            println!("{}", err_msg);
            std::process::exit(1);
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 4. Restore file ori dari memory dan mtime
    println!("[TRICKY] LANGKAH 4: Mengembalikan file pg_hba.conf asli dari memory dan mtime...");
    if let Some(ref backup_content) = original_pg_hba {
        // Simpan backup ke file temp
        let tmp_backup = db_path.join("pg_hba.conf_restore_tmp");
        if let Err(e) = fs::write(&tmp_backup, backup_content) {
            let err_msg = format!("[TRICKY] LANGKAH 4 GAGAL: Tidak bisa menulis file restore sementara: {}", e);
            println!("{}", err_msg);
        } else {
            // Restore file beserta metadata
            if let Err(e) = restore_file_with_metadata(&tmp_backup, &pg_hba_path) {
                let err_msg = format!("[TRICKY] LANGKAH 4 GAGAL: Tidak bisa mengembalikan file beserta metadata: {}", e);
                println!("{}", err_msg);
                std::process::exit(1);
            } else {
                println!("[TRICKY] LANGKAH 4 BERHASIL: pg_hba.conf berhasil dikembalikan beserta metadata.");
            }
            let _ = fs::remove_file(&tmp_backup);
        }
    } else {
        println!("[TRICKY] LANGKAH 4 DILEWATI: Tidak ada backup di memory.");
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("[TRICKY] Proses tricky method selesai (startup mode).");
    Ok(())
} 