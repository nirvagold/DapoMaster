import { useState } from "react";
import PemilihanPenggunaView, { Pengguna, Semester, TahunAjaran } from "./components/PemilihanPenggunaView";
import Sidebar from "./components/Sidebar";
import DashboardView from "./components/DashboardView";
import SiswaView from "./components/SiswaView";
import LulusanView from "./components/LulusanView";
import KeluarView from "./components/KeluarView";
import ValidasiView from "./components/ValidasiView";
import LogPanel from "./components/LogPanel";

function App() {
  const [user, setUser] = useState<Pengguna | null>(null);
  const [selectedSemester, setSelectedSemester] = useState<Semester | null>(null);
  const [selectedTahunAjaran, setSelectedTahunAjaran] = useState<TahunAjaran | null>(null);
  const [activePath, setActivePath] = useState("/dashboard");

  const renderContent = () => {
    if (activePath.startsWith("/siswa")) {
      const page = activePath.split("/")[2] || "daftar";
      const pageTitle = page.charAt(0).toUpperCase() + page.slice(1);
      return <SiswaView pageTitle={pageTitle} user={user} semester={selectedSemester} tahunAjaran={selectedTahunAjaran} />;
    }
    
    if (activePath.startsWith("/lulusan")) {
      const page = activePath.split("/")[2] || "daftar";
      const pageTitle = page.charAt(0).toUpperCase() + page.slice(1);
      return <LulusanView pageTitle={pageTitle} semester={selectedSemester} tahunAjaran={selectedTahunAjaran} />;
    }
    
    if (activePath.startsWith("/keluar")) {
      const page = activePath.split("/")[2] || "daftar";
      const pageTitle = page.charAt(0).toUpperCase() + page.slice(1);
      return <KeluarView pageTitle={pageTitle} semester={selectedSemester} tahunAjaran={selectedTahunAjaran} />;
    }
    
    if (activePath === "/validasi") {
      return <ValidasiView user={user} />;
    }
    
    return <DashboardView semester={selectedSemester} tahunAjaran={selectedTahunAjaran} />;
  };

  return (
    <div className="flex h-screen bg-gray-800 text-gray-200">
      {!user ? (
        <div className="bg-gray-900 text-gray-200 min-h-screen flex flex-col justify-center items-center w-full">
          <PemilihanPenggunaView onLanjut={(u, semester, tahunAjaran) => { 
            setUser(u); 
            setSelectedSemester(semester);
            setSelectedTahunAjaran(tahunAjaran);
          }} />
        </div>
      ) : (
        <>
          <Sidebar onNavigate={setActivePath} activePath={activePath} semester={selectedSemester} tahunAjaran={selectedTahunAjaran} />
          <main className="flex-1 p-6 overflow-y-auto">
            {renderContent()}
    </main>
        </>
      )}
      <LogPanel />
    </div>
  );
}

export default App;
