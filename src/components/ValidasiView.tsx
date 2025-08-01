import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ValidationResult {
  peserta_didik_id: string;
  nama: string;
  field_error: string;
  error_type: string;
  error_message: string;
  status: string;
}

interface ValidationSummary {
  total_checked: number;
  total_errors: number;
  errors_by_type: ErrorTypeCount[];
}

interface ErrorTypeCount {
  error_type: string;
  count: number;
  field_name: string;
}

const ValidasiView: React.FC = () => {
  const [validationData, setValidationData] = useState<ValidationResult[]>([]);
  const [validationSummary, setValidationSummary] = useState<ValidationSummary | null>(null);
  const [autoFixLoading, setAutoFixLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<'normal' | 'stealth'>('stealth');

  // Fix hobby yang bernilai -1 atau NULL
  const handleFixHobbyMinusOne = async () => {
    if (confirm('Yakin ingin memperbaiki id_hobby yang bernilai -1 atau NULL dengan nilai random dari tabel ref.jenis_hobby?')) {
      setAutoFixLoading(true);
      try {
        const result = await invoke<string>('auto_fix_hobby_minus_one_stealth');
        alert(result);
      } catch (error) {
        console.error('Error fixing hobby -1/NULL:', error);
        alert(`Error: ${error}`);
      } finally {
        setAutoFixLoading(false);
      }
    }
  };

  // Fix cita-cita yang bernilai NULL atau -1
  const handleFixCitaNullZero = async () => {
    if (confirm('Yakin ingin memperbaiki id_cita yang bernilai NULL atau -1 dengan nilai random dari tabel ref.jenis_cita?')) {
      setAutoFixLoading(true);
      try {
        const result = await invoke<string>('auto_fix_cita_null_zero_stealth');
        alert(result);
      } catch (error) {
        console.error('Error fixing cita-cita NULL/-1:', error);
        alert(`Error: ${error}`);
      } finally {
        setAutoFixLoading(false);
      }
    }
  };

  // Fix NIK ayah tidak valid menjadi NULL
  const handleFixNikAyahInvalid = async () => {
    if (confirm('Yakin ingin memperbaiki NIK ayah tidak valid (spasi, dummy, dll) menjadi NULL?')) {
      setAutoFixLoading(true);
      try {
        const result = await invoke<string>('auto_fix_nik_ayah_invalid_stealth');
        alert(result);
      } catch (error) {
        console.error('Error fixing NIK ayah invalid:', error);
        alert(`Error: ${error}`);
    } finally {
        setAutoFixLoading(false);
      }
    }
  };

  // Fix NIK ibu tidak valid menjadi NULL
  const handleFixNikIbuInvalid = async () => {
    if (confirm('Yakin ingin memperbaiki NIK ibu tidak valid (spasi, dummy, dll) menjadi NULL?')) {
      setAutoFixLoading(true);
      try {
        const result = await invoke<string>('auto_fix_nik_ibu_invalid_stealth');
        alert(result);
      } catch (error) {
        console.error('Error fixing NIK ibu invalid:', error);
        alert(`Error: ${error}`);
      } finally {
        setAutoFixLoading(false);
      }
    }
  };

  const getStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
      case 'CRITICAL':
        return 'text-red-600 bg-red-100';
      case 'ERROR':
        return 'text-red-600 bg-red-50';
      case 'WARNING':
        return 'text-yellow-600 bg-yellow-50';
      default:
        return 'text-gray-600 bg-gray-50';
    }
  };

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-7xl mx-auto">
      {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">Validasi Data Siswa</h1>
          <p className="text-gray-600">Perbaiki data siswa yang bermasalah secara otomatis</p>
        </div>

        {/* Tab Navigation */}
        <div className="mb-6">
          <div className="border-b border-gray-200">
            <nav className="-mb-px flex space-x-8">
          <button
                onClick={() => setActiveTab('normal')}
                className={`py-2 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'normal'
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                Validasi Normal
          </button>
          <button
                onClick={() => setActiveTab('stealth')}
                className={`py-2 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'stealth'
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                Validasi Stealth
          </button>
            </nav>
          </div>
        </div>
          
        {/* Quick Fix Section */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Quick Fix</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <button
              onClick={handleFixHobbyMinusOne}
              disabled={autoFixLoading}
              className="flex items-center gap-2 bg-teal-600 text-white px-4 py-3 rounded-lg hover:bg-teal-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200 font-medium"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              {autoFixLoading ? 'Memperbaiki...' : 'Fix Hobby -1/NULL'}
            </button>

        <button
              onClick={handleFixCitaNullZero}
              disabled={autoFixLoading}
              className="flex items-center gap-2 bg-purple-600 text-white px-4 py-3 rounded-lg hover:bg-purple-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200 font-medium"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
              {autoFixLoading ? 'Memperbaiki...' : 'Fix Cita-cita NULL/-1'}
        </button>
            
                  <button
              onClick={handleFixNikAyahInvalid}
              disabled={autoFixLoading}
              className="flex items-center gap-2 bg-red-600 text-white px-4 py-3 rounded-lg hover:bg-red-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200 font-medium"
                  >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              {autoFixLoading ? 'Memperbaiki...' : 'Fix NIK Ayah → NULL'}
                  </button>
            
            <button
              onClick={handleFixNikIbuInvalid}
              disabled={autoFixLoading}
              className="flex items-center gap-2 bg-pink-600 text-white px-4 py-3 rounded-lg hover:bg-pink-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200 font-medium"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              {autoFixLoading ? 'Memperbaiki...' : 'Fix NIK Ibu → NULL'}
            </button>
          </div>
          </div>

        {/* Info Section */}
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg className="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
              </svg>
              </div>
            <div className="ml-3">
              <h3 className="text-sm font-medium text-blue-800">Informasi Validasi</h3>
              <div className="mt-2 text-sm text-blue-700">
                <p>• <strong>Fix Hobby -1/NULL:</strong> Memperbaiki id_hobby yang bernilai -1 atau NULL dengan nilai random dari tabel ref.jenis_hobby</p>
                <p>• <strong>Fix Cita-cita NULL/-1:</strong> Memperbaiki id_cita yang bernilai NULL atau -1 dengan nilai random dari tabel ref.jenis_cita</p>
                <p>• <strong>Fix NIK Ayah → NULL:</strong> Memperbaiki NIK ayah tidak valid (spasi, dummy, dll) menjadi NULL</p>
                <p>• <strong>Fix NIK Ibu → NULL:</strong> Memperbaiki NIK ibu tidak valid (spasi, dummy, dll) menjadi NULL</p>
                <p>• Semua perbaikan dilakukan dalam mode stealth (tanpa jejak audit)</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ValidasiView;