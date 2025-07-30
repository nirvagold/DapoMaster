import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, AlertTriangle, ChevronLeft, ChevronRight, Search, Edit, Trash2 } from "lucide-react";
import clsx from "clsx";
import Modal from "./Modal"; // Impor komponen Modal
import SiswaForm from "./SiswaForm";
import { SiswaFormData } from "./SiswaForm"; // Impor tipe
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

// Tipe data baru untuk Rombel
type RombonganBelajar = {
  rombongan_belajar_id: string;
  nama: string;
};

// Tipe data ini harus cocok dengan struct di Rust
type PesertaDidik = {
  peserta_didik_id: string;
  nama: string;
  jenis_kelamin: string;
  nisn: string;
  nik?: string;
  tempat_lahir?: string;
  tanggal_lahir: string; // Di frontend, tanggal jadi string
  agama_id: number; // Diperbaiki
};

// Pindahkan definisi tipe ini agar bisa di-export
export type Agama = {
  agama_id: number;
  nama: string;
};

// Tipe data referensi baru
export type JenisPendaftaran = {
  jenis_pendaftaran_id: number;
  nama: string; // Diperbaiki
};
export type Hobby = { id_hobby: number; nm_hobby: string; };
export type Cita = { id_cita: number; nm_cita: string; };

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
  const debouncedSearchTerm = useDebounce(searchTerm, 500); // 500ms delay
  const [rombels, setRombels] = useState<RombonganBelajar[]>([]);
  const [selectedRombel, setSelectedRombel] = useState<string>("");
  const [agamaList, setAgamaList] = useState<Agama[]>([]);
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
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingSiswa, setEditingSiswa] = useState<PesertaDidik | null>(null);
  const [referensi, setReferensi] = useState({
    jenisPendaftaran: [] as JenisPendaftaran[],
    hobbies: [] as Hobby[],
    citas: [] as Cita[],
    agama: [] as Agama[],
  });
  
  // Saat klik edit, isi formDataEdit saja
  const handleEditClick = (siswa: PesertaDidik) => {
    setEditingSiswa(siswa);
    setFormDataEdit({
      nama: siswa.nama,
      nisn: siswa.nisn || "",
      jenis_kelamin: siswa.jenis_kelamin,
      tempat_lahir: siswa.tempat_lahir || "",
      tanggal_lahir: siswa.tanggal_lahir.split('T')[0],
      agama_id: String(siswa.agama_id),
      a_pernah_paud: false,
      a_pernah_tk: false,
      nipd: siswa.nik || "",
      tanggal_masuk_sekolah: siswa.tanggal_lahir.split('T')[0],
      jenis_pendaftaran_id: "",
      id_hobby: "",
      id_cita: "",
      alamat_jalan: "",
      desa_kelurahan: "",
      kode_wilayah: "",
      nama_ibu_kandung: "",
      kewarganegaraan: "ID",
      nik: siswa.nik || "",
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
    setIsModalOpen(true);
  };

  const handleDeleteClick = async (siswa: PesertaDidik) => {
    if (window.confirm(`Apakah Anda yakin ingin menghapus siswa bernama ${siswa.nama}?`)) {
      try {
        await invoke("delete_siswa", { pesertaDidikId: siswa.peserta_didik_id });
        // Refresh data setelah hapus
        fetchData(currentPage, debouncedSearchTerm, selectedRombel);
      } catch (err) {
        setError(String(err));
      }
    }
  };
  
  // Handler submit form edit
  const handleUpdateSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingSiswa) return;
    setLoading(true);
    setError(null);
    setSuccessMessage(null);
    const payload = {
      ...formDataEdit,
      agama_id: parseInt(formDataEdit.agama_id, 10),
    };
    try {
      const result = await invoke<string>("update_siswa", { payload, pesertaDidikId: editingSiswa.peserta_didik_id });
      setSuccessMessage(result);
      resetFormEdit();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
    setIsModalOpen(false);
    setEditingSiswa(null);
    fetchData(currentPage, debouncedSearchTerm, selectedRombel);
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
        const [jenisPendaftaran, hobbies, citas, agama] = await Promise.all([
          invoke<JenisPendaftaran[]>("get_all_jenis_pendaftaran"),
          invoke<Hobby[]>("get_all_hobby"),
          invoke<Cita[]>("get_all_cita"),
          invoke<Agama[]>("get_all_agama"),
        ]);
        setReferensi({ jenisPendaftaran, hobbies, citas, agama });
      } catch (err) {
        setError(`Gagal memuat data referensi: ${err}`);
      }
    };
    fetchReferensi();
  }, []);

  // Efek utama untuk fetch data
  useEffect(() => {
    if (pageTitle === "Daftar") {
      setCurrentPage(1);
      fetchData(1, debouncedSearchTerm, selectedRombel);
    }
  }, [pageTitle, debouncedSearchTerm, selectedRombel]);

  // Efek untuk paginasi
  useEffect(() => {
    if (pageTitle === "Daftar") {
      fetchData(currentPage, debouncedSearchTerm, selectedRombel);
    }
  }, [currentPage]);
  
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
    setLoading(true);
    setError(null);
    setSuccessMessage(null);
    const payload = {
      ...formDataRegistrasi,
      agama_id: parseInt(formDataRegistrasi.agama_id, 10),
      sekolah_id: user?.sekolah_id,
    };
    console.log('[REGISTRASI] Mulai proses registrasi siswa:', payload);
    try {
      const result = await invoke<string>("registrasi_siswa_baru", { payload });
      console.log('[REGISTRASI] Registrasi siswa berhasil:', result);
      setSuccessMessage(result);
      resetFormRegistrasi();
    } catch (err) {
      console.error('[REGISTRASI] Registrasi siswa gagal:', err);
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
    if (pageTitle === "Registrasi") {
      invoke<WilayahReferensi[]>("get_wilayah_by_level_and_parent", { level: 1, parent: null })
        .then((provinsi) => setWilayahOptions((prev) => ({ ...prev, provinsi })))
        .catch((err) => setError(`Gagal memuat provinsi: ${err}`));
    }
  }, [pageTitle]);

  // Fetch kabupaten saat provinsi berubah
  useEffect(() => {
    console.log("selectedWilayah.provinsi berubah:", selectedWilayah.provinsi);
    if (selectedWilayah.provinsi) {
      // Kirim kode parent apa adanya, tanpa padEnd
      const parentKode = selectedWilayah.provinsi.trim();
      console.log("fetch kabupaten parent:", parentKode);
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
    console.log("handleWilayahChange", level, kode);
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

  // Handler perubahan desa/kelurahan (dropdown terakhir)
  // const handleInputChangeWilayah = (e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement>) => {
  //   if (e.target.name === 'kode_wilayah') {
  //     const selectedDesa = wilayahOptions.desa.find(d => d.kode_wilayah === e.target.value);
  //     setFormDataRegistrasi((prev) => ({ ...prev, kode_wilayah: e.target.value, desa_kelurahan: selectedDesa ? selectedDesa.nama : '' }));
  //     setFormDataEdit((prev) => ({ ...prev, kode_wilayah: e.target.value, desa_kelurahan: selectedDesa ? selectedDesa.nama : '' }));
  //   } else {
  //     handleInputChangeRegistrasi(e);
  //     handleInputChangeEdit(e);
  //   }
  // };

  // Handler untuk form registrasi
  const handleInputChangeWilayahRegistrasi = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormDataRegistrasi((prev: SiswaFormData) => ({ ...prev, [name]: value }));
  };

  // Pada renderRegistrationForm, gunakan formDataRegistrasi dan handler registrasi
  const renderRegistrationForm = () => (
    <div>
      {/* Notifikasi error */}
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
      {/* Notifikasi sukses */}
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

  const renderContent = () => {
    if (pageTitle === 'Registrasi') {
      return renderRegistrationForm();
    }
    if (pageTitle !== "Daftar") {
      return <p>Konten untuk halaman {pageTitle} akan ditampilkan di sini.</p>;
    }

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
              <th className="p-3 hidden sm:table-cell">Jenis Kelamin</th>
              <th className="p-3 hidden md:table-cell">Tanggal Lahir</th>
              <th className="p-3 text-right">Aksi</th>
            </tr>
          </thead>
          <tbody>
            {data.map((siswa) => (
              <tr key={siswa.peserta_didik_id} className="border-b border-gray-800 hover:bg-gray-700/50">
                <td className="p-3 font-medium" data-label="Nama">{siswa.nama}</td>
                <td className="p-3 text-gray-400" data-label="NISN">{siswa.nisn}</td>
                <td className="p-3 text-gray-400 hidden sm:table-cell" data-label="Jenis Kelamin">{siswa.jenis_kelamin === 'L' ? 'Laki-laki' : 'Perempuan'}</td>
                <td className="p-3 text-gray-400 hidden md:table-cell" data-label="Tanggal Lahir">{new Date(siswa.tanggal_lahir).toLocaleDateString("id-ID")}</td>
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
      
      {pageTitle === 'Daftar' && (
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
      {pageTitle === 'Daftar' && data.length > 0 && renderPagination()}
      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Edit Data Siswa">
        {editingSiswa && (
          <SiswaForm
            formData={formDataEdit}
            agamaList={agamaList}
            loading={loading}
            onInputChange={handleInputChangeEdit}
            onCheckboxChange={handleCheckboxChangeEdit}
            onSubmit={handleUpdateSubmit}
            submitText="Simpan Perubahan"
            referensi={referensi}
            wilayahOptions={{ provinsi: [], kabupaten: [], kecamatan: [], desa: [] }}
            selectedWilayah={{ provinsi: '', kabupaten: '', kecamatan: '' }}
            onWilayahChange={() => {}}
          />
        )}
      </Modal>
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