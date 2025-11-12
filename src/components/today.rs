use dioxus::prelude::*;

const TODAY_BG: Asset = asset!("/assets/images/bg-today-large.svg");
const SUNNY: Asset = asset!("/assets/images/icon-sunny.webp");

#[component]
pub fn TodayHeader() -> Element {
    rsx! {
        div {
            background_image: "url({TODAY_BG})",
            class: "bg-no-repeat rounded-r-3xl bg-cover",

            div { class: "flex pt-28 pb-24 justify-between pl-5 pr-5",
                div { class: "flex flex-col gap-1.5",
                    h2 { class: "text-2xl", style: "color: hsl(0, 0%, 100%)", "Berlin, Germany" }
                    h3 { class: "text-xl", color: "hsl(250, 6%, 84%)", "Tuesday, Aug 5, 2025" }
                }

                div { class: "flex gap-3",
                    img { width: "78px", height: "auto", src: SUNNY }
                    h1 { class: "text-7xl", color: "hsl(0, 0%, 100%)", "20°" }
                }
            }
        }
    }
}

#[component]
pub fn TodayDetail() -> Element {
    rsx! {
        div { class: "flex gap-5 justify-between",

            div {
                class: "flex flex-col gap-3 pt-2 pb-2 pl-4 pr-15 rounded-xl grow",
                background_color: "hsl(243, 23%, 24%)",
                p { class: "text-sm", color: "hsl(250, 6%, 70%)", "Feels like" }
                p { class: "text-2xl", color: "hsl(250, 6%, 84%)", "20°" }
            }

            div {
                class: "flex flex-col gap-3 pt-2 pb-2 pl-4 pr-15 rounded-xl grow",
                background_color: "hsl(243, 23%, 24%)",
                p { class: "text-sm", color: "hsl(250, 6%, 70%)", "Humidity" }
                p { class: "text-2xl", color: "hsl(250, 6%, 84%)", "46%" }
            }

            div {
                class: "flex flex-col gap-3 pt-2 pb-2 pl-4 pr-15 rounded-xl grow",
                background_color: "hsl(243, 23%, 24%)",
                p { class: "text-sm", color: "hsl(250, 6%, 70%)", "Wind" }
                p { class: "text-2xl", color: "hsl(250, 6%, 84%)", "14 km/h" }
            }


            div {
                class: "flex flex-col gap-3 pt-2 pb-2 pl-4 pr-15 rounded-xl grow",
                background_color: "hsl(243, 23%, 24%)",
                p { class: "text-sm", color: "hsl(250, 6%, 70%)", "Precipitation" }
                p { class: "text-2xl", color: "hsl(250, 6%, 84%)", "0 mm" }
            }
        
        }
    }
}
