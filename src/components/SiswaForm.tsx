import React from 'react';
import { Agama, JenisPendaftaran, Hobby, Cita, JenisKeluar, JenisTinggal, AlatTransportasi } from './SiswaView';

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
  // Kolom tambahan dari tabel peserta_didik
  nik: string;
  no_kk: string;
  rt: string;
  rw: string;
  nama_dusun: string;
  kode_pos: string;
  lintang: string;
  bujur: string;
  jenis_tinggal_id: string;
  alat_transportasi_id: string;
  nik_ayah: string;
  nik_ibu: string;
  anak_keberapa: string;
  nik_wali: string;
  nomor_telepon_rumah: string;
  nomor_telepon_seluler: string;
  email: string;
};

// Tipe untuk wilayah
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
  onCheckboxChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
  submitText?: string;
  referensi: {
    jenisPendaftaran: JenisPendaftaran[];
    hobbies: Hobby[];
    citas: Cita[];
    agama: Agama[];
    jenisKeluar: JenisKeluar[];
    jenisTinggal: JenisTinggal[];
    alatTransportasi: AlatTransportasi[];
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
    <form onSubmit={onSubmit} className="space-y-8 bg-gray-800 p-6 rounded-xl shadow-lg max-w-4xl mx-auto border border-gray-700">
      {/* Data Pribadi */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Pribadi</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label htmlFor="nama" className="block text-sm font-semibold text-gray-300 mb-2">Nama Lengkap *</label>
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
            <label className="block text-sm font-semibold text-gray-300 mb-2">Jenis Kelamin *</label>
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
              <label htmlFor="tanggal_lahir" className="block text-sm font-semibold text-gray-300 mb-2">Tanggal Lahir *</label>
              <input type="date" name="tanggal_lahir" id="tanggal_lahir" value={formData.tanggal_lahir} onChange={onInputChange} required
                     className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
            </div>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="agama_id" className="block text-sm font-semibold text-gray-300 mb-2">Agama *</label>
            <select name="agama_id" id="agama_id" value={formData.agama_id} onChange={onInputChange} required
                    className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
              <option value="">-- Pilih Agama --</option>
              {agamaList.map(agama => (
                <option key={agama.agama_id} value={agama.agama_id}>{agama.nama}</option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="kewarganegaraan" className="block text-sm font-semibold text-gray-300 mb-2">Kewarganegaraan</label>
            <input type="text" name="kewarganegaraan" id="kewarganegaraan" value={formData.kewarganegaraan} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="nik" className="block text-sm font-semibold text-gray-300 mb-2">NIK</label>
            <input type="text" name="nik" id="nik" value={formData.nik} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="no_kk" className="block text-sm font-semibold text-gray-300 mb-2">Nomor KK</label>
            <input type="text" name="no_kk" id="no_kk" value={formData.no_kk} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
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
            <label htmlFor="tanggal_masuk_sekolah" className="block text-sm font-semibold text-gray-300 mb-2">Tanggal Masuk Sekolah *</label>
            <input type="date" name="tanggal_masuk_sekolah" id="tanggal_masuk_sekolah" value={formData.tanggal_masuk_sekolah} onChange={onInputChange} required 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="jenis_pendaftaran_id" className="block text-sm font-semibold text-gray-300 mb-2">Jenis Pendaftaran *</label>
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
              <select name="id_hobby" id="id_hobby" value={formData.id_hobby} onChange={onInputChange}
                      className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
                <option value="">-- Pilih Hobi --</option>
                {referensi.hobbies.map(h => (
                  <option key={h.id_hobby} value={h.id_hobby}>{h.nm_hobby}</option>
                ))}
              </select>
            </div>
            <div>
              <label htmlFor="id_cita" className="block text-sm font-semibold text-gray-300 mb-2">Cita-cita</label>
              <select name="id_cita" id="id_cita" value={formData.id_cita} onChange={onInputChange}
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
          <label htmlFor="alamat_jalan" className="block text-sm font-semibold text-gray-300 mb-2">Alamat Jalan *</label>
          <input type="text" name="alamat_jalan" id="alamat_jalan" value={formData.alamat_jalan} onChange={onInputChange} required className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-semibold text-gray-300 mb-2">Provinsi *</label>
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
            <label className="block text-sm font-semibold text-gray-300 mb-2">Kabupaten/Kota *</label>
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
            <label className="block text-sm font-semibold text-gray-300 mb-2">Kecamatan *</label>
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
            <label className="block text-sm font-semibold text-gray-300 mb-2">Desa/Kelurahan *</label>
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
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="rt" className="block text-sm font-semibold text-gray-300 mb-2">RT</label>
            <input type="text" name="rt" id="rt" value={formData.rt} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="rw" className="block text-sm font-semibold text-gray-300 mb-2">RW</label>
            <input type="text" name="rw" id="rw" value={formData.rw} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="nama_dusun" className="block text-sm font-semibold text-gray-300 mb-2">Nama Dusun</label>
            <input type="text" name="nama_dusun" id="nama_dusun" value={formData.nama_dusun} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="kode_pos" className="block text-sm font-semibold text-gray-300 mb-2">Kode Pos</label>
            <input type="text" name="kode_pos" id="kode_pos" value={formData.kode_pos} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="lintang" className="block text-sm font-semibold text-gray-300 mb-2">Koordinat Lintang</label>
            <input type="text" name="lintang" id="lintang" value={formData.lintang} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="bujur" className="block text-sm font-semibold text-gray-300 mb-2">Koordinat Bujur</label>
            <input type="text" name="bujur" id="bujur" value={formData.bujur} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="jenis_tinggal_id" className="block text-sm font-semibold text-gray-300 mb-2">Jenis Tempat Tinggal</label>
            <select name="jenis_tinggal_id" id="jenis_tinggal_id" value={formData.jenis_tinggal_id} onChange={onInputChange}
                    className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
              <option value="">-- Pilih Jenis Tempat Tinggal --</option>
              {referensi.jenisTinggal.map(jt => (
                <option key={jt.jenis_tinggal_id} value={jt.jenis_tinggal_id}>{jt.nama}</option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="alat_transportasi_id" className="block text-sm font-semibold text-gray-300 mb-2">Alat Transportasi</label>
            <select name="alat_transportasi_id" id="alat_transportasi_id" value={formData.alat_transportasi_id} onChange={onInputChange}
                    className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition">
              <option value="">-- Pilih Alat Transportasi --</option>
              {referensi.alatTransportasi.map(at => (
                <option key={at.alat_transportasi_id} value={at.alat_transportasi_id}>{at.nama}</option>
              ))}
            </select>
          </div>
        </div>
      </div>

      {/* Data Orang Tua */}
      <div>
        <h3 className="text-xl font-bold text-pink-400 mb-6 tracking-wide">Data Orang Tua</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label htmlFor="nama_ibu_kandung" className="block text-sm font-semibold text-gray-300 mb-2">Nama Ibu Kandung *</label>
            <input type="text" name="nama_ibu_kandung" id="nama_ibu_kandung" value={formData.nama_ibu_kandung} onChange={onInputChange} required className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="anak_keberapa" className="block text-sm font-semibold text-gray-300 mb-2">Anak Keberapa</label>
            <input type="text" name="anak_keberapa" id="anak_keberapa" value={formData.anak_keberapa} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="nik_ayah" className="block text-sm font-semibold text-gray-300 mb-2">NIK Ayah</label>
            <input type="text" name="nik_ayah" id="nik_ayah" value={formData.nik_ayah} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="nik_ibu" className="block text-sm font-semibold text-gray-300 mb-2">NIK Ibu</label>
            <input type="text" name="nik_ibu" id="nik_ibu" value={formData.nik_ibu} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="nik_wali" className="block text-sm font-semibold text-gray-300 mb-2">NIK Wali</label>
            <input type="text" name="nik_wali" id="nik_wali" value={formData.nik_wali} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="nomor_telepon_rumah" className="block text-sm font-semibold text-gray-300 mb-2">Telepon Rumah</label>
            <input type="text" name="nomor_telepon_rumah" id="nomor_telepon_rumah" value={formData.nomor_telepon_rumah} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
          <div>
            <label htmlFor="nomor_telepon_seluler" className="block text-sm font-semibold text-gray-300 mb-2">Telepon Seluler</label>
            <input type="text" name="nomor_telepon_seluler" id="nomor_telepon_seluler" value={formData.nomor_telepon_seluler} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
          <div>
            <label htmlFor="email" className="block text-sm font-semibold text-gray-300 mb-2">Email</label>
            <input type="email" name="email" id="email" value={formData.email} onChange={onInputChange} 
                   className="w-full bg-gray-900 border border-gray-700 rounded-lg p-3 focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition"/>
          </div>
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