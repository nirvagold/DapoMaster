import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, AlertTriangle, ChevronLeft, ChevronRight, Search, Edit, Trash2, Plus, User, FileText } from "lucide-react";
import clsx from "clsx";
import SiswaForm from "./SiswaForm";
import { SiswaFormData } from "./SiswaForm";
import { WilayahReferensi } from "./SiswaForm";
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";

// Hook untuk debouncing
function useDebounce<T>(value: T, delay: number): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);
  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);
    return () => {
      clearTimeout(handler);
    };
  }, [value, delay]);
  return debouncedValue;
}

// Tipe data untuk Rombel
type RombonganBelajar = {
  rombongan_belajar_id: string;
  nama: string;
};

// Tipe data Peserta Didik yang diperluas
type PesertaDidik = {
  peserta_didik_id: string;
  nama: string;
  jenis_kelamin: string;
  nisn: string;
  nik?: string;
  tempat_lahir?: string;
  tanggal_lahir: string;
  agama_id: number;
  // Data tambahan dari analisis database
  kewarganegaraan?: string;
  alamat_jalan?: string;
  desa_kelurahan?: string;
  kode_wilayah?: string;
  nama_ibu_kandung?: string;
  no_kk?: string;
  rt?: string; // Changed from number to string for BigDecimal compatibility
  rw?: string; // Changed from number to string for BigDecimal compatibility
  nama_dusun?: string;
  kode_pos?: string;
  lintang?: string; // Changed from number to string for BigDecimal compatibility
  bujur?: string; // Changed from number to string for BigDecimal compatibility
  jenis_tinggal_id?: string; // Changed from number to string for BigDecimal compatibility
  alat_transportasi_id?: string; // Changed from number to string for BigDecimal compatibility
  nik_ayah?: string;
  nik_ibu?: string;
  anak_keberapa?: string; // Changed from number to string for BigDecimal compatibility
  nik_wali?: string;
  nomor_telepon_rumah?: string;
  nomor_telepon_seluler?: string;
  email?: string;
  // Data dari registrasi_peserta_didik
  nipd?: string;
  tanggal_masuk_sekolah?: string;
  jenis_pendaftaran_id?: string; // Changed from number to string for BigDecimal compatibility
  id_hobby?: string; // Changed from number to string for BigDecimal compatibility
  id_cita?: string; // Changed from number to string for BigDecimal compatibility
  a_pernah_paud?: string; // Changed from number to string for BigDecimal compatibility
  a_pernah_tk?: string; // Changed from number to string for BigDecimal compatibility
  jenis_keluar_id?: string; // Changed from number to string for BigDecimal compatibility
  tanggal_keluar?: string;
  alasan_keluar?: string;
  // Data dari anggota_rombel
  rombongan_belajar_id?: string;
  nama_rombel?: string;
};

// Tipe data referensi
export type Agama = {
  agama_id: number;
  nama: string;
};

export type JenisPendaftaran = {
  jenis_pendaftaran_id: number;
  nama: string;
};

export type Hobby = { 
  id_hobby: number; 
  nm_hobby: string; 
};

export type Cita = { 
  id_cita: number; 
  nm_cita: string; 
};

export type JenisKeluar = {
  jenis_keluar_id: string;
  ket_keluar: string;
};

export type JenisTinggal = {
  jenis_tinggal_id: string; // Changed from number to string for BigDecimal compatibility
  nama: string;
};

export type AlatTransportasi = {
  alat_transportasi_id: string; // Changed from number to string for BigDecimal compatibility
  nama: string;
};

const PAGE_SIZE = 15;

export default function SiswaView({ pageTitle, user, semester, tahunAjaran }: { 
  pageTitle: string, 
  user: any,
  semester: Semester | null,
  tahunAjaran: TahunAjaran | null
}) {
  const [data, setData] = useState<PesertaDidik[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [totalSiswa, setTotalSiswa] = useState(0);
  const [searchTerm, setSearchTerm] = useState("");
  const debouncedSearchTerm = useDebounce(searchTerm, 500);
  const [rombels, setRombels] = useState<RombonganBelajar[]>([]);
  const [selectedRombel, setSelectedRombel] = useState<string>("");
  const [agamaList, setAgamaList] = useState<Agama[]>([]);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  
  // State untuk form registrasi siswa baru
  const [formDataRegistrasi, setFormDataRegistrasi] = useState<SiswaFormData>({
    nama: "", nisn: "", jenis_kelamin: "L", tempat_lahir: "", tanggal_lahir: "", agama_id: "",
    nipd: "", tanggal_masuk_sekolah: "", jenis_pendaftaran_id: "", id_hobby: "", id_cita: "",
    a_pernah_paud: false, a_pernah_tk: false,
    alamat_jalan: "",
    desa_kelurahan: "",
    kode_wilayah: "",
    nama_ibu_kandung: "",
    kewarganegaraan: "ID",
    nik: "",
    no_kk: "",
    rt: "",
    rw: "",
    nama_dusun: "",
    kode_pos: "",
    lintang: "",
    bujur: "",
    jenis_tinggal_id: "",
    alat_transportasi_id: "",
    nik_ayah: "",
    nik_ibu: "",
    anak_keberapa: "",
    nik_wali: "",
    nomor_telepon_rumah: "",
    nomor_telepon_seluler: "",
    email: "",
  });

  // State untuk form edit siswa
  const [formDataEdit, setFormDataEdit] = useState<SiswaFormData>({
    nama: "", nisn: "", jenis_kelamin: "L", tempat_lahir: "", tanggal_lahir: "", agama_id: "",
    nipd: "", tanggal_masuk_sekolah: "", jenis_pendaftaran_id: "", id_hobby: "", id_cita: "",
    a_pernah_paud: false, a_pernah_tk: false,
    alamat_jalan: "",
    desa_kelurahan: "",
    kode_wilayah: "",
    nama_ibu_kandung: "",
    kewarganegaraan: "ID",
    nik: "",
    no_kk: "",
    rt: "",
    rw: "",
    nama_dusun: "",
    kode_pos: "",
    lintang: "",
    bujur: "",
    jenis_tinggal_id: "",
    alat_transportasi_id: "",
    nik_ayah: "",
    nik_ibu: "",
    anak_keberapa: "",
    nik_wali: "",
    nomor_telepon_rumah: "",
    nomor_telepon_seluler: "",
    email: "",
  });

  // State untuk mode tampilan
  const [viewMode, setViewMode] = useState<'list' | 'registrasi' | 'edit'>('list');
  const [editingSiswa, setEditingSiswa] = useState<PesertaDidik | null>(null);
  
  const [referensi, setReferensi] = useState({
    jenisPendaftaran: [] as JenisPendaftaran[],
    hobbies: [] as Hobby[],
    citas: [] as Cita[],
    agama: [] as Agama[],
    jenisKeluar: [] as JenisKeluar[],
    jenisTinggal: [] as JenisTinggal[],
    alatTransportasi: [] as AlatTransportasi[],
  });
  
  // Handler untuk edit siswa
  const handleEditClick = (siswa: PesertaDidik) => {
    console.log('[EDIT] Mulai proses edit siswa:', siswa.nama, 'ID:', siswa.peserta_didik_id);
    console.log('[EDIT] Data siswa lengkap:', siswa);
    
    setEditingSiswa(siswa);
    
    const formData = {
      nama: siswa.nama,
      nisn: siswa.nisn || "",
      jenis_kelamin: siswa.jenis_kelamin,
      tempat_lahir: siswa.tempat_lahir || "",
      tanggal_lahir: siswa.tanggal_lahir.split('T')[0],
      agama_id: String(siswa.agama_id),
      nipd: siswa.nipd || "",
      tanggal_masuk_sekolah: siswa.tanggal_masuk_sekolah || siswa.tanggal_lahir.split('T')[0],
      jenis_pendaftaran_id: String(siswa.jenis_pendaftaran_id || ""),
      id_hobby: String(siswa.id_hobby || ""),
      id_cita: String(siswa.id_cita || ""),
      a_pernah_paud: siswa.a_pernah_paud === "1",
      a_pernah_tk: siswa.a_pernah_tk === "1",
      alamat_jalan: siswa.alamat_jalan || "",
      desa_kelurahan: siswa.desa_kelurahan || "",
      kode_wilayah: siswa.kode_wilayah || "",
      nama_ibu_kandung: siswa.nama_ibu_kandung || "",
      kewarganegaraan: siswa.kewarganegaraan || "ID",
      nik: siswa.nik || "",
      no_kk: siswa.no_kk || "",
      rt: siswa.rt || "",
      rw: siswa.rw || "",
      nama_dusun: siswa.nama_dusun || "",
      kode_pos: siswa.kode_pos || "",
      lintang: siswa.lintang || "",
      bujur: siswa.bujur || "",
      jenis_tinggal_id: String(siswa.jenis_tinggal_id || ""),
      alat_transportasi_id: String(siswa.alat_transportasi_id || ""),
      nik_ayah: siswa.nik_ayah || "",
      nik_ibu: siswa.nik_ibu || "",
      anak_keberapa: siswa.anak_keberapa || "",
      nik_wali: siswa.nik_wali || "",
      nomor_telepon_rumah: siswa.nomor_telepon_rumah || "",
      nomor_telepon_seluler: siswa.nomor_telepon_seluler || "",
      email: siswa.email || "",
    };
    
    console.log('[EDIT] Form data yang akan diisi:', formData);
    setFormDataEdit(formData);
    setViewMode('edit');
    
    console.log('[EDIT] Berhasil masuk ke mode edit');
  };

  const handleDeleteClick = async (siswa: PesertaDidik) => {
    console.log('[DELETE] Konfirmasi penghapusan siswa:', siswa.nama, 'ID:', siswa.peserta_didik_id);
    
    if (window.confirm(`Apakah Anda yakin ingin menghapus siswa bernama ${siswa.nama}?`)) {
      console.log('[DELETE] User mengkonfirmasi penghapusan');
      
      try {
        console.log('[DELETE] Memanggil command delete_siswa...');
        await invoke("delete_siswa", { pesertaDidikId: siswa.peserta_didik_id });
        console.log('[DELETE] Penghapusan siswa berhasil');
        
        console.log('[DELETE] Refresh data siswa...');
        fetchData(currentPage, debouncedSearchTerm, selectedRombel);
        setSuccessMessage("Siswa berhasil dihapus");
      } catch (err) {
        console.error('[DELETE] Penghapusan siswa gagal:', err);
        console.error('[DELETE] Error details:', {
          message: err,
          type: typeof err,
          stack: err instanceof Error ? err.stack : 'No stack trace'
        });
        setError(String(err));
      }
    } else {
      console.log('[DELETE] User membatalkan penghapusan');
    }
  };
  
  // Handler submit form edit
  const handleUpdateSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingSiswa) return;
    
    console.log('[UPDATE] Mulai proses update siswa:', editingSiswa.peserta_didik_id);
    console.log('[UPDATE] Data siswa yang akan diupdate:', editingSiswa);
    
    setLoading(true);
    setError(null);
    setSuccessMessage(null);
    
    const payload = {
      ...formDataEdit,
      agama_id: parseInt(formDataEdit.agama_id, 10),
      jenis_pendaftaran_id: formDataEdit.jenis_pendaftaran_id ? parseFloat(formDataEdit.jenis_pendaftaran_id) : 0,
      id_hobby: formDataEdit.id_hobby ? parseFloat(formDataEdit.id_hobby) : 0,
      id_cita: formDataEdit.id_cita ? parseFloat(formDataEdit.id_cita) : null,
      a_pernah_paud: formDataEdit.a_pernah_paud ? "1" : "0",
      a_pernah_tk: formDataEdit.a_pernah_tk ? "1" : "0",
      sekolah_asal: null, // Add missing field
      sekolah_id: user?.sekolah_id,
      pengguna_id: user?.pengguna_id,
    };
    
    console.log('[UPDATE] Payload yang akan dikirim:', payload);
    console.log('[UPDATE] User ID:', user?.pengguna_id);
    console.log('[UPDATE] Sekolah ID:', user?.sekolah_id);
    
    try {
      console.log('[UPDATE] Memanggil command update_siswa_ghost (STEALTH MODE)...');
      const result = await invoke<string>("update_siswa_ghost", { payload, pesertaDidikId: editingSiswa.peserta_didik_id });
      console.log('[UPDATE] Update siswa berhasil (STEALTH MODE):', result);
      
      setSuccessMessage(result);
      resetFormEdit();
      setViewMode('list');
      setEditingSiswa(null);
      
      console.log('[UPDATE] Refresh data siswa...');
      fetchData(currentPage, debouncedSearchTerm, selectedRombel);
    } catch (err) {
      console.error('[UPDATE] Update siswa gagal:', err);
      setError(String(err));
    } finally {
      setLoading(false);
      console.log('[UPDATE] Proses update selesai.');
    }
  };

  const totalPages = Math.ceil(totalSiswa / PAGE_SIZE);

  const fetchData = (page: number, search: string, rombelId: string) => {
    setLoading(true);
    setError(null);

    const params = {
      search: search || null,
      rombelId: rombelId || null
    };

    invoke<number>("get_total_siswa", params)
      .then(setTotalSiswa)
      .catch(err => {
        console.error("Error fetching total siswa:", err);
        setError(`Gagal mengambil total siswa: ${err}`);
      });

    invoke<PesertaDidik[]>("get_daftar_siswa", { page, pageSize: PAGE_SIZE, ...params })
      .then(setData)
      .catch(err => {
        console.error("Error fetching daftar siswa:", err);
        setError(`Gagal mengambil daftar siswa: ${err}`);
      })
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    if (pageTitle === "Daftar") {
      invoke<RombonganBelajar[]>("get_all_rombels")
        .then(setRombels)
        .catch(err => setError(String(err)));
    }
  }, [pageTitle]);
  
  useEffect(() => {
    if (pageTitle === "Registrasi") {
      invoke<Agama[]>("get_all_agama")
        .then(setAgamaList)
        .catch(err => setError(`Gagal mengambil data agama: ${err}`));
    }
  }, [pageTitle]);

  // useEffect untuk memuat semua data referensi
  useEffect(() => {
    const fetchReferensi = async () => {
      try {
        console.log('[REFERENSI] Mulai memuat data referensi...');
        const [jenisPendaftaran, hobbies, citas, agama, jenisKeluar, jenisTinggal, alatTransportasi] = await Promise.all([
          invoke<JenisPendaftaran[]>("get_all_jenis_pendaftaran"),
          invoke<Hobby[]>("get_all_hobby"),
          invoke<Cita[]>("get_all_cita"),
          invoke<Agama[]>("get_all_agama"),
          invoke<JenisKeluar[]>("get_all_jenis_keluar"),
          invoke<JenisTinggal[]>("get_all_jenis_tinggal"),
          invoke<AlatTransportasi[]>("get_all_alat_transportasi"),
        ]);
        console.log('[REFERENSI] Data agama berhasil dimuat:', agama);
        setReferensi({ jenisPendaftaran, hobbies, citas, agama, jenisKeluar, jenisTinggal, alatTransportasi });
      } catch (err) {
        console.error('[REFERENSI] Error memuat data referensi:', err);
        setError(`Gagal memuat data referensi: ${err}`);
      }
    };
    fetchReferensi();
  }, []);

  // Efek utama untuk fetch data
  useEffect(() => {
    if (pageTitle === "Daftar" && viewMode === 'list') {
      setCurrentPage(1);
      fetchData(1, debouncedSearchTerm, selectedRombel);
    }
  }, [pageTitle, debouncedSearchTerm, selectedRombel, viewMode]);

  // Efek untuk paginasi
  useEffect(() => {
    if (pageTitle === "Daftar" && viewMode === 'list') {
      fetchData(currentPage, debouncedSearchTerm, selectedRombel);
    }
  }, [currentPage, viewMode]);
  
  const renderPagination = () => (
    <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mt-4 gap-4">
      <span className="text-sm text-gray-400">
        Halaman {currentPage} dari {totalPages} (Total: {totalSiswa} siswa)
      </span>
      <div className="flex gap-2">
        <button
          onClick={() => setCurrentPage(p => p - 1)}
          disabled={currentPage === 1 || loading}
          className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gray-700 rounded-md hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <ChevronLeft size={16} />
          Previous
        </button>
        <button
          onClick={() => setCurrentPage(p => p + 1)}
          disabled={currentPage === totalPages || loading}
          className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gray-700 rounded-md hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Next
          <ChevronRight size={16} />
        </button>
      </div>
    </div>
  );

  // Handler untuk form registrasi
  const handleCheckboxChangeRegistrasi = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, checked } = e.target;
    setFormDataRegistrasi((prev: SiswaFormData) => ({ ...prev, [name]: checked }));
  };

  // Handler untuk form edit
  const handleInputChangeEdit = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormDataEdit((prev: SiswaFormData) => ({ ...prev, [name]: value }));
  };
  
  const handleCheckboxChangeEdit = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, checked } = e.target;
    setFormDataEdit((prev: SiswaFormData) => ({ ...prev, [name]: checked }));
  };

  // Handler submit form registrasi
  const handleFormSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    console.log('[REGISTRASI] Mulai proses registrasi siswa baru');
    console.log('[REGISTRASI] Form data registrasi:', formDataRegistrasi);
    
    setLoading(true);
    setError(null);
    setSuccessMessage(null);
    
    const payload = {
      ...formDataRegistrasi,
      agama_id: parseInt(formDataRegistrasi.agama_id, 10),
      sekolah_id: user?.sekolah_id,
      pengguna_id: user?.pengguna_id,
    };
    
    console.log('[REGISTRASI] Payload yang akan dikirim:', payload);
    console.log('[REGISTRASI] User ID:', user?.pengguna_id);
    console.log('[REGISTRASI] Sekolah ID:', user?.sekolah_id);
    
    try {
      console.log('[REGISTRASI] Memanggil command registrasi_siswa_baru...');
      const result = await invoke<string>("registrasi_siswa_baru", { payload });
      console.log('[REGISTRASI] Registrasi siswa berhasil:', result);
      
      setSuccessMessage(result);
      resetFormRegistrasi();
      setViewMode('list');
      
      console.log('[REGISTRASI] Form berhasil direset dan kembali ke daftar');
    } catch (err) {
      console.error('[REGISTRASI] Registrasi siswa gagal:', err);
      console.error('[REGISTRASI] Error details:', {
        message: err,
        type: typeof err,
        stack: err instanceof Error ? err.stack : 'No stack trace'
      });
      setError(String(err));
    } finally {
      setLoading(false);
      console.log('[REGISTRASI] Proses registrasi selesai.');
    }
  };

  // Reset form registrasi
  const resetFormRegistrasi = () => {
    setFormDataRegistrasi({
      nama: "", nisn: "", jenis_kelamin: "L", tempat_lahir: "", tanggal_lahir: "", agama_id: "",
      nipd: "", tanggal_masuk_sekolah: "", jenis_pendaftaran_id: "", id_hobby: "", id_cita: "",
      a_pernah_paud: false, a_pernah_tk: false,
      alamat_jalan: "",
      desa_kelurahan: "",
      kode_wilayah: "",
      nama_ibu_kandung: "",
      kewarganegaraan: "ID",
      nik: "",
      no_kk: "",
      rt: "",
      rw: "",
      nama_dusun: "",
      kode_pos: "",
      lintang: "",
      bujur: "",
      jenis_tinggal_id: "",
      alat_transportasi_id: "",
      nik_ayah: "",
      nik_ibu: "",
      anak_keberapa: "",
      nik_wali: "",
      nomor_telepon_rumah: "",
      nomor_telepon_seluler: "",
      email: "",
    });
  };

  // Reset form edit
  const resetFormEdit = () => {
    setFormDataEdit({
      nama: "", nisn: "", jenis_kelamin: "L", tempat_lahir: "", tanggal_lahir: "", agama_id: "",
      nipd: "", tanggal_masuk_sekolah: "", jenis_pendaftaran_id: "", id_hobby: "", id_cita: "",
      a_pernah_paud: false, a_pernah_tk: false,
      alamat_jalan: "",
      desa_kelurahan: "",
      kode_wilayah: "",
      nama_ibu_kandung: "",
      kewarganegaraan: "ID",
      nik: "",
      no_kk: "",
      rt: "",
      rw: "",
      nama_dusun: "",
      kode_pos: "",
      lintang: "",
      bujur: "",
      jenis_tinggal_id: "",
      alat_transportasi_id: "",
      nik_ayah: "",
      nik_ibu: "",
      anak_keberapa: "",
      nik_wali: "",
      nomor_telepon_rumah: "",
      nomor_telepon_seluler: "",
      email: "",
    });
  };

  // State untuk dropdown wilayah bertingkat
  const [wilayahOptions, setWilayahOptions] = useState<{
    provinsi: WilayahReferensi[];
    kabupaten: WilayahReferensi[];
    kecamatan: WilayahReferensi[];
    desa: WilayahReferensi[];
  }>({ provinsi: [], kabupaten: [], kecamatan: [], desa: [] });
  
  const [selectedWilayah, setSelectedWilayah] = useState<{
    provinsi: string;
    kabupaten: string;
    kecamatan: string;
  }>({ provinsi: "", kabupaten: "", kecamatan: "" });

  // Fetch provinsi saat mount
  useEffect(() => {
    if (viewMode === 'registrasi' || viewMode === 'edit') {
      invoke<WilayahReferensi[]>("get_wilayah_by_level_and_parent", { level: 1, parent: null })
        .then((provinsi) => setWilayahOptions((prev) => ({ ...prev, provinsi })))
        .catch((err) => setError(`Gagal memuat provinsi: ${err}`));
    }
  }, [viewMode]);

  // Fetch kabupaten saat provinsi berubah
  useEffect(() => {
    if (selectedWilayah.provinsi) {
      const parentKode = selectedWilayah.provinsi.trim();
      invoke<WilayahReferensi[]>("get_wilayah_by_level_and_parent", { level: 2, parent: parentKode })
        .then((kabupaten) => setWilayahOptions((prev) => ({ ...prev, kabupaten })))
        .catch((err) => setError(`Gagal memuat kabupaten: ${err}`));
    } else {
      setWilayahOptions((prev) => ({ ...prev, kabupaten: [], kecamatan: [], desa: [] }));
    }
  }, [selectedWilayah.provinsi]);

  // Fetch kecamatan saat kabupaten berubah
  useEffect(() => {
    if (selectedWilayah.kabupaten) {
      const parentKode = selectedWilayah.kabupaten.trim();
      invoke<WilayahReferensi[]>("get_wilayah_by_level_and_parent", { level: 3, parent: parentKode })
        .then((kecamatan) => setWilayahOptions((prev) => ({ ...prev, kecamatan })))
        .catch((err) => setError(`Gagal memuat kecamatan: ${err}`));
    } else {
      setWilayahOptions((prev) => ({ ...prev, kecamatan: [], desa: [] }));
    }
  }, [selectedWilayah.kabupaten]);

  // Fetch desa saat kecamatan berubah
  useEffect(() => {
    if (selectedWilayah.kecamatan) {
      const parentKode = selectedWilayah.kecamatan.trim();
      invoke<WilayahReferensi[]>("get_wilayah_by_level_and_parent", { level: 4, parent: parentKode })
        .then((desa) => setWilayahOptions((prev) => ({ ...prev, desa })))
        .catch((err) => setError(`Gagal memuat desa/kelurahan: ${err}`));
    } else {
      setWilayahOptions((prev) => ({ ...prev, desa: [] }));
    }
  }, [selectedWilayah.kecamatan]);

  // Handler perubahan dropdown wilayah
  const handleWilayahChange = (level: 'provinsi' | 'kabupaten' | 'kecamatan' | 'desa', kode: string) => {
    if (level === 'provinsi') {
      setSelectedWilayah({ provinsi: kode, kabupaten: '', kecamatan: '' });
      setFormDataRegistrasi((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
      setFormDataEdit((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
    } else if (level === 'kabupaten') {
      setSelectedWilayah((prev) => ({ ...prev, kabupaten: kode, kecamatan: '' }));
      setFormDataRegistrasi((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
      setFormDataEdit((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
    } else if (level === 'kecamatan') {
      setSelectedWilayah((prev) => ({ ...prev, kecamatan: kode }));
      setFormDataRegistrasi((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
      setFormDataEdit((prev) => ({ ...prev, kode_wilayah: '', desa_kelurahan: '' }));
    }
  };

  // Handler untuk form registrasi
  const handleInputChangeWilayahRegistrasi = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormDataRegistrasi((prev: SiswaFormData) => ({ ...prev, [name]: value }));
  };

  // Render form registrasi
  const renderRegistrationForm = () => (
    <div>
      {/* Header dengan tombol kembali */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-pink-500">Registrasi Siswa Baru</h2>
        <button
          onClick={() => setViewMode('list')}
          className="flex items-center gap-2 px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition"
        >
          <ChevronLeft size={16} />
          Kembali ke Daftar
        </button>
      </div>

      {/* Notifikasi */}
      {error && (
        <div className="mb-4 bg-red-700/90 text-white px-4 py-3 rounded-lg flex items-center justify-between shadow border border-red-600">
          <span className="font-semibold">{error}</span>
          <button
            onClick={() => setError(null)}
            className="ml-4 text-red-200 hover:text-white focus:outline-none"
            aria-label="Tutup notifikasi"
          >
            &times;
          </button>
        </div>
      )}
      
      {successMessage && (
        <div className="mb-4 bg-green-700/90 text-white px-4 py-3 rounded-lg flex items-center justify-between shadow border border-green-600">
          <span className="font-semibold">{successMessage}</span>
          <button
            onClick={() => setSuccessMessage(null)}
            className="ml-4 text-green-200 hover:text-white focus:outline-none"
            aria-label="Tutup notifikasi"
          >
            &times;
          </button>
        </div>
      )}

      <SiswaForm 
        formData={formDataRegistrasi}
        agamaList={agamaList}
        loading={loading}
        onInputChange={handleInputChangeWilayahRegistrasi}
        onCheckboxChange={handleCheckboxChangeRegistrasi}
        onSubmit={handleFormSubmit}
        submitText="Registrasi Siswa Baru"
        referensi={referensi}
        wilayahOptions={wilayahOptions}
        selectedWilayah={selectedWilayah}
        onWilayahChange={handleWilayahChange}
      />
    </div>
  );

  // Render form edit
  const renderEditForm = () => (
    <div>
      {/* Header dengan tombol kembali */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-pink-500">Edit Data Siswa</h2>
        <button
          onClick={() => {
            setViewMode('list');
            setEditingSiswa(null);
            resetFormEdit();
          }}
          className="flex items-center gap-2 px-4 py-2 bg-gray-700 text-white rounded-lg hover:bg-gray-600 transition"
        >
          <ChevronLeft size={16} />
          Kembali ke Daftar
        </button>
      </div>

      {/* Notifikasi */}
      {error && (
        <div className="mb-4 bg-red-700/90 text-white px-4 py-3 rounded-lg flex items-center justify-between shadow border border-red-600">
          <span className="font-semibold">{error}</span>
          <button
            onClick={() => setError(null)}
            className="ml-4 text-red-200 hover:text-white focus:outline-none"
            aria-label="Tutup notifikasi"
          >
            &times;
          </button>
        </div>
      )}
      
      {successMessage && (
        <div className="mb-4 bg-green-700/90 text-white px-4 py-3 rounded-lg flex items-center justify-between shadow border border-green-600">
          <span className="font-semibold">{successMessage}</span>
          <button
            onClick={() => setSuccessMessage(null)}
            className="ml-4 text-green-200 hover:text-white focus:outline-none"
            aria-label="Tutup notifikasi"
          >
            &times;
          </button>
        </div>
      )}

      <SiswaForm 
        formData={formDataEdit}
        agamaList={referensi.agama}
        loading={loading}
        onInputChange={handleInputChangeEdit}
        onCheckboxChange={handleCheckboxChangeEdit}
        onSubmit={handleUpdateSubmit}
        submitText="Simpan Perubahan"
        referensi={referensi}
        wilayahOptions={wilayahOptions}
        selectedWilayah={selectedWilayah}
        onWilayahChange={handleWilayahChange}
      />
    </div>
  );

  // Render daftar siswa
  const renderSiswaList = () => {
    if (error) {
      return (
        <div style={{ display: "flex", alignItems: "center", gap: 8, color: "#f87171" }}>
          <AlertTriangle />
          <span>Error: {error}</span>
        </div>
      );
    }

    if (loading && data.length === 0) {
      return (
        <div style={{ display: "flex", alignItems: "center", justifyContent: "center", gap: 8, fontSize: 18 }}>
          <Loader2 className="animate-spin" />
          <span>Memuat data siswa...</span>
        </div>
      );
    }

    return (
      <div className={clsx("transition-opacity", { "opacity-50": loading })}>
        <table className="w-full text-left border-collapse">
          <thead className="border-b border-gray-700">
            <tr>
              <th className="p-3">Nama</th>
              <th className="p-3">NISN</th>
              <th className="p-3 hidden sm:table-cell">NIPD</th>
              <th className="p-3 hidden sm:table-cell">Jenis Kelamin</th>
              <th className="p-3 hidden md:table-cell">Tanggal Lahir</th>
              <th className="p-3 hidden lg:table-cell">Rombel</th>
              <th className="p-3 text-right">Aksi</th>
            </tr>
          </thead>
          <tbody>
            {data.map((siswa) => (
              <tr key={siswa.peserta_didik_id} className="border-b border-gray-800 hover:bg-gray-700/50">
                <td className="p-3 font-medium" data-label="Nama">{siswa.nama}</td>
                <td className="p-3 text-gray-400" data-label="NISN">{siswa.nisn}</td>
                <td className="p-3 text-gray-400 hidden sm:table-cell" data-label="NIPD">{siswa.nipd || '-'}</td>
                <td className="p-3 text-gray-400 hidden sm:table-cell" data-label="Jenis Kelamin">{siswa.jenis_kelamin === 'L' ? 'Laki-laki' : 'Perempuan'}</td>
                <td className="p-3 text-gray-400 hidden md:table-cell" data-label="Tanggal Lahir">{new Date(siswa.tanggal_lahir).toLocaleDateString("id-ID")}</td>
                <td className="p-3 text-gray-400 hidden lg:table-cell" data-label="Rombel">{siswa.nama_rombel || '-'}</td>
                <td className="p-3 text-right">
                  <div className="flex justify-end gap-2">
                    <button onClick={() => handleEditClick(siswa)} className="p-1.5 text-blue-400 hover:bg-gray-600 rounded-md">
                      <Edit size={16} />
                    </button>
                    <button onClick={() => handleDeleteClick(siswa)} className="p-1.5 text-red-500 hover:bg-gray-600 rounded-md">
                      <Trash2 size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  };

  // Render konten berdasarkan mode
  const renderContent = () => {
    if (viewMode === 'registrasi') {
      return renderRegistrationForm();
    }
    
    if (viewMode === 'edit') {
      return renderEditForm();
    }

    if (pageTitle === 'Daftar') {
      return renderSiswaList();
    }

    return <p>Konten untuk halaman {pageTitle} akan ditampilkan di sini.</p>;
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-pink-500 mb-6">Data Siswa: {pageTitle}</h1>
      
      {/* Informasi Semester dan Tahun Ajaran */}
      {(semester || tahunAjaran) && (
        <div className="mb-6 p-4 bg-gray-800 rounded-lg border border-gray-700">
          <div className="flex items-center gap-4 text-sm">
            {tahunAjaran && (
              <div className="flex items-center gap-2">
                <span className="text-gray-400">Tahun Ajaran:</span>
                <span className="font-medium text-white">{tahunAjaran.nama}</span>
              </div>
            )}
            {semester && (
              <div className="flex items-center gap-2">
                <span className="text-gray-400">Semester:</span>
                <span className="font-medium text-white">{semester.nama}</span>
              </div>
            )}
          </div>
        </div>
      )}
      
      {/* Tombol aksi untuk mode list */}
      {pageTitle === 'Daftar' && viewMode === 'list' && (
        <div className="flex flex-col sm:flex-row gap-4 mb-6">
          <button
            onClick={() => setViewMode('registrasi')}
            className="flex items-center gap-2 px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700 transition"
          >
            <Plus size={16} />
            Registrasi Siswa Baru
          </button>
        </div>
      )}

      {/* Search dan filter untuk mode list */}
      {pageTitle === 'Daftar' && viewMode === 'list' && (
        <div className="flex flex-col sm:flex-row gap-4 mb-4">
          {/* Search Bar */}
          <div className="relative flex-grow">
            <Search size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              placeholder="Cari nama atau NISN siswa..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 bg-gray-900 border border-gray-700 rounded-md focus:ring-2 focus:ring-pink-600 focus:border-pink-600 outline-none"
            />
          </div>
          {/* Filter Rombel */}
          <div className="relative">
            <select
              value={selectedRombel}
              onChange={(e) => setSelectedRombel(e.target.value)}
              className="w-full sm:w-auto pl-3 pr-8 py-2 bg-gray-900 border border-gray-700 rounded-md focus:ring-2 focus:ring-pink-600 outline-none"
            >
              <option value="">Semua Rombel</option>
              {rombels.map(r => (
                <option key={r.rombongan_belajar_id} value={r.rombongan_belajar_id}>{r.nama}</option>
              ))}
            </select>
          </div>
        </div>
      )}

      {renderContent()}
      
      {pageTitle === 'Daftar' && viewMode === 'list' && data.length > 0 && renderPagination()}
    </div>
  );
}

// Tambahkan ini di App.css jika belum ada
/*
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
.animate-spin {
  animation: spin 1s linear infinite;
}
*/ 