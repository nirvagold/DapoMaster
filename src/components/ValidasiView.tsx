import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Loader2, AlertTriangle, CheckCircle, Play, BarChart3, History, Clock, Trash2, RotateCcw, Shield, Eye } from 'lucide-react';
import { Pengguna } from './PemilihanPenggunaView';

export type ValidasiDetail = {
  peserta_didik_id: string;
  nama: string;
  field: string;
  action: string;
  success: boolean;
  message: string;
  old_value?: string;
  new_value?: string;
};

export type ValidasiResult = {
  success: boolean;
  message: string;
  session_id: string;
  total_processed: number;
  success_count: number;
  error_count: number;
  details: ValidasiDetail[];
};

export type ValidasiSession = {
  session_id: string;
  timestamp: string;
  total_processed: number;
  success_count: number;
  error_count: number;
  details: ValidasiDetail[];
  status: 'Running' | 'Completed' | 'Failed';
};

export type ValidasiStats = {
  total_siswa?: number;
  nik_ayah_invalid?: number;
  tanpa_hobby?: number;
  tanpa_cita_cita?: number;
  tahun_lahir_ayah_invalid?: number;
  nik_wali_invalid?: number;
  tahun_lahir_wali_invalid?: number;
  kps_pkh_invalid?: number;
};

export default function ValidasiView({ user }: { user: Pengguna | null }) {
  const [validasiResult, setValidasiResult] = useState<ValidasiResult | null>(null);
  const [validasiStats, setValidasiStats] = useState<ValidasiStats | null>(null);
  const [validasiSessions, setValidasiSessions] = useState<ValidasiSession[]>([]);
  const [selectedSession, setSelectedSession] = useState<ValidasiSession | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showDetails, setShowDetails] = useState(false);
  const [activeTab, setActiveTab] = useState<'current' | 'history'>('current');
  const [backupTable, setBackupTable] = useState<string | null>(null);
  const [showValidationWarning, setShowValidationWarning] = useState(false);

  // Fetch statistik validasi dan session saat komponen mount
  useEffect(() => {
    fetchValidasiStats();
    fetchValidasiSessions();
  }, []);

  const fetchValidasiStats = async () => {
    try {
      const stats = await invoke<ValidasiStats>('get_validasi_stats');
      setValidasiStats(stats);
    } catch (err) {
      console.error('Error fetching validasi stats:', err);
    }
  };

  const fetchValidasiSessions = async () => {
    try {
      const sessions = await invoke<ValidasiSession[]>('get_validasi_sessions');
      setValidasiSessions(sessions.sort((a: ValidasiSession, b: ValidasiSession) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()));
    } catch (err) {
      console.error('Error fetching validasi sessions:', err);
    }
  };

  const handleValidateBeforeFix = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const stats = await invoke<ValidasiStats>('validate_before_fix');
      setValidasiStats(stats);
      
      const totalErrors = (stats.nik_ayah_invalid || 0) + (stats.tanpa_hobby || 0) + (stats.tanpa_cita_cita || 0) + 
                         (stats.tahun_lahir_ayah_invalid || 0) + (stats.nik_wali_invalid || 0) + (stats.tahun_lahir_wali_invalid || 0) + 
                         (stats.kps_pkh_invalid || 0);
      
      if (totalErrors > 0) {
        setShowValidationWarning(true);
        alert(`Validasi selesai!\n\nDitemukan ${totalErrors} error yang perlu diperbaiki.\n\nSilakan klik "Jalankan Perbaikan Otomatis" untuk memperbaiki error tersebut.`);
      } else {
        alert('Validasi selesai!\n\nTidak ada error yang ditemukan. Data sudah valid.');
      }
    } catch (err) {
      setError(String(err));
      alert(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleAutoFix = async () => {
    if (!window.confirm('Apakah Anda yakin ingin menjalankan perbaikan otomatis? Tindakan ini akan mengubah data siswa.')) {
      return;
    }

    setLoading(true);
    setError(null);
    setValidasiResult(null);
    setShowValidationWarning(false);

    try {
      if (!user) {
        throw new Error('Pengguna tidak ditemukan. Silakan login ulang.');
      }
      
      const result = await invoke<ValidasiResult>('auto_fix_validasi_errors', { 
        pengguna_id: user.pengguna_id 
      });
      setValidasiResult(result);
      
      // Extract backup table name from console logs (this is a simplified approach)
      // In a real implementation, you'd want to return this from the backend
      if (result.session_id) {
        setBackupTable(`peserta_didik_backup_${result.session_id.split('_')[1]}`);
      }
      
      // Refresh statistik dan session setelah perbaikan
      await Promise.all([fetchValidasiStats(), fetchValidasiSessions()]);
      
      if (result.success) {
        alert(`Perbaikan otomatis berhasil!\n\n${result.message}\n\nBackup table: ${backupTable || 'Tidak tersedia'}`);
      } else {
        alert(`Perbaikan otomatis selesai dengan beberapa error:\n\n${result.message}`);
      }
    } catch (err) {
      setError(String(err));
      alert(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleRollback = async () => {
    if (!backupTable) {
      alert('Tidak ada backup table yang tersedia untuk rollback.');
      return;
    }

    if (!window.confirm(`Apakah Anda yakin ingin melakukan rollback ke backup table: ${backupTable}?\n\nTindakan ini akan mengembalikan semua data ke kondisi sebelum perbaikan.`)) {
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const result = await invoke<string>('rollback_validasi_changes', { backupTable });
      alert(`Rollback berhasil!\n\n${result}`);
      
      // Reset states
      setValidasiResult(null);
      setBackupTable(null);
      setShowValidationWarning(false);
      
      // Refresh data
      await Promise.all([fetchValidasiStats(), fetchValidasiSessions()]);
    } catch (err) {
      setError(String(err));
      alert(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCleanupSessions = async () => {
    if (!window.confirm('Apakah Anda yakin ingin membersihkan session validasi yang lama? (lebih dari 24 jam)')) {
      return;
    }

    try {
      await invoke('cleanup_old_validasi_sessions', { hours: 24 });
      await fetchValidasiSessions();
      alert('Session validasi lama berhasil dibersihkan!');
    } catch (err) {
      alert(`Error: ${err}`);
    }
  };

  const getFieldColor = (field: string) => {
    switch (field) {
      case 'NIK Ayah':
      case 'NIK Wali':
        return 'text-red-400';
      case 'Hobby':
      case 'Cita-cita':
        return 'text-blue-400';
      case 'Tahun Lahir Ayah':
      case 'Tahun Lahir Wali':
        return 'text-yellow-400';
      case 'KPS/PKH':
        return 'text-orange-400';
      default:
        return 'text-gray-400';
    }
  };

  const getActionColor = (action: string) => {
    if (action.includes('Diisi random')) return 'text-green-400';
    if (action.includes('Dihapus')) return 'text-red-400';
    if (action.includes('Warning')) return 'text-yellow-400';
    return 'text-gray-400';
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Completed':
        return 'text-green-400';
      case 'Failed':
        return 'text-red-400';
      case 'Running':
        return 'text-yellow-400';
      default:
        return 'text-gray-400';
    }
  };

  const formatTimestamp = (timestamp: string) => {
    return new Date(timestamp).toLocaleString('id-ID');
  };

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-100 mb-2">
            Sistem Validasi Otomatis
          </h1>
          <p className="text-gray-400">
            Perbaiki error validasi secara otomatis dengan sistem yang pintar
          </p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={handleValidateBeforeFix}
            disabled={loading}
            className="flex items-center gap-2 px-4 py-3 bg-gradient-to-r from-blue-500 to-cyan-600 text-white rounded-lg font-semibold hover:from-blue-600 hover:to-cyan-700 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
          >
            {loading ? (
              <Loader2 className="animate-spin" size={20} />
            ) : (
              <Eye size={20} />
            )}
            {loading ? 'Memvalidasi...' : 'Validasi Data'}
          </button>
          
          <button
            onClick={handleAutoFix}
            disabled={loading || !showValidationWarning}
            className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-pink-500 to-purple-600 text-white rounded-lg font-semibold hover:from-pink-600 hover:to-purple-700 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
          >
            {loading ? (
              <Loader2 className="animate-spin" size={20} />
            ) : (
              <Play size={20} />
            )}
            {loading ? 'Memperbaiki...' : 'Jalankan Perbaikan Otomatis'}
          </button>
          
          {backupTable && (
            <button
              onClick={handleRollback}
              disabled={loading}
              className="flex items-center gap-2 px-4 py-3 bg-gradient-to-r from-red-500 to-orange-600 text-white rounded-lg font-semibold hover:from-red-600 hover:to-orange-700 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
            >
              {loading ? (
                <Loader2 className="animate-spin" size={20} />
              ) : (
                <RotateCcw size={20} />
              )}
              {loading ? 'Rollback...' : 'Rollback'}
            </button>
          )}
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-gray-800 p-1 rounded-lg">
        <button
          onClick={() => setActiveTab('current')}
          className={`flex-1 py-2 px-4 rounded-md font-medium transition-colors ${
            activeTab === 'current'
              ? 'bg-pink-600 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-700'
          }`}
        >
          <BarChart3 className="inline mr-2" size={16} />
          Statistik & Perbaikan
        </button>
        <button
          onClick={() => setActiveTab('history')}
          className={`flex-1 py-2 px-4 rounded-md font-medium transition-colors ${
            activeTab === 'history'
              ? 'bg-pink-600 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-700'
          }`}
        >
          <History className="inline mr-2" size={16} />
          Riwayat Validasi
        </button>
      </div>

      {/* Current Tab Content */}
      {activeTab === 'current' && (
        <>
          {/* Statistik Validasi */}
          {validasiStats && (
            <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
              <div className="flex items-center gap-2 mb-4">
                <BarChart3 className="text-blue-400" size={24} />
                <h2 className="text-xl font-semibold text-gray-100">Statistik Error Validasi</h2>
              </div>
              
              <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-4">
                {Object.entries(validasiStats).map(([key, value]) => (
                  <div key={key} className="bg-gray-700 rounded-lg p-4 text-center">
                    <div className="text-2xl font-bold text-pink-400">{value || 0}</div>
                    <div className="text-sm text-gray-300 capitalize">
                      {key.replace('_', ' ')}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Error Display */}
          {error && (
            <div className="bg-red-900/20 border border-red-600 text-red-400 p-4 rounded-lg flex items-center gap-2">
              <AlertTriangle size={20} />
              <span>{error}</span>
            </div>
          )}

          {/* Validation Warning */}
          {showValidationWarning && (
            <div className="bg-yellow-900/20 border border-yellow-600 text-yellow-400 p-4 rounded-lg flex items-center gap-2">
              <Shield size={20} />
              <span>Data telah divalidasi dan ditemukan error yang perlu diperbaiki. Silakan klik "Jalankan Perbaikan Otomatis" untuk memperbaiki error tersebut.</span>
            </div>
          )}

          {/* Result Summary */}
          {validasiResult && (
            <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold text-gray-100">Hasil Perbaikan Otomatis</h2>
                <div className="flex items-center gap-2">
                  <span className="text-sm text-gray-400">Session ID: {validasiResult.session_id}</span>
                  <button
                    onClick={() => setShowDetails(!showDetails)}
                    className="text-blue-400 hover:text-blue-300 text-sm"
                  >
                    {showDetails ? 'Sembunyikan Detail' : 'Tampilkan Detail'}
                  </button>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-blue-400">{validasiResult.total_processed}</div>
                  <div className="text-sm text-gray-300">Total Diproses</div>
                </div>
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-green-400">{validasiResult.success_count}</div>
                  <div className="text-sm text-gray-300">Berhasil</div>
                </div>
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-red-400">{validasiResult.error_count}</div>
                  <div className="text-sm text-gray-300">Error</div>
                </div>
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-purple-400">
                    {validasiResult.total_processed > 0 
                      ? Math.round((validasiResult.success_count / validasiResult.total_processed) * 100)
                      : 0}%
                  </div>
                  <div className="text-sm text-gray-300">Success Rate</div>
                </div>
              </div>

              <div className="bg-gray-700 rounded-lg p-4">
                <p className="text-gray-300">{validasiResult.message}</p>
              </div>

              {/* Detail Results */}
              {showDetails && validasiResult.details.length > 0 && (
                <div className="mt-6">
                  <h3 className="text-lg font-semibold text-gray-100 mb-4">Detail Perbaikan</h3>
                  <div className="bg-gray-900 rounded-lg overflow-hidden">
                    <div className="overflow-x-auto">
                      <table className="w-full">
                        <thead className="bg-gray-800">
                          <tr>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Nama Siswa</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Field</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Aksi</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Nilai Lama</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Nilai Baru</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Status</th>
                            <th className="px-4 py-3 text-left text-gray-300 text-sm">Pesan</th>
                          </tr>
                        </thead>
                        <tbody>
                          {validasiResult.details.map((detail, index) => (
                            <tr key={index} className="border-b border-gray-700 hover:bg-gray-800">
                              <td className="px-4 py-3 text-gray-300 text-sm">{detail.nama}</td>
                              <td className="px-4 py-3">
                                <span className={`text-sm font-medium ${getFieldColor(detail.field)}`}>
                                  {detail.field}
                                </span>
                              </td>
                              <td className="px-4 py-3">
                                <span className={`text-sm ${getActionColor(detail.action)}`}>
                                  {detail.action}
                                </span>
                              </td>
                              <td className="px-4 py-3 text-gray-400 text-sm">
                                {detail.old_value || '-'}
                              </td>
                              <td className="px-4 py-3 text-gray-400 text-sm">
                                {detail.new_value || '-'}
                              </td>
                              <td className="px-4 py-3">
                                {detail.success ? (
                                  <CheckCircle className="text-green-400" size={16} />
                                ) : (
                                  <AlertTriangle className="text-red-400" size={16} />
                                )}
                              </td>
                              <td className="px-4 py-3 text-gray-400 text-sm max-w-xs truncate" title={detail.message}>
                                {detail.message}
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </div>
                </div>
              )}
            </div>
          )}
        </>
      )}

      {/* History Tab Content */}
      {activeTab === 'history' && (
        <div className="space-y-6">
          {/* Header dengan cleanup button */}
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold text-gray-100">Riwayat Validasi</h2>
            <button
              onClick={handleCleanupSessions}
              className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              <Trash2 size={16} />
              Bersihkan Session Lama
            </button>
          </div>

          {/* Session List */}
          <div className="bg-gray-800 rounded-lg border border-gray-700">
            {validasiSessions.length === 0 ? (
              <div className="p-8 text-center text-gray-400">
                <History size={48} className="mx-auto mb-4 opacity-50" />
                <p>Belum ada riwayat validasi</p>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-gray-700">
                    <tr>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Waktu</th>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Status</th>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Total</th>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Berhasil</th>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Error</th>
                      <th className="px-4 py-3 text-left text-gray-300 text-sm">Aksi</th>
                    </tr>
                  </thead>
                  <tbody>
                    {validasiSessions.map((session) => (
                      <tr key={session.session_id} className="border-b border-gray-700 hover:bg-gray-700">
                        <td className="px-4 py-3 text-gray-300 text-sm">
                          <div className="flex items-center gap-2">
                            <Clock size={14} />
                            {formatTimestamp(session.timestamp)}
                          </div>
                        </td>
                        <td className="px-4 py-3">
                          <span className={`text-sm font-medium ${getStatusColor(session.status)}`}>
                            {session.status}
                          </span>
                        </td>
                        <td className="px-4 py-3 text-gray-300 text-sm">{session.total_processed}</td>
                        <td className="px-4 py-3 text-green-400 text-sm">{session.success_count}</td>
                        <td className="px-4 py-3 text-red-400 text-sm">{session.error_count}</td>
                        <td className="px-4 py-3">
                          <button
                            onClick={() => setSelectedSession(selectedSession?.session_id === session.session_id ? null : session)}
                            className="text-blue-400 hover:text-blue-300 text-sm"
                          >
                            {selectedSession?.session_id === session.session_id ? 'Sembunyikan' : 'Lihat Detail'}
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>

          {/* Selected Session Details */}
          {selectedSession && (
            <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-gray-100">
                  Detail Session: {selectedSession.session_id}
                </h3>
                <span className={`text-sm font-medium ${getStatusColor(selectedSession.status)}`}>
                  {selectedSession.status}
                </span>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-blue-400">{selectedSession.total_processed}</div>
                  <div className="text-sm text-gray-300">Total Diproses</div>
                </div>
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-green-400">{selectedSession.success_count}</div>
                  <div className="text-sm text-gray-300">Berhasil</div>
                </div>
                <div className="bg-gray-700 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-red-400">{selectedSession.error_count}</div>
                  <div className="text-sm text-gray-300">Error</div>
                </div>
              </div>

              {selectedSession.details.length > 0 && (
                <div className="bg-gray-900 rounded-lg overflow-hidden">
                  <div className="overflow-x-auto">
                    <table className="w-full">
                      <thead className="bg-gray-800">
                        <tr>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Nama Siswa</th>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Field</th>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Aksi</th>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Nilai Lama</th>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Nilai Baru</th>
                          <th className="px-4 py-3 text-left text-gray-300 text-sm">Status</th>
                        </tr>
                      </thead>
                      <tbody>
                        {selectedSession.details.map((detail, index) => (
                          <tr key={index} className="border-b border-gray-700 hover:bg-gray-800">
                            <td className="px-4 py-3 text-gray-300 text-sm">{detail.nama}</td>
                            <td className="px-4 py-3">
                              <span className={`text-sm font-medium ${getFieldColor(detail.field)}`}>
                                {detail.field}
                              </span>
                            </td>
                            <td className="px-4 py-3">
                              <span className={`text-sm ${getActionColor(detail.action)}`}>
                                {detail.action}
                              </span>
                            </td>
                            <td className="px-4 py-3 text-gray-400 text-sm">
                              {detail.old_value || '-'}
                            </td>
                            <td className="px-4 py-3 text-gray-400 text-sm">
                              {detail.new_value || '-'}
                            </td>
                            <td className="px-4 py-3">
                              {detail.success ? (
                                <CheckCircle className="text-green-400" size={16} />
                              ) : (
                                <AlertTriangle className="text-red-400" size={16} />
                              )}
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      )}

      {/* Informasi Sistem */}
      <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
        <h2 className="text-xl font-semibold text-gray-100 mb-4">Cara Kerja Sistem Validasi Otomatis</h2>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="space-y-4">
            <h3 className="text-lg font-medium text-blue-400">Validasi & Backup</h3>
            <ul className="space-y-2 text-gray-300 text-sm">
              <li className="flex items-start gap-2">
                <span className="text-blue-400">•</span>
                <span><strong>Validasi Sebelum Perbaikan:</strong> Cek error tanpa mengubah data</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">•</span>
                <span><strong>Auto Backup:</strong> Backup otomatis sebelum perubahan</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-orange-400">•</span>
                <span><strong>Rollback:</strong> Kembalikan data ke kondisi sebelum perbaikan</span>
              </li>
            </ul>
          </div>
          
          <div className="space-y-4">
            <h3 className="text-lg font-medium text-pink-400">Perbaikan Otomatis</h3>
            <ul className="space-y-2 text-gray-300 text-sm">
              <li className="flex items-start gap-2">
                <span className="text-red-400">•</span>
                <span><strong>NIK Ayah/Wali:</strong> Dihapus jika format tidak valid</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-blue-400">•</span>
                <span><strong>Hobby & Cita-cita:</strong> Diisi random dari referensi</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-yellow-400">•</span>
                <span><strong>Tahun Lahir:</strong> Dihapus jika format tidak valid</span>
              </li>
            </ul>
          </div>
          
          <div className="space-y-4">
            <h3 className="text-lg font-medium text-purple-400">Fitur Tambahan</h3>
            <ul className="space-y-2 text-gray-300 text-sm">
              <li className="flex items-start gap-2">
                <span className="text-green-400">•</span>
                <span><strong>Progress Tracking:</strong> Monitor progress real-time</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-purple-400">•</span>
                <span><strong>Session Management:</strong> Riwayat lengkap dengan audit trail</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-blue-400">•</span>
                <span><strong>Batch Processing:</strong> Perbaikan dalam batch untuk performa</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}