import { createRoot } from "react-dom/client";
import SplashScreen from "./SplashScreen";

const root = createRoot(document.getElementById("splash-root")!);
root.render(<SplashScreen />); 