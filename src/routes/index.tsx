import { $, component$ } from "@builder.io/qwik";
import type { DocumentHead } from "@builder.io/qwik-city";
import { FancyButton } from "~/components/qwark/fancy-button/fancy_button";
import { invoke } from "@tauri-apps/api/tauri";

export default component$(() => {
  /**
   * Creates a project. Valid parameters are: "rs", "nx", "ts"
   */
  const create_proj = $(async function create_proj(name: string, proj_type: string) {
    await invoke("create_proj", { name, proj_type });
  });

  return (
    <>
      <div>
        <FancyButton text="Create rust project" onButtonClick$={() => create_proj("test", "rs")} />
        <FancyButton text="Create typescript project" onButtonClick$={() => create_proj("test", "ts")} />
      </div>
    </>
  );
});

export const head: DocumentHead = {
  title: "Welcome to Qwik",
  meta: [
    {
      name: "description",
      content: "Qwik site description",
    },
  ],
};
