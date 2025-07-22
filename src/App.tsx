import { useState } from "react";
import PemilihanPenggunaView, { Pengguna } from "./components/PemilihanPenggunaView";
import LogPanel from "./components/LogPanel";
import Sidebar from "./components/Sidebar";
import DashboardView from "./components/DashboardView";
import SiswaView from "./components/SiswaView";

function App() {
  const [user, setUser] = useState<Pengguna | null>(null);
  const [activePath, setActivePath] = useState("/dashboard");

  const renderContent = () => {
    if (activePath.startsWith("/siswa")) {
      const page = activePath.split("/")[2] || "daftar";
      const pageTitle = page.charAt(0).toUpperCase() + page.slice(1);
      return <SiswaView pageTitle={pageTitle} />;
  }
    return <DashboardView />;
  };

  return (
    <div className="flex h-screen bg-gray-800 text-gray-200">
      {!user ? (
        <div className="bg-gray-900 text-gray-200 min-h-screen flex flex-col justify-center items-center w-full">
          <PemilihanPenggunaView onLanjut={u => { setUser(u); console.log("User terpilih:", u); }} />
        </div>
      ) : (
        <>
          <Sidebar onNavigate={setActivePath} activePath={activePath} />
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
