# Daftar Lengkap Tabel Database Dapodik

## Informasi Database
- **Database**: `pendataan`
- **Host**: `localhost`
- **Port**: `54532`
- **User**: `dapodik_user`
- **Password**: `17Agustus1945`

## Skema Database

Database ini memiliki **7 skema** dengan total **265 tabel**:

### 1. Skema `public` (140 tabel)
Skema utama yang berisi data operasional aplikasi Dapodik.

#### Tabel Utama Data Siswa:
- `peserta_didik` - Data utama peserta didik
- `peserta_didik_baru` - Data peserta didik baru
- `peserta_didik_longitudinal` - Data longitudinal peserta didik
- `registrasi_peserta_didik` - Data registrasi peserta didik
- `rombongan_belajar` - Data rombongan belajar/kelas
- `anggota_rombel` - Relasi siswa dengan rombongan belajar

#### Tabel Data PTK (Pendidik dan Tenaga Kependidikan):
- `ptk` - Data utama PTK
- `ptk_baru` - Data PTK baru
- `ptk_terdaftar` - Data PTK terdaftar

#### Tabel Data Sekolah:
- `sekolah` - Data sekolah
- `sekolah_longitudinal` - Data longitudinal sekolah
- `sekolah_paud` - Data sekolah PAUD

#### Tabel Data Sarana Prasarana:
- `bangunan` - Data bangunan sekolah
- `bangunan_longitudinal` - Data longitudinal bangunan
- `bangunan_dari_blockgrant` - Data bangunan dari blockgrant
- `ruang` - Data ruangan
- `ruang_longitudinal` - Data longitudinal ruangan
- `tanah` - Data tanah sekolah
- `tanah_longitudinal` - Data longitudinal tanah
- `tanah_dari_blockgrant` - Data tanah dari blockgrant
- `alat` - Data alat pendidikan
- `alat_longitudinal` - Data longitudinal alat
- `alat_dari_blockgrant` - Data alat dari blockgrant
- `angkutan` - Data angkutan sekolah
- `angkutan_dari_blockgrant` - Data angkutan dari blockgrant

#### Tabel Data Akademik:
- `pembelajaran` - Data pembelajaran
- `jadwal` - Data jadwal pelajaran
- `nilai_test` - Data nilai test
- `prestasi` - Data prestasi siswa
- `aktivitas_kepanitiaan` - Data aktivitas kepanitiaan
- `anggota_akt_pd` - Anggota aktivitas peserta didik
- `akt_pd` - Aktivitas peserta didik
- `kepanitiaan` - Data kepanitiaan
- `anggota_panitia` - Anggota panitia
- `kelas_ekskul` - Data kelas ekstrakurikuler

#### Tabel Data Kesejahteraan:
- `kesejahteraan` - Data kesejahteraan
- `kesejahteraan_pd` - Data kesejahteraan peserta didik
- `beasiswa_peserta_didik` - Data beasiswa peserta didik
- `beasiswa_ptk` - Data beasiswa PTK
- `bantuan_pd` - Data bantuan peserta didik
- `tunjangan` - Data tunjangan

#### Tabel Data Ijazah dan Kelulusan:
- `ijazah_pd` - Data ijazah peserta didik
- `sertifikasi_pd` - Data sertifikasi peserta didik

#### Tabel Data Riwayat:
- `rwy_pend_formal` - Riwayat pendidikan formal
- `rwy_kerja` - Riwayat kerja
- `rwy_kepangkatan` - Riwayat kepangkatan
- `rwy_struktural` - Riwayat struktural
- `rwy_fungsional` - Riwayat fungsional
- `rwy_sertifikasi` - Riwayat sertifikasi

#### Tabel Data Keluarga:
- `anak` - Data anak
- `paspor_pd` - Data paspor peserta didik
- `paspor_ptk` - Data paspor PTK
- `kitas_pd` - Data KITAS peserta didik
- `kitas_ptk` - Data KITAS PTK

#### Tabel Data Lainnya:
- `jurusan_sp` - Data jurusan sekolah
- `jurusan_kerjasama` - Data kerjasama jurusan
- `jur_sp_long` - Data longitudinal jurusan
- `akreditasi_sp` - Data akreditasi sekolah
- `akreditasi_prodi` - Data akreditasi program studi
- `buku` - Data buku
- `buku_longitudinal` - Data longitudinal buku
- `buku_pelajaran` - Data buku pelajaran
- `buku_ptk` - Data buku PTK
- `sanitasi` - Data sanitasi
- `sumber_dana` - Data sumber dana
- `sumber_dana_sekolah` - Data sumber dana sekolah
- `unit_usaha` - Data unit usaha
- `unit_usaha_kerjasama` - Data kerjasama unit usaha
- `yayasan` - Data yayasan
- `mou` - Data MoU
- `dudi` - Data dunia usaha dan industri
- `gugus_sekolah` - Data gugus sekolah
- `anggota_gugus` - Anggota gugus
- `lembaga_non_sekolah` - Data lembaga non sekolah
- `layanan_khusus` - Data layanan khusus
- `program_inklusi` - Data program inklusi
- `tracer_lulusan` - Data tracer lulusan
- `tugas_tambahan` - Data tugas tambahan
- `karya_tulis` - Data karya tulis
- `penghargaan` - Data penghargaan
- `inpassing` - Data inpassing
- `diklat` - Data diklat
- `bidang_sdm` - Data bidang SDM
- `bimbing_pd` - Data bimbingan peserta didik
- `blockgrant` - Data blockgrant
- `sasaran_blockgrant` - Data sasaran blockgrant
- `daya_tampung` - Data daya tampung
- `demografi` - Data demografi
- `guru_sasaran_pengawas` - Data guru sasaran pengawas
- `pengawas_terdaftar` - Data pengawas terdaftar
- `sasaran_pengawasan` - Data sasaran pengawasan
- `instalasi` - Data instalasi
- `izin_operasional` - Data izin operasional
- `naungan` - Data naungan
- `pesan` - Data pesan
- `author` - Data author
- `data_dynamic` - Data dinamis
- `validasi` - Data validasi
- `versi_db` - Data versi database
- `sync_log` - Log sinkronisasi
- `sync_primer` - Data sinkronisasi primer
- `sync_session` - Data sesi sinkronisasi
- `table_sync_log` - Log sinkronisasi tabel
- `table_sync` - Data sinkronisasi tabel
- `trace_log` - Log trace

#### Tabel Validasi (vld_*):
- `vld_peserta_didik` - Validasi peserta didik
- `vld_pd_long` - Validasi longitudinal peserta didik
- `vld_ptk` - Validasi PTK
- `vld_ptk_terdaftar` - Validasi PTK terdaftar
- `vld_reg_pd` - Validasi registrasi peserta didik
- `vld_rombel` - Validasi rombongan belajar
- `vld_sekolah` - Validasi sekolah
- `vld_bangunan` - Validasi bangunan
- `vld_tanah` - Validasi tanah
- `vld_alat` - Validasi alat
- `vld_angkutan` - Validasi angkutan
- `vld_anak` - Validasi anak
- `vld_bea_pd` - Validasi beasiswa peserta didik
- `vld_bea_ptk` - Validasi beasiswa PTK
- `vld_buku_ptk` - Validasi buku PTK
- `vld_demografi` - Validasi demografi
- `vld_dudi` - Validasi dunia usaha dan industri
- `vld_inpassing` - Validasi inpassing
- `vld_jurusan_sp` - Validasi jurusan sekolah
- `vld_karya_tulis` - Validasi karya tulis
- `vld_kesejahteraan` - Validasi kesejahteraan
- `vld_mou` - Validasi MoU
- `vld_nilai_rapor` - Validasi nilai rapor
- `vld_nilai_test` - Validasi nilai test
- `vld_nonsekolah` - Validasi non sekolah
- `vld_pembelajaran` - Validasi pembelajaran
- `vld_penghargaan` - Validasi penghargaan
- `vld_prestasi` - Validasi prestasi
- `vld_tugas_tambahan` - Validasi tugas tambahan
- `vld_tunjangan` - Validasi tunjangan
- `vld_un` - Validasi UN
- `vld_yayasan` - Validasi yayasan
- `vld_akt_pd` - Validasi aktivitas peserta didik
- `vld_rwy_fungsional` - Validasi riwayat fungsional
- `vld_rwy_kepangkatan` - Validasi riwayat kepangkatan
- `vld_rwy_kerja` - Validasi riwayat kerja
- `vld_rwy_pend_formal` - Validasi riwayat pendidikan formal
- `vld_rwy_sertifikasi` - Validasi riwayat sertifikasi
- `vld_rwy_struktural` - Validasi riwayat struktural

### 2. Skema `man_akses` (8 tabel)
Skema untuk manajemen akses dan pengguna.

- `pengguna` - Data pengguna sistem
- `peran` - Data peran pengguna
- `role_pengguna` - Relasi pengguna dengan peran
- `menu` - Data menu aplikasi
- `menu_role` - Relasi menu dengan peran
- `aplikasi` - Data aplikasi
- `log_otentikasi` - Log otentikasi
- `log_otorisasi` - Log otorisasi

### 3. Skema `ref` (102 tabel)
Skema referensi yang berisi data master dan lookup tables.

#### Tabel Referensi Utama:
- `agama` - Referensi agama
- `jenis_pendaftaran` - Referensi jenis pendaftaran
- `jenis_hobby` - Referensi jenis hobi
- `jenis_cita` - Referensi jenis cita-cita
- `jenis_ijazah` - Referensi jenis ijazah
- `jenis_keluar` - Referensi jenis keluar
- `jenis_ptk` - Referensi jenis PTK
- `jenis_rombel` - Referensi jenis rombongan belajar
- `jenis_sarana` - Referensi jenis sarana
- `jenis_prasarana` - Referensi jenis prasarana
- `jenis_bantuan` - Referensi jenis bantuan
- `jenis_beasiswa` - Referensi jenis beasiswa
- `jenis_kesejahteraan` - Referensi jenis kesejahteraan
- `jenis_tunjangan` - Referensi jenis tunjangan
- `jenis_prestasi` - Referensi jenis prestasi
- `jenis_penghargaan` - Referensi jenis penghargaan
- `jenis_sertifikasi` - Referensi jenis sertifikasi
- `jenis_test` - Referensi jenis test
- `jenis_tinggal` - Referensi jenis tinggal
- `jenis_lembaga` - Referensi jenis lembaga
- `jenis_kerusakan` - Referensi jenis kerusakan
- `jenis_hapus_buku` - Referensi jenis hapus buku
- `jenis_kepanitiaan` - Referensi jenis kepanitiaan
- `jenis_aktivitas_kepanitiaan` - Referensi jenis aktivitas kepanitiaan
- `jenis_akt_pd` - Referensi jenis aktivitas peserta didik
- `jenis_diklat` - Referensi jenis diklat
- `jenis_gugus` - Referensi jenis gugus
- `jenis_pesan` - Referensi jenis pesan

#### Tabel Referensi Pendidikan:
- `semester` - Referensi semester
- `tahun_ajaran` - Referensi tahun ajaran
- `jenjang_pendidikan` - Referensi jenjang pendidikan
- `jenjang_kepengawasan` - Referensi jenjang kepengawasan
- `tingkat_pendidikan` - Referensi tingkat pendidikan
- `tingkat_prestasi` - Referensi tingkat prestasi
- `tingkat_penghargaan` - Referensi tingkat penghargaan
- `kurikulum` - Referensi kurikulum
- `status_di_kurikulum` - Referensi status di kurikulum
- `mata_pelajaran` - Referensi mata pelajaran
- `mata_pelajaran_kurikulum` - Referensi mata pelajaran kurikulum
- `group_matpel` - Referensi grup mata pelajaran
- `map_bidang_mata_pelajaran` - Referensi mapping bidang mata pelajaran
- `mulok` - Referensi muatan lokal
- `kompetensi` - Referensi kompetensi
- `bidang_studi` - Referensi bidang studi
- `jurusan` - Referensi jurusan

#### Tabel Referensi Wilayah:
- `mst_wilayah` - Master wilayah
- `level_wilayah` - Referensi level wilayah
- `tetangga_kabkota` - Referensi tetangga kabupaten/kota
- `kategori_desa` - Referensi kategori desa

#### Tabel Referensi Pekerjaan dan Ekonomi:
- `pekerjaan` - Referensi pekerjaan
- `penghasilan` - Referensi penghasilan
- `bidang_usaha` - Referensi bidang usaha
- `kelompok_usaha` - Referensi kelompok usaha
- `bank` - Referensi bank
- `sumber_gaji` - Referensi sumber gaji
- `sumber_dana` - Referensi sumber dana

#### Tabel Referensi Sarana Prasarana:
- `pemakai_sarana` - Referensi pemakai sarana
- `pemakai_prasarana` - Referensi pemakai prasarana
- `standar_sarana` - Referensi standar sarana
- `keahlian_laboratorium` - Referensi keahlian laboratorium
- `sumber_air` - Referensi sumber air
- `sumber_listrik` - Referensi sumber listrik
- `status_kepemilikan` - Referensi status kepemilikan
- `status_kepemilikan_sarpras` - Referensi status kepemilikan sarana prasarana

#### Tabel Referensi PTK:
- `jabatan_ptk` - Referensi jabatan PTK
- `jabatan_tugas_ptk` - Referensi jabatan tugas PTK
- `jabatan_fungsional` - Referensi jabatan fungsional
- `pangkat_golongan` - Referensi pangkat golongan
- `status_kepegawaian` - Referensi status kepegawaian
- `status_keaktifan_pegawai` - Referensi status keaktifan pegawai
- `lembaga_pengangkat` - Referensi lembaga pengangkat
- `lembaga_akreditasi` - Referensi lembaga akreditasi
- `lemb_sertifikasi` - Referensi lembaga sertifikasi
- `sertifikasi_iso` - Referensi sertifikasi ISO

#### Tabel Referensi Lainnya:
- `negara` - Referensi negara
- `kebutuhan_khusus` - Referensi kebutuhan khusus
- `status_anak` - Referensi status anak
- `kategori_tk` - Referensi kategori TK
- `alat_transportasi` - Referensi alat transportasi
- `akses_internet` - Referensi akses internet
- `fasilitas_layanan` - Referensi fasilitas layanan
- `gelar_akademik` - Referensi gelar akademik
- `bentuk_lembaga` - Referensi bentuk lembaga
- `bentuk_pendidikan` - Referensi bentuk pendidikan
- `klasifikasi_lembaga` - Referensi klasifikasi lembaga
- `kelompok_bidang` - Referensi kelompok bidang
- `alasan_layak_pip` - Referensi alasan layak PIP
- `batas_waktu_rapor` - Referensi batas waktu rapor
- `waktu_penyelenggaraan` - Referensi waktu penyelenggaraan
- `jadwal_paud` - Referensi jadwal PAUD
- `ekstra_kurikuler` - Referensi ekstrakurikuler
- `errortype` - Referensi tipe error
- `akreditasi` - Referensi akreditasi
- `variabel` - Referensi variabel
- `variabel_value` - Referensi nilai variabel
- `template_rapor` - Template rapor
- `template_un` - Template UN
- `table_sync` - Referensi sinkronisasi tabel

### 4. Skema `nilai` (5 tabel)
Skema untuk data nilai akademik.

- `nilai_rapor` - Data nilai rapor
- `nilai_smt` - Data nilai semester
- `nilai_ekskul` - Data nilai ekstrakurikuler
- `matev_rapor` - Data evaluasi mata pelajaran rapor
- `un` - Data Ujian Nasional

### 5. Skema `pustaka` (7 tabel)
Skema untuk data perpustakaan.

- `biblio` - Data bibliografi
- `classifications` - Data klasifikasi
- `daftar_author` - Data daftar author
- `frequency` - Data frekuensi
- `gmd` - Data GMD (General Material Designation)
- `mapel_biblio` - Mapping mata pelajaran bibliografi
- `publisher` - Data penerbit
- `tingkat_biblio` - Data tingkat bibliografi

### 6. Skema `audit` (1 tabel)
Skema untuk audit trail.

- `logged_actions` - Log aksi yang dilakukan

### 7. Skema `blob` (1 tabel)
Skema untuk data binary large object.

- `large_object` - Data objek besar

## Ringkasan Statistik

| Skema | Jumlah Tabel | Deskripsi |
|-------|-------------|-----------|
| `public` | 140 | Data operasional utama |
| `ref` | 102 | Data referensi/master |
| `man_akses` | 8 | Manajemen akses |
| `nilai` | 5 | Data nilai akademik |
| `pustaka` | 7 | Data perpustakaan |
| `audit` | 1 | Audit trail |
| `blob` | 1 | Data binary |
| **Total** | **265** | **Semua tabel** |

## Catatan Penting

1. **Tabel Utama**: Tabel-tabel di skema `public` adalah yang paling sering digunakan untuk operasional sehari-hari
2. **Tabel Referensi**: Tabel-tabel di skema `ref` berisi data master yang digunakan untuk dropdown dan validasi
3. **Tabel Validasi**: Tabel-tabel dengan prefix `vld_` digunakan untuk menyimpan hasil validasi data
4. **Tabel Longitudinal**: Tabel-tabel dengan suffix `_longitudinal` menyimpan data historis/perubahan waktu
5. **Tabel Sync**: Tabel-tabel dengan prefix `sync_` digunakan untuk sinkronisasi data dengan server pusat

Database ini merupakan database standar Dapodik yang digunakan untuk manajemen data pendidikan di Indonesia.