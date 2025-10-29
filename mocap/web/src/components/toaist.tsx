import { Toast, toaster } from "@kobalte/core/toast";

import "./toast.css";
import CloseIcon from "../close.svg?component-solid";

export interface ShowToastProps {
    message: string,
}
export function showToast(props: ShowToastProps) {
    console.log("Showing toast", props);

    const id = toaster.show(pp => (
        <Toast toastId={pp.toastId} class="toast">
            <div
                style={{
                    display: "flex",
                    "flex-direction": "row",
                    gap: "10px",
                    "align-content": "center",
                    "align-items": "center",
                    "padding-left": "20px"
                }}
            >
                <Toast.Description style={{
                    "font-weight": 600,
                    color: "white"
                }}>
                    {props.message}
                </Toast.Description>
                <Toast.CloseButton
                    class="toast__close-button">
                    <CloseIcon />
                </Toast.CloseButton>
            </div>
        </Toast>
    ));
}