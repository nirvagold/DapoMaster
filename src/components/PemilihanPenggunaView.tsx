import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type Pengguna = {
  pengguna_id: string; // Sesuaikan dengan struct di Rust
  nama: string;
  sekolah_id: string;
};

export type Semester = {
  semester_id: string;
  nama: string;
  tahun_ajaran_id: string;
};

export type TahunAjaran = {
  tahun_ajaran_id: string;
  nama: string;
};

export default function PemilihanPenggunaView({ onLanjut }: { onLanjut: (user: Pengguna, semester: Semester | null, tahunAjaran: TahunAjaran | null) => void }) {
  const [pengguna, setPengguna] = useState<Pengguna[]>([]);
  const [semesterList, setSemesterList] = useState<Semester[]>([]);
  const [tahunAjaranList, setTahunAjaranList] = useState<TahunAjaran[]>([]);
  const [selected, setSelected] = useState<string>("");
  const [selectedSemester, setSelectedSemester] = useState<string>("");
  const [selectedTahunAjaran, setSelectedTahunAjaran] = useState<string>("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<Pengguna[]>("ambil_semua_pengguna")
      .then(setPengguna)
      .catch(err => setError(String(err)));
  }, []);

  useEffect(() => {
    invoke<Semester[]>("get_all_semester")
      .then(setSemesterList)
      .catch(err => setError(String(err)));
  }, []);

  useEffect(() => {
    invoke<TahunAjaran[]>("get_all_tahun_ajaran")
      .then(setTahunAjaranList)
      .catch(err => setError(String(err)));
  }, []);

  if (error) {
    return <div className="text-red-500 text-center mt-10">Error: {error}</div>;
  }

  const handleLanjutkan = () => {
    const user = pengguna.find(u => u.pengguna_id === selected);
    const semester = semesterList.find(s => s.semester_id === selectedSemester) || null;
    const tahunAjaran = tahunAjaranList.find(t => t.tahun_ajaran_id === selectedTahunAjaran) || null;
    
    if (user) {
      onLanjut(user, semester, tahunAjaran);
    }
  };

  return (
    <div className="flex flex-col items-center gap-4">
      <h1 className="text-3xl font-bold text-pink-500 mb-4">Pilih Pengguna & Semester</h1>
      
      <div className="space-y-4 w-full max-w-md">
        <div>
          <label className="block text-sm font-semibold text-gray-300 mb-2">Pilih Pengguna</label>
          <select
            value={selected}
            onChange={e => setSelected(e.target.value)}
            className="w-full p-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-pink-600 outline-none"
          >
            <option value="">-- Pilih Pengguna --</option>
            {pengguna.map(u => (
              <option key={u.pengguna_id} value={u.pengguna_id}>{u.nama}</option>
            ))}
          </select>
        </div>

        <div>
          <label className="block text-sm font-semibold text-gray-300 mb-2">Pilih Tahun Ajaran</label>
          <select
            value={selectedTahunAjaran}
            onChange={e => setSelectedTahunAjaran(e.target.value)}
            className="w-full p-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-pink-600 outline-none"
          >
            <option value="">-- Pilih Tahun Ajaran --</option>
            {tahunAjaranList.map(t => (
              <option key={t.tahun_ajaran_id} value={t.tahun_ajaran_id}>{t.nama}</option>
            ))}
          </select>
        </div>

        <div>
          <label className="block text-sm font-semibold text-gray-300 mb-2">Pilih Semester</label>
          <select
            value={selectedSemester}
            onChange={e => setSelectedSemester(e.target.value)}
            className="w-full p-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-pink-600 outline-none"
          >
            <option value="">-- Pilih Semester --</option>
            {semesterList
              .filter(s => !selectedTahunAjaran || s.tahun_ajaran_id === selectedTahunAjaran)
              .map(s => (
                <option key={s.semester_id} value={s.semester_id}>{s.nama}</option>
              ))}
          </select>
        </div>
      </div>

      <button
        disabled={!selected}
        onClick={handleLanjutkan}
        className="px-6 py-2 bg-pink-600 text-white font-semibold rounded-md hover:bg-pink-700 disabled:bg-pink-800 disabled:cursor-not-allowed transition"
      >
        Lanjutkan
      </button>
    </div>
  );
} 