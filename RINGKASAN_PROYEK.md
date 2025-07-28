# Ringkasan Detail Proyek: DapoMaster

Dokumen ini memberikan ringkasan teknis dan fungsional yang komprehensif dari aplikasi **DapoMaster**. Informasi ini ditujukan untuk presentasi kepada klien, memberikan gambaran jelas tentang arsitektur, fitur, dan keunggulan aplikasi.

---

## 1. Ringkasan Eksekutif

**DapoMaster** adalah aplikasi desktop lintas platform (Windows, macOS, Linux) yang dirancang untuk menjadi alat bantu manajemen data pendidikan yang kuat dan efisien, kemungkinan besar sebagai *helper* untuk sistem Dapodik nasional. Aplikasi ini memungkinkan operator sekolah untuk mengelola data siswa, statistik sekolah, dan data referensi penting lainnya secara terpusat dengan antarmuka yang modern dan responsif.

Dibangun dengan teknologi terkini, DapoMaster menggabungkan performa tinggi dari backend Rust dengan antarmuka pengguna yang dinamis dari React, menghasilkan aplikasi yang cepat, aman, dan andal.

---

## 2. Arsitektur & Tumpukan Teknologi

Aplikasi ini menggunakan arsitektur modern yang memisahkan logika backend dan frontend untuk skalabilitas dan pemeliharaan yang lebih mudah.

*   **Framework Inti**: **Tauri**
    *   Memungkinkan pembuatan aplikasi desktop menggunakan teknologi web (HTML, CSS, JavaScript/TypeScript) untuk antarmuka, sementara backend berjalan secara *native* untuk performa maksimal.
*   **Logika Backend**: **Rust**
    *   Bahasa pemrograman yang terkenal dengan keamanannya (*memory safety*) dan kecepatannya. Semua operasi berat seperti kalkulasi, interaksi database, dan logika bisnis dijalankan di sini.
*   **Antarmuka Frontend**: **React & TypeScript**
    *   **React**: Library JavaScript populer untuk membangun antarmuka pengguna yang interaktif dan berbasis komponen.
    *   **TypeScript**: Menambahkan sistem tipe statis ke JavaScript, mengurangi *bug* dan meningkatkan kualitas kode.
    *   **Vite**: *Build tool* modern yang sangat cepat untuk pengembangan frontend.
    *   **Tailwind CSS**: Framework CSS *utility-first* untuk desain antarmuka yang cepat dan kustom.
*   **Database**: **PostgreSQL**
    *   Sistem manajemen database relasional yang kuat dan andal. Interaksi dengan database dilakukan dari sisi backend (Rust) menggunakan library `sqlx` untuk koneksi yang aman dan efisien.

---

## 3. Alur Aplikasi & Antarmuka Pengguna (UI/UX)

Pengalaman pengguna dirancang agar intuitif dan efisien.

1.  **Inisialisasi Aplikasi**: Saat dibuka, aplikasi menampilkan **splash screen** singkat yang memberikan kesan profesional sambil memuat sumber daya di latar belakang.
2.  **Pemilihan Pengguna**: Layar pertama yang muncul adalah pemilihan **Operator/Pengguna**. Ini mengindikasikan sistem yang aman dan siap untuk multi-peran, di mana setiap tindakan dapat diatribusikan ke pengguna tertentu.
3.  **Antarmuka Utama**: Setelah memilih pengguna, antarmuka utama ditampilkan, yang terdiri dari tiga bagian utama:
    *   **Sidebar (Menu Navigasi)**: Di sisi kiri, menyediakan akses mudah ke semua modul utama aplikasi seperti Dasbor dan Siswa.
    *   **Area Konten Utama**: Bagian tengah yang secara dinamis menampilkan konten berdasarkan menu yang dipilih (misalnya, menampilkan grafik statistik di Dasbor atau tabel data di menu Siswa).
    *   **Panel Log**: Di bagian bawah, panel ini secara **real-time** menampilkan log aktivitas dari backend. Ini adalah fitur unggulan untuk transparansi, memudahkan pelacakan proses, dan membantu *debugging* dengan cepat.

---

## 4. Fitur Utama (Fungsionalitas Rinci)

Aplikasi ini memiliki serangkaian fitur yang komprehensif untuk manajemen data sekolah.

### a. Dasbor & Statistik (`dashboard.rs`)

*   **Fungsi**: Memberikan gambaran umum (snapshot) kondisi sekolah secara real-time.
*   **Data yang Ditampilkan**:
    *   Jumlah Total Siswa
    *   Jumlah Total Pendidik & Tenaga Kependidikan (PTK)
    *   Jumlah Total Rombongan Belajar (Rombel)
    *   Jumlah Total Jurusan
*   **Tujuan**: Membantu manajemen sekolah dalam mengambil keputusan cepat berdasarkan data agregat.

### b. Manajemen Data Siswa (`siswa.rs`)

Ini adalah modul inti dari aplikasi, menyediakan fungsionalitas **CRUD** (Create, Read, Update, Delete) yang lengkap.

*   **Membaca Data (Read)**:
    *   **Daftar Siswa**: Menampilkan daftar semua siswa dengan paginasi (data dibagi per halaman) untuk performa yang optimal.
    *   **Pencarian & Filter**: Pengguna dapat dengan mudah mencari siswa berdasarkan **Nama** atau **NISN**, dan memfilter daftar berdasarkan **Rombongan Belajar**.
*   **Membuat Data (Create)**:
    *   **Registrasi Siswa Baru**: Formulir pendaftaran yang sangat detail, mencakup:
        *   Data Pribadi (Nama, NISN, NIK, TTL, Jenis Kelamin, Agama)
        *   Data Pendaftaran (Tanggal Masuk, Jenis Pendaftaran, Sekolah Asal)
        *   Data Tambahan (Hobi, Cita-cita, Riwayat PAUD/TK)
        *   Data Alamat (dengan pemilihan wilayah berjenjang)
        *   Data Orang Tua (Nama Ibu Kandung)
    *   Proses ini berjalan dalam satu **transaksi database** untuk memastikan semua data terkait berhasil disimpan atau tidak sama sekali (integritas data).
*   **Mengubah Data (Update)**: Memungkinkan operator untuk mengedit dan memperbarui informasi siswa yang sudah ada.
*   **Menghapus Data (Delete)**: Menghapus data siswa secara permanen dari sistem, termasuk data terkait di tabel lain (seperti keanggotaan rombel) untuk menjaga kebersihan database.

### c. Manajemen Data Referensi (`referensi.rs`)

*   **Fungsi**: Menyediakan data master yang konsisten untuk digunakan di seluruh aplikasi, terutama pada formulir isian.
*   **Data yang Disediakan**:
    *   Daftar Rombongan Belajar
    *   Daftar Agama
    *   Jenis Pendaftaran
    *   Daftar Hobi & Cita-cita
    *   **Referensi Wilayah**: Fitur penting yang memungkinkan pemilihan alamat secara berjenjang (Provinsi -> Kabupaten/Kota -> Kecamatan -> Desa/Kelurahan), memastikan data alamat yang valid dan terstruktur.

### d. Manajemen Pengguna (`pengguna.rs`)

*   **Fungsi**: Mengelola akses ke aplikasi.
*   **Fitur Saat Ini**: Mengambil daftar semua pengguna yang memiliki peran sebagai operator sekolah (`peran_id=10`) untuk ditampilkan di layar pemilihan pengguna awal.

---

## 5. Struktur Database (PostgreSQL)

Struktur database dirancang secara relasional untuk memastikan integritas dan efisiensi data. Tabel-tabel utama yang teridentifikasi adalah:

*   `peserta_didik`: Tabel induk untuk semua data pribadi siswa.
*   `registrasi_peserta_didik`: Mencatat riwayat registrasi siswa di sekolah.
*   `rombongan_belajar` & `anggota_rombel`: Mengelola kelas/kelompok belajar dan siswa di dalamnya.
*   `man_akses.pengguna` & `man_akses.role_pengguna`: Mengelola pengguna dan hak aksesnya.
*   `ref.*`: Skema khusus yang berisi banyak tabel referensi (seperti `ref.agama`, `ref.mst_wilayah`) untuk memastikan data yang dimasukkan seragam dan valid.
*   Tabel lain seperti `ptk` (data guru) dan `jurusan_sp` (data jurusan).

---

## 6. Keunggulan & Poin Jual

*   **Performa Tinggi**: Dengan backend Rust, aplikasi berjalan sangat cepat, bahkan saat mengolah data dalam jumlah besar.
*   **Keamanan**: Rust mencegah banyak celah keamanan umum secara default. Interaksi database yang aman menggunakan `sqlx` juga mengurangi risiko injeksi SQL.
*   **Pengalaman Pengguna Modern**: Antarmuka yang dibangun dengan React terasa responsif dan modern, tidak seperti aplikasi desktop tradisional.
*   **Transparansi Operasional**: Panel log real-time memberikan umpan balik instan kepada pengguna tentang proses yang sedang berjalan, membangun kepercayaan dan memudahkan pemecahan masalah.
*   **Data Terstruktur & Valid**: Penggunaan data referensi yang ekstensif memastikan bahwa data yang dimasukkan ke dalam sistem memiliki tingkat validitas yang tinggi.
*   **Lintas Platform**: Dapat di-deploy di Windows, macOS, dan Linux dengan basis kode yang sama.