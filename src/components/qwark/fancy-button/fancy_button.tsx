import { component$ } from "@builder.io/qwik";
import type { PropFunction } from "@builder.io/qwik";
import "./fancy_button.css";
import type { IClass } from "../../util";

interface IFancyButton extends IClass {
  text: string;
  /**
   * Allows you to set custom behaviour when button is clicked
   * Using this as a parameter: ```<FancyButton onButtonClick$={() => yourFunctionCall()}>```
   */
  onButtonClick$?: PropFunction;
}

export const FancyButton = component$((props: IFancyButton) => {
  return (
    <div class="fancy-button">
      <button class={props.class} onClick$={props.onButtonClick$}>
        {props.text}
      </button>
    </div>
  );
});
