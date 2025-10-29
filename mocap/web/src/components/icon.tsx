import { JSX } from "solid-js/jsx-runtime";

export interface IconProps extends JSX.ImgHTMLAttributes<HTMLImageElement> {
}

export function Icon(props: IconProps) {
    return (
        <img
            class="manga-tools icon select-disable"
            style={{

            }}
            {...props}
        />
    );
}