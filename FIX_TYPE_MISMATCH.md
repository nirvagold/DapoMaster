# Perbaikan Masalah Tipe Data (Type Mismatch)

## Ringkasan Masalah

Aplikasi mengalami error tipe data yang tidak kompatibel antara Rust dan database PostgreSQL. Masalah utama terjadi pada kolom `peserta_didik_id` dan kolom tahun lahir yang memiliki tipe data berbeda di database dan kode Rust.

## Error yang Ditemukan

### 1. Error UUID vs String
```
Error: Error getting peserta_didik_id: error occurred while decoding column "peserta_didik_id": mismatched types; Rust type `alloc::string::String` (as SQL type `TEXT`) is not compatible with SQL type `UUID`
```

**Lokasi:** Semua fungsi validasi di `src-tauri/src/commands/validasi.rs`

**Penyebab:** Kolom `peserta_didik_id` di database bertipe `UUID` tetapi kode Rust mencoba membaca sebagai `String`.

### 2. Error NUMERIC vs String
```
Error: Error getting tahun_lahir_ayah: error occurred while decoding column "tahun_lahir_ayah": mismatched types; Rust type `core::option::Option<alloc::string::String>` (as SQL type `TEXT`) is not compatible with SQL type `NUMERIC`
```

**Lokasi:** Fungsi `fix_tahun_lahir_ayah` dan `fix_tahun_lahir_wali`

**Penyebab:** Kolom `tahun_lahir_ayah` dan `tahun_lahir_wali` di database bertipe `NUMERIC` tetapi kode Rust mencoba membaca sebagai `String`.

## Perbaikan yang Dilakukan

### 1. Menambahkan Import SqlxUuid
```rust
use sqlx::types::Uuid as SqlxUuid;
```

### 2. Mengubah Tipe Data peserta_didik_id
**Sebelum:**
```rust
let peserta_didik_id: String = row.try_get("peserta_didik_id")
    .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
```

**Sesudah:**
```rust
let peserta_didik_id: SqlxUuid = row.try_get("peserta_didik_id")
    .map_err(|e| format!("Error getting peserta_didik_id: {}", e))?;
```

### 3. Mengubah Binding Parameter
**Sebelum:**
```rust
.bind(&peserta_didik_id)
```

**Sesudah:**
```rust
.bind(peserta_didik_id)
```

### 4. Mengubah Tipe Data tahun_lahir
**Sebelum:**
```rust
let old_value: Option<String> = row.try_get("tahun_lahir_ayah")
    .map_err(|e| format!("Error getting tahun_lahir_ayah: {}", e))?;
```

**Sesudah:**
```rust
let old_value: Option<String> = row.try_get::<Option<BigDecimal>, _>("tahun_lahir_ayah")
    .map_err(|e| format!("Error getting tahun_lahir_ayah: {}", e))?
    .map(|v| v.to_string());
```

### 5. Mengubah Parameter create_validasi_detail
**Sebelum:**
```rust
details.push(create_validasi_detail(
    peserta_didik_id,
    nama,
    // ...
));
```

**Sesudah:**
```rust
details.push(create_validasi_detail(
    peserta_didik_id.to_string(),
    nama,
    // ...
));
```

## Fungsi yang Diperbaiki

1. `fix_nik_ayah`
2. `fix_hobby`
3. `fix_cita_cita`
4. `fix_tahun_lahir_ayah`
5. `fix_nik_wali`
6. `fix_tahun_lahir_wali`
7. `fix_kps_pkh`

## Verifikasi

Build aplikasi berhasil tanpa error:
```bash
cargo build
# Output: Finished `dev` profile [unoptimized + debuginfo] target(s) in 48.11s
```

## Kesimpulan

Semua masalah tipe data telah berhasil diperbaiki. Aplikasi sekarang dapat membaca data dari database dengan tipe data yang benar sesuai dengan skema database PostgreSQL.

## Catatan Penting

- Pastikan selalu menggunakan tipe data yang sesuai antara Rust dan database
- Untuk UUID, gunakan `SqlxUuid` dari `sqlx::types::Uuid`
- Untuk NUMERIC, gunakan `BigDecimal` dan konversi ke string jika diperlukan
- Selalu test build setelah melakukan perubahan tipe data 