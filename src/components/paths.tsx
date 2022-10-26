import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useContext } from "preact/hooks";
import { SettingsContext } from "../contexts/settings";

export const Paths = () => {
  const settings = useContext(SettingsContext);

  const addPath = async () => {
    const selected = await open({
      directory: true,
    });

    if (!selected) return;

    const output = Array.isArray(selected)
      ? selected.forEach(async (path) => {
          await invoke("add_path", { path, recursive: false });
        })
      : await invoke("add_path", { path: selected, recursive: false });

    console.debug(output);
  };

  return (
    <>
      <button type="button" onClick={addPath}>
        Add path
      </button>
    </>
  );
};
