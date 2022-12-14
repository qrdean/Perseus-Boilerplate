use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

use crate::components::nav::NavComponent;

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        NavComponent()
        p { "About." }
        p { "new" }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About Page | Perseus Example - Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
