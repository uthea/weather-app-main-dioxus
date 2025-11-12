use super::asset::*;
use dioxus::prelude::*;

#[component]
pub fn DailyForcast() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3.5",
            p { color: "hsl(0, 0%, 100%)", "Daily Forcast" }
            div { class: "flex gap-2.5",

                div {
                    class: "flex flex-col gap-2 items-center p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Tue" }
                    img { class: "w-12", src: RAIN_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "20°" }
                        span { color: "hsl(0, 0%, 100%)", "14°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center  p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Wed" }
                    img { class: "w-12", src: DRIZZLE_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "21°" }
                        span { color: "hsl(0, 0%, 100%)", "15°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Thu" }
                    img { class: "w-12", src: SUNNY_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "24°" }
                        span { color: "hsl(0, 0%, 100%)", "14°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Fri" }
                    img { class: "w-12", src: PARTLY_CLOUDY_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "25°" }
                        span { color: "hsl(0, 0%, 100%)", "13°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Sat" }
                    img { class: "w-12", src: STORM_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "21°" }
                        span { color: "hsl(0, 0%, 100%)", "15°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Sun" }
                    img { class: "w-12", src: SNOW_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "25°" }
                        span { color: "hsl(0, 0%, 100%)", "16°" }
                    }
                }

                div {
                    class: "flex flex-col gap-2 items-center  p-2 rounded-xl grow",
                    background_color: "hsl(243, 23%, 24%)",
                    p { color: "hsl(0, 0%, 100%)", "Mon" }
                    img { class: "w-12", src: FOG_ICON }

                    div { class: "flex justify-around gap-3",
                        span { color: "hsl(0, 0%, 100%)", "24°" }
                        span { color: "hsl(0, 0%, 100%)", "15°" }
                    }
                }
            

            }
        
        }

    }
}

#[component]
pub fn HourlyForcast() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 p-3.5 rounded-xl justify-between",
            background_color: "hsl(243, 23%, 24%)",
            div { class: "flex gap-4 items-center",
                p { class: "text-sm", color: "hsl(0, 0%, 100%)", "Hourly forecast" }
                button {
                    // TODO: dropdown action
                    div {
                        class: "flex gap-2 items-center py-1 px-2 rounded-lg",
                        style: "background-color: hsl(243, 23%, 30%)",
                        p {
                            style: "color: hsl(0, 0%, 100%)",
                            class: "text-sm",
                            "Tuesday"
                        }
                        img { src: DROPDOWN_ICON }
                    }
                }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "3 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "4 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "5 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "6 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "7 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "8 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "9 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }

            div {
                class: "flex gap-1 justify-between items-center p-2 rounded-md",
                style: "background-color: hsl(243, 23%, 30%)",
                div { class: "flex  gap-1 items-center",
                    img { class: "w-8", src: OVERCAST_ICON }
                    p { class: "text-sm", color: "hsl(0, 0%, 100%)", "10 PM" }
                }

                span { class: "text-xs", color: "hsl(0, 0%, 100%)", "20°" }
            }
        

        }
    }
}
