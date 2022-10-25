import { invoke } from "@tauri-apps/api/tauri";
import { FunctionalComponent } from "preact";
import { useContext, useState } from "preact/hooks";
import { SettingsContext } from "../../contexts/settings";
import { GreeterInput } from "./input";

export const Greeter: FunctionalComponent = () => {
  const settings = useContext(SettingsContext);
  const [greetMsg, setGreetMsg] = useState("");

  const greet = async () => {
    console.debug(settings);
    setGreetMsg(await invoke("greet", { name: settings.name.value }));
  };

  return (
    <>
      <div class="row">
        <div>
          <GreeterInput />
          <button type="button" onClick={greet}>
            Greet
          </button>
        </div>
      </div>
      <p>{greetMsg}</p>
    </>
  );
};
