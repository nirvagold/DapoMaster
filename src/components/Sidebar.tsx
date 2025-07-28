import { Home, Users, ChevronDown, ChevronRight, Book, CheckSquare, MoreHorizontal, Calendar, Clock } from "lucide-react";
import { useState } from "react";
import clsx from "clsx"; // Utility untuk conditional classes
import type { Semester, TahunAjaran } from "./PemilihanPenggunaView";

type MenuItem = {
  id: string;
  label: string;
  icon: React.ElementType;
  path?: string;
  children?: MenuItem[];
};

const menuItems: MenuItem[] = [
  { id: "dashboard", label: "Dashboard", icon: Home, path: "/dashboard" },
  {
    id: "data-siswa",
    label: "Data Siswa",
    icon: Users,
    path: "/siswa", // Base path for parent
    children: [
      { id: "daftar-siswa", label: "Daftar Siswa", icon: Book, path: "/siswa/daftar" },
      { id: "registrasi", label: "Registrasi", icon: CheckSquare, path: "/siswa/registrasi" },
      { id: "lainnya", label: "Lainnya", icon: MoreHorizontal, path: "/siswa/lainnya" },
    ],
  },
];

export default function Sidebar({ 
  onNavigate, 
  activePath, 
  semester, 
  tahunAjaran 
}: { 
  onNavigate: (path: string) => void; 
  activePath: string;
  semester: Semester | null;
  tahunAjaran: TahunAjaran | null;
}) {
  const [openMenus, setOpenMenus] = useState({ "data-siswa": true });

  const toggleMenu = (id: string) => {
    setOpenMenus(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const renderMenuItem = (item: MenuItem) => {
    const isOpen = item.children && openMenus[item.id];
    const isActive = activePath === item.path || (item.children && activePath.startsWith(item.path!));

    const menuItemClasses = clsx(
      "flex items-center p-2.5 rounded-md cursor-pointer transition-colors duration-200",
      {
        "bg-pink-600 text-white": isActive,
        "text-gray-400 hover:bg-gray-700 hover:text-white": !isActive,
      }
    );

    const subMenuItemClasses = (childPath: string) => clsx(
      "flex items-center p-2 rounded-md cursor-pointer transition-colors duration-200 text-sm",
      {
        "bg-pink-600/50 text-white font-semibold": activePath === childPath,
        "text-gray-400 hover:bg-gray-700 hover:text-white": activePath !== childPath,
      }
    );

    return (
      <div key={item.id} className="mb-1">
        <div onClick={() => (item.children ? toggleMenu(item.id) : onNavigate(item.path!))} className={menuItemClasses}>
          <item.icon size={20} className="mr-3" />
          <span className="flex-1 font-medium">{item.label}</span>
          {item.children && (
            <ChevronRight size={16} className={clsx("transition-transform", { "rotate-90": isOpen })} />
          )}
        </div>
        {isOpen && (
          <div className="ml-6 mt-2 space-y-1">
            {item.children?.map(child => (
              <div key={child.id} onClick={() => onNavigate(child.path!)} className={subMenuItemClasses(child.path!)}>
                <child.icon size={16} className="mr-2 opacity-80" />
                {child.label}
              </div>
            ))}
          </div>
        )}
      </div>
    );
  };

  return (
    <aside className="w-64 bg-gray-900 p-4 flex flex-col border-r border-gray-700">
      <div className="text-center mb-8">
        <h2 className="text-2xl font-bold text-pink-600">DapoMaster</h2>
      </div>
      
      {/* Informasi Semester dan Tahun Ajaran */}
      {(semester || tahunAjaran) && (
        <div className="mb-6 p-3 bg-gray-800 rounded-lg border border-gray-700">
          <h3 className="text-sm font-semibold text-pink-400 mb-2 flex items-center gap-2">
            <Calendar size={16} />
            Konteks Akademik
          </h3>
          <div className="space-y-2 text-xs">
            {tahunAjaran && (
              <div className="flex items-center gap-2">
                <Clock className="text-gray-400" size={12} />
                <span className="text-gray-300">{tahunAjaran.nama}</span>
              </div>
            )}
            {semester && (
              <div className="flex items-center gap-2">
                <Calendar className="text-gray-400" size={12} />
                <span className="text-gray-300">{semester.nama}</span>
              </div>
            )}
          </div>
        </div>
      )}
      
      <nav>{menuItems.map(renderMenuItem)}</nav>
    </aside>
  );
} 