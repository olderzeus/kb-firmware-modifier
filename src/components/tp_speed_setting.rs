use dioxus::prelude::*;
use crate::models::TrackPointAccelerationCoeffs;

#[component]
pub fn TrackPointSpeedChart(
    tp_data: Signal<TrackPointAccelerationCoeffs>,
) -> Element {
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
    // let selected_speed = 4;

    let colors = [
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

    let x_ticks = [0, 8, 16, 24, 32, 40, 42];
    let y_ticks = [0, 32, 64, 96, 128];

    let x_to_px = move |x: f32| plot_x + (x - X_MIN) / (X_MAX - X_MIN) * plot_w;
    let y_to_px = move |y: f32| plot_y + (1.0 - (y - Y_MIN) / (Y_MAX - Y_MIN)) * plot_h;

    let make_points = |coeffs: &[i8]| {
        coeffs
            .iter()
            .enumerate()
            .map(|(tilt, coeff)| {
                let speed = (0 + *coeff as i32).clamp(Y_MIN as i32, Y_MAX as i32) as f32;
                let px = x_to_px(tilt as f32);
                let py = y_to_px(speed);
                format!("{px:.2},{py:.2}")
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    rsx! {
        div {
            class: "w-full rounded-2xl border border-zinc-700 bg-zinc-900 p-4 shadow-lg",
            style: "max-width: 980px;",
            /*
            div {
                class: "mb-3 flex items-center justify-between",
                div {
                    class: "text-sm font-semibold text-zinc-100",
                    "TrackPoint Speed Curve"
                }
                div {
                    class: "text-xs text-zinc-400",
                    "Cursor speed = tilt level + acceleration coefficient"
                }
            }
            */
            
            svg {
                width: WIDTH,
                height: HEIGHT,
                view_box: format!("0 0 {} {}", WIDTH, HEIGHT),
                class: "h-auto w-full",

                // Background panel for plot area
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

                // Horizontal grid
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

                // Vertical grid
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

                // Reference line y = x
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

                // Axes
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

                // X tick labels
                for tick in x_ticks {
                    text {
                        x: x_to_px(tick as f32),
                        y: plot_y + plot_h + 22.0,
                        text_anchor: "middle",
                        font_size: "12",
                        fill: "#fafafa",
                        "{tick}"
                    }
                }

                // Y tick labels
                for tick in y_ticks {
                    text {
                        x: plot_x - 10.0,
                        y: y_to_px(tick as f32) + 4.0,
                        text_anchor: "end",
                        font_size: "12",
                        fill: "#fafafa",
                        "{tick}"
                    }
                }

                // Axis titles
                text {
                    x: plot_x + plot_w / 2.0,
                    y: HEIGHT - 10.0,
                    text_anchor: "middle",
                    font_size: "13",
                    fill: "#e4e4e7",
                    "Tilt amount"
                }
                text {
                    x: 20,
                    y: plot_y + plot_h / 2.0,
                    transform: format!("rotate(-90 20 {})", plot_y + plot_h / 2.0),
                    text_anchor: "middle",
                    font_size: "13",
                    fill: "#e4e4e7",
                    "Cursor speed"
                }

                // All series
                for i in 0..9 {
                    // rsx! {
                        polyline {
                            fill: "none",
                            stroke: colors[i],
                            // stroke_width: if i == selected_speed { "3.5" } else { "1.8" },
                            // opacity: if i == selected_speed { "1.0" } else { "0.45" },
                            stroke_width: "2.0",
                            opacity: "1.0",
                            points: make_points(&data.coeffs[i])
                        }
                    // }
                }

                // Legend
                    for i in 0..9 {
                        {
                            let lx = WIDTH - 125.0;
                            let ly = 42.0 + i as f32 * 28.0;
                            // let active = i == selected_speed;
                            rsx! {
                                rect {
                                    x: lx - 8.0,
                                    y: ly - 12.0,
                                    width: 102.0,
                                    height: 20.0,
                                    rx: 6,
                                    ry: 6,
                                    // fill: if active { "#27272a" } else { "transparent" },
                                    // stroke: if active { "#52525b" } else { "transparent" },
                                    fill: "transparent",
                                    stroke: "transparent",
                                    stroke_width: "1"
                                }

                                line {
                                    x1: lx,
                                    y1: ly,
                                    x2: lx + 22.0,
                                    y2: ly,
                                    stroke: colors[i],
                                    // stroke_width: if active { "3.5" } else { "2" },
                                    // opacity: if active { "1.0" } else { "0.65" }
                                    stroke_width: "2",
                                    opacity: "1.0"
                                }

                                text {
                                    x: lx + 30.0,
                                    y: ly + 4.0,
                                    // font_size: if active { "13" } else { "12" },
                                    // fill: if active { "#ffffff" } else { "#d4d4d8" },
                                    // font_weight: if active { "700" } else { "400" },
                                    font_size: "13",
                                    fill: "#d4d4d8",
                                    font_weight: "400",
                                    "Speed {i + 1}"
                                }
                            }
                        }

                    }


                // Selected series endpoint marker
                /*
                {
                    let last_tilt = 42usize;
                    let coeff = data.coeffs[selected_speed][last_tilt];
                    let speed = (last_tilt as i32 + coeff as i32).clamp(Y_MIN as i32, Y_MAX as i32) as f32;
                    let cx = x_to_px(last_tilt as f32);
                    let cy = y_to_px(speed);

                    rsx! {
                        circle {
                            cx: cx,
                            cy: cy,
                            r: "4.5",
                            fill: colors[selected_speed],
                            stroke: "#ffffff",
                            stroke_width: "1.5"
                        }
                    }
                }
                */
            }
            
            
            /*
            div {
                class: "mt-3 flex flex-wrap gap-x-6 gap-y-1 text-xs text-zinc-400",
                div { "Range: tilt 0-43" }
                div { "Output range: 0-170" }
                div { "Selected: Speed {selected_speed + 1}" }
                div { "Dashed line: linear reference (y = x)" }
            }
            */
        
        }
    }
}