use crate::models::TrackPointSpeedSettings;
use dioxus::prelude::*;

const COLORS: [&str; 9] = [
    "#5E81AC", // 1
    "#81A1C1", // 2
    "#88C0D0", // 3
    "#A3BE8C", // 4
    "#EBCB8B", // 5
    "#D08770", // 6
    "#BF616A", // 7
    "#B48EAD", // 8
    "#E5E9F0", // 9
];

#[component]
pub fn TrackPointSpeedChart(tp_data: Signal<TrackPointSpeedSettings>) -> Element {
    const WIDTH: f32 = 600.0;
    const HEIGHT: f32 = 480.0;

    const MARGIN_LEFT: f32 = 72.0;
    const MARGIN_RIGHT: f32 = 150.0;
    const MARGIN_TOP: f32 = 28.0;
    const MARGIN_BOTTOM: f32 = 56.0;

    const X_MIN: f32 = 0.0;
    const X_MAX: f32 = 42.0;
    const Y_MIN: f32 = 0.0;
    const Y_MAX: f32 = 130.0;

    let plot_x = MARGIN_LEFT;
    let plot_y = MARGIN_TOP;
    let plot_w = WIDTH - MARGIN_LEFT - MARGIN_RIGHT;
    let plot_h = HEIGHT - MARGIN_TOP - MARGIN_BOTTOM;

    let data = tp_data.read();



    let x_ticks = [0, 8, 16, 24, 32, 40];
    let y_ticks = [0, 32, 64, 96, 128];

    let x_to_px = move |x: f32| plot_x + (x - X_MIN) / (X_MAX - X_MIN) * plot_w;
    let y_to_px = move |y: f32| plot_y + (1.0 - (y - Y_MIN) / (Y_MAX - Y_MIN)) * plot_h;

    let make_points = |speeds: &[i8]| {
        speeds
            .iter()
            .enumerate()
            .map(|(tilt, speed)| {
                let speed = (*speed as i32).clamp(Y_MIN as i32, Y_MAX as i32) as f32;
                let px = x_to_px(tilt as f32);
                let py = y_to_px(speed);
                format!("{px:.2},{py:.2}")
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    rsx! {
        div {
            class: "w-full rounded-2xl bg-gray-900 p-4",
            style: "max-width: 1280px;",

            // ===== Chart =====
            svg {
                width: WIDTH,
                height: HEIGHT,
                view_box: format!("0 0 {} {}", WIDTH, HEIGHT),
                class: "h-auto w-full",

                rect {
                    x: plot_x,
                    y: plot_y,
                    width: plot_w,
                    height: plot_h,
                    rx: 8,
                    ry: 8,
                    fill: "#111827",
                    stroke: "#3f3f46",
                    stroke_width: "1"
                }

                for tick in y_ticks {
                    line {
                        x1: x_to_px(X_MIN),
                        y1: y_to_px(tick as f32),
                        x2: x_to_px(X_MAX),
                        y2: y_to_px(tick as f32),
                        stroke: "#27272a",
                        stroke_width: "1"
                    }
                }

                for tick in x_ticks {
                    line {
                        x1: x_to_px(tick as f32),
                        y1: y_to_px(Y_MIN),
                        x2: x_to_px(tick as f32),
                        y2: y_to_px(Y_MAX),
                        stroke: "#27272a",
                        stroke_width: "1"
                    }
                }

                polyline {
                    fill: "none",
                    stroke: "#52525b",
                    stroke_width: "1.5",
                    stroke_dasharray: "5 4",
                    points: (0..=42)
                        .map(|tilt| {
                            let px = x_to_px(tilt as f32);
                            let py = y_to_px(tilt as f32);
                            format!("{px:.2},{py:.2}")
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                }

                line {
                    x1: x_to_px(X_MIN),
                    y1: y_to_px(Y_MIN),
                    x2: x_to_px(X_MAX),
                    y2: y_to_px(Y_MIN),
                    stroke: "#d4d4d8",
                    stroke_width: "1.5"
                }
                line {
                    x1: x_to_px(X_MIN),
                    y1: y_to_px(Y_MIN),
                    x2: x_to_px(X_MIN),
                    y2: y_to_px(Y_MAX),
                    stroke: "#d4d4d8",
                    stroke_width: "1.5"
                }

                for tick in x_ticks {
                    text {
                        x: x_to_px(tick as f32),
                        y: plot_y + plot_h + 22.0,
                        text_anchor: "middle",
                        font_size: "14",
                        fill: "#fafafa",
                        "{tick}"
                    }
                }

                for tick in y_ticks {
                    text {
                        x: plot_x - 10.0,
                        y: y_to_px(tick as f32) + 4.0,
                        text_anchor: "end",
                        font_size: "14",
                        fill: "#fafafa",
                        "{tick}"
                    }
                }

                text {
                    x: plot_x + plot_w / 2.0,
                    y: HEIGHT - 10.0,
                    text_anchor: "middle",
                    font_size: "16",
                    fill: "#e4e4e7",
                    "Tilt amount"
                }
                text {
                    x: 20,
                    y: plot_y + plot_h / 2.0,
                    transform: format!("rotate(-90 20 {})", plot_y + plot_h / 2.0),
                    text_anchor: "middle",
                    font_size: "16",
                    fill: "#e4e4e7",
                    "Cursor speed"
                }

                for i in 0..9 {
                    polyline {
                        fill: "none",
                        stroke: COLORS[i],
                        stroke_width: "2.0",
                        opacity: "1.0",
                        points: make_points(&data.coeffs[i])
                    }
                }

                for i in 0..9 {
                    {
                        let lx = WIDTH - 125.0;
                        let ly = 42.0 + i as f32 * 28.0;
                        rsx! {
                            line {
                                x1: lx,
                                y1: ly,
                                x2: lx + 22.0,
                                y2: ly,
                                stroke: COLORS[i],
                                stroke_width: "2",
                                opacity: "1.0"
                            }

                            text {
                                x: lx + 30.0,
                                y: ly + 4.0,
                                font_size: "14",
                                fill: "#d4d4d8",
                                font_weight: "400",
                                "Speed {i + 1}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TrackPointSpeedTableEditor(
    tp_data: Signal<TrackPointSpeedSettings>,
) -> Element {
    rsx! {
        div {
            class: "mb-3 text-lg font-semibold text-gray-300",
            "Manual settings"
        }

        div { class: "min-h-0 max-h-[600px] overflow-y-auto border border-gray-100",

            table { class: "table-fixed border-collapse text-xs text-gray-200",

                thead { class: "",
                    tr { class: "",

                        th {
                            class: "sticky left-0 top-0 z-30 bg-gray-800 px-3 py-2 text-center",
                            "Tilt"
                        }

                        for speed_idx in 0..9 {
                            th {
                                key: "speed-header-{speed_idx}",
                                class: "sticky top-0 z-20 bg-gray-800 px-3 py-2 text-center",
                                "Speed {speed_idx + 1}"
                            }
                        }
                    }
                }

                tbody {
                    for tilt in 0..43 {
                        tr {
                            key: "tilt-row-{tilt}",
                            class: "",

                            td {
                                class: "sticky left-0 z-10 bg-gray-800 text-center px-2 py-1 font-medium",
                                "{tilt}"
                            }

                            for speed_idx in 0..9 {
                                td {
                                    key: "cell-{tilt}-{speed_idx}",
                                    class: "border border-gray-700 p-1",

                                    input {
                                        class: "
                                            w-14
                                            bg-gray-900
                                            text-gray-100
                                            text-center
                                            focus:outline-none
                                            focus:ring-2 focus:ring-blue-500
                                            focus:border-blue-500
                                            transition
                                        ",
                                        r#type: "number",
                                        min: "0",
                                        max: "127",
                                        step: "1",
                                        value: "{tp_data().coeffs[speed_idx][tilt]}",
                                        oninput: move |evt| {
                                            if let Ok(value) = evt.value().parse::<i16>() {
                                                tp_data.write().coeffs[speed_idx][tilt] =
                                                    value.clamp(0, 127) as i8;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const PRESET_1: [[i8; 43]; 9] = [
        [
             0,     1,     1,     2,     2,     3,     3,     4,     4,     5,     5,     6,
             7,     7,     8,     8,     9,     9,    10,    10,    11,    12,    12,    13,
            13,    14,    14,    15,    15,    16,    16,    17,    18,    18,    19,    19,
            20,    20,    21,    21,    22,    23,    23,
        ],
        [
             0,     1,     2,     2,     3,     4,     4,     5,     6,     7,     7,     8,
             9,    10,    10,    11,    12,    13,    13,    14,    15,    16,    16,    17,
            18,    19,    19,    20,    21,    22,    22,    23,    24,    25,    25,    26,
            27,    28,    28,    29,    30,    31,    32,
        ],
        [
             0,     1,     2,     3,     3,     4,     5,     6,     7,     8,     9,     9,
            10,    11,    12,    13,    14,    14,    15,    16,    17,    18,    19,    20,
            20,    21,    22,    23,    24,    25,    26,    26,    27,    28,    29,    30,
            31,    31,    32,    33,    34,    35,    36,
        ],
        [
             0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,
            12,    13,    14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
            24,    25,    26,    27,    28,    29,    30,    31,    32,    33,    34,    35,
            36,    37,    38,    39,    40,    41,    42,
        ],
        [
             0,     1,     3,     4,     5,     6,     8,     9,    10,    11,    13,    14,
            15,    16,    18,    19,    20,    21,    23,    24,    25,    26,    28,    29,
            30,    31,    33,    34,    35,    36,    38,    39,    40,    41,    43,    44,
            45,    46,    48,    49,    50,    51,    53,
        ],
        [
             0,     1,     3,     5,     6,     8,     9,    11,    12,    14,    16,    17,
            19,    20,    22,    23,    25,    26,    28,    29,    31,    33,    34,    36,
            37,    39,    40,    42,    43,    45,    47,    48,    50,    51,    53,    54,
            56,    57,    59,    60,    62,    64,    65,
        ],
        [
             0,     1,     4,     6,     7,     9,    11,    13,    15,    17,    19,    20,
            22,    24,    26,    28,    30,    31,    33,    35,    37,    39,    41,    43,
            44,    46,    48,    50,    52,    54,    56,    57,    59,    61,    63,    65,
            67,    68,    70,    72,    74,    76,    78,
        ],
        [
             0,     1,     4,     6,     8,    10,    12,    14,    16,    18,    21,    23,
            25,    27,    29,    31,    33,    35,    37,    39,    41,    43,    45,    47,
            49,    51,    53,    55,    57,    59,    62,    64,    66,    68,    70,    72,
            74,    76,    78,    80,    82,    84,    86,
        ],
        [
             0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
            63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 93, 96, 99,102,105,108,111,114,117,120,123,
           126
        ],
    ];

const PRESET_2: [[i8; 43]; 9] = [
        [
             0,     1,     2,     2,     3,     4,     4,     5,     6,     7,     7,     8,
             9,    10,    10,    11,    12,    13,    13,    14,    15,    16,    16,    17,
            18,    19,    19,    20,    21,    22,    22,    23,    24,    25,    25,    26,
            27,    28,    28,    29,    30,    31,    32,
        ],
        [
             0,     1,     2,     3,     3,     4,     5,     6,     7,     8,     9,     9,
            10,    11,    12,    13,    14,    14,    15,    16,    17,    18,    19,    20,
            20,    21,    22,    23,    24,    25,    26,    26,    27,    28,    29,    30,
            31,    31,    32,    33,    34,    35,    36,
        ],
        [
             0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,
            12,    13,    14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
            24,    25,    26,    27,    28,    29,    30,    31,    32,    33,    34,    35,
            36,    37,    38,    39,    40,    41,    42,
        ],
        [
             0,     1,     3,     4,     5,     6,     8,     9,    10,    11,    13,    14,
            15,    16,    18,    19,    20,    21,    23,    24,    25,    26,    28,    29,
            30,    31,    33,    34,    35,    36,    38,    39,    40,    41,    43,    44,
            45,    46,    48,    49,    50,    51,    53,
        ],
        [
             0,     1,     3,     5,     6,     8,     9,    11,    12,    14,    16,    17,
            19,    20,    22,    23,    25,    26,    28,    29,    31,    33,    34,    36,
            37,    39,    40,    42,    43,    45,    47,    48,    50,    51,    53,    54,
            56,    57,    59,    60,    62,    64,    65,
        ],
        [
             0,     1,     4,     6,     7,     9,    11,    13,    15,    17,    19,    20,
            22,    24,    26,    28,    30,    31,    33,    35,    37,    39,    41,    43,
            44,    46,    48,    50,    52,    54,    56,    57,    59,    61,    63,    65,
            67,    68,    70,    72,    74,    76,    78,
        ],
        [
             0,     1,     4,     6,     8,    10,    12,    14,    16,    18,    21,    23,
            25,    27,    29,    31,    33,    35,    37,    39,    41,    43,    45,    47,
            49,    51,    53,    55,    57,    59,    62,    64,    66,    68,    70,    72,
            74,    76,    78,    80,    82,    84,    86,
        ],
        [
             0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
            63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 93, 96, 99,102,105,108,111,114,117,120,123,
           126
        ],
        [
             0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 75, 78,
            81, 84, 87, 90, 93, 96, 99,102,104,106,108,110,112,114,116,118,120,122,124,125,126,
           127
        ],
    ];

const PRESET_3: [[i8; 43]; 9] = [
        [
             0,     1,     2,     3,     3,     4,     5,     6,     7,     8,     9,     9,
            10,    11,    12,    13,    14,    14,    15,    16,    17,    18,    19,    20,
            20,    21,    22,    23,    24,    25,    26,    26,    27,    28,    29,    30,
            31,    31,    32,    33,    34,    35,    36,
        ],
        [
             0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,
            12,    13,    14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
            24,    25,    26,    27,    28,    29,    30,    31,    32,    33,    34,    35,
            36,    37,    38,    39,    40,    41,    42,
        ],
        [
             0,     1,     3,     4,     5,     6,     8,     9,    10,    11,    13,    14,
            15,    16,    18,    19,    20,    21,    23,    24,    25,    26,    28,    29,
            30,    31,    33,    34,    35,    36,    38,    39,    40,    41,    43,    44,
            45,    46,    48,    49,    50,    51,    53,
        ],
        [
             0,     1,     3,     5,     6,     8,     9,    11,    12,    14,    16,    17,
            19,    20,    22,    23,    25,    26,    28,    29,    31,    33,    34,    36,
            37,    39,    40,    42,    43,    45,    47,    48,    50,    51,    53,    54,
            56,    57,    59,    60,    62,    64,    65,
        ],
        [
             0,     1,     4,     6,     7,     9,    11,    13,    15,    17,    19,    20,
            22,    24,    26,    28,    30,    31,    33,    35,    37,    39,    41,    43,
            44,    46,    48,    50,    52,    54,    56,    57,    59,    61,    63,    65,
            67,    68,    70,    72,    74,    76,    78,
        ],
        [
             0,     1,     4,     6,     8,    10,    12,    14,    16,    18,    21,    23,
            25,    27,    29,    31,    33,    35,    37,    39,    41,    43,    45,    47,
            49,    51,    53,    55,    57,    59,    62,    64,    66,    68,    70,    72,
            74,    76,    78,    80,    82,    84,    86,
        ],
        [
             0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
            63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 93, 96, 99,102,105,108,111,114,117,120,123,
           126
        ],
        [
             0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 75, 78,
            81, 84, 87, 90, 93, 96, 99,102,104,106,108,110,112,114,116,118,120,122,124,125,126,
           127
        ],
        [
             0,  5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 84, 88, 90, 92,
            94, 96, 98,100,102,104,106,108,110,112,114,116,118,119,120,121,122,123,124,125,126,
           127
        ],
    ];

const PRESET_4: [[i8; 43]; 9] = [
        [
             0,     1,     2,     3,     4,     5,     6,     7,     8,     9,    10,    11,
            12,    13,    14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
            24,    25,    26,    27,    28,    29,    30,    31,    32,    33,    34,    35,
            36,    37,    38,    39,    40,    41,    42,
        ],
        [
             0,     1,     3,     4,     5,     6,     8,     9,    10,    11,    13,    14,
            15,    16,    18,    19,    20,    21,    23,    24,    25,    26,    28,    29,
            30,    31,    33,    34,    35,    36,    38,    39,    40,    41,    43,    44,
            45,    46,    48,    49,    50,    51,    53,
        ],
        [
             0,     1,     3,     5,     6,     8,     9,    11,    12,    14,    16,    17,
            19,    20,    22,    23,    25,    26,    28,    29,    31,    33,    34,    36,
            37,    39,    40,    42,    43,    45,    47,    48,    50,    51,    53,    54,
            56,    57,    59,    60,    62,    64,    65,
        ],
        [
             0,     1,     4,     6,     7,     9,    11,    13,    15,    17,    19,    20,
            22,    24,    26,    28,    30,    31,    33,    35,    37,    39,    41,    43,
            44,    46,    48,    50,    52,    54,    56,    57,    59,    61,    63,    65,
            67,    68,    70,    72,    74,    76,    78,
        ],
        [
             0,     1,     4,     6,     8,    10,    12,    14,    16,    18,    21,    23,
            25,    27,    29,    31,    33,    35,    37,    39,    41,    43,    45,    47,
            49,    51,    53,    55,    57,    59,    62,    64,    66,    68,    70,    72,
            74,    76,    78,    80,    82,    84,    86,
        ],
        [
             0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
            63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 93, 96, 99,102,105,108,111,114,117,120,123,
           126
        ],
        [
             0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 75, 78,
            81, 84, 87, 90, 93, 96, 99,102,104,106,108,110,112,114,116,118,120,122,124,125,126,
           127
        ],
        [
             0,  5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 84, 88, 90, 92,
            94, 96, 98,100,102,104,106,108,110,112,114,116,118,119,120,121,122,123,124,125,126,
           127
        ],
        [
             0,  6, 12, 18, 24, 30, 36, 42, 48, 54, 60, 66, 72, 78, 83, 88, 92, 96, 99,101,103,
           105,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,
           127
        ],
    ];

fn apply_preset(mut tp_data: Signal<TrackPointSpeedSettings>, preset: [[i8; 43]; 9]) {
    tp_data.write().coeffs = preset.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
}

#[component]
pub fn TrackPointSpeedPresetButtons(
    tp_data: Signal<TrackPointSpeedSettings>,
    default_coeffs: Vec<Vec<i8>>,
) -> Element {



    let button_class = "px-4 py-2 bg-gray-800 text-white rounded shadow hover:bg-gray-700 border border-white";

    rsx! {
        
        div {
            class: "mb-3 text-lg font-semibold text-gray-300",
            "Easy settings"
        }

        div {
            class: "mb-6 flex flex-wrap gap-4",

            button {
                class: "{button_class}",
                onclick: move |_| {
                    tp_data.write().coeffs = default_coeffs.clone();
                },
                "Set to default"
            }

            button {
                class: "{button_class}",
                onclick: move |_| apply_preset(tp_data, PRESET_1),
                "Preset 1"
            }

            button {
                class: "{button_class}",
                onclick: move |_| apply_preset(tp_data, PRESET_2),
                "Preset 2"
            }

            button {
                class: "{button_class}",
                onclick: move |_| apply_preset(tp_data, PRESET_3),
                "Preset 3"
            }

            button {
                class: "{button_class}",
                onclick: move |_| apply_preset(tp_data, PRESET_4),
                "Preset 4"
            }
        }
        div {
            svg {
                style: "width: 100%; height: 50px; margin-bottom: 20px;",
                view_box: "0 0 300 50",
                xmlns: "http://www.w3.org/2000/svg",
                line { x1: "0", y1: "25", x2: "280", y2: "25", stroke: "#f3f4f6", stroke_width: "2" },
                polygon { points: "280,20 290,25 280,30", fill: "#f3f4f6" },
                text { x: "300", y: "30", font_size: "16", font_family: "Arial", fill: "#f3f4f6", "faster" }
            }
        }
    }
}