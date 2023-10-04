import { component$ } from "@builder.io/qwik";
import "./image_button.css";
import { IClass } from "../../util";

interface IImageButton extends IClass {
  imgClass?: string
  alt: string;
  borderRadius?: number
  onClick?: string;
  img_src: string;
  img_w: number;
  img_h: number;
}

export const ImageButton = component$((props: IImageButton) => {
    return (
      <div
      class="image-button" style={{width: "100"}}>
        <button class={props.class}>
          <img
            width={props.img_w}
            height={props.img_h}
            alt={props.alt}
            src={props.img_src}
            class={props.imgClass}
          />
        </button>
      </div>
    );
  }
);
