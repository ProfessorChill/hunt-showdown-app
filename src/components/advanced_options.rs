use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::content::ToolSlotPreference;
use crate::randomizer::Config;

#[derive(PartialEq, Properties)]
pub struct AdvancedOptionsProps {
    pub is_active: bool,
    pub config: Config,
    pub on_options_close: Callback<Config>,
}

#[function_component]
pub fn AdvancedOptions(props: &AdvancedOptionsProps) -> Html {
    let AdvancedOptionsProps {
        is_active,
        config,
        on_options_close,
    } = props;

    let config_handle = use_state(|| config.clone());
    let config = (*config_handle).clone();

    let on_options_close_click = {
        let on_options_close = on_options_close.clone();
        let config = config.clone();

        move |_| {
            on_options_close.emit(config.clone());
        }
    };

    let on_bloodline_rank_input = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let mut config = config.clone();

                if !input.value().is_empty() {
                    config.max_rank = input.value().parse::<u8>().unwrap_or(100);
                    config_handle.set(config);
                }
            }
        }
    };

    let on_max_cost_input = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();

            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let mut config = config.clone();

                if !input.value().is_empty() {
                    config.max_cost = match input.value().parse::<u16>() {
                        Ok(cost) => Some(cost),
                        Err(_) => None,
                    };
                } else {
                    config.max_cost = None;
                }

                config_handle.set(config);
            }
        }
    };

    let on_dualwield_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.dual_wield = !config.dual_wield;
            if !config.dual_wield {
                config.always_dual_wield = false;
            }
            config_handle.set(config);
        }
    };

    let on_duplicate_weapons_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.duplicate_weapons = !config.duplicate_weapons;
            if !config.duplicate_weapons {
                config.always_duplicate_weapons = false;
            }
            config_handle.set(config);
        }
    };

    let on_custom_ammo_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.custom_ammo = !config.custom_ammo;
            if !config.custom_ammo {
                config.always_custom_ammo = false;
            }
            config_handle.set(config);
        }
    };

    let on_quartermaster_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.quartermaster = !config.quartermaster;
            if !config.quartermaster {
                config.always_quartermaster = false;
            }
            config_handle.set(config);
        }
    };

    let on_always_dual_wield_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.always_dual_wield = !config.always_dual_wield;
            config.always_quartermaster = false;
            config_handle.set(config);
        }
    };

    let always_dual_wield_html = if config.dual_wield {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.always_dual_wield}
                    onchange={on_always_dual_wield_toggle}
                />
                {"Always Dual Wield"}
            </label>
        })
    } else {
        None
    };

    let on_always_duplicate_weapons_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.always_duplicate_weapons = !config.always_duplicate_weapons;
            config.always_quartermaster = false;
            config_handle.set(config);
        }
    };

    let always_duplicate_weapons_html = if config.duplicate_weapons {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.always_duplicate_weapons}
                    onchange={on_always_duplicate_weapons_toggle}
                />
                {"Always Duplicate Weapons"}
            </label>
        })
    } else {
        None
    };

    let on_always_custom_ammo_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.always_custom_ammo = !config.always_custom_ammo;
            config_handle.set(config);
        }
    };

    let always_custom_ammo_html = if config.custom_ammo {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.always_custom_ammo}
                    onchange={on_always_custom_ammo_toggle}
                />
                {"Always Custom Ammo"}
            </label>
        })
    } else {
        None
    };

    let on_always_quartermaster_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.always_quartermaster = !config.always_quartermaster;
            config.always_dual_wield = false;
            config.always_duplicate_weapons = false;
            config_handle.set(config);
        }
    };

    let always_quartermaster_html = if config.quartermaster {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.always_quartermaster}
                    onchange={on_always_quartermaster_toggle}
                />
                {"Always Quartermaster"}
            </label>
        })
    } else {
        None
    };

    let is_active = if *is_active { Some("is-active") } else { None };

    let tool_preferences_html = config
        .tool_preferences
        .iter()
        .enumerate()
        .map(|(slot, slot_preference)| {
            let config_handle = config_handle.clone();
            let config = config.clone();

            let on_select_changed = {
                move |e: Event| {
                    let target: Option<EventTarget> = e.target();

                    let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

                    if let Some(select) = select {
                        let mut config = config.clone();
                        config.tool_preferences[slot] = select.value().into();
                        config_handle.set(config);
                    }
                }
            };

            html! {
                <div class={classes!("column")}>
                    <div class={classes!("select")}>
                        <select onchange={on_select_changed}>
                            <option
                                selected={*slot_preference == ToolSlotPreference::NoPreference}
                                value={ToolSlotPreference::NoPreference.to_string()}
                            >{"No Preference"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Medkit}
                                value={ToolSlotPreference::Medkit.to_string()}
                            >{"Medkit"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Melee}
                                value={ToolSlotPreference::Melee.to_string()}
                            >{"Melee"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Throwable}
                                value={ToolSlotPreference::Throwable.to_string()}
                            >{"Throwable"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Tripmines}
                                value={ToolSlotPreference::Tripmines.to_string()}
                            >{"Tripmines"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Decoys}
                                value={ToolSlotPreference::Decoys.to_string()}
                            >{"Decoys"}</option>
                            <option
                                selected={*slot_preference == ToolSlotPreference::Others}
                                value={ToolSlotPreference::Others.to_string()}
                            >{"Others"}</option>
                        </select>
                    </div>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!("modal", is_active)}>
            <div class={classes!("modal-background")} onclick={on_options_close_click.clone()}></div>

            <div class={classes!("modal-content", "options-container")}>
                <p class={classes!("subtitle", "has-text-centered")}>{"General Limits"}</p>

                <div class={classes!("columns", "is-centered")}>
                    <div class={classes!("column")}>
                        <div class={classes!("field")}>
                            <label class={classes!("label")}>{"Bloodline Rank (Ignored for now)"}</label>
                            <div class={classes!("control")}>
                                <input
                                    class={classes!("input")}
                                    type="number"
                                    placeholder={"Bloodline Rank"}
                                    value={config.max_rank.to_string()}
                                    oninput={on_bloodline_rank_input}
                                />
                            </div>
                        </div>
                    </div>


                    <div class={classes!("column")}>
                        <div class={classes!("field")}>
                            <label class={classes!("label")}>{"Max Cost"}</label>
                            <div class={classes!("control")}>
                                <input
                                    class={classes!("input")}
                                    type="number"
                                    placeholder={"Max Cost"}
                                    value={match config.max_cost {
                                        Some(cost) => cost.to_string(),
                                        None => "".to_string(),
                                    }}
                                    oninput={on_max_cost_input}
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <p class={classes!("subtitle", "has-text-centered")}>{"General Options"}</p>

                <div class={classes!("columns")}>
                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.dual_wield}
                                onchange={on_dualwield_toggle}
                            />
                            {"Dual Wield"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.duplicate_weapons}
                                onchange={on_duplicate_weapons_toggle}
                            />
                            {"Duplicate Weapons"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.custom_ammo}
                                onchange={on_custom_ammo_toggle}
                            />
                            {"Custom Ammo"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.quartermaster}
                                onchange={on_quartermaster_toggle}
                            />
                            {"Quartermaster"}
                        </label>
                    </div>
                </div>

                <p class={classes!("subtitle", "has-text-centered")}>{"Always Toggles"}</p>

                <div class={classes!("columns")}>
                    <div class={classes!("column")}>{always_dual_wield_html}</div>
                    <div class={classes!("column")}>{always_duplicate_weapons_html}</div>
                    <div class={classes!("column")}>{always_custom_ammo_html}</div>
                    <div class={classes!("column")}>{always_quartermaster_html}</div>
                </div>

                <p class={classes!("subtitle", "has-text-centered")}>{"Tool Preferences"}</p>

                <div class={classes!("columns")}>
                    {tool_preferences_html}
                </div>

                <div class={classes!("columns", "is-centered")}>
                    <div class={classes!("column", "is-flex-grow-0")}>
                        <button
                            class={classes!("button", "is-primary")}
                            onclick={on_options_close_click.clone()}
                        >{"Confirm"}</button>
                    </div>
                </div>
            </div>

            <button
                class={classes!("modal-close", "is-large")}
                aria-label="close"
                onclick={on_options_close_click}
            ></button>
        </div>
    }
}
