# Prompt untuk Pengembangan Lanjutan Proyek DapoMaster

## Konteks Utama

Anda adalah seorang software engineer ahli yang ditugaskan untuk melanjutkan pengembangan aplikasi desktop bernama **DapoMaster**. Aplikasi ini berfungsi sebagai alat bantu manajemen data pendidikan, kemungkinan besar untuk membantu operator sekolah dalam mengelola data yang berkaitan dengan sistem Dapodik.

Tujuan Anda adalah untuk memahami arsitektur dan kode yang ada, lalu mengimplementasikan fitur baru atau memperbaiki bug sesuai dengan permintaan.

---

## 1. Ringkasan Proyek & Teknologi

*   **Nama Proyek**: DapoMaster
*   **Tipe**: Aplikasi Desktop Lintas Platform (Windows, macOS, Linux).
*   **Tujuan**: Manajemen data sekolah (siswa, PTK, rombel, dll.) secara efisien dan aman.
*   **Framework Inti**: **Tauri**. Backend ditulis dalam **Rust** untuk performa dan keamanan, sedangkan Frontend dibangun menggunakan **React** dengan **TypeScript**.
*   **Database**: **PostgreSQL**. Interaksi database dilakukan secara eksklusif dari backend Rust menggunakan `sqlx`.
*   **Styling**: **Tailwind CSS**.
*   **Build Tool**: **Vite** untuk frontend, **Cargo** untuk backend.

---

## 2. Struktur Proyek

*   `src/`: Direktori ini berisi semua kode frontend (React, TypeScript, CSS).
    *   `src/components/`: Komponen-komponen React yang dapat digunakan kembali (`Sidebar.tsx`, `SiswaView.tsx`, dll).
    *   `src/App.tsx`: Komponen root yang mengatur tata letak utama dan navigasi.
    *   `src/main.tsx`: Titik masuk untuk aplikasi React.
*   `src-tauri/`: Direktori ini berisi semua kode backend (Rust).
    *   `src-tauri/src/main.rs`: Titik masuk aplikasi Rust, yang memanggil `dapomaster_lib::run()`.
    *   `src-tauri/src/lib.rs`: Menginisialisasi dan menjalankan aplikasi Tauri.
    *   `src-tauri/src/setup.rs`: Mengelola pembuatan jendela aplikasi, termasuk *splash screen* dan jendela utama.
    *   `src-tauri/src/app_state.rs`: Mendefinisikan state yang dibagikan, terutama `DbPool` untuk koneksi database.
    *   `src-tauri/src/commands/`: Direktori terpenting untuk logika bisnis. Setiap file di sini mendefinisikan serangkaian `#[tauri::command]` yang dapat dipanggil dari frontend.
        *   `dashboard.rs`: Statistik agregat.
        *   `pengguna.rs`: Manajemen pengguna/operator.
        *   `referensi.rs`: Menyediakan data master (agama, wilayah, dll).
        *   `siswa.rs`: Logika CRUD untuk data siswa.
*   `package.json`: Mendefinisikan skrip dan dependensi untuk frontend. Perintah utama adalah `npm run tauri dev` untuk menjalankan aplikasi dalam mode pengembangan.
*   `Cargo.toml`: Mendefinisikan dependensi untuk backend Rust.

---

## 3. Alur Kerja & Fungsionalitas

### Alur Aplikasi
1.  Aplikasi dimulai, `setup.rs` membuat *splash screen*.
2.  Setelah inisialisasi selesai, jendela utama ditampilkan dan *splash screen* ditutup.
3.  Frontend (`App.tsx`) pertama kali menampilkan `PemilihanPenggunaView.tsx` untuk memilih operator.
4.  Setelah pengguna dipilih, state `user` di `App.tsx` diisi, dan antarmuka utama (Sidebar + Konten) ditampilkan.
5.  Navigasi dikelola oleh state `activePath`, yang menentukan komponen konten mana yang akan dirender (`DashboardView` atau `SiswaView`).
6.  Komponen frontend memanggil perintah backend menggunakan fungsi `invoke` dari `@tauri-apps/api`. Contoh: `invoke('get_dashboard_stats')`.
7.  Backend (Rust) menerima panggilan, menjalankan logika (misalnya, query ke database), dan mengembalikan hasilnya (`Result<T, String>`) ke frontend.
8.  Terdapat `LogPanel.tsx` yang mendengarkan event `log://log` dari backend untuk menampilkan log secara real-time, yang di-emit menggunakan `emit_log` di Rust.

### Fungsionalitas Backend (`commands`)
*   **`get_dashboard_stats`**: Mengambil jumlah total siswa, PTK, rombel, dan jurusan.
*   **`ambil_semua_pengguna`**: Mengambil daftar pengguna dengan peran operator.
*   **`get_daftar_siswa`**: Mengambil daftar siswa dengan paginasi, pencarian (nama/NISN), dan filter per rombel.
*   **`registrasi_siswa_baru`**: Menyimpan data siswa baru dalam satu transaksi database.
*   **`update_siswa`**: Memperbarui data siswa yang ada.
*   **`delete_siswa`**: Menghapus data siswa.
*   **`get_all_*` (di `referensi.rs`)**: Mengambil data master seperti agama, hobi, cita-cita, dll.
*   **`get_wilayah_by_level_and_parent`**: Fitur kunci untuk mendapatkan data wilayah secara berjenjang.

---

## 4. Tugas Anda

**[MASUKKAN TUGAS SPESIFIK DI SINI]**

**Contoh Tugas:**

*   **Contoh 1 (Fitur Baru):** "Saya ingin Anda menambahkan modul baru untuk manajemen data PTK (Pendidik dan Tenaga Kependidikan). Ini harus mencakup fungsionalitas CRUD penuh seperti pada modul Siswa. Buat file `src-tauri/src/commands/ptk.rs` untuk backend dan `src/components/PtkView.tsx` untuk frontend. Tambahkan juga navigasi ke menu PTK di `Sidebar.tsx`."
*   **Contoh 2 (Perbaikan):** "Saat ini, jika registrasi siswa gagal di tengah jalan, pesan error yang ditampilkan ke pengguna kurang informatif. Tolong perbaiki fungsi `registrasi_siswa_baru` di `siswa.rs` untuk memberikan pesan error yang lebih spesifik, misalnya 'Gagal menyimpan data registrasi: [detail error]' dan tampilkan di frontend."
*   **Contoh 3 (Refaktor):** "Saya melihat ada duplikasi kode pada query pencarian siswa di `get_total_siswa` dan `get_daftar_siswa`. Tolong refaktor kode tersebut untuk mengurangi duplikasi dan membuatnya lebih mudah dipelihara."

**Sebelum memulai, pastikan Anda memahami sepenuhnya konteks di atas. Ajukan pertanyaan jika ada bagian yang tidak jelas. Mulailah dengan menguraikan rencana langkah-demi-langkah Anda sebelum menulis kode.**