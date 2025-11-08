use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/images/logo.svg");
const UNITS: Asset = asset!("/assets/images/icon-units.svg");
const DROPDOWN_ICON: Asset = asset!("/assets/images/icon-dropdown.svg");

#[component]
pub fn Nav() -> Element {
    rsx! {

        div {
            class: "flex justify-between items-center",
            class: "pt-4 pb-4",
            style: "background-color: hsl(243, 96%, 9%)",


            img { src: LOGO }

            button {
                // TODO: dropdown action
                div {
                    class: "flex gap-2 items-center p-1.5 rounded-lg",
                    style: "background-color: hsl(243, 23%, 30%)",
                    img { src: UNITS }
                    p { style: "color: hsl(0, 0%, 100%)", "Units" }
                    img { src: DROPDOWN_ICON }
                }
            }
        

        }
    }
}
