import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import clsx from "clsx";
import { ChevronDown, Trash2, Server, Code } from "lucide-react";

export type LogEntry = {
  source: "frontend" | "backend";
  type: "log" | "info" | "error" | "warn";
  message: string;
  time: string;
};

type ConsoleMethod = "log" | "warn" | "error" | "info";

export default function LogPanel() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isExpanded, setIsExpanded] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  const addLog = (source: "frontend" | "backend", type: LogEntry["type"], message: string) => {
    setLogs(prevLogs => [
      ...prevLogs.slice(-100), // Batasi jumlah log
      { source, type, message, time: new Date().toLocaleTimeString() }
    ]);
  };
  
  // Monkey-patch console
  useEffect(() => {
    const originalConsole: { [key in ConsoleMethod]?: (...args: any[]) => void } = {};
    const methodsToOverride: ConsoleMethod[] = ["log", "warn", "error", "info"];

    methodsToOverride.forEach(method => {
      originalConsole[method] = console[method];
      console[method] = (...args: any[]) => {
        originalConsole[method]?.apply(console, args);
        const message = args.map(arg => 
          typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
        ).join(' ');
        addLog("frontend", method, message);
      };
    });

    return () => { // Cleanup
      methodsToOverride.forEach(method => {
        console[method] = originalConsole[method]!;
      });
    };
  }, []);

  // Listen for backend logs
  useEffect(() => {
    const unlisten = listen<string>("backend_log", event => {
      addLog("backend", "info", event.payload || "");
    });
    return () => { unlisten.then(f => f()); };
  }, []);
  
  // Auto-scroll
  useEffect(() => {
    if (isExpanded) {
      bottomRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs, isExpanded]);

  return (
    <div className="fixed bottom-0 left-0 right-0 z-50 text-white">
      <div 
        className="bg-gray-800 border-t border-gray-700 px-4 py-1 flex items-center cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <ChevronDown size={18} className={clsx("transition-transform mr-2", { "rotate-180": isExpanded })} />
        <span className="font-bold text-sm">Log Panel</span>
        <div className="flex-grow" />
        <button 
          onClick={(e) => { e.stopPropagation(); setLogs([]); }}
          className="p-1 rounded-full hover:bg-gray-600"
        >
          <Trash2 size={14} />
        </button>
      </div>
      {isExpanded && (
        <div className="h-40 bg-gray-900/95 p-2 backdrop-blur-sm overflow-y-auto font-mono text-xs">
          {logs.map((log, i) => (
            <div key={i} className={clsx("flex items-start gap-3 p-1 rounded", {
              "text-red-400": log.type === "error",
              "text-yellow-400": log.type === "warn",
            })}>
              <span className="opacity-60">[{log.time}]</span>
              {log.source === 'backend' ? <Server size={14} className="flex-shrink-0 mt-0.5" /> : <Code size={14} className="flex-shrink-0 mt-0.5" />}
              <span className="font-semibold">{log.type}</span>
              <pre className="whitespace-pre-wrap flex-grow">{log.message}</pre>
            </div>
          ))}
          <div ref={bottomRef} />
        </div>
      )}
    </div>
  );
} 