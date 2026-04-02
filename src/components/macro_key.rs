use dioxus::prelude::*;
use std::collections::BTreeMap;
use std::sync::Arc;
use crate::models::{GeneralSeitting, MacroKey, KeyLabel};
// use crate::components::selects::SelectKeyID;


/// Combination Key Setting Component
/// Set of 8 boolean boxes (indicating left Ctrl, left Shift, left Alt, left GUI, right Ctrl, right Shift, right Alt, right GUI) and a select box for key ID
#[component]
pub fn MacroKeySetting(
    general_setting: Arc<GeneralSeitting>,
    map_key_label: BTreeMap<u8, KeyLabel>,
    macro_key_map: Signal<BTreeMap<u8, MacroKey>>,
) -> Element {
    let macro_key_map_read = macro_key_map.read();
    let (macro_key_map_read_01_12, macro_key_map_read_13_24) = macro_key_map_read.iter()
        .partition::<BTreeMap<_, _>, _>(|(tid, _)| *tid < &244);
    rsx! {
        div { class: "flex gap-12 px-4 divide-x divide-gray-200",
            div { class: "flex flex-col space-y-2",
                { macro_key_map_read_01_12.keys().map(|&trigger_id| {
                    let label = format!("Macro {:02}", trigger_id - 231);
                    rsx!(
                        div { class: "flex gap-4 py-2",
                            span { class: "text-right whitespace-nowrap", {label} },
                            SelectMacroKeyID {
                                general_setting: general_setting.clone(),
                                map_key_label: map_key_label.clone(),
                                macro_key_map,
                                trigger_id: *trigger_id
                            },
                            MacroKeyItem {
                                trigger_id: *trigger_id,
                                macro_key_map
                            }
                        }
                    )
                })}
            }
            div { class: "flex flex-col space-y-2 pl-8",
                { macro_key_map_read_13_24.keys().map(|&trigger_id| {
                    let label = format!("Macro {:02}", trigger_id - 231);
                    rsx!(
                        div { class: "flex gap-4 py-2",
                            span { class: "text-right whitespace-nowrap", {label} },
                            SelectMacroKeyID {
                                general_setting: general_setting.clone(),
                                map_key_label: map_key_label.clone(),
                                macro_key_map,
                                trigger_id: *trigger_id,
                            },
                            MacroKeyItem {
                                trigger_id: *trigger_id,
                                macro_key_map
                            }
                        }
                    )
                })}
            }

        }
    }
}

#[component]
pub fn SelectMacroKeyID(
    general_setting: Arc<GeneralSeitting>,
    map_key_label: BTreeMap<u8, KeyLabel>,
    macro_key_map: Signal<BTreeMap<u8, MacroKey>>,
    trigger_id: u8,
) -> Element {
    rsx!{
        div {
            class: "min-w-[12 rem]",
            select {
                class: "w-full px-2 py-1 border border-gray-300 rounded text-gray-700 text-sm",
                id: "options",
                value: macro_key_map().get(&trigger_id).unwrap().key_id,
                onchange: move |evt| {
                    let new_id: u8 = evt.value().clone().parse().unwrap();
                    let mut macro_key_map_mut = macro_key_map.write();
                    macro_key_map_mut.get_mut(&trigger_id).unwrap().key_id = new_id;
                },
                {
                    general_setting.avail_hid_usage_names
                        .iter()
                        .filter(|&(&kid, ref _name)| {kid < 213})
                        .map(|(&kid, name)| {
                            let (label, class) = match map_key_label.get(&kid) {
                                Some(ks) if !ks.default.is_empty() => {
                                    let label = if ks.shifted.is_empty() {
                                        ks.default.clone()
                                    } else {
                                        format!("{} and {}", ks.default, ks.shifted)
                                    };
                                    (label, "text-gray-700")
                                }
                                _ => (
                                    format!("{{ {:02X}: {} }}", kid, name),
                                    "text-gray-400",
                                ),
                            };

                            rsx! {
                                option {
                                    class: class,
                                    value: kid,
                                    label: label,
                                    selected: kid == macro_key_map().get(&trigger_id).unwrap().key_id,
                                }
                            }
                        })
                }
            }
        }
    }
}


#[component]
pub fn MacroKeyItem(
    trigger_id: u8,
    macro_key_map: Signal<BTreeMap<u8, MacroKey>>,
) -> Element {
    rsx! (
        div { class: "flex gap-3 text-sm",
            div {
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().left_ctrl,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().left_ctrl = evt.checked();
                        },
                    }
                    span { "LCtrl" }
                }
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().right_ctrl,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().right_ctrl = evt.checked();
                        },
                    }
                    span { "RCtrl" }
                }
            }
            div {
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().left_shift,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().left_shift = evt.checked();
                        },
                    }
                    span { "LShift" }
                }
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().right_shift,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().right_shift = evt.checked();
                        },
                    }
                    span { "RShift" }
                }
            }
            div {
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().left_alt,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().left_alt = evt.checked();
                        },
                    }
                    span { "LAlt" }
                }
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().right_alt,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().right_alt = evt.checked();
                        },
                    }
                    span { "RAlt" }
                }
            }
            div {
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().left_gui,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().left_gui = evt.checked();
                        },
                    }
                    span { "LWin" }
                }
                div { class: "flex items-center gap-1",
                    input {
                        r#type: "checkbox",
                        checked: macro_key_map().get(&trigger_id).unwrap().right_gui,
                        onchange: move |evt| {
                            let mut macro_key_map_mut = macro_key_map.write();
                            macro_key_map_mut.get_mut(&trigger_id).unwrap().right_gui = evt.checked();
                        },
                    }
                    span { "RWin" }
                }
            }
        }
    )
}