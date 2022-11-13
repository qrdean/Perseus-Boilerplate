mod error_pages;
mod templates;
mod global_state;

use perseus::{Html, PerseusApp, PerseusRoot, SsrNode};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::testpage::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .global_state_creator(crate::global_state::get_global_state_creator())
        .static_alias("/favicon.ico", "static/favicon.ico")
        .static_alias("/test.txt", "static/test.txt")
        .index_view(||{
            head()
        })
}

pub fn head() -> sycamore::prelude::View<SsrNode> {
    sycamore::view! {
        html() {
            head {
                link(rel = "stylesheet", href = ".perseus/static/normalize.css")
                link(rel = "stylesheet", href = ".perseus/static/skeleton.css")
            }
            body(class = "container") {
                PerseusRoot()
            }
        }
    }
}

