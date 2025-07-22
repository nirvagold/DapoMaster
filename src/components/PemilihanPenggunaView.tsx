import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type Pengguna = {
  pengguna_id: string; // Sesuaikan dengan struct di Rust
  nama: string;
};

export default function PemilihanPenggunaView({ onLanjut }: { onLanjut: (user: Pengguna) => void }) {
  const [pengguna, setPengguna] = useState<Pengguna[]>([]);
  const [selected, setSelected] = useState<string>("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<Pengguna[]>("ambil_semua_pengguna")
      .then(setPengguna)
      .catch(err => setError(String(err)));
  }, []);

  if (error) {
    return <div className="text-red-500 text-center mt-10">Error: {error}</div>;
  }

  return (
    <div className="flex flex-col items-center gap-4">
      <h1 className="text-3xl font-bold text-pink-500 mb-4">Pilih Pengguna</h1>
      <select
        value={selected}
        onChange={e => setSelected(e.target.value)}
        className="min-w-[250px] p-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-pink-600 outline-none"
      >
        <option value="">-- Pilih Pengguna --</option>
        {pengguna.map(u => (
          <option key={u.pengguna_id} value={u.pengguna_id}>{u.nama}</option>
        ))}
      </select>
      <button
        disabled={!selected}
        onClick={() => {
          const user = pengguna.find(u => u.pengguna_id === selected);
          if (user) onLanjut(user);
        }}
        className="px-6 py-2 bg-pink-600 text-white font-semibold rounded-md hover:bg-pink-700 disabled:bg-pink-800 disabled:cursor-not-allowed transition"
      >
        Lanjutkan
      </button>
    </div>
  );
} 