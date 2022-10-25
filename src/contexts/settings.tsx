import { Signal, signal } from "@preact/signals";
import { createContext, FunctionalComponent } from "preact";

export class Settings {
  name: Signal<String> = signal("");
  paths: Signal<String[]> = signal([]);

  addPaths(paths: String | String[]) {
    this.paths.value = Array.isArray(paths)
      ? [...this.paths.value, ...paths]
      : [...this.paths.value, paths];
  }
}

const settings = new Settings();

export const SettingsContext = createContext(settings);

export const SettingsProvider: FunctionalComponent = ({ children }) => (
  <SettingsContext.Provider value={settings}>
    {children}
  </SettingsContext.Provider>
);
