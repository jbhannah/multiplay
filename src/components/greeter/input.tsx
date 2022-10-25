import { FunctionalComponent } from "preact";
import { useContext } from "preact/hooks";
import { SettingsContext } from "../../contexts/settings";

export const GreeterInput: FunctionalComponent = () => {
  const settings = useContext(SettingsContext);

  return (
    <input
      id="greet-input"
      onChange={(e) => (settings.name.value = e.currentTarget.value)}
      placeholder="Enter a name..."
    />
  );
};
