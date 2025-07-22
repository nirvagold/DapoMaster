import React from 'react';
import { Agama, JenisPendaftaran, Hobby, Cita } from './SiswaView'; // Impor semua tipe

export type SiswaFormData = {
  nama: string;
  nisn: string;
  jenis_kelamin: string;
  tempat_lahir: string;
  tanggal_lahir: string;
  agama_id: string;
  nipd: string;
  tanggal_masuk_sekolah: string;
  jenis_pendaftaran_id: string;
  id_hobby: string;
  id_cita: string;
  a_pernah_paud: boolean;
  a_pernah_tk: boolean;
  alamat_jalan: string;
  desa_kelurahan: string;
  kode_wilayah: string;
  nama_ibu_kandung: string;
  kewarganegaraan: string;
};

// Tambahkan tipe baru untuk wilayah
export type WilayahReferensi = {
  kode_wilayah: string;
  nama: string;
  id_level_wilayah: number;
  mst_kode_wilayah?: string | null;
};

interface SiswaFormProps {
  formData: SiswaFormData;
  agamaList: Agama[];
  loading: boolean;
  onInputChange: (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => void;
  onCheckboxChange: (e: React.ChangeEvent<HTMLInputElement>) => void; // Prop baru
  onSubmit: (e: React.FormEvent) => void;
  submitText?: string;
  referensi: {
    jenisPendaftaran: JenisPendaftaran[];
    hobbies: Hobby[];
    citas: Cita[];
    agama: Agama[];
  };
  wilayahOptions: {
    provinsi: WilayahReferensi[];
    kabupaten: WilayahReferensi[];
    kecamatan: WilayahReferensi[];
    desa: WilayahReferensi[];
  };
  selectedWilayah: {
    provinsi: string;
    kabupaten: string;
    kecamatan: string;
  };
  onWilayahChange: (level: 'provinsi' | 'kabupaten' | 'kecamatan' | 'desa', kode: string) => void;
}

const SiswaForm: React.FC<SiswaFormProps> = ({
  formData,
  agamaList,
  loading,
  onInputChange,
  onCheckboxChange,
  onSubmit,
  submitText = "Simpan",
  referensi,
  wilayahOptions,
  selectedWilayah,
  onWilayahChange,
}) => {
  return (
    <form onSubmit={onSubmit} className="space-y-8 bg-gray-800 p-6 rounded-xl shadow-lg max-w-2xl mx-auto border border-gray-700">
      {/* Data Pribadi */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Pribadi</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label htmlFor="nama" className="block text-sm font-semibold text-gray-300 mb-2">Nama Lengkap</label>
            <input type="text" name="nama" id="nama" value={formData.nama} onChange={onInputChange} required 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="nisn" className="block text-sm font-semibold text-gray-300 mb-2">NISN</label>
            <input type="text" name="nisn" id="nisn" value={formData.nisn} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Jenis Kelamin</label>
            <select name="jenis_kelamin" value={formData.jenis_kelamin} onChange={onInputChange} 
                    className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
              <option value="L">Laki-laki</option>
              <option value="P">Perempuan</option>
            </select>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label htmlFor="tempat_lahir" className="block text-sm font-semibold text-gray-300 mb-2">Tempat Lahir</label>
              <input type="text" name="tempat_lahir" id="tempat_lahir" value={formData.tempat_lahir} onChange={onInputChange} 
                     className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
            </div>
            <div>
              <label htmlFor="tanggal_lahir" className="block text-sm font-semibold text-gray-300 mb-2">Tanggal Lahir</label>
              <input type="date" name="tanggal_lahir" id="tanggal_lahir" value={formData.tanggal_lahir} onChange={onInputChange} required
                     className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
            </div>
          </div>
        </div>
        <div className="mt-6">
          <label htmlFor="agama_id" className="block text-sm font-semibold text-gray-300 mb-2">Agama</label>
          <select name="agama_id" id="agama_id" value={formData.agama_id} onChange={onInputChange} required
                  className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
            <option value="">-- Pilih Agama --</option>
            {agamaList.map(agama => (
              <option key={agama.agama_id} value={agama.agama_id}>{agama.nama}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Data Sekolah */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Sekolah</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label htmlFor="nipd" className="block text-sm font-semibold text-gray-300 mb-2">NIPD</label>
            <input type="text" name="nipd" id="nipd" value={formData.nipd} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="tanggal_masuk_sekolah" className="block text-sm font-semibold text-gray-300 mb-2">Tanggal Masuk Sekolah</label>
            <input type="date" name="tanggal_masuk_sekolah" id="tanggal_masuk_sekolah" value={formData.tanggal_masuk_sekolah} onChange={onInputChange} required 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="jenis_pendaftaran_id" className="block text-sm font-semibold text-gray-300 mb-2">Jenis Pendaftaran</label>
            <select name="jenis_pendaftaran_id" id="jenis_pendaftaran_id" value={formData.jenis_pendaftaran_id} onChange={onInputChange} required
                    className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
              <option value="">-- Pilih Jenis Pendaftaran --</option>
              {referensi.jenisPendaftaran.map(jp => (
                <option key={jp.jenis_pendaftaran_id} value={jp.jenis_pendaftaran_id}>{jp.nama}</option>
              ))}
            </select>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label htmlFor="id_hobby" className="block text-sm font-semibold text-gray-300 mb-2">Hobi</label>
              <select name="id_hobby" id="id_hobby" value={formData.id_hobby} onChange={onInputChange} required
                      className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
                <option value="">-- Pilih Hobi --</option>
                {referensi.hobbies.map(h => (
                  <option key={h.id_hobby} value={h.id_hobby}>{h.nm_hobby}</option>
                ))}
              </select>
            </div>
            <div>
              <label htmlFor="id_cita" className="block text-sm font-semibold text-gray-300 mb-2">Cita-cita</label>
              <select name="id_cita" id="id_cita" value={formData.id_cita} onChange={onInputChange} required
                      className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
                <option value="">-- Pilih Cita-cita --</option>
                {referensi.citas.map(c => (
                  <option key={c.id_cita} value={c.id_cita}>{c.nm_cita}</option>
                ))}
              </select>
            </div>
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-6 mt-6">
          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input type="checkbox" name="a_pernah_paud" checked={formData.a_pernah_paud} onChange={onCheckboxChange} className="w-4 h-4 rounded bg-gray-700 border-gray-600 text-pink-600 focus:ring-pink-500"/>
            <span className="text-sm font-medium text-gray-300">Pernah PAUD</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input type="checkbox" name="a_pernah_tk" checked={formData.a_pernah_tk} onChange={onCheckboxChange} className="w-4 h-4 rounded bg-gray-700 border-gray-600 text-pink-600 focus:ring-pink-500"/>
            <span className="text-sm font-medium text-gray-300">Pernah TK</span>
          </label>
        </div>
      </div>

      {/* Data Alamat */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Alamat</h3>
        <div className="mb-6">
          <label htmlFor="alamat_jalan" className="block text-sm font-semibold text-gray-300 mb-2">Alamat Jalan</label>
          <input type="text" name="alamat_jalan" id="alamat_jalan" value={formData.alamat_jalan} onChange={onInputChange} required className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Provinsi</label>
            <select
              name="provinsi"
              value={selectedWilayah.provinsi}
              onChange={e => onWilayahChange('provinsi', e.target.value)}
              required
              className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"
            >
              <option value="">-- Pilih Provinsi --</option>
              {wilayahOptions.provinsi.map(w => (
                <option key={w.kode_wilayah} value={w.kode_wilayah}>{w.nama}</option>
              ))}
            </select>
          </div>
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Kabupaten/Kota</label>
            <select
              name="kabupaten"
              value={selectedWilayah.kabupaten}
              onChange={e => onWilayahChange('kabupaten', e.target.value)}
              required
              className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"
            >
              <option value="">-- Pilih Kabupaten/Kota --</option>
              {wilayahOptions.kabupaten.map(w => (
                <option key={w.kode_wilayah} value={w.kode_wilayah}>{w.nama}</option>
              ))}
            </select>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Kecamatan</label>
            <select
              name="kecamatan"
              value={selectedWilayah.kecamatan}
              onChange={e => onWilayahChange('kecamatan', e.target.value)}
              required
              className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"
            >
              <option value="">-- Pilih Kecamatan --</option>
              {wilayahOptions.kecamatan.map(w => (
                <option key={w.kode_wilayah} value={w.kode_wilayah}>{w.nama}</option>
              ))}
            </select>
          </div>
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Desa/Kelurahan</label>
            <select
              name="kode_wilayah"
              value={formData.kode_wilayah}
              onChange={onInputChange}
              required
              className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"
            >
              <option value="">-- Pilih Desa/Kelurahan --</option>
              {wilayahOptions.desa.map(w => (
                <option key={w.kode_wilayah} value={w.kode_wilayah}>{w.nama}</option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Data Orang Tua */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Orang Tua</h3>
        <div>
          <label htmlFor="nama_ibu_kandung" className="block text-sm font-semibold text-gray-300 mb-2">Nama Ibu Kandung</label>
          <input type="text" name="nama_ibu_kandung" id="nama_ibu_kandung" value={formData.nama_ibu_kandung} onChange={onInputChange} required className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
        </div>
      </div>

      {/* Submit Button */}
      <div className="flex justify-end pt-6">
        <button type="submit" disabled={loading} className="px-8 py-3 bg-pink-600 text-white font-bold rounded-lg shadow-md hover:bg-pink-700 focus:ring-2 focus:ring-pink-400 focus:outline-none transition disabled:opacity-50 disabled:cursor-not-allowed">
          {loading ? "Menyimpan..." : submitText}
        </button>
      </div>
    </form>
  );
};

export default SiswaForm; 