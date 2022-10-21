use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::content::ToolSlotPreference;
use crate::randomizer::{config::ToggleOption, Config};

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

                if input.value().is_empty() {
                    config.max_cost = None;
                } else {
                    config.max_cost = input.value().parse::<u16>().ok();
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

            config.toggle_option(ToggleOption::DualWield);

            if !config.option_exists(ToggleOption::DualWield) {
                config.remove_option(ToggleOption::AlwaysDualWield);
            }

            config_handle.set(config);
        }
    };

    let on_duplicate_weapons_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::DuplicateWeapons);

            if !config.option_exists(ToggleOption::DuplicateWeapons) {
                config.remove_option(ToggleOption::AlwaysDuplicateWeapons);
            }

            config_handle.set(config);
        }
    };

    let on_custom_ammo_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::CustomAmmo);

            if !config.option_exists(ToggleOption::CustomAmmo) {
                config.remove_option(ToggleOption::AlwaysCustomAmmo);
            }

            config_handle.set(config);
        }
    };

    let on_quartermaster_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();

            config.toggle_option(ToggleOption::Quartermaster);

            if !config.option_exists(ToggleOption::Quartermaster) {
                config.remove_option(ToggleOption::AlwaysQuartermaster);
            }

            config_handle.set(config);
        }
    };

    let on_always_dual_wield_toggle = {
        let config_handle = config_handle.clone();
        let config = config.clone();

        move |_| {
            let mut config = config.clone();
            config.toggle_option(ToggleOption::AlwaysDualWield);
            config.remove_option(ToggleOption::AlwaysQuartermaster);
            config_handle.set(config);
        }
    };

    let always_dual_wield_html = if config.option_exists(ToggleOption::DualWield) {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.option_exists(ToggleOption::AlwaysDualWield)}
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
            config.toggle_option(ToggleOption::AlwaysDuplicateWeapons);
            config.remove_option(ToggleOption::AlwaysQuartermaster);
            config_handle.set(config);
        }
    };

    let always_duplicate_weapons_html = if config.option_exists(ToggleOption::DuplicateWeapons) {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.option_exists(ToggleOption::AlwaysDuplicateWeapons)}
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
            config.toggle_option(ToggleOption::AlwaysCustomAmmo);
            config_handle.set(config);
        }
    };

    let always_custom_ammo_html = if config.option_exists(ToggleOption::CustomAmmo) {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.option_exists(ToggleOption::AlwaysCustomAmmo)}
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
            config.toggle_option(ToggleOption::AlwaysQuartermaster);
            config.remove_option(ToggleOption::AlwaysDualWield);
            config.remove_option(ToggleOption::AlwaysDuplicateWeapons);
            config_handle.set(config);
        }
    };

    let always_quartermaster_html = if config.option_exists(ToggleOption::Quartermaster) {
        Some(html! {
            <label class={classes!("checkbox")}>
                <input
                    type="checkbox"
                    checked={config.option_exists(ToggleOption::AlwaysQuartermaster)}
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
                        let value = select.value();

                        config.tool_preferences[slot] =
                            value.clone().try_into().unwrap_or_else(|_| {
                                panic!("Cannot conversion {value} to tool_preferences slot.")
                            });
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
                                    value={config.max_cost.map_or_else(String::new, |cost| cost.to_string())}
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
                                checked={config.option_exists(ToggleOption::DualWield)}
                                onchange={on_dualwield_toggle}
                            />
                            {"Dual Wield"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.option_exists(ToggleOption::DuplicateWeapons)}
                                onchange={on_duplicate_weapons_toggle}
                            />
                            {"Duplicate Weapons"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.option_exists(ToggleOption::CustomAmmo)}
                                onchange={on_custom_ammo_toggle}
                            />
                            {"Custom Ammo"}
                        </label>
                    </div>

                    <div class={classes!("column")}>
                        <label class={classes!("checkbox")}>
                            <input
                                type="checkbox"
                                checked={config.option_exists(ToggleOption::Quartermaster)}
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
