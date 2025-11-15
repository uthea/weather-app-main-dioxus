use crate::model::{SearchResponse, SearchResult};
use dioxus::prelude::*;

const SEARCH_ICON: Asset = asset!("/assets/images/icon-search.svg");

#[component]
pub fn Search(onsearch: EventHandler<String>) -> Element {
    let mut text = use_signal(|| "".to_string());
    let mut search_result = use_signal(|| None::<Vec<SearchResult>>);

    let fetch_result = move || async move {
        if text().is_empty() {
            return Some(vec![]);
        }

        let response = reqwest::get(format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={text}"
        ))
        .await
        .unwrap()
        .json::<SearchResponse>()
        .await
        .unwrap()
        .results;

        if response.as_ref().is_none() {
            return Some(vec![]);
        }

        Some(response.unwrap())
    };

    rsx! {
        div { class: "flex flex-col w-full gap-8 items-center",
            h1 { class: "text-3xl", style: "color: hsl(0, 0%, 100%)",
                "How's the sky looking today ?"
            }
            div { class: "flex items-center gap-2",
                div {
                    input {
                        background_color: "hsl(243, 23%, 24%)",
                        color: "hsl(0, 0%, 100%)",
                        background_image: "url({SEARCH_ICON})",
                        background_position: "4% 50%",
                        class: "bg-size-[14px] pl-8 pt-1 pb-1 bg-no-repeat rounded-md",
                        placeholder: "Search for a place...",
                        value: "{text}",
                        onchange: move |e| {
                            let value = e.value();

                            if value.is_empty() {
                                search_result.set(None);
                            }
                            *text.write() = e.value();
                        },
                    }

                    if search_result().is_none() {
                        div {}
                    } else {
                        SearchDropdown { search_result: search_result().unwrap() }
                    }
                

                }

                button {
                    color: "hsl(0, 0%, 100%)",
                    background_color: "hsl(233, 67%, 56%)",
                    class: "py-1.5 px-4 rounded-md text-sm",
                    onclick: move |_| async move {
                        *search_result.write() = fetch_result().await;
                        onsearch.call(text());
                    },
                    "Search"
                }
            }
        

        }


    }
}

#[component]
fn SearchDropdown(search_result: Vec<SearchResult>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 p-1",
            background_color: "hsl(243, 23%, 24%)",
            div { class: "p-1",
                span { color: "hsl(0, 0%, 100%)",
                    if search_result.is_empty() {
                        "Not Found"
                    } else {
                        "Found"
                    }
                }
            }
        }
    }
}
