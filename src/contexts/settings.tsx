import { Signal, signal } from "@preact/signals";
import { createContext, FunctionalComponent } from "preact";

export class Settings {
  name: Signal<String> = signal("");
}

const settings = new Settings();

export const SettingsContext = createContext(settings);

export const SettingsProvider: FunctionalComponent = ({ children }) => (
  <SettingsContext.Provider value={settings}>
    {children}
  </SettingsContext.Provider>
);
