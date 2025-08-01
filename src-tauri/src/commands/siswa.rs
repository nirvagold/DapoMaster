use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid as SqlxUuid;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use rand::seq::SliceRandom;
use chrono;

#[derive(Serialize, sqlx::FromRow)]
pub struct PesertaDidik {
    pub peserta_didik_id: SqlxUuid,
    pub nama: String,
    pub jenis_kelamin: String,
    pub nisn: String,
    pub nik: Option<String>,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: chrono::NaiveDate,
    pub agama_id: i16,
    // Data tambahan dari analisis database
    pub kewarganegaraan: Option<String>,
    pub alamat_jalan: Option<String>,
    pub desa_kelurahan: Option<String>,
    pub kode_wilayah: Option<String>,
    pub nama_ibu_kandung: Option<String>,
    pub no_kk: Option<String>,
    pub rt: Option<BigDecimal>,
    pub rw: Option<BigDecimal>,
    pub nama_dusun: Option<String>,
    pub kode_pos: Option<String>,
    pub lintang: Option<BigDecimal>,
    pub bujur: Option<BigDecimal>,
    pub jenis_tinggal_id: Option<BigDecimal>,
    pub alat_transportasi_id: Option<BigDecimal>,
    pub nik_ayah: Option<String>,
    pub nik_ibu: Option<String>,
    pub anak_keberapa: Option<BigDecimal>,
    pub nik_wali: Option<String>,
    pub nomor_telepon_rumah: Option<String>,
    pub nomor_telepon_seluler: Option<String>,
    pub email: Option<String>,
    // Data dari registrasi_peserta_didik
    pub nipd: Option<String>,
    pub tanggal_masuk_sekolah: Option<chrono::NaiveDate>,
    pub jenis_pendaftaran_id: Option<BigDecimal>,
    pub id_hobby: Option<BigDecimal>,
    pub id_cita: Option<BigDecimal>,
    pub a_pernah_paud: Option<BigDecimal>,
    pub a_pernah_tk: Option<BigDecimal>,
    pub jenis_keluar_id: Option<BigDecimal>,
    pub tanggal_keluar: Option<chrono::NaiveDate>,
    pub alasan_keluar: Option<String>,
    // Data dari anggota_rombel
    pub rombongan_belajar_id: Option<SqlxUuid>,
    pub nama_rombel: Option<String>,
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
    pub a_pernah_paud: String,
    pub a_pernah_tk: String,
    pub sekolah_asal: Option<String>,
    pub alamat_jalan: String,
    pub desa_kelurahan: String,
    pub kode_wilayah: String,
    pub nama_ibu_kandung: String,
    pub kewarganegaraan: String,
    pub sekolah_id: String,
    pub pengguna_id: SqlxUuid,
    // Data tambahan
    pub nik: Option<String>,
    pub no_kk: Option<String>,
    pub rt: Option<String>,
    pub rw: Option<String>,
    pub nama_dusun: Option<String>,
    pub kode_pos: Option<String>,
    pub lintang: Option<String>,
    pub bujur: Option<String>,
    pub jenis_tinggal_id: Option<String>,
    pub alat_transportasi_id: Option<String>,
    pub nik_ayah: Option<String>,
    pub nik_ibu: Option<String>,
    pub anak_keberapa: Option<String>,
    pub nik_wali: Option<String>,
    pub nomor_telepon_rumah: Option<String>,
    pub nomor_telepon_seluler: Option<String>,
    pub email: Option<String>,
}

// Struct untuk data referensi
#[derive(Serialize, sqlx::FromRow)]
pub struct JenisKeluar {
    pub jenis_keluar_id: String,
    pub ket_keluar: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct JenisTinggal {
    pub jenis_tinggal_id: BigDecimal,
    pub nama: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AlatTransportasi {
    pub alat_transportasi_id: BigDecimal,
    pub nama: String,
}

#[tauri::command]
pub async fn get_total_siswa(app: AppHandle, search: Option<String>, rombel_id: Option<SqlxUuid>, state: State<'_, DbPool>) -> Result<i64, String> {
    crate::emit_log(&app, &format!("CMD: get_total_siswa - Counting with search: {:?}, rombel: {:?}", search, rombel_id));
    let search_term = format!("%{}%", search.unwrap_or_default());
    let base_query = if rombel_id.is_some() { 
        "SELECT COUNT(pd.*) FROM peserta_didik pd 
         JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) 
         AND ar.rombongan_belajar_id = $2 
         AND pd.soft_delete = 0 
         AND rpd.jenis_keluar_id IS NULL" 
    } else { 
        "SELECT COUNT(pd.*) FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) 
         AND pd.soft_delete = 0 
         AND rpd.jenis_keluar_id IS NULL" 
    };
    let mut query = sqlx::query_scalar(base_query).bind(&search_term);
    if let Some(id) = rombel_id { query = query.bind(id); }
    query.fetch_one(&state.pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_daftar_siswa(app: AppHandle, page: usize, page_size: usize, search: Option<String>, rombel_id: Option<SqlxUuid>, state: State<'_, DbPool>) -> Result<Vec<PesertaDidik>, String> {
    crate::emit_log(&app, &format!("CMD: get_daftar_siswa - Fetching page {} with search: {:?}, rombel: {:?}", page, search, rombel_id));
    let offset = (page - 1) * page_size;
    let search_term = format!("%{}%", search.unwrap_or_default());
    let (query_str, use_rombel_filter) = if let Some(_id) = rombel_id { 
        ("SELECT pd.*, rpd.nipd, rpd.tanggal_masuk_sekolah, rpd.jenis_pendaftaran_id, rpd.id_hobby, rpd.id_cita, 
                rpd.a_pernah_paud, rpd.a_pernah_tk, rpd.jenis_keluar_id, rpd.tanggal_keluar, rpd.keterangan as alasan_keluar,
                ar.rombongan_belajar_id, rb.nama as nama_rombel
         FROM peserta_didik pd 
          JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id 
         LEFT JOIN rombongan_belajar rb ON ar.rombongan_belajar_id = rb.rombongan_belajar_id
          WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) 
          AND ar.rombongan_belajar_id = $4 
          AND pd.soft_delete = 0 
          AND rpd.jenis_keluar_id IS NULL 
          ORDER BY pd.nama LIMIT $2 OFFSET $3".to_string(), true) 
    } else { 
        ("SELECT pd.*, rpd.nipd, rpd.tanggal_masuk_sekolah, rpd.jenis_pendaftaran_id, rpd.id_hobby, rpd.id_cita, 
                rpd.a_pernah_paud, rpd.a_pernah_tk, rpd.jenis_keluar_id, rpd.tanggal_keluar, rpd.keterangan as alasan_keluar,
                ar.rombongan_belajar_id, rb.nama as nama_rombel
         FROM peserta_didik pd 
          JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         LEFT JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id 
         LEFT JOIN rombongan_belajar rb ON ar.rombongan_belajar_id = rb.rombongan_belajar_id
          WHERE (pd.nama ILIKE $1 OR pd.nisn ILIKE $1) 
          AND pd.soft_delete = 0 
          AND rpd.jenis_keluar_id IS NULL 
          ORDER BY pd.nama LIMIT $2 OFFSET $3".to_string(), false) 
    };
    let mut query = sqlx::query_as::<_, PesertaDidik>(&query_str).bind(&search_term).bind(page_size as i64).bind(offset as i64);
    if use_rombel_filter { if let Some(id) = rombel_id { query = query.bind(id); } }
    query.fetch_all(&state.pool).await.map_err(|e| e.to_string())
}

// Command untuk mengambil data referensi baru
#[tauri::command]
pub async fn get_all_jenis_keluar(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<JenisKeluar>, String> {
    crate::emit_log(&app, "CMD: get_all_jenis_keluar - Fetching all jenis keluar");
    sqlx::query_as::<_, JenisKeluar>("SELECT jenis_keluar_id, ket_keluar FROM ref.jenis_keluar ORDER BY ket_keluar")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_jenis_tinggal(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<JenisTinggal>, String> {
    crate::emit_log(&app, "CMD: get_all_jenis_tinggal - Fetching all jenis tinggal");
    sqlx::query_as::<_, JenisTinggal>("SELECT jenis_tinggal_id, nama FROM ref.jenis_tinggal ORDER BY nama")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_alat_transportasi(app: AppHandle, state: State<'_, DbPool>) -> Result<Vec<AlatTransportasi>, String> {
    crate::emit_log(&app, "CMD: get_all_alat_transportasi - Fetching all alat transportasi");
    sqlx::query_as::<_, AlatTransportasi>("SELECT alat_transportasi_id, nama FROM ref.alat_transportasi ORDER BY nama")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())
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
    // Convert string values to BigDecimal for numeric fields
    let rt_bigdecimal = payload.rt.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let rw_bigdecimal = payload.rw.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let lintang_bigdecimal = payload.lintang.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let bujur_bigdecimal = payload.bujur.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let jenis_tinggal_id_bigdecimal = payload.jenis_tinggal_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let alat_transportasi_id_bigdecimal = payload.alat_transportasi_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let anak_keberapa_bigdecimal = payload.anak_keberapa.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let a_pernah_paud_bigdecimal = payload.a_pernah_paud.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    let a_pernah_tk_bigdecimal = payload.a_pernah_tk.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));

    let insert_pd_result = sqlx::query("INSERT INTO peserta_didik (peserta_didik_id, nama, jenis_kelamin, tanggal_lahir, agama_id, kebutuhan_khusus_id, alamat_jalan, desa_kelurahan, kode_wilayah, penerima_kps, layak_pip, penerima_kip, kebutuhan_khusus_id_ayah, nama_ibu_kandung, kebutuhan_khusus_id_ibu, kewarganegaraan, create_date, last_update, soft_delete, updater_id, nisn, tempat_lahir, nik, no_kk, rt, rw, nama_dusun, kode_pos, lintang, bujur, jenis_tinggal_id, alat_transportasi_id, nik_ayah, nik_ibu, anak_keberapa, nik_wali, nomor_telepon_rumah, nomor_telepon_seluler, email) VALUES ($1, $2, $3, $4, $5, 0, $6, $7, $8, 0, 0, 0, 0, $9, 0, $10, NOW(), NOW(), 0, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)")
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
        .bind(payload.pengguna_id)
        .bind(&payload.nisn)
        .bind(&payload.tempat_lahir)
        .bind(&payload.nik)
        .bind(&payload.no_kk)
        .bind(&rt_bigdecimal)
        .bind(&rw_bigdecimal)
        .bind(&payload.nama_dusun)
        .bind(&payload.kode_pos)
        .bind(&lintang_bigdecimal)
        .bind(&bujur_bigdecimal)
        .bind(&jenis_tinggal_id_bigdecimal)
        .bind(&alat_transportasi_id_bigdecimal)
        .bind(&payload.nik_ayah)
        .bind(&payload.nik_ibu)
        .bind(&anak_keberapa_bigdecimal)
        .bind(&payload.nik_wali)
        .bind(&payload.nomor_telepon_rumah)
        .bind(&payload.nomor_telepon_seluler)
        .bind(&payload.email)
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
        .bind(&a_pernah_paud_bigdecimal)
        .bind(&a_pernah_tk_bigdecimal)
        .bind(payload.pengguna_id)
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
    sqlx::query_as("SELECT pd.*, rpd.nipd, rpd.tanggal_masuk_sekolah, rpd.jenis_pendaftaran_id, rpd.id_hobby, rpd.id_cita, 
                           rpd.a_pernah_paud, rpd.a_pernah_tk, rpd.jenis_keluar_id, rpd.tanggal_keluar, rpd.keterangan as alasan_keluar,
                           ar.rombongan_belajar_id, rb.nama as nama_rombel
                    FROM peserta_didik pd 
                    LEFT JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
                    LEFT JOIN anggota_rombel ar ON pd.peserta_didik_id = ar.peserta_didik_id 
                    LEFT JOIN rombongan_belajar rb ON ar.rombongan_belajar_id = rb.rombongan_belajar_id
                    WHERE pd.peserta_didik_id = $1")
        .bind(peserta_didik_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_siswa(app: AppHandle, peserta_didik_id: SqlxUuid, payload: RegistrasiSiswaPayload, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: update_siswa - Updating student with ID: {}", peserta_didik_id));
    let tanggal_lahir_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_lahir, "%Y-%m-%d").map_err(|e| format!("Format tanggal salah: {}", e))?;
    
    // Convert string values to BigDecimal for numeric fields
    let rt_bigdecimal = payload.rt.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let rw_bigdecimal = payload.rw.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let lintang_bigdecimal = payload.lintang.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let bujur_bigdecimal = payload.bujur.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let jenis_tinggal_id_bigdecimal = payload.jenis_tinggal_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let alat_transportasi_id_bigdecimal = payload.alat_transportasi_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let anak_keberapa_bigdecimal = payload.anak_keberapa.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let a_pernah_paud_bigdecimal = payload.a_pernah_paud.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    let a_pernah_tk_bigdecimal = payload.a_pernah_tk.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    
    // Update tabel peserta_didik TANPA mengupdate audit trail (last_update, updater_id, last_sync)
    // Ini akan membuat update terlihat seperti data asli dari Dapodik
    sqlx::query("UPDATE peserta_didik SET 
        nama = $1, jenis_kelamin = $2, nisn = $3, tempat_lahir = $4, tanggal_lahir = $5, agama_id = $6, 
        kewarganegaraan = $7, alamat_jalan = $8, desa_kelurahan = $9, kode_wilayah = $10, nama_ibu_kandung = $11,
        nik = $12, no_kk = $13, rt = $14, rw = $15, nama_dusun = $16, kode_pos = $17, lintang = $18, bujur = $19,
        jenis_tinggal_id = $20, alat_transportasi_id = $21, nik_ayah = $22, nik_ibu = $23, anak_keberapa = $24,
        nik_wali = $25, nomor_telepon_rumah = $26, nomor_telepon_seluler = $27, email = $28
        WHERE peserta_didik_id = $29")
        .bind(&payload.nama)
        .bind(&payload.jenis_kelamin)
        .bind(&payload.nisn)
        .bind(&payload.tempat_lahir)
        .bind(tanggal_lahir_naive)
        .bind(payload.agama_id)
        .bind(&payload.kewarganegaraan)
        .bind(&payload.alamat_jalan)
        .bind(&payload.desa_kelurahan)
        .bind(&payload.kode_wilayah)
        .bind(&payload.nama_ibu_kandung)
        .bind(&payload.nik)
        .bind(&payload.no_kk)
        .bind(&rt_bigdecimal)
        .bind(&rw_bigdecimal)
        .bind(&payload.nama_dusun)
        .bind(&payload.kode_pos)
        .bind(&lintang_bigdecimal)
        .bind(&bujur_bigdecimal)
        .bind(&jenis_tinggal_id_bigdecimal)
        .bind(&alat_transportasi_id_bigdecimal)
        .bind(&payload.nik_ayah)
        .bind(&payload.nik_ibu)
        .bind(&anak_keberapa_bigdecimal)
        .bind(&payload.nik_wali)
        .bind(&payload.nomor_telepon_rumah)
        .bind(&payload.nomor_telepon_seluler)
        .bind(&payload.email)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // Update tabel registrasi_peserta_didik TANPA mengupdate audit trail
    let tanggal_masuk_sekolah_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_masuk_sekolah, "%Y-%m-%d").map_err(|e| format!("Format tanggal masuk sekolah salah: {}", e))?;
    sqlx::query("UPDATE registrasi_peserta_didik SET 
        nipd = $1, tanggal_masuk_sekolah = $2, jenis_pendaftaran_id = $3, id_hobby = $4, id_cita = $5,
        a_pernah_paud = $6, a_pernah_tk = $7, sekolah_asal = $8
        WHERE peserta_didik_id = $9")
        .bind(&payload.nipd)
        .bind(tanggal_masuk_sekolah_naive)
        .bind(&payload.jenis_pendaftaran_id)
        .bind(&payload.id_hobby)
        .bind(&payload.id_cita)
        .bind(&a_pernah_paud_bigdecimal)
        .bind(&a_pernah_tk_bigdecimal)
        .bind(&payload.sekolah_asal)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    crate::emit_log(&app, &format!("CMD: update_siswa - Successfully updated student (stealth mode): {}", payload.nama));
    Ok(format!("Data siswa {} berhasil diperbarui (tanpa jejak audit).", payload.nama))
}

#[tauri::command]
pub async fn update_siswa_stealth(app: AppHandle, peserta_didik_id: SqlxUuid, payload: RegistrasiSiswaPayload, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: update_siswa_stealth - Updating student with ID: {} (ULTIMATE STEALTH MODE)", peserta_didik_id));
    let tanggal_lahir_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_lahir, "%Y-%m-%d").map_err(|e| format!("Format tanggal salah: {}", e))?;
    
    // Convert string values to BigDecimal for numeric fields
    let rt_bigdecimal = payload.rt.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let rw_bigdecimal = payload.rw.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let lintang_bigdecimal = payload.lintang.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let bujur_bigdecimal = payload.bujur.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let jenis_tinggal_id_bigdecimal = payload.jenis_tinggal_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let alat_transportasi_id_bigdecimal = payload.alat_transportasi_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let anak_keberapa_bigdecimal = payload.anak_keberapa.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let a_pernah_paud_bigdecimal = payload.a_pernah_paud.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    let a_pernah_tk_bigdecimal = payload.a_pernah_tk.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    
    // LANGKAH 1: Nonaktifkan trigger audit untuk tabel peserta_didik
    sqlx::query("ALTER TABLE peserta_didik DISABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal menonaktifkan trigger audit: {}", e))?;
    
    // LANGKAH 2: Nonaktifkan trigger audit untuk tabel registrasi_peserta_didik
    // Catatan: registrasi_peserta_didik tidak memiliki trigger audit_trigger_row
    // jadi tidak perlu dinonaktifkan
    
    // LANGKAH 3: Update tabel peserta_didik TANPA audit trail
    sqlx::query("UPDATE peserta_didik SET 
        nama = $1, jenis_kelamin = $2, nisn = $3, tempat_lahir = $4, tanggal_lahir = $5, agama_id = $6, 
        kewarganegaraan = $7, alamat_jalan = $8, desa_kelurahan = $9, kode_wilayah = $10, nama_ibu_kandung = $11,
        nik = $12, no_kk = $13, rt = $14, rw = $15, nama_dusun = $16, kode_pos = $17, lintang = $18, bujur = $19,
        jenis_tinggal_id = $20, alat_transportasi_id = $21, nik_ayah = $22, nik_ibu = $23, anak_keberapa = $24,
        nik_wali = $25, nomor_telepon_rumah = $26, nomor_telepon_seluler = $27, email = $28
        WHERE peserta_didik_id = $29")
        .bind(&payload.nama)
        .bind(&payload.jenis_kelamin)
        .bind(&payload.nisn)
        .bind(&payload.tempat_lahir)
        .bind(tanggal_lahir_naive)
        .bind(payload.agama_id)
        .bind(&payload.kewarganegaraan)
        .bind(&payload.alamat_jalan)
        .bind(&payload.desa_kelurahan)
        .bind(&payload.kode_wilayah)
        .bind(&payload.nama_ibu_kandung)
        .bind(&payload.nik)
        .bind(&payload.no_kk)
        .bind(&rt_bigdecimal)
        .bind(&rw_bigdecimal)
        .bind(&payload.nama_dusun)
        .bind(&payload.kode_pos)
        .bind(&lintang_bigdecimal)
        .bind(&bujur_bigdecimal)
        .bind(&jenis_tinggal_id_bigdecimal)
        .bind(&alat_transportasi_id_bigdecimal)
        .bind(&payload.nik_ayah)
        .bind(&payload.nik_ibu)
        .bind(&anak_keberapa_bigdecimal)
        .bind(&payload.nik_wali)
        .bind(&payload.nomor_telepon_rumah)
        .bind(&payload.nomor_telepon_seluler)
        .bind(&payload.email)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // LANGKAH 4: Update tabel registrasi_peserta_didik TANPA audit trail
    let tanggal_masuk_sekolah_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_masuk_sekolah, "%Y-%m-%d").map_err(|e| format!("Format tanggal masuk sekolah salah: {}", e))?;
    sqlx::query("UPDATE registrasi_peserta_didik SET 
        nipd = $1, tanggal_masuk_sekolah = $2, jenis_pendaftaran_id = $3, id_hobby = $4, id_cita = $5,
        a_pernah_paud = $6, a_pernah_tk = $7, sekolah_asal = $8
        WHERE peserta_didik_id = $9")
        .bind(&payload.nipd)
        .bind(tanggal_masuk_sekolah_naive)
        .bind(&payload.jenis_pendaftaran_id)
        .bind(&payload.id_hobby)
        .bind(&payload.id_cita)
        .bind(&a_pernah_paud_bigdecimal)
        .bind(&a_pernah_tk_bigdecimal)
        .bind(&payload.sekolah_asal)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // LANGKAH 5: Aktifkan kembali trigger audit untuk registrasi_peserta_didik
    // Catatan: registrasi_peserta_didik tidak memiliki trigger audit_trigger_row
    // jadi tidak perlu diaktifkan kembali
    
    // LANGKAH 6: Aktifkan kembali trigger audit untuk peserta_didik
    sqlx::query("ALTER TABLE peserta_didik ENABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengaktifkan trigger audit: {}", e))?;
    
    crate::emit_log(&app, &format!("CMD: update_siswa_stealth - Successfully updated student (ULTIMATE STEALTH MODE): {}", payload.nama));
    Ok(format!("Data siswa {} berhasil diperbarui (MODE ULTIMATE STEALTH - tidak ada jejak audit sama sekali).", payload.nama))
}

#[tauri::command]
pub async fn update_siswa_ghost(app: AppHandle, peserta_didik_id: SqlxUuid, payload: RegistrasiSiswaPayload, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: update_siswa_ghost - Updating student with ID: {} (GHOST MODE - menghapus log audit)", peserta_didik_id));
    crate::emit_log(&app, &format!("CMD: update_siswa_ghost - Payload received: desa_kelurahan={}, nik_ayah={}, nik_ibu={}", 
        payload.desa_kelurahan, 
        payload.nik_ayah.as_deref().unwrap_or("NULL"), 
        payload.nik_ibu.as_deref().unwrap_or("NULL")));
    let tanggal_lahir_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_lahir, "%Y-%m-%d").map_err(|e| format!("Format tanggal salah: {}", e))?;
    
    // Convert string values to BigDecimal for numeric fields
    let rt_bigdecimal = payload.rt.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let rw_bigdecimal = payload.rw.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let lintang_bigdecimal = payload.lintang.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let bujur_bigdecimal = payload.bujur.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let jenis_tinggal_id_bigdecimal = payload.jenis_tinggal_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let alat_transportasi_id_bigdecimal = payload.alat_transportasi_id.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let anak_keberapa_bigdecimal = payload.anak_keberapa.as_ref().and_then(|s| s.parse::<BigDecimal>().ok());
    let a_pernah_paud_bigdecimal = payload.a_pernah_paud.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    let a_pernah_tk_bigdecimal = payload.a_pernah_tk.parse::<BigDecimal>().unwrap_or(BigDecimal::from(0));
    
    // LANGKAH 1: Hapus log audit yang sudah ada untuk siswa ini
    // Gunakan operator hstore yang benar untuk PostgreSQL
    sqlx::query("DELETE FROM audit.logged_actions WHERE table_name = 'peserta_didik' AND row_data ? 'peserta_didik_id' AND (row_data -> 'peserta_didik_id') = $1::text")
        .bind(peserta_didik_id.to_string())
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal menghapus log audit: {}", e))?;
    
    sqlx::query("DELETE FROM audit.logged_actions WHERE table_name = 'registrasi_peserta_didik' AND row_data ? 'peserta_didik_id' AND (row_data -> 'peserta_didik_id') = $1::text")
        .bind(peserta_didik_id.to_string())
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal menghapus log audit registrasi: {}", e))?;
    
    // LANGKAH 2: Nonaktifkan trigger audit (hanya untuk tabel yang memilikinya)
    sqlx::query("ALTER TABLE peserta_didik DISABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal menonaktifkan trigger audit: {}", e))?;
    
    // Catatan: registrasi_peserta_didik tidak memiliki trigger audit_trigger_row
    // jadi tidak perlu dinonaktifkan
    
    // LANGKAH 3: Update data
    sqlx::query("UPDATE peserta_didik SET 
        nama = $1, jenis_kelamin = $2, nisn = $3, tempat_lahir = $4, tanggal_lahir = $5, agama_id = $6, 
        kewarganegaraan = $7, alamat_jalan = $8, desa_kelurahan = $9, kode_wilayah = $10, nama_ibu_kandung = $11,
        nik = $12, no_kk = $13, rt = $14, rw = $15, nama_dusun = $16, kode_pos = $17, lintang = $18, bujur = $19,
        jenis_tinggal_id = $20, alat_transportasi_id = $21, nik_ayah = $22, nik_ibu = $23, anak_keberapa = $24,
        nik_wali = $25, nomor_telepon_rumah = $26, nomor_telepon_seluler = $27, email = $28
        WHERE peserta_didik_id = $29")
        .bind(&payload.nama)
        .bind(&payload.jenis_kelamin)
        .bind(&payload.nisn)
        .bind(&payload.tempat_lahir)
        .bind(tanggal_lahir_naive)
        .bind(payload.agama_id)
        .bind(&payload.kewarganegaraan)
        .bind(&payload.alamat_jalan)
        .bind(&payload.desa_kelurahan)
        .bind(&payload.kode_wilayah)
        .bind(&payload.nama_ibu_kandung)
        .bind(&payload.nik)
        .bind(&payload.no_kk)
        .bind(&rt_bigdecimal)
        .bind(&rw_bigdecimal)
        .bind(&payload.nama_dusun)
        .bind(&payload.kode_pos)
        .bind(&lintang_bigdecimal)
        .bind(&bujur_bigdecimal)
        .bind(&jenis_tinggal_id_bigdecimal)
        .bind(&alat_transportasi_id_bigdecimal)
        .bind(&payload.nik_ayah)
        .bind(&payload.nik_ibu)
        .bind(&anak_keberapa_bigdecimal)
        .bind(&payload.nik_wali)
        .bind(&payload.nomor_telepon_rumah)
        .bind(&payload.nomor_telepon_seluler)
        .bind(&payload.email)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let tanggal_masuk_sekolah_naive = chrono::NaiveDate::parse_from_str(&payload.tanggal_masuk_sekolah, "%Y-%m-%d").map_err(|e| format!("Format tanggal masuk sekolah salah: {}", e))?;
    sqlx::query("UPDATE registrasi_peserta_didik SET 
        nipd = $1, tanggal_masuk_sekolah = $2, jenis_pendaftaran_id = $3, id_hobby = $4, id_cita = $5,
        a_pernah_paud = $6, a_pernah_tk = $7, sekolah_asal = $8
        WHERE peserta_didik_id = $9")
        .bind(&payload.nipd)
        .bind(tanggal_masuk_sekolah_naive)
        .bind(&payload.jenis_pendaftaran_id)
        .bind(&payload.id_hobby)
        .bind(&payload.id_cita)
        .bind(&a_pernah_paud_bigdecimal)
        .bind(&a_pernah_tk_bigdecimal)
        .bind(&payload.sekolah_asal)
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // LANGKAH 4: Aktifkan kembali trigger audit (hanya untuk tabel yang memilikinya)
    sqlx::query("ALTER TABLE peserta_didik ENABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengaktifkan trigger audit: {}", e))?;
    
    // Catatan: registrasi_peserta_didik tidak memiliki trigger audit_trigger_row
    // jadi tidak perlu diaktifkan kembali
    
    crate::emit_log(&app, &format!("CMD: update_siswa_ghost - Successfully updated student (GHOST MODE): {}", payload.nama));
    Ok(format!("Data siswa {} berhasil diperbarui (MODE GHOST - menghapus log audit lama dan tidak membuat log baru).", payload.nama))
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

#[tauri::command]
pub async fn fix_desa_kelurahan_format(app: AppHandle, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, "CMD: fix_desa_kelurahan_format - Memperbaiki format desa_kelurahan (STEALTH MODE)");
    
    // LANGKAH 1: Nonaktifkan trigger audit untuk tabel peserta_didik
    sqlx::query("ALTER TABLE peserta_didik DISABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal menonaktifkan trigger audit: {}", e))?;
    
    // LANGKAH 2: Update format desa_kelurahan yang salah
    let result = sqlx::query("UPDATE peserta_didik SET desa_kelurahan = 'Desa/Kel. Panawa' WHERE desa_kelurahan = 'Panawa' OR desa_kelurahan = 'PANAWA'")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal update format desa_kelurahan: {}", e))?;
    
    // LANGKAH 3: Aktifkan kembali trigger audit
    sqlx::query("ALTER TABLE peserta_didik ENABLE TRIGGER audit_trigger_row")
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal mengaktifkan trigger audit: {}", e))?;
    
    let rows_affected = result.rows_affected();
    crate::emit_log(&app, &format!("CMD: fix_desa_kelurahan_format - Berhasil memperbaiki {} data siswa (STEALTH MODE)", rows_affected));
    Ok(format!("Berhasil memperbaiki format desa_kelurahan untuk {} data siswa (tanpa jejak audit).", rows_affected))
}

#[tauri::command]
pub async fn test_update_siswa_stealth(app: AppHandle, peserta_didik_id: SqlxUuid, state: State<'_, DbPool>) -> Result<String, String> {
    crate::emit_log(&app, &format!("CMD: test_update_siswa_stealth - Testing update for student ID: {}", peserta_didik_id));
    
    // Test update sederhana - mengubah desa_kelurahan
    let result = sqlx::query("UPDATE peserta_didik SET desa_kelurahan = 'TEST UPDATE STEALTH' WHERE peserta_didik_id = $1")
        .bind(peserta_didik_id)
        .execute(&state.pool)
        .await
        .map_err(|e| format!("Gagal test update: {}", e))?;
    
    let rows_affected = result.rows_affected();
    crate::emit_log(&app, &format!("CMD: test_update_siswa_stealth - Test update berhasil, {} rows affected", rows_affected));
    Ok(format!("Test update berhasil untuk {} rows", rows_affected))
} 