use crate::app_state::DbPool;
use tauri::{AppHandle, State};
use serde::Serialize;
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::types::Uuid as SqlxUuid;
use bigdecimal::BigDecimal;
use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder};
use calamine::{Reader, open_workbook_auto, DataType};
use std::collections::HashMap;

#[tauri::command]
pub async fn open_import_dialog() -> Result<Option<String>, String> {
    use rfd::FileDialog;
    
    let file_path = FileDialog::new()
        .add_filter("Excel Files", &["xlsx", "xls"])
        .set_title("Pilih File Excel untuk Import")
        .pick_file();
    
    Ok(file_path.map(|path| path.to_string_lossy().to_string()))
}

#[derive(Serialize)]
pub struct ExportResult {
    pub success: bool,
    pub message: String,
    pub file_path: Option<String>,
}

#[derive(Serialize)]
pub struct ImportResult {
    pub success: bool,
    pub message: String,
    pub total_rows: i32,
    pub success_count: i32,
    pub error_count: i32,
    pub errors: Vec<ImportError>,
}

#[derive(Serialize)]
pub struct ImportError {
    pub row: i32,
    pub field: String,
    pub message: String,
}

#[tauri::command]
pub async fn export_lulusan_to_excel(
    app: AppHandle,
    state: State<'_, DbPool>,
) -> Result<ExportResult, String> {
    crate::emit_log(&app, "CMD: export_lulusan_to_excel - Starting export process.");

    // Ambil semua data lulusan (tanpa kolom yang dihapus)
    let query = "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.tanggal_lahir,
                        pd.nama_ayah, pd.nama_ibu_kandung, ip.jenis_ijazah_id,
                        ji.nama as nama_ijazah, ip.nomor, ip.penandatangan,
                        ip.tanggal_ttd
                 FROM peserta_didik pd 
                 JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
                 LEFT JOIN ijazah_pd ip ON rpd.registrasi_id = ip.registrasi_id
                 LEFT JOIN ref.jenis_ijazah ji ON ip.jenis_ijazah_id = ji.jenis_ijazah_id
                 WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1'
                 ORDER BY pd.nama";

    let rows = sqlx::query_as::<_, (SqlxUuid, String, String, NaiveDate, Option<String>, String, Option<BigDecimal>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>)>(query)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Buat nama file dengan timestamp
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    let filename = format!("template_lulusan_{}.xlsx", timestamp);
    
    // Tentukan path untuk menyimpan file
    let desktop_path = dirs::desktop_dir()
        .ok_or("Tidak dapat menemukan folder Desktop")?
        .join(&filename);
    
    let file_path = desktop_path.to_string_lossy().to_string();

    // Buat workbook Excel
    let mut workbook = Workbook::new();
    
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Template Lulusan")
        .map_err(|e| format!("Gagal mengatur nama worksheet: {}", e))?;

    // Buat format untuk header
    let header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_background_color(rust_xlsxwriter::Color::Blue)
        .set_font_color(rust_xlsxwriter::Color::White);

    // Buat format untuk data
    let data_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left);

    // Buat format untuk tanggal
    let date_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_num_format("dd/mm/yyyy");

    // Header (tanpa kolom yang dihapus)
    let headers = [
        "No", "Nama", "NISN", "Tanggal Lahir", "Nama Ayah", "Nama Ibu", 
        "Jenis Ijazah", "Nomor Ijazah", "Penandatangan", "Tanggal Tanda Tangan"
    ];

    // Tulis header
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, &header_format)
            .map_err(|e| format!("Gagal menulis header: {}", e))?;
    }

    // Tulis data
    for (row_idx, row) in rows.iter().enumerate() {
        let row_num = (row_idx + 1) as u32;
        
        // No
        worksheet.write_number_with_format(row_num, 0, (row_idx + 1) as f64, &data_format)
            .map_err(|e| format!("Gagal menulis nomor: {}", e))?;
        
        // Nama
        worksheet.write_string_with_format(row_num, 1, &row.1, &data_format)
            .map_err(|e| format!("Gagal menulis nama: {}", e))?;
        
        // NISN
        worksheet.write_string_with_format(row_num, 2, &row.2, &data_format)
            .map_err(|e| format!("Gagal menulis NISN: {}", e))?;
        
        // Tanggal Lahir
        worksheet.write_string_with_format(row_num, 3, &row.3.format("%Y-%m-%d").to_string(), &date_format)
            .map_err(|e| format!("Gagal menulis tanggal lahir: {}", e))?;
        
        // Nama Ayah
        let nama_ayah = row.4.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 4, nama_ayah, &data_format)
            .map_err(|e| format!("Gagal menulis nama ayah: {}", e))?;
        
        // Nama Ibu
        worksheet.write_string_with_format(row_num, 5, &row.5, &data_format)
            .map_err(|e| format!("Gagal menulis nama ibu: {}", e))?;
        
        // Jenis Ijazah
        let jenis_ijazah = row.7.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 6, jenis_ijazah, &data_format)
            .map_err(|e| format!("Gagal menulis jenis ijazah: {}", e))?;
        
        // Nomor Ijazah
        let nomor_ijazah = row.8.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 7, nomor_ijazah, &data_format)
            .map_err(|e| format!("Gagal menulis nomor ijazah: {}", e))?;
        
        // Penandatangan
        let penandatangan = row.9.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 8, penandatangan, &data_format)
            .map_err(|e| format!("Gagal menulis penandatangan: {}", e))?;
        
        // Tanggal Tanda Tangan
        if let Some(tanggal_tanda_tangan) = row.10 {
            worksheet.write_string_with_format(row_num, 9, &tanggal_tanda_tangan.format("%Y-%m-%d").to_string(), &date_format)
                .map_err(|e| format!("Gagal menulis tanggal tanda tangan: {}", e))?;
        } else {
            worksheet.write_string_with_format(row_num, 9, "-", &data_format)
                .map_err(|e| format!("Gagal menulis tanggal tanda tangan: {}", e))?;
        }
    }

    // Set lebar kolom otomatis
    worksheet.autofit();

    // Simpan workbook
    workbook.save(&file_path)
        .map_err(|e| format!("Gagal menyimpan workbook: {}", e))?;

    crate::emit_log(&app, &format!("CMD: export_lulusan_to_excel - Successfully exported {} records to {}", rows.len(), file_path));

    Ok(ExportResult {
        success: true,
        message: format!("Berhasil mengekspor {} data lulusan ke file Excel: {}", rows.len(), filename),
        file_path: Some(file_path),
    })
}

#[tauri::command]
pub async fn import_lulusan_from_excel(
    app: AppHandle,
    state: State<'_, DbPool>,
    file_path: String,
) -> Result<ImportResult, String> {
    crate::emit_log(&app, "CMD: import_lulusan_from_excel - Starting import process.");

    let mut workbook = open_workbook_auto(&file_path)
        .map_err(|e| format!("Gagal membuka file Excel: {}", e))?;

    let range = workbook.worksheet_range_at(0)
        .ok_or("Tidak dapat menemukan worksheet")?
        .map_err(|e| format!("Gagal membaca worksheet: {}", e))?;

    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    let mut total_rows = 0;

    // Ambil semua jenis ijazah untuk validasi
    let jenis_ijazah_map = get_jenis_ijazah_map(&state).await?;

    // Mulai dari baris 2 (setelah header)
    for (row_idx, row) in range.rows().skip(1).enumerate() {
        total_rows += 1;
        let excel_row = row_idx + 2; // +2 karena skip header dan index dimulai dari 0

        if row.len() < 6 {
            errors.push(ImportError {
                row: excel_row as i32,
                field: "Semua".to_string(),
                message: "Jumlah kolom tidak mencukupi".to_string(),
            });
            error_count += 1;
            continue;
        }

        // Parse data dari Excel
        let nama = get_string_value(&row[1], "Nama", excel_row, &mut errors);
        let nisn = get_string_value(&row[2], "NISN", excel_row, &mut errors);
        let nama_ibu = get_string_value(&row[5], "Nama Ibu", excel_row, &mut errors);
        
        let jenis_ijazah = get_optional_string_value(&row[6], "Jenis Ijazah", excel_row, &mut errors);
        let nomor_ijazah = get_optional_string_value(&row[7], "Nomor Ijazah", excel_row, &mut errors);
        let penandatangan = get_optional_string_value(&row[8], "Penandatangan", excel_row, &mut errors);
        let tanggal_tanda_tangan = get_optional_date_value(&row[9], "Tanggal Tanda Tangan", excel_row, &mut errors);

        // Jika ada error pada data wajib, skip baris ini
        if nama.is_none() || nisn.is_none() || nama_ibu.is_none() {
            error_count += 1;
            continue;
        }

        let nama = nama.unwrap();
        let nisn = nisn.unwrap();
        let nama_ibu = nama_ibu.unwrap();

        // Validasi data dengan database
        match validate_and_update_data(&state, &nama, &nisn, &nama_ibu, &jenis_ijazah, &nomor_ijazah, &penandatangan, &tanggal_tanda_tangan, &jenis_ijazah_map, excel_row, &mut errors).await {
            Ok(_) => {
                success_count += 1;
                crate::emit_log(&app, &format!("CMD: import_lulusan_from_excel - Successfully processed row {}: {} ({})", excel_row, nama, nisn));
            }
            Err(e) => {
                error_count += 1;
                errors.push(ImportError {
                    row: excel_row as i32,
                    field: "Database".to_string(),
                    message: e,
                });
            }
        }
    }

    let message = if error_count == 0 {
        format!("Berhasil mengimport semua {} data lulusan", success_count)
    } else {
        format!("Berhasil mengimport {} data, {} error ditemukan", success_count, error_count)
    };

    crate::emit_log(&app, &format!("CMD: import_lulusan_from_excel - Import completed. Success: {}, Errors: {}", success_count, error_count));

    Ok(ImportResult {
        success: error_count == 0,
        message,
        total_rows,
        success_count,
        error_count,
        errors,
    })
}

#[tauri::command]
pub async fn export_siswa_keluar_to_excel(
    app: AppHandle,
    state: State<'_, DbPool>,
) -> Result<ExportResult, String> {
    crate::emit_log(&app, "CMD: export_siswa_keluar_to_excel - Starting export process.");

    // Ambil semua data siswa keluar
    let query = "SELECT pd.peserta_didik_id, pd.nama, pd.nisn, pd.nik, pd.tanggal_lahir,
                        pd.nama_ayah, pd.nama_ibu_kandung, rpd.jenis_keluar_id,
                        jk.nama as ket_keluar, rpd.tanggal_keluar
                 FROM peserta_didik pd 
                 JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
                 LEFT JOIN ref.jenis_keluar jk ON rpd.jenis_keluar_id = jk.jenis_keluar_id
                 WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id IS NOT NULL AND rpd.jenis_keluar_id != '1'
                 ORDER BY pd.nama";

    let rows = sqlx::query_as::<_, (SqlxUuid, String, String, Option<String>, NaiveDate, Option<String>, String, Option<String>, Option<NaiveDate>)>(query)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Buat nama file dengan timestamp
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    let filename = format!("data_siswa_keluar_{}.xlsx", timestamp);
    
    // Tentukan path untuk menyimpan file
    let desktop_path = dirs::desktop_dir()
        .ok_or("Tidak dapat menemukan folder Desktop")?
        .join(&filename);
    
    let file_path = desktop_path.to_string_lossy().to_string();

    // Buat workbook Excel
    let mut workbook = Workbook::new();
    
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Data Siswa Keluar")
        .map_err(|e| format!("Gagal mengatur nama worksheet: {}", e))?;

    // Buat format untuk header
    let header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_background_color(rust_xlsxwriter::Color::Red)
        .set_font_color(rust_xlsxwriter::Color::White);

    // Buat format untuk data
    let data_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left);

    // Buat format untuk tanggal
    let date_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_num_format("dd/mm/yyyy");

    // Header
    let headers = [
        "No", "Nama", "NISN", "NIK", "Tanggal Lahir", "Nama Ayah", "Nama Ibu", 
        "Alasan Keluar", "Tanggal Keluar"
    ];

    // Tulis header
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, &header_format)
            .map_err(|e| format!("Gagal menulis header: {}", e))?;
    }

    // Tulis data
    for (row_idx, row) in rows.iter().enumerate() {
        let row_num = (row_idx + 1) as u32;
        
        // No
        worksheet.write_number_with_format(row_num, 0, (row_idx + 1) as f64, &data_format)
            .map_err(|e| format!("Gagal menulis nomor: {}", e))?;
        
        // Nama
        worksheet.write_string_with_format(row_num, 1, &row.1, &data_format)
            .map_err(|e| format!("Gagal menulis nama: {}", e))?;
        
        // NISN
        worksheet.write_string_with_format(row_num, 2, &row.2, &data_format)
            .map_err(|e| format!("Gagal menulis NISN: {}", e))?;
        
        // NIK
        let nik = row.3.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 3, nik, &data_format)
            .map_err(|e| format!("Gagal menulis NIK: {}", e))?;
        
        // Tanggal Lahir
        worksheet.write_string_with_format(row_num, 4, &row.4.format("%Y-%m-%d").to_string(), &date_format)
            .map_err(|e| format!("Gagal menulis tanggal lahir: {}", e))?;
        
        // Nama Ayah
        let nama_ayah = row.5.as_deref().unwrap_or("-");
        worksheet.write_string_with_format(row_num, 5, nama_ayah, &data_format)
            .map_err(|e| format!("Gagal menulis nama ayah: {}", e))?;
        
        // Nama Ibu
        worksheet.write_string_with_format(row_num, 6, &row.6, &data_format)
            .map_err(|e| format!("Gagal menulis nama ibu: {}", e))?;
        
        // Alasan Keluar
        let ket_keluar = row.7.as_deref().unwrap_or("Tidak diketahui");
        worksheet.write_string_with_format(row_num, 7, ket_keluar, &data_format)
            .map_err(|e| format!("Gagal menulis alasan keluar: {}", e))?;
        
        // Tanggal Keluar
        if let Some(tanggal_keluar) = row.8 {
            worksheet.write_string_with_format(row_num, 8, &tanggal_keluar.format("%Y-%m-%d").to_string(), &date_format)
                .map_err(|e| format!("Gagal menulis tanggal keluar: {}", e))?;
        } else {
            worksheet.write_string_with_format(row_num, 8, "-", &data_format)
                .map_err(|e| format!("Gagal menulis tanggal keluar: {}", e))?;
        }
    }

    // Set lebar kolom otomatis
    worksheet.autofit();

    // Simpan workbook
    workbook.save(&file_path)
        .map_err(|e| format!("Gagal menyimpan workbook: {}", e))?;

    crate::emit_log(&app, &format!("CMD: export_siswa_keluar_to_excel - Successfully exported {} records to {}", rows.len(), file_path));

    Ok(ExportResult {
        success: true,
        message: format!("Berhasil mengekspor {} data siswa keluar ke file Excel: {}", rows.len(), filename),
        file_path: Some(file_path),
    })
}

// Helper functions
async fn get_jenis_ijazah_map(state: &State<'_, DbPool>) -> Result<HashMap<String, BigDecimal>, String> {
    let rows = sqlx::query_as::<_, (BigDecimal, String)>(
        "SELECT jenis_ijazah_id, nama FROM ref.jenis_ijazah ORDER BY nama"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for (id, nama) in rows {
        map.insert(nama.to_lowercase(), id);
    }
    Ok(map)
}

fn get_string_value(cell: &DataType, field_name: &str, row: usize, errors: &mut Vec<ImportError>) -> Option<String> {
    match cell {
        DataType::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        DataType::Int(i) => Some(i.to_string()),
        DataType::Float(f) => Some(f.to_string()),
        _ => {
            errors.push(ImportError {
                row: row as i32,
                field: field_name.to_string(),
                message: "Data wajib tidak boleh kosong".to_string(),
            });
            None
        }
    }
}

fn get_optional_string_value(cell: &DataType, _field_name: &str, _row: usize, _errors: &mut Vec<ImportError>) -> Option<String> {
    match cell {
        DataType::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        DataType::Int(i) => Some(i.to_string()),
        DataType::Float(f) => Some(f.to_string()),
        _ => None
    }
}

fn get_optional_date_value(cell: &DataType, field_name: &str, row: usize, errors: &mut Vec<ImportError>) -> Option<String> {
    match cell {
        DataType::String(s) if !s.trim().is_empty() => {
            // Coba parse berbagai format tanggal
            let date_str = s.trim();
            if let Ok(_) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                Some(date_str.to_string())
            } else if let Ok(_) = NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
                Some(date_str.to_string())
            } else if let Ok(_) = NaiveDate::parse_from_str(date_str, "%d-%m-%Y") {
                Some(date_str.to_string())
            } else {
                errors.push(ImportError {
                    row: row as i32,
                    field: field_name.to_string(),
                    message: "Format tanggal tidak valid (gunakan YYYY-MM-DD, DD/MM/YYYY, atau DD-MM-YYYY)".to_string(),
                });
                None
            }
        }
        DataType::Float(f) => {
            // Excel menyimpan tanggal sebagai serial number (hari sejak 1 Januari 1900)
            // Tapi ada bug di Excel: tahun 1900 dianggap kabisat padahal bukan
            // Jadi kita perlu menyesuaikan dengan offset 2 hari
            let excel_epoch = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
            let days = f.floor() as i64;
            
            // Debug log untuk melihat nilai Excel
            println!("DEBUG: Excel Float value: {}, days: {}", f, days);
            
            // Perbaikan untuk bug Excel 1900 leap year
            let adjusted_days = if days > 59 { days - 1 } else { days };
            
            if let Some(date) = excel_epoch.checked_add_days(chrono::Days::new(adjusted_days as u64)) {
                let result = date.format("%Y-%m-%d").to_string();
                println!("DEBUG: Converted Float date: {} -> {} (adjusted_days: {})", f, result, adjusted_days);
                Some(result)
            } else {
                errors.push(ImportError {
                    row: row as i32,
                    field: field_name.to_string(),
                    message: "Nilai tanggal Excel tidak valid".to_string(),
                });
                None
            }
        }
        DataType::DateTime(dt) => {
            // Untuk DateTime dari Excel, kita konversi ke string
            // dt adalah f64 yang merepresentasikan tanggal Excel
            // Nilai 45839 adalah Excel serial number untuk tanggal 1 Juli 2025
            // Excel menggunakan epoch 1 Januari 1900 = 1, tapi ada bug leap year
            // Untuk tanggal setelah 28 Februari 1900, Excel menambahkan 1 hari ekstra
            let excel_epoch = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
            let days = dt.floor() as i64;
            
            // Debug log untuk melihat nilai Excel
            println!("DEBUG: Excel DateTime value: {}, days: {}", dt, days);
            
            // Perbaikan untuk bug Excel 1900 leap year
            // Excel menganggap 1900 sebagai tahun kabisat, padahal bukan
            // Jadi untuk tanggal setelah 28 Februari 1900, kita kurangi 1 hari
            // Tapi untuk tanggal 1 Juli 2025 (45839), kita perlu kurangi 2 hari
            let adjusted_days = if days > 60 { days - 2 } else { days };
            
            if let Some(date) = excel_epoch.checked_add_days(chrono::Days::new(adjusted_days as u64)) {
                let result = date.format("%Y-%m-%d").to_string();
                println!("DEBUG: Converted as Excel serial: {} -> {} (adjusted_days: {})", dt, result, adjusted_days);
                Some(result)
            } else {
                errors.push(ImportError {
                    row: row as i32,
                    field: field_name.to_string(),
                    message: "Nilai tanggal Excel tidak valid".to_string(),
                });
                None
            }
        }
        _ => None
    }
}

async fn validate_and_update_data(
    state: &State<'_, DbPool>,
    nama: &str,
    nisn: &str,
    nama_ibu: &str,
    jenis_ijazah: &Option<String>,
    nomor_ijazah: &Option<String>,
    penandatangan: &Option<String>,
    tanggal_tanda_tangan: &Option<String>,
    jenis_ijazah_map: &HashMap<String, BigDecimal>,
    row: usize,
    errors: &mut Vec<ImportError>,
) -> Result<(), String> {
    // Cari siswa berdasarkan nama, NISN, dan nama ibu
    let siswa = sqlx::query_as::<_, (SqlxUuid, String)>(
        "SELECT pd.peserta_didik_id, pd.nama 
         FROM peserta_didik pd 
         JOIN registrasi_peserta_didik rpd ON pd.peserta_didik_id = rpd.peserta_didik_id 
         WHERE pd.soft_delete = 0 AND rpd.jenis_keluar_id = '1' 
         AND LOWER(pd.nama) = LOWER($1) 
         AND pd.nisn = $2 
         AND LOWER(pd.nama_ibu_kandung) = LOWER($3)"
    )
    .bind(nama)
    .bind(nisn)
    .bind(nama_ibu)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let (peserta_didik_id, nama_db) = match siswa {
        Some(s) => s,
        None => {
            errors.push(ImportError {
                row: row as i32,
                field: "Data Siswa".to_string(),
                message: format!("Siswa tidak ditemukan: {} (NISN: {}, Ibu: {})", nama, nisn, nama_ibu),
            });
            return Err("Siswa tidak ditemukan".to_string());
        }
    };

    // Validasi nama jika berbeda (case insensitive)
    if nama.to_lowercase() != nama_db.to_lowercase() {
        errors.push(ImportError {
            row: row as i32,
            field: "Nama".to_string(),
            message: format!("Nama tidak cocok dengan database: '{}' vs '{}'", nama, nama_db),
        });
    }

    // Validasi jenis ijazah jika ada
    let jenis_ijazah_id = if let Some(ref jenis) = jenis_ijazah {
        match jenis_ijazah_map.get(&jenis.to_lowercase()) {
            Some(id) => Some(id.clone()),
            None => {
                errors.push(ImportError {
                    row: row as i32,
                    field: "Jenis Ijazah".to_string(),
                    message: format!("Jenis ijazah '{}' tidak ditemukan dalam database", jenis),
                });
                None
            }
        }
    } else {
        None
    };

    // Parse tanggal tanda tangan
    let tanggal_ttd = if let Some(ref tgl_str) = tanggal_tanda_tangan {
        match NaiveDate::parse_from_str(tgl_str, "%Y-%m-%d") {
            Ok(tgl) => Some(tgl),
            Err(_) => {
                // Coba format lain
                match NaiveDate::parse_from_str(tgl_str, "%d/%m/%Y") {
                    Ok(tgl) => Some(tgl),
                    Err(_) => {
                        match NaiveDate::parse_from_str(tgl_str, "%d-%m-%Y") {
                            Ok(tgl) => Some(tgl),
                            Err(_) => {
                                errors.push(ImportError {
                                    row: row as i32,
                                    field: "Tanggal Tanda Tangan".to_string(),
                                    message: format!("Format tanggal tidak valid: {}", tgl_str),
                                });
                                None
                            }
                        }
                    }
                }
            }
        }
    } else {
        None
    };

    // Update data ijazah
    let result = sqlx::query(
        "UPDATE ijazah_pd 
         SET jenis_ijazah_id = $1, nomor = $2, penandatangan = $3, tanggal_ttd = $4, last_update = NOW()
         WHERE registrasi_id = (SELECT registrasi_id FROM registrasi_peserta_didik WHERE peserta_didik_id = $5)"
    )
    .bind(&jenis_ijazah_id)
    .bind(nomor_ijazah)
    .bind(penandatangan)
    .bind(tanggal_ttd)
    .bind(peserta_didik_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Gagal mengupdate data: {}", e))
    }
}