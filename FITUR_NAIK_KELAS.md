# Fitur Naik Kelas - DapoMaster

## Deskripsi
Fitur Naik Kelas memungkinkan pengguna untuk melihat daftar siswa yang akan naik kelas dari semester sebelumnya ke semester baru. Fitur ini membantu dalam perencanaan dan persiapan proses kenaikan kelas siswa.

## Fitur Utama

### 1. Tampilan Data Siswa Naik Kelas
- Menampilkan daftar siswa dari semester sebelumnya yang akan naik kelas
- Informasi yang ditampilkan:
  - Nama siswa
  - NISN
  - Rombel sebelumnya
  - Rombel baru (dihitung otomatis)
  - Tingkat pendidikan sebelumnya
  - Tingkat pendidikan baru

### 2. Filter Semester
- Pemilihan semester sebelumnya untuk mengambil data siswa
- Hanya menampilkan semester yang masih aktif (belum expired)

### 3. Perhitungan Otomatis
- Rombel baru dihitung otomatis berdasarkan tingkat pendidikan
- Mapping tingkat pendidikan:
  - Kelas 1 → Kelas 2
  - Kelas 2 → Kelas 3
  - Kelas 3 → Kelas 4
  - Kelas 4 → Kelas 5
  - Kelas 5 → Kelas 6

## Struktur Database

### Tabel yang Digunakan
1. **anggota_rombel** - Data keanggotaan siswa dalam rombel
2. **rombongan_belajar** - Data rombel/kelas
3. **peserta_didik** - Data siswa
4. **ref.tingkat_pendidikan** - Referensi tingkat pendidikan
5. **ref.semester** - Referensi semester

### Query Utama
```sql
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
```

## Implementasi Teknis

### Backend (Rust)
- **File**: `src-tauri/src/commands/naik_kelas.rs`
- **Commands**:
  - `get_siswa_naik_kelas` - Mengambil data siswa untuk naik kelas
  - `get_daftar_semester` - Mengambil daftar semester yang tersedia

### Frontend (React/TypeScript)
- **File**: `src/components/NaikKelasView.tsx`
- **Fitur**:
  - Dropdown pemilihan semester
  - Tabel data siswa dengan perhitungan otomatis
  - Informasi dan panduan penggunaan

### Struktur Data
```typescript
interface Semester {
  semester_id: string;
  nama: string;
  tahun_ajaran_id: string;
  semester: string;
}

interface SiswaRombel {
  peserta_didik_id: string;
  nama: string;
  nisn: string;
  nama_rombel: string;
  tingkat_pendidikan_id: number;
  tingkat_pendidikan_nama: string;
}
```

## Cara Penggunaan

1. **Akses Menu**: Klik menu "Naik Kelas" di sidebar
2. **Pilih Semester**: Pilih semester sebelumnya dari dropdown
3. **Muat Data**: Klik tombol "Muat Data Siswa"
4. **Review Data**: Periksa data siswa yang akan naik kelas
5. **Referensi**: Gunakan data ini sebagai referensi untuk proses naik kelas manual

## Batasan dan Catatan

### Batasan
- Hanya menampilkan siswa dari kelas 1-5 (SD)
- Tidak melakukan proses naik kelas otomatis (hanya referensi)
- Membutuhkan semester yang valid dan aktif

### Catatan Penting
- Fitur ini hanya menampilkan data sebagai referensi
- Proses naik kelas sebenarnya harus dilakukan manual di Dapodik
- Data yang ditampilkan berdasarkan keanggotaan rombel di semester sebelumnya

## Pengembangan Selanjutnya

### Fitur yang Dapat Ditambahkan
1. **Proses Naik Kelas Otomatis**: Implementasi untuk melakukan naik kelas secara otomatis
2. **Filter Tambahan**: Filter berdasarkan rombel, tingkat pendidikan, dll
3. **Export Data**: Export data siswa naik kelas ke Excel
4. **Validasi Data**: Validasi kelengkapan data siswa sebelum naik kelas
5. **Riwayat Naik Kelas**: Menyimpan riwayat proses naik kelas

### Perbaikan Teknis
1. **Optimasi Query**: Optimasi query untuk performa yang lebih baik
2. **Error Handling**: Penanganan error yang lebih robust
3. **Loading States**: Indikator loading yang lebih informatif
4. **Responsive Design**: Tampilan yang responsif untuk berbagai ukuran layar

## Troubleshooting

### Masalah Umum
1. **Data Tidak Muncul**: Pastikan semester yang dipilih memiliki data siswa
2. **Error Database**: Periksa koneksi database dan hak akses
3. **Tipe Data Mismatch**: Pastikan tipe data di backend sesuai dengan database

### Solusi
1. **Restart Aplikasi**: Restart aplikasi jika terjadi error
2. **Periksa Log**: Periksa log aplikasi untuk informasi error detail
3. **Validasi Data**: Pastikan data di database valid dan konsisten 