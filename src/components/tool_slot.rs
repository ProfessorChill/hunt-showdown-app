use yew::prelude::*;

use crate::content::GenericItem;
use crate::TRANSPARENT_B64;

#[derive(PartialEq, Properties)]
pub struct ToolSlotProps {
    pub tool: Option<GenericItem>,
    pub locked: bool,
    pub id: usize,
    pub on_tool_slot_clicked: Callback<usize>,
    pub on_tool_toggle_lock: Callback<usize>,
    pub on_tool_delete: Callback<usize>,
    pub on_tool_randomize: Callback<usize>,
}

#[function_component]
pub fn ToolSlot(props: &ToolSlotProps) -> Html {
    let ToolSlotProps {
        tool,
        locked,
        id,
        on_tool_slot_clicked,
        on_tool_toggle_lock,
        on_tool_delete,
        on_tool_randomize,
    } = props;

    let id = *id;

    let (lock_src, lock_alt) = if *locked {
        ("/images/icons/Lock.svg", "Locked")
    } else {
        ("/images/icons/Unlock.svg", "Unlocked")
    };

    let on_tool_slot_clicked_handle = {
        let on_tool_slot_clicked = on_tool_slot_clicked.clone();

        move |_: MouseEvent| {
            on_tool_slot_clicked.emit(id);
        }
    };

    let on_tool_toggle_lock_handle = {
        let on_tool_toggle_lock = on_tool_toggle_lock.clone();

        move |_: MouseEvent| {
            on_tool_toggle_lock.emit(id);
        }
    };

    let on_tool_delete_handle = {
        let on_tool_delete = on_tool_delete.clone();

        move |_: MouseEvent| {
            on_tool_delete.emit(id);
        }
    };

    let on_tool_randomize_handle = {
        let on_tool_randomize = on_tool_randomize.clone();

        move |_: MouseEvent| {
            on_tool_randomize.emit(id);
        }
    };

    if let Some(tool) = tool {
        let tool = tool.clone();

        html! {
            <div class={classes!("tool-slot")}>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_tool_toggle_lock_handle} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_tool_delete_handle} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_tool_randomize_handle} />
                </div>
                <img
                    class={classes!("tool-img", "item-selected")}
                    src={tool.to_tool_path()}
                    alt={tool.name.clone()}
                    onclick={on_tool_slot_clicked_handle}
                />
                <span class={classes!("tool-name")}>{tool.name}</span>
            </div>
        }
    } else {
        html! {
            <div class={classes!("tool-slot")}>
                <div class={classes!("item-actions")}>
                    <img src={lock_src} alt={lock_alt} onclick={on_tool_toggle_lock_handle} />
                    <img src="/images/icons/TrashCan.svg" alt="Delete" onclick={on_tool_delete_handle} />
                    <img src="/images/icons/Dice.svg" alt="Randomize" onclick={on_tool_randomize_handle} />
                </div>
                <img
                    class={classes!("tool-img")}
                    src={TRANSPARENT_B64}
                    alt={""}
                    onclick={on_tool_slot_clicked_handle}
                />
            </div>
        }
    }
}
