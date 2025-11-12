use super::asset::*;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/images/logo.svg");
const UNITS: Asset = asset!("/assets/images/icon-units.svg");

#[component]
pub fn Nav() -> Element {
    rsx! {

        div { class: "flex justify-between items-center", class: "pt-4",
            img { src: LOGO }

            button {
                // TODO: dropdown action
                div {
                    class: "flex gap-2 items-center p-2 rounded-lg",
                    style: "background-color: hsl(243, 23%, 24%)",
                    img { src: UNITS }
                    p { style: "color: hsl(0, 0%, 100%)", "Units" }
                    img { src: DROPDOWN_ICON }
                }
            }
        

        }
    }
}
