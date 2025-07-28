import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Users, GraduationCap, School, BookCopy, Loader2, AlertTriangle, Calendar, Clock } from "lucide-react";
import type { LucideProps } from 'lucide-react';
import React from "react";
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";

type DashboardStats = {
  total_siswa: number;
  total_ptk: number;
  total_rombel: number;
  total_jurusan: number;
};

type StatCardProps = {
  icon: React.ComponentType<LucideProps>;
  title: string;
  value: number | undefined;
  color: string;
  loading: boolean;
};

const StatCard: React.FC<StatCardProps> = ({ icon: Icon, title, value, color, loading }) => (
  <div className={`bg-gray-800 p-6 rounded-lg border-l-4 ${color}`}>
    <div className="flex items-center justify-between">
      <div>
        <p className="text-sm font-medium text-gray-400">{title}</p>
        {loading ? (
          <Loader2 className="animate-spin mt-1" />
        ) : (
          <p className="text-3xl font-bold">{value}</p>
        )}
      </div>
      <Icon className="w-12 h-12 text-gray-600" />
    </div>
  </div>
);

export default function DashboardView({ semester, tahunAjaran }: { semester: Semester | null, tahunAjaran: TahunAjaran | null }) {
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<DashboardStats>("get_dashboard_stats")
      .then(setStats)
      .catch(err => setError(String(err)))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div>
      <h1 className="text-3xl font-bold text-pink-500 mb-6">Dashboard</h1>
      
      {/* Informasi Semester dan Tahun Ajaran */}
      {(semester || tahunAjaran) && (
        <div className="mb-6 p-4 bg-gray-800 rounded-lg border border-gray-700">
          <h2 className="text-lg font-semibold text-pink-400 mb-3 flex items-center gap-2">
            <Calendar size={20} />
            Informasi Akademik
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {tahunAjaran && (
              <div className="flex items-center gap-3">
                <Clock className="text-gray-400" size={18} />
                <div>
                  <p className="text-sm text-gray-400">Tahun Ajaran</p>
                  <p className="font-medium text-white">{tahunAjaran.nama}</p>
                </div>
              </div>
            )}
            {semester && (
              <div className="flex items-center gap-3">
                <Calendar className="text-gray-400" size={18} />
                <div>
                  <p className="text-sm text-gray-400">Semester</p>
                  <p className="font-medium text-white">{semester.nama}</p>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
      
      {error && (
        <div className="bg-red-800 text-white p-4 rounded-md flex items-center gap-3">
          <AlertTriangle />
          <span>Gagal memuat data statistik: {error}</span>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard icon={Users} title="Total Peserta Didik" value={stats?.total_siswa} color="border-cyan-500" loading={loading} />
        <StatCard icon={GraduationCap} title="Total PTK" value={stats?.total_ptk} color="border-orange-500" loading={loading} />
        <StatCard icon={School} title="Total Rombel" value={stats?.total_rombel} color="border-fuchsia-500" loading={loading} />
        <StatCard icon={BookCopy} title="Total Jurusan" value={stats?.total_jurusan} color="border-lime-500" loading={loading} />
      </div>

      <div className="mt-8 text-gray-400">
        <p>Ringkasan data dan grafik lainnya akan ditampilkan di sini di masa mendatang.</p>
      </div>
    </div>
  );
} 