import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2, AlertTriangle, ChevronLeft, ChevronRight, Search, FileSpreadsheet, Edit, Upload } from "lucide-react";
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";

export type SiswaLulus = {
  peserta_didik_id: string;
  nama: string;
  nisn: string;
  tanggal_lahir: string;
  nama_ayah: string | null;
  nama_ibu_kandung: string;
  jenis_ijazah_id: string | null; // Ubah dari number ke string karena BigDecimal di Rust
  nama_ijazah: string | null;
  nomor: string | null;
  penandatangan: string | null;
  tanggal_tanda_tangan: string | null;
};

export type JenisIjazah = {
  jenis_ijazah_id: string; // Ubah dari number ke string karena BigDecimal di Rust
  nama: string;
};

export type ImportResult = {
  success: boolean;
  message: string;
  total_rows: number;
  success_count: number;
  error_count: number;
  errors: ImportError[];
};

export type ImportError = {
  row: number;
  field: string;
  message: string;
};

export default function LulusanView({ 
  pageTitle, 
  semester, 
  tahunAjaran 
}: { 
  pageTitle: string; 
  semester: Semester | null;
  tahunAjaran: TahunAjaran | null;
}) {
  const [currentPage, setCurrentPage] = useState(1);
  const [searchTerm, setSearchTerm] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [siswaLulus, setSiswaLulus] = useState<SiswaLulus[]>([]);
  const [totalSiswa, setTotalSiswa] = useState(0);
  const [jenisIjazah, setJenisIjazah] = useState<JenisIjazah[]>([]);
  const [isEditMode, setIsEditMode] = useState(false);
  const [editingData, setEditingData] = useState<{ [key: string]: Partial<SiswaLulus> }>({});
  const [bulkUpdateLoading, setBulkUpdateLoading] = useState(false);
  const [importLoading, setImportLoading] = useState(false);
  const [importResult, setImportResult] = useState<ImportResult | null>(null);
  const pageSize = 10;
  const totalPages = Math.ceil(totalSiswa / pageSize);

  const fetchData = async (page: number, search: string = "") => {
    try {
      setLoading(true);
      setError(null);
      
      const [total, data] = await Promise.all([
        invoke<number>("get_total_siswa_lulus", { page, pageSize, search: search || null }),
        invoke<SiswaLulus[]>("get_daftar_siswa_lulus", { page, pageSize, search: search || null })
      ]);
      
      setTotalSiswa(total);
      setSiswaLulus(data);
    } catch (err) {
      console.error("Error fetching data:", err);
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const fetchJenisIjazah = async () => {
    try {
      const data = await invoke<JenisIjazah[]>("get_all_jenis_ijazah");
      setJenisIjazah(data);
    } catch (err) {
      console.error("Error fetching jenis ijazah:", err);
    }
  };

  const handleBulkUpdate = async () => {
    try {
      setBulkUpdateLoading(true);
      const updates = Object.entries(editingData).map(([peserta_didik_id, data]) => ({
        peserta_didik_id,
        jenis_ijazah_id: data.jenis_ijazah_id || null,
        nomor: data.nomor || null,
        penandatangan: data.penandatangan || null,
        tanggal_tanda_tangan: data.tanggal_tanda_tangan || null,
      }));

      await invoke("update_bulk_ijazah", { payload: { updates } });
      
      // Refresh data
      await fetchData(currentPage, searchTerm);
      setIsEditMode(false);
      setEditingData({});
      alert("Berhasil mengupdate data ijazah!");
    } catch (err) {
      console.error("Error updating bulk:", err);
      alert("Gagal mengupdate data: " + err);
    } finally {
      setBulkUpdateLoading(false);
    }
  };

  const handleEditField = (peserta_didik_id: string, field: keyof SiswaLulus, value: string | null) => {
    setEditingData(prev => ({
      ...prev,
      [peserta_didik_id]: {
        ...prev[peserta_didik_id],
        [field]: value
      }
    }));
  };

  useEffect(() => {
    fetchData(currentPage, searchTerm);
  }, [currentPage, searchTerm]);

  useEffect(() => {
    fetchJenisIjazah();
  }, []);

  const handleSearch = (value: string) => {
    setSearchTerm(value);
    setCurrentPage(1);
  };

  const handleExportExcel = async () => {
    try {
      setLoading(true);
      const result = await invoke<{success: boolean, message: string, file_path?: string}>("export_lulusan_to_excel");
      
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

  const handleImportExcel = async () => {
    try {
      setImportLoading(true);
      setImportResult(null);
      
      const filePath = await invoke<string>("open_import_dialog");

      if (filePath) {
        try {
          const result = await invoke<ImportResult>("import_lulusan_from_excel", { filePath });
          setImportResult(result);
          
          if (result.success) {
            alert(`Import berhasil!\n${result.message}`);
            // Refresh data setelah import berhasil
            await fetchData(currentPage, searchTerm);
          } else {
            alert(`Import selesai dengan beberapa error:\n${result.message}\n\nTotal baris: ${result.total_rows}\nBerhasil: ${result.success_count}\nError: ${result.error_count}`);
          }
        } catch (err) {
          setError(err as string);
        } finally {
          setImportLoading(false);
        }
      }
    } catch (err) {
      setError(err as string);
      setImportLoading(false);
    }
  };

  if (loading && siswaLulus.length === 0) {
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
      <h1 className="text-3xl font-bold text-pink-500 mb-6">Data Lulusan: {pageTitle}</h1>
      
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

      {pageTitle.toLowerCase() === "import" ? (
        // Halaman Import
        <div className="bg-gray-800 rounded-lg p-6">
          <div className="text-center mb-8">
            <Upload className="mx-auto mb-4 text-orange-500" size={48} />
            <h2 className="text-2xl font-bold text-white mb-2">Import Data Lulusan</h2>
            <p className="text-gray-400 mb-6">
              Upload file Excel yang berisi data ijazah lulusan. File harus sesuai dengan template yang telah disediakan.
            </p>
            
            <div className="bg-blue-900/20 border border-blue-600 rounded-lg p-4 mb-6 text-left">
              <h3 className="text-blue-400 font-semibold mb-2">Petunjuk Import:</h3>
              <ul className="text-gray-300 text-sm space-y-1">
                <li>• Kolom Nama, NISN, dan Nama Ibu adalah kunci untuk mencocokkan data</li>
                <li>• Sistem akan otomatis mencocokkan data berdasarkan ketiga kolom tersebut</li>
                <li>• Jika ada perbedaan nama, sistem akan memberikan peringatan</li>
                <li>• Hanya kolom Jenis Ijazah, Nomor Ijazah, Penandatangan, dan Tanggal Tanda Tangan yang dapat diubah</li>
                <li>• Format tanggal yang didukung: YYYY-MM-DD, DD/MM/YYYY, DD-MM-YYYY</li>
              </ul>
            </div>

            <div className="flex justify-center gap-4">
              <button
                onClick={handleImportExcel}
                disabled={importLoading}
                className="flex items-center gap-2 px-6 py-3 bg-orange-600 text-white font-semibold rounded-md hover:bg-orange-700 transition disabled:bg-gray-600"
              >
                <Upload size={20} />
                {importLoading ? "Importing..." : "Pilih File Excel"}
              </button>
              
              <button
                onClick={handleExportExcel}
                disabled={loading}
                className="flex items-center gap-2 px-6 py-3 bg-green-600 text-white font-semibold rounded-md hover:bg-green-700 transition disabled:bg-gray-600"
              >
                <FileSpreadsheet size={20} />
                Download Template
              </button>
            </div>
          </div>

          {importResult && (
            <div className={`p-4 rounded-lg border ${importResult.success ? 'bg-green-900/20 border-green-600' : 'bg-orange-900/20 border-orange-600'}`}>
              <h3 className={`font-semibold mb-2 ${importResult.success ? 'text-green-400' : 'text-orange-400'}`}>
                Hasil Import
              </h3>
              <p className="text-gray-300 mb-2">{importResult.message}</p>
              <div className="text-sm text-gray-400 mb-3">
                <p>Total baris: {importResult.total_rows}</p>
                <p>Berhasil: {importResult.success_count}</p>
                <p>Error: {importResult.error_count}</p>
              </div>
              {importResult.errors.length > 0 && (
                <div>
                  <h4 className="text-orange-400 font-medium mb-2">Detail Error:</h4>
                  <div className="max-h-60 overflow-y-auto bg-gray-900 p-3 rounded">
                    {importResult.errors.map((error, index) => (
                      <div key={index} className="text-sm text-gray-300 mb-1">
                        <span className="text-orange-400">Baris {error.row}:</span> {error.field} - {error.message}
                      </div>
                    ))}
                  </div>
                </div>
              )}
              <button
                onClick={() => setImportResult(null)}
                className="mt-4 px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition"
              >
                Tutup
              </button>
            </div>
          )}
        </div>
      ) : (
        // Halaman Daftar Lulusan (existing code)
        <div className="bg-gray-800 rounded-lg p-6">
          {importResult && (
            <div className={`mb-6 p-4 rounded-lg border ${importResult.success ? 'bg-green-900/20 border-green-600' : 'bg-orange-900/20 border-orange-600'}`}>
              <h3 className={`font-semibold mb-2 ${importResult.success ? 'text-green-400' : 'text-orange-400'}`}>
                Hasil Import
              </h3>
              <p className="text-gray-300 mb-2">{importResult.message}</p>
              <div className="text-sm text-gray-400">
                <p>Total baris: {importResult.total_rows}</p>
                <p>Berhasil: {importResult.success_count}</p>
                <p>Error: {importResult.error_count}</p>
              </div>
              {importResult.errors.length > 0 && (
                <div className="mt-3">
                  <h4 className="text-orange-400 font-medium mb-2">Detail Error:</h4>
                  <div className="max-h-40 overflow-y-auto">
                    {importResult.errors.map((error, index) => (
                      <div key={index} className="text-sm text-gray-300 mb-1">
                        <span className="text-orange-400">Baris {error.row}:</span> {error.field} - {error.message}
                      </div>
                    ))}
                  </div>
                </div>
              )}
              <button
                onClick={() => setImportResult(null)}
                className="mt-3 px-3 py-1 bg-gray-600 text-white text-sm rounded hover:bg-gray-700 transition"
              >
                Tutup
              </button>
            </div>
          )}

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
                Total: {totalSiswa} lulusan
              </span>
            </div>
            
            <div className="flex items-center gap-2">
              {!isEditMode ? (
                <>
                  <button
                    onClick={() => setIsEditMode(true)}
                    className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white font-semibold rounded-md hover:bg-blue-700 transition"
                  >
                    <Edit size={20} />
                    Edit Bulk
                  </button>
                  <button
                    onClick={handleImportExcel}
                    disabled={importLoading}
                    className="flex items-center gap-2 px-4 py-2 bg-orange-600 text-white font-semibold rounded-md hover:bg-orange-700 transition disabled:bg-gray-600"
                  >
                    <Upload size={20} />
                    {importLoading ? "Importing..." : "Import Excel"}
                  </button>
                  <button
                    onClick={handleExportExcel}
                    disabled={loading}
                    className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white font-semibold rounded-md hover:bg-green-700 transition disabled:bg-gray-600"
                  >
                    <FileSpreadsheet size={20} />
                    Export Template
                  </button>
                </>
              ) : (
                <>
                  <button
                    onClick={handleBulkUpdate}
                    disabled={bulkUpdateLoading}
                    className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white font-semibold rounded-md hover:bg-green-700 transition disabled:bg-gray-600"
                  >
                    {bulkUpdateLoading ? "Menyimpan..." : "Simpan Perubahan"}
                  </button>
                  <button
                    onClick={() => {
                      setIsEditMode(false);
                      setEditingData({});
                    }}
                    className="flex items-center gap-2 px-4 py-2 bg-gray-600 text-white font-semibold rounded-md hover:bg-gray-700 transition"
                  >
                    Batal
                  </button>
                </>
              )}
            </div>
          </div>

          <div className="overflow-x-auto">
            <table className="w-full text-left">
              <thead className="bg-gray-700 text-gray-300">
                <tr>
                  <th className="px-4 py-3 font-semibold">No</th>
                  <th className="px-4 py-3 font-semibold">Nama</th>
                  <th className="px-4 py-3 font-semibold">NISN</th>
                  <th className="px-4 py-3 font-semibold">Tanggal Lahir</th>
                  <th className="px-4 py-3 font-semibold">Nama Ayah</th>
                  <th className="px-4 py-3 font-semibold">Nama Ibu</th>
                  <th className="px-4 py-3 font-semibold">Jenis Ijazah</th>
                  <th className="px-4 py-3 font-semibold">Nomor Ijazah</th>
                  <th className="px-4 py-3 font-semibold">Penandatangan</th>
                  <th className="px-4 py-3 font-semibold">Tanggal Tanda Tangan</th>
                </tr>
              </thead>
              <tbody className="text-gray-300">
                {siswaLulus.map((siswa, index) => (
                  <tr key={siswa.peserta_didik_id} className="border-b border-gray-700 hover:bg-gray-700">
                    <td className="px-4 py-3">{(currentPage - 1) * pageSize + index + 1}</td>
                    <td className="px-4 py-3 font-medium">{siswa.nama}</td>
                    <td className="px-4 py-3">{siswa.nisn}</td>
                    <td className="px-4 py-3">{new Date(siswa.tanggal_lahir).toLocaleDateString('id-ID')}</td>
                    <td className="px-4 py-3">{siswa.nama_ayah || "-"}</td>
                    <td className="px-4 py-3">{siswa.nama_ibu_kandung}</td>
                    <td className="px-4 py-3">
                      {isEditMode ? (
                        <select
                          value={editingData[siswa.peserta_didik_id]?.jenis_ijazah_id || siswa.jenis_ijazah_id || ""}
                          onChange={(e) => handleEditField(siswa.peserta_didik_id, "jenis_ijazah_id", e.target.value)}
                          className="w-full px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
                        >
                          <option value="">Pilih Jenis Ijazah</option>
                          {jenisIjazah.map((ijazah) => (
                            <option key={ijazah.jenis_ijazah_id} value={ijazah.jenis_ijazah_id}>
                              {ijazah.nama}
                            </option>
                          ))}
                        </select>
                      ) : (
                        siswa.nama_ijazah || "-"
                      )}
                    </td>
                    <td className="px-4 py-3">
                      {isEditMode ? (
                        <input
                          type="text"
                          value={editingData[siswa.peserta_didik_id]?.nomor || siswa.nomor || ""}
                          onChange={(e) => handleEditField(siswa.peserta_didik_id, "nomor", e.target.value)}
                          className="w-full px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
                          placeholder="Nomor Ijazah"
                        />
                      ) : (
                        siswa.nomor || "-"
                      )}
                    </td>
                    <td className="px-4 py-3">
                      {isEditMode ? (
                        <input
                          type="text"
                          value={editingData[siswa.peserta_didik_id]?.penandatangan || siswa.penandatangan || ""}
                          onChange={(e) => handleEditField(siswa.peserta_didik_id, "penandatangan", e.target.value)}
                          className="w-full px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
                          placeholder="Penandatangan"
                        />
                      ) : (
                        siswa.penandatangan || "-"
                      )}
                    </td>
                    <td className="px-4 py-3">
                      {isEditMode ? (
                        <input
                          type="date"
                          value={editingData[siswa.peserta_didik_id]?.tanggal_tanda_tangan || siswa.tanggal_tanda_tangan || ""}
                          onChange={(e) => handleEditField(siswa.peserta_didik_id, "tanggal_tanda_tangan", e.target.value)}
                          className="w-full px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
                        />
                      ) : (
                        siswa.tanggal_tanda_tangan ? new Date(siswa.tanggal_tanda_tangan).toLocaleDateString('id-ID') : "-"
                      )}
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
      )}
    </div>
  );
}