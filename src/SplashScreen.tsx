import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

const TOTAL_STEPS = 7;

export default function SplashScreen() {
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState(0);
  const [done, setDone] = useState<null | boolean>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let step = 0;
    const unlistenLog = listen<string>("tricky_log", (event) => {
      setLogs((prev) => [...prev, event.payload || ""]);
      step++;
      setProgress(step);
    });
    const unlistenDone = listen<{ success: boolean; error?: string }>("tricky_done", (event) => {
      setDone(event.payload.success);
      if (!event.payload.success && event.payload.error) setError(event.payload.error);
      setTimeout(() => {
        if (event.payload.success) {
          Window.getCurrent().close(); // Splash window close
        }
      }, 1200);
    });
    return () => {
      unlistenLog.then((f) => f());
      unlistenDone.then((f) => f());
    };
  }, []);

  return (
    <div style={{
      width: "100%",
      height: "100vh",
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      justifyContent: "center",
      background: "#1e293b",
      color: "#fff",
      fontFamily: "sans-serif"
    }}>
      <h2 style={{ marginBottom: 8 }}>DapoMaster Initializing...</h2>
      <div style={{ width: 320, height: 16, background: "#334155", borderRadius: 8, overflow: "hidden", marginBottom: 16 }}>
        <div style={{ width: `${(progress / TOTAL_STEPS) * 100}%`, height: "100%", background: "#38bdf8", transition: "width 0.3s" }} />
      </div>
      <div style={{ width: 320, minHeight: 80, background: "#0f172a", borderRadius: 8, padding: 12, fontSize: 14, boxSizing: "border-box", marginBottom: 16 }}>
        {logs.map((log, i) => (
          <div key={i} style={{ color: log.includes("gagal") ? "#f87171" : "#fff" }}>{log}</div>
        ))}
        {done === null && <div style={{ color: "#94a3b8" }}>Memulai tricky method...</div>}
        {done === true && <div style={{ color: "#4ade80" }}>Selesai! Membuka aplikasi...</div>}
        {done === false && <div style={{ color: "#f87171" }}>Gagal: {error}</div>}
      </div>
      <div style={{ fontSize: 12, color: "#94a3b8" }}>© {new Date().getFullYear()} DapoMaster</div>
    </div>
  );
} 