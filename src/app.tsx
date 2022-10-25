import "./app.css";
import preactLogo from "./assets/preact.svg";
import { Greeter } from "./components/greeter";
import { SettingsProvider } from "./contexts/settings";

export const App = () => {
  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>
      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://preactjs.com" target="_blank">
          <img src={preactLogo} class="logo preact" alt="Preact logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Preact logos to learn more.</p>

      <SettingsProvider>
        <Greeter />
      </SettingsProvider>
    </div>
  );
};
