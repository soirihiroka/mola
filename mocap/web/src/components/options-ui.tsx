import { Accessor, createSignal, For, Setter } from "solid-js";
import { DropdownMenu } from "@kobalte/core/dropdown-menu";

// import objSelectIcon from "../../icons/adwaita-icon-theme/actions/object-select-symbolic.svg"
// import downIcon from "../../icons/adwaita-icon-theme/ui/pan-down-symbolic.svg"

export interface DropdownUIProps {
    value: Accessor<string>;
    setValue: Setter<string>;
    options: Accessor<string[]>;
}

export function OptionsUI(props: DropdownUIProps) {
    const [open, setOpen] = createSignal(false);
    return (
        <DropdownMenu open={open()} onOpenChange={setOpen}>
            <DropdownMenu.Trigger class={open() ? "dropdown-menu__trigger open" : "dropdown-menu__trigger"}>
                {props.value()}
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
                <DropdownMenu.Content
                    class="dropdown-menu__content"
                >
                    <DropdownMenu.Arrow />
                    <DropdownMenu.RadioGroup value={props.value()} onChange={props.setValue}>
                        <For
                            each={props.options()}
                        >
                            {
                                option => <DropdownMenu.RadioItem value={option}
                                    onSelect={() => { setOpen(false) }}
                                    class="dropdown-menu__radio-item"
                                    style={{
                                        display: "flex", "flex-direction": "row"
                                    }}
                                >
                                    <div
                                        class="dropdown-menu__radio-text"
                                    >
                                        {option}
                                    </div>
                                    <DropdownMenu.ItemIndicator class="dropdown-menu__item-indicator">
                                        {/* <img src={objSelectIcon}></img> */}
                                    </DropdownMenu.ItemIndicator>
                                </DropdownMenu.RadioItem>
                            }
                        </For>
                    </DropdownMenu.RadioGroup>
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu>
    );
}