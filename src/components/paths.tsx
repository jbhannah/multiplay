import { open } from "@tauri-apps/api/dialog";
import { useContext } from "preact/hooks";
import { SettingsContext } from "../contexts/settings";

export const Paths = () => {
  const settings = useContext(SettingsContext);

  const addPath = async () => {
    const selected = await open({
      directory: true,
    });

    if (selected) settings.addPaths(selected);

    console.debug(settings);
  };

  return (
    <>
      <button type="button" onClick={addPath}>
        Add path
      </button>
      <ul>
        {settings.paths.value.map((path) => (
          <li key={path}>{path}</li>
        ))}
      </ul>
    </>
  );
};
