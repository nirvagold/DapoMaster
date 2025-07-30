import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, AlertTriangle, ChevronLeft, ChevronRight, Search, FileSpreadsheet } from "lucide-react";
import clsx from "clsx";
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";

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
      <div className="flex items-center justify-center h-64">
        <Loader2 className="animate-spin text-pink-500" size={32} />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <AlertTriangle className="text-red-500 mx-auto mb-4" size={32} />
          <p className="text-red-500">Error: {error}</p>
        </div>
      </div>
    );
  }

  return (
    <div>
      <h1 className="text-3xl font-bold text-pink-500 mb-6">Data Siswa Keluar: {pageTitle}</h1>
      
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

      <div className="bg-gray-800 rounded-lg p-6">
        <div className="flex justify-between items-center mb-6">
          <div className="flex items-center gap-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={20} />
              <input
                type="text"
                placeholder="Cari nama atau NISN..."
                value={searchTerm}
                onChange={(e) => handleSearch(e.target.value)}
                className="pl-10 pr-4 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-pink-600 outline-none"
              />
            </div>
            <span className="text-gray-400">
              Total: {totalSiswa} siswa keluar
            </span>
          </div>
          
          <button
            onClick={handleExportExcel}
            disabled={loading}
            className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white font-semibold rounded-md hover:bg-green-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <FileSpreadsheet size={20} />
            Export Excel
          </button>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-left">
            <thead className="bg-gray-700 text-gray-300">
              <tr>
                <th className="px-4 py-3 font-semibold">No</th>
                <th className="px-4 py-3 font-semibold">Nama</th>
                <th className="px-4 py-3 font-semibold">NISN</th>
                <th className="px-4 py-3 font-semibold">NIK</th>
                <th className="px-4 py-3 font-semibold">Tanggal Lahir</th>
                <th className="px-4 py-3 font-semibold">Nama Ayah</th>
                <th className="px-4 py-3 font-semibold">Nama Ibu</th>
                <th className="px-4 py-3 font-semibold">Alasan Keluar</th>
                <th className="px-4 py-3 font-semibold">Tanggal Keluar</th>
              </tr>
            </thead>
            <tbody className="text-gray-300">
              {siswaKeluar.map((siswa, index) => (
                <tr key={siswa.peserta_didik_id} className="border-b border-gray-700 hover:bg-gray-700">
                  <td className="px-4 py-3">{(currentPage - 1) * pageSize + index + 1}</td>
                  <td className="px-4 py-3 font-medium">{siswa.nama}</td>
                  <td className="px-4 py-3">{siswa.nisn}</td>
                  <td className="px-4 py-3">{siswa.nik || "-"}</td>
                  <td className="px-4 py-3">{new Date(siswa.tanggal_lahir).toLocaleDateString('id-ID')}</td>
                  <td className="px-4 py-3">{siswa.nama_ayah || "-"}</td>
                  <td className="px-4 py-3">{siswa.nama_ibu_kandung}</td>
                  <td className="px-4 py-3">
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
                  <td className="px-4 py-3">
                    {siswa.tanggal_keluar ? new Date(siswa.tanggal_keluar).toLocaleDateString('id-ID') : "-"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {totalPages > 1 && (
          <div className="flex justify-between items-center mt-6">
            <button
              onClick={() => setCurrentPage(prev => Math.max(prev - 1, 1))}
              disabled={currentPage === 1}
              className="flex items-center gap-2 px-4 py-2 bg-gray-700 text-white rounded-md hover:bg-gray-600 disabled:bg-gray-800 disabled:cursor-not-allowed transition"
            >
              <ChevronLeft size={20} />
              Sebelumnya
            </button>
            
            <span className="text-gray-400">
              Halaman {currentPage} dari {totalPages}
            </span>
            
            <button
              onClick={() => setCurrentPage(prev => Math.min(prev + 1, totalPages))}
              disabled={currentPage === totalPages}
              className="flex items-center gap-2 px-4 py-2 bg-gray-700 text-white rounded-md hover:bg-gray-600 disabled:bg-gray-800 disabled:cursor-not-allowed transition"
            >
              Selanjutnya
              <ChevronRight size={20} />
            </button>
          </div>
        )}
      </div>
    </div>
  );
}