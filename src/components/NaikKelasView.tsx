import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Loader2, AlertTriangle, ArrowUp, Calendar, Users, GraduationCap } from 'lucide-react';

interface Semester {
  semester_id: string;
  nama: string;
  tahun_ajaran_id: string;
  semester: string;
}

interface SiswaRombel {
  peserta_didik_id: string; // UUID akan dikonversi ke string oleh serde
  nama: string;
  nisn: string;
  nama_rombel: string;
  tingkat_pendidikan_id: string; // BigDecimal akan dikonversi ke string oleh serde
  tingkat_pendidikan_nama: string;
}

const NaikKelasView: React.FC = () => {
  const [semesterList, setSemesterList] = useState<Semester[]>([]);
  const [selectedSemesterSebelumnya, setSelectedSemesterSebelumnya] = useState<string>('');
  const [siswaList, setSiswaList] = useState<SiswaRombel[]>([]);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string>('');
  const [messageType, setMessageType] = useState<'success' | 'error' | ''>('');

  // Load daftar semester saat komponen dimount
  useEffect(() => {
    loadDaftarSemester();
  }, []);

  const loadDaftarSemester = async () => {
    try {
      setLoading(true);
      const result = await invoke<Semester[]>('get_daftar_semester');
      setSemesterList(result);
      setMessage('Berhasil memuat daftar semester');
      setMessageType('success');
    } catch (error) {
      setMessage(`Error: ${error}`);
      setMessageType('error');
    } finally {
      setLoading(false);
    }
  };

  const loadSiswaNaikKelas = async () => {
    if (!selectedSemesterSebelumnya) {
      setMessage('Pilih semester sebelumnya terlebih dahulu');
      setMessageType('error');
      return;
    }

    try {
      setLoading(true);
      const result = await invoke<SiswaRombel[]>('get_siswa_naik_kelas', {
        semesterSebelumnya: selectedSemesterSebelumnya
      });
      setSiswaList(result);
      setMessage(`Berhasil memuat ${result.length} siswa untuk naik kelas`);
      setMessageType('success');
    } catch (error) {
      setMessage(`Error: ${error}`);
      setMessageType('error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-pink-500 mb-6 flex items-center gap-3">
        <ArrowUp className="w-8 h-8" />
        Fitur Naik Kelas
      </h1>
      
      {/* Pesan */}
      {message && (
        <div className={`mb-6 p-4 rounded-lg flex items-center gap-3 ${
          messageType === 'error' 
            ? 'bg-red-800 text-white border border-red-700' 
            : 'bg-green-800 text-white border border-green-700'
        }`}>
          {messageType === 'error' ? <AlertTriangle size={20} /> : <GraduationCap size={20} />}
          <span>{message}</span>
        </div>
      )}

      {/* Form Pilihan Semester */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700 mb-6">
        <h2 className="text-lg font-semibold text-pink-400 mb-4 flex items-center gap-2">
          <Calendar size={20} />
          Pilihan Semester
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-400 mb-2">
              Semester Sebelumnya
            </label>
            <select
              value={selectedSemesterSebelumnya}
              onChange={(e) => setSelectedSemesterSebelumnya(e.target.value)}
              className="w-full p-3 bg-gray-900 border border-gray-700 rounded-md focus:ring-2 focus:ring-pink-600 focus:border-pink-600 outline-none text-white"
            >
              <option value="">Pilih Semester</option>
              {semesterList.map((semester) => (
                <option key={semester.semester_id} value={semester.semester_id}>
                  {semester.nama}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="mt-6">
          <button
            onClick={loadSiswaNaikKelas}
            disabled={loading || !selectedSemesterSebelumnya}
            className="flex items-center gap-2 px-6 py-3 bg-pink-600 text-white rounded-lg hover:bg-pink-700 disabled:bg-gray-600 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Loading...
              </>
            ) : (
              <>
                <Users size={20} />
                Muat Data Siswa
              </>
            )}
          </button>
        </div>
      </div>

      {/* Data Siswa */}
      {siswaList.length > 0 && (
        <div className="bg-gray-800 p-6 rounded-lg border border-gray-700 mb-6">
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-lg font-semibold text-pink-400 flex items-center gap-2">
              <GraduationCap size={20} />
              Data Siswa untuk Naik Kelas ({siswaList.length} siswa)
            </h2>
          </div>
          
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-700">
              <thead className="bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    No
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    Nama Siswa
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    NISN
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    Rombel Sebelumnya
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    Rombel Baru
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    Tingkat Sebelumnya
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                    Tingkat Baru
                  </th>
                </tr>
              </thead>
              <tbody className="bg-gray-800 divide-y divide-gray-700">
                {siswaList.map((siswa, index) => {
                  const tingkatPendidikanId = parseInt(siswa.tingkat_pendidikan_id);
                  const tingkatBaru = tingkatPendidikanId < 6 ? tingkatPendidikanId + 1 : tingkatPendidikanId;
                  const rombelBaru = `Kelas ${tingkatBaru}${siswa.nama_rombel.slice(-1)}`;
                  
                  return (
                    <tr key={siswa.peserta_didik_id} className="hover:bg-gray-700 transition-colors">
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                        {index + 1}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-white font-medium">
                        {siswa.nama}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                        {siswa.nisn}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                        {siswa.nama_rombel}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-pink-400 font-medium">
                        {rombelBaru}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                        {siswa.tingkat_pendidikan_nama}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-pink-400 font-medium">
                        Kelas {tingkatBaru}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Informasi */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
        <h3 className="text-lg font-semibold text-pink-400 mb-4 flex items-center gap-2">
          <AlertTriangle size={20} />
          Informasi Fitur Naik Kelas
        </h3>
        <div className="space-y-2 text-gray-300">
          <div className="flex items-start gap-2">
            <span className="text-pink-400 mt-1">•</span>
            <span>Fitur ini menampilkan siswa yang akan naik kelas dari semester sebelumnya</span>
          </div>
          <div className="flex items-start gap-2">
            <span className="text-pink-400 mt-1">•</span>
            <span>Hanya menampilkan siswa dari kelas 1-5 (SD)</span>
          </div>
          <div className="flex items-start gap-2">
            <span className="text-pink-400 mt-1">•</span>
            <span>Rombel baru akan otomatis dihitung berdasarkan tingkat pendidikan</span>
          </div>
          <div className="flex items-start gap-2">
            <span className="text-pink-400 mt-1">•</span>
            <span>Untuk memproses naik kelas, gunakan fitur ini sebagai referensi</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NaikKelasView; 