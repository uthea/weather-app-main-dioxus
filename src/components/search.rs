use dioxus::prelude::*;

const SEARCH_ICON: Asset = asset!("/assets/images/icon-search.svg");

#[component]
pub fn Search() -> Element {
    rsx! {
        div { class: "flex flex-col w-full gap-8 items-center",
            h1 { class: "text-3xl", style: "color: hsl(0, 0%, 100%)",
                "How's the sky looking today ?"
            }
            div { class: "flex items-center gap-2",
                input {
                    background_color: "hsl(243, 23%, 24%)",
                    color: "hsl(0, 0%, 100%)",
                    background_image: "url({SEARCH_ICON})",
                    background_position: "4% 50%",
                    class: "bg-size-[14px] pl-8 pt-1 pb-1 bg-no-repeat rounded-md",
                    placeholder: "Search for a place...",
                }
                button {
                    color: "hsl(0, 0%, 100%)",
                    background_color: "hsl(233, 67%, 56%)",
                    class: "py-1.5 px-4 rounded-md text-sm",
                    "Search"
                }
            }
        }
    }
}
