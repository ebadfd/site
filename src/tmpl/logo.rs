use maud::{html, Markup};

pub fn logo() -> Markup {
    html! {
        object data="/static/logo-main.svg" width="300" height="300" {};
    }
}
