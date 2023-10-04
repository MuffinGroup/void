import { component$ } from "@builder.io/qwik";
import { FancyButton } from "~/components/qwark/fancy-button/fancy_button";

export default component$(() => {
    return (
      <>
        <div>
          <FancyButton text="Create rust project" onButtonClick$={() => window.location.href="/project_manager/"} />
        </div>
      </>
    );
  });