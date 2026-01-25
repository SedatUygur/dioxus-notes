use dioxus::prelude::*;

fn main(){ dioxus::launch(app); }

#[component]
fn app() -> Element {
  rsx!{
    div { h1 { "Dioxus (Notes)" } }
  }
}