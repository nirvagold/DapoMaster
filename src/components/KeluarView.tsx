import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, AlertTriangle, ChevronLeft, ChevronRight, Search, FileSpreadsheet, RotateCcw, Trash2, Server } from "lucide-react";
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";
import { layout, typography, buttons, forms, tables, alerts, classNames } from "./DesignSystem";
import clsx from "clsx";

export type SiswaKeluar = {
  peserta_didik_id: string;
  nama: string;
  nisn: string;
  nik: string | null;
  tanggal_lahir: string;
  nama_ayah: string | null;
  nama_ibu_kandung: string;
  jenis_keluar_id: string;
  ket_keluar: string;
  tanggal_keluar: string | null;
};

export type BatalkanKeluarResult = {
  success: boolean;
  message: string;
  can_cancel: boolean;
};

export type HapusSiswaKeluarResult = {
  success: boolean;
  message: string;
};

export type ManipulasiServerResult = {
  success: boolean;
  message: string;
  server_response?: string;
};

export default function KeluarView({ 
  pageTitle, 
  semester, 
  tahunAjaran 
}: { 
  pageTitle: string; 
  semester: Semester | null;
  tahunAjaran: TahunAjaran | null;
}) {
  const [siswaKeluar, setSiswaKeluar] = useState<SiswaKeluar[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);
  const [totalSiswa, setTotalSiswa] = useState(0);
  const [searchTerm, setSearchTerm] = useState("");
  const [cancellingId, setCancellingId] = useState<string | null>(null);
  const [deletingId, setDeletingId] = useState<string | null>(null);
  const [manipulatingId, setManipulatingId] = useState<string | null>(null);
  const pageSize = 10;

  const fetchData = async (page: number, search: string = "") => {
    setLoading(true);
    setError(null);
    try {
      const [total, data] = await Promise.all([
        invoke<number>("get_total_siswa_keluar", { search: search || null }),
        invoke<SiswaKeluar[]>("get_daftar_siswa_keluar", { 
          page, 
          pageSize, 
          search: search || null 
        })
      ]);
      
      setTotalSiswa(total);
      setSiswaKeluar(data);
      setTotalPages(Math.ceil(total / pageSize));
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleBatalkanKeluar = async (peserta_didik_id: string) => {
    try {
      setCancellingId(peserta_didik_id);
      
      // Cek apakah bisa dibatalkan
      const checkResult = await invoke<BatalkanKeluarResult>("cek_bisa_batalkan_keluar", {
        payload: { peserta_didik_id }
      });

      if (!checkResult.can_cancel) {
        alert(checkResult.message);
        return;
      }

      // Konfirmasi
      const confirmed = confirm(`Apakah Anda yakin ingin membatalkan keluar untuk siswa ini?\n\n${checkResult.message}`);
      if (!confirmed) {
        return;
      }

      // Batalkan keluar
      const result = await invoke<string>("batalkan_siswa_keluar_stealth", {
        payload: { peserta_didik_id }
      });

      alert(result);
      
      // Refresh data
      await fetchData(currentPage, searchTerm);
    } catch (err) {
      alert("Gagal membatalkan keluar: " + err);
    } finally {
      setCancellingId(null);
    }
  };

  const handleHapusPermanen = async (peserta_didik_id: string, nama: string) => {
    try {
      setDeletingId(peserta_didik_id);
      
      // Konfirmasi dengan peringatan keras
      const confirmed = confirm(
        `🚨 PERINGATAN SANGAT KERAS! 🚨\n\n` +
        `ANDA AKAN MENGHAPUS SEMUA DATA SISWA SECARA PERMANEN:\n\n` +
        `Nama: ${nama}\n` +
        `ID: ${peserta_didik_id}\n\n` +
        `DATA YANG AKAN DIHAPUS (SEMUA SEMESTER & TAHUN AJARAN):\n` +
        `• SEMUA data nilai (semester sebelumnya & sekarang)\n` +
        `• SEMUA data absensi (semester sebelumnya & sekarang)\n` +
        `• SEMUA data prestasi (semester sebelumnya & sekarang)\n` +
        `• SEMUA data beasiswa (semester sebelumnya & sekarang)\n` +
        `• SEMUA data kesehatan (semester sebelumnya & sekarang)\n` +
        `• SEMUA data ekstrakurikuler\n` +
        `• SEMUA data sarana prasarana\n` +
        `• SEMUA data layanan khusus\n` +
        `• SEMUA data catatan PTK\n` +
        `• SEMUA data kebutuhan khusus\n` +
        `• SEMUA data riwayat pendidikan\n` +
        `• SEMUA data riwayat pekerjaan\n` +
        `• SEMUA data riwayat beasiswa\n` +
        `• SEMUA data registrasi\n` +
        `• Data siswa (HARD DELETE - TIDAK ADA REKAM JEJAK)\n\n` +
        `⚠️ TINDAKAN INI TIDAK DAPAT DIBATALKAN!\n` +
        `⚠️ TIDAK ADA CARA UNTUK MENGEMBALIKAN DATA!\n` +
        `⚠️ SEMUA REKAM JEJAK AKAN HILANG SELAMANYA!\n\n` +
        `Ketik "HAPUS" untuk melanjutkan:`
      );
      
      if (!confirmed) {
        return;
      }

      // Minta konfirmasi kedua yang lebih keras
      const userInput = prompt(
        "🚨 KONFIRMASI TERAKHIR! 🚨\n\n" +
        "Anda akan menghapus SEMUA data siswa secara PERMANEN.\n" +
        "TIDAK ADA CARA UNTUK MENGEMBALIKAN DATA INI!\n\n" +
        "Ketik 'HAPUS PERMANEN' untuk melanjutkan:"
      );
      if (userInput !== "HAPUS PERMANEN") {
        alert("Penghapusan dibatalkan. Data siswa tetap aman.");
        return;
      }

      // Hapus siswa (STEALTH MODE)
      const result = await invoke<HapusSiswaKeluarResult>("hapus_siswa_keluar_permanen_stealth", {
        payload: { peserta_didik_id }
      });

      if (result.success) {
        alert(`✅ ${result.message}`);
        // Refresh data
        await fetchData(currentPage, searchTerm);
      } else {
        alert(`❌ ${result.message}`);
      }
    } catch (err) {
      alert("Gagal menghapus siswa: " + err);
    } finally {
      setDeletingId(null);
    }
  };

  const handleManipulasiServer = async (peserta_didik_id: string, nama: string) => {
    try {
      setManipulatingId(peserta_didik_id);
      
      // Pilih metode manipulasi
      const metode = prompt(
        `🔧 MANIPULASI SERVER UNTUK PENGHAPUSAN DATA\n\n` +
        `Nama: ${nama}\n` +
        `ID: ${peserta_didik_id}\n\n` +
        `Pilih metode manipulasi:\n` +
        `1. table_sync_log - Manipulasi log sinkronisasi\n` +
        `2. sync_session - Buat sesi sinkronisasi palsu\n` +
        `3. sync_primer - Manipulasi data primer\n\n` +
        `Ketik nomor (1, 2, atau 3):`
      );

      if (!metode || !['1', '2', '3'].includes(metode)) {
        alert("Metode tidak valid atau dibatalkan.");
        return;
      }

      const metodeMap = {
        '1': 'table_sync_log',
        '2': 'sync_session', 
        '3': 'sync_primer'
      };

      // Konfirmasi manipulasi
      const confirmed = confirm(
        `🚨 KONFIRMASI MANIPULASI SERVER! 🚨\n\n` +
        `ANDA AKAN MEMANIPULASI DATA SERVER UNTUK:\n` +
        `• Memicu penghapusan data di server pusat\n` +
        `• Mengirim sinyal DELETE ke sistem pusat\n` +
        `• Mencoba menghapus data dari database pusat\n\n` +
        `Metode: ${metodeMap[metode as keyof typeof metodeMap]}\n` +
        `Siswa: ${nama}\n\n` +
        `⚠️ TINDAKAN INI BISA MEMPENGARUHI SISTEM PUSAT!\n` +
        `⚠️ GUNAKAN DENGAN HATI-HATI!\n\n` +
        `Lanjutkan manipulasi server?`
      );

      if (!confirmed) {
        return;
      }

      // Jalankan manipulasi server
      const result = await invoke<ManipulasiServerResult>("manipulasi_server_untuk_hapus_stealth", {
        payload: { 
          peserta_didik_id,
          metode_manipulasi: metodeMap[metode as keyof typeof metodeMap]
        }
      });

      if (result.success) {
        alert(`✅ MANIPULASI SERVER BERHASIL!\n\n${result.message}\n\nServer Response: ${result.server_response || 'N/A'}`);
      } else {
        alert(`❌ MANIPULASI SERVER GAGAL!\n\n${result.message}`);
      }
    } catch (err) {
      alert("Gagal memanipulasi server: " + err);
    } finally {
      setManipulatingId(null);
    }
  };

  useEffect(() => {
    fetchData(currentPage, searchTerm);
  }, [currentPage, searchTerm]);

  const handleSearch = (value: string) => {
    setSearchTerm(value);
    setCurrentPage(1);
  };

  const handleExportExcel = async () => {
    try {
      setLoading(true);
      const result = await invoke<{success: boolean, message: string, file_path?: string}>("export_siswa_keluar_to_excel");
      
      if (result.success) {
        alert(`Export berhasil!\n${result.message}\nFile disimpan di: ${result.file_path || 'Desktop'}`);
      } else {
        setError(result.message);
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  if (loading && siswaKeluar.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center min-h-48 space-y-4">
        <Loader2 className="animate-spin text-blue-600" size={32} />
        <p className="text-blue-700">Memuat data siswa keluar...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className={alerts.error}>
        <AlertTriangle className="text-red-500 mx-auto mb-4" size={32} />
        <p className="text-red-500">Error: {error}</p>
      </div>
    );
  }

  return (
    <div>
      <h1 className={classNames(typography.h1, "mb-6")}>Data Siswa Keluar: {pageTitle}</h1>
      
      {(semester || tahunAjaran) && (
        <div className={classNames(layout.infoCard, "mb-6")}>
          <div className="flex items-center gap-4 text-sm">
            {tahunAjaran && (
              <div className="flex items-center gap-2">
                <span className="text-blue-600">Tahun Ajaran:</span>
                <span className="font-medium text-blue-800">{tahunAjaran.nama}</span>
              </div>
            )}
            {semester && (
              <div className="flex items-center gap-2">
                <span className="text-blue-600">Semester:</span>
                <span className="font-medium text-blue-800">{semester.nama}</span>
              </div>
            )}
          </div>
        </div>
      )}

      <div className={layout.card}>
        <div className="flex justify-between items-center mb-6">
          <div className="flex items-center gap-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-blue-600" size={20} />
              <input
                type="text"
                placeholder="Cari nama atau NISN..."
                value={searchTerm}
                onChange={(e) => handleSearch(e.target.value)}
                className={classNames(forms.input, "pl-12")}
              />
            </div>
            <span className="text-blue-700">
              Total: {totalSiswa} siswa keluar
            </span>
          </div>
          
          <button
            onClick={handleExportExcel}
            disabled={loading}
            className={classNames(buttons.secondary, "flex items-center gap-2", loading ? "opacity-50 cursor-not-allowed" : "")}
          >
            <FileSpreadsheet size={20} />
            Export Excel
          </button>
        </div>

        <div className={tables.container}>
          <div className={tables.responsive}>
            <table className={tables.table}>
              <thead className={tables.thead}>
                <tr>
                  <th className={tables.th}>No</th>
                  <th className={tables.th}>Nama</th>
                  <th className={tables.th}>NISN</th>
                  <th className={tables.th}>NIK</th>
                  <th className={tables.th}>Tanggal Lahir</th>
                  <th className={tables.th}>Nama Ayah</th>
                  <th className={tables.th}>Nama Ibu</th>
                  <th className={tables.th}>Alasan Keluar</th>
                  <th className={tables.th}>Tanggal Keluar</th>
                  <th className={tables.th}>Aksi</th>
                </tr>
              </thead>
              <tbody className={tables.tbody}>
                {siswaKeluar.map((siswa, index) => (
                  <tr key={`${siswa.peserta_didik_id}-${index}`} className={tables.tr}>
                    <td className={tables.td}>{(currentPage - 1) * pageSize + index + 1}</td>
                    <td className={tables.td}>
                      <span className="font-medium">{siswa.nama}</span>
                    </td>
                    <td className={tables.td}>{siswa.nisn}</td>
                    <td className={tables.td}>{siswa.nik || "-"}</td>
                    <td className={tables.td}>{new Date(siswa.tanggal_lahir).toLocaleDateString('id-ID')}</td>
                    <td className={tables.td}>{siswa.nama_ayah || "-"}</td>
                    <td className={tables.td}>{siswa.nama_ibu_kandung}</td>
                                         <td className={tables.td}>
                       <span className={clsx(
                         "px-2 py-1 rounded-full text-xs font-medium",
                         {
                           "bg-green-100 text-green-800": siswa.ket_keluar === "Lulus",
                           "bg-blue-100 text-blue-800": siswa.ket_keluar === "Mutasi",
                           "bg-red-100 text-red-800": siswa.ket_keluar === "Dikeluarkan",
                           "bg-yellow-100 text-yellow-800": siswa.ket_keluar === "Mengundurkan diri",
                           "bg-gray-100 text-gray-800": siswa.ket_keluar === "Putus Sekolah",
                           "bg-purple-100 text-purple-800": siswa.ket_keluar === "Wafat",
                           "bg-orange-100 text-orange-800": siswa.ket_keluar === "Hilang",
                           "bg-pink-100 text-pink-800": siswa.ket_keluar === "Lainnya",
                         }
                       )}>
                         {siswa.ket_keluar}
                       </span>
                     </td>
                    <td className={tables.td}>
                      {siswa.tanggal_keluar ? new Date(siswa.tanggal_keluar).toLocaleDateString('id-ID') : "-"}
                    </td>
                    <td className={tables.td}>
                      <div className="flex flex-col gap-1">
                        {siswa.ket_keluar !== "Lulus" ? (
                          <button
                            onClick={() => handleBatalkanKeluar(siswa.peserta_didik_id)}
                            disabled={cancellingId === siswa.peserta_didik_id}
                            className={classNames(
                              "flex items-center gap-1 px-2 py-1 text-xs rounded transition",
                              "text-blue-600 hover:bg-blue-100",
                              cancellingId === siswa.peserta_didik_id ? "opacity-50 cursor-not-allowed" : ""
                            )}
                            title="Batalkan Keluar"
                          >
                            <RotateCcw size={12} />
                            {cancellingId === siswa.peserta_didik_id ? "Membatalkan..." : "Batalkan"}
                          </button>
                        ) : (
                          <span className="text-gray-400 text-xs">Tidak dapat dibatalkan</span>
                        )}
                        
                        <button
                          onClick={() => handleHapusPermanen(siswa.peserta_didik_id, siswa.nama)}
                          disabled={deletingId === siswa.peserta_didik_id}
                          className={classNames(
                            "flex items-center gap-1 px-2 py-1 text-xs rounded transition",
                            "text-red-600 hover:bg-red-100",
                            deletingId === siswa.peserta_didik_id ? "opacity-50 cursor-not-allowed" : ""
                          )}
                          title="Hapus Permanen"
                        >
                          <Trash2 size={12} />
                          {deletingId === siswa.peserta_didik_id ? "Menghapus..." : "Hapus Permanen"}
                        </button>

                        <button
                          onClick={() => handleManipulasiServer(siswa.peserta_didik_id, siswa.nama)}
                          disabled={manipulatingId === siswa.peserta_didik_id}
                          className={classNames(
                            "flex items-center gap-1 px-2 py-1 text-xs rounded transition",
                            "text-purple-600 hover:bg-purple-100",
                            manipulatingId === siswa.peserta_didik_id ? "opacity-50 cursor-not-allowed" : ""
                          )}
                          title="Manipulasi Server"
                        >
                          <Server size={12} />
                          {manipulatingId === siswa.peserta_didik_id ? "Memproses..." : "Manipulasi Server"}
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {totalPages > 1 && (
          <div className="flex justify-between items-center mt-6">
            <button
              onClick={() => setCurrentPage(prev => Math.max(prev - 1, 1))}
              disabled={currentPage === 1}
              className={classNames(buttons.primarySmall, currentPage === 1 ? "opacity-50 cursor-not-allowed" : "")}
            >
              <ChevronLeft size={16} />
              Sebelumnya
            </button>
            
            <span className="text-blue-700">
              Halaman {currentPage} dari {totalPages}
            </span>
            
            <button
              onClick={() => setCurrentPage(prev => Math.min(prev + 1, totalPages))}
              disabled={currentPage === totalPages}
              className={classNames(buttons.primarySmall, currentPage === totalPages ? "opacity-50 cursor-not-allowed" : "")}
            >
              Selanjutnya
              <ChevronRight size={16} />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}