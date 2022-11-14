mod error_pages;
mod templates;
mod components;
mod global_state;

use perseus::{Html, PerseusApp, PerseusRoot, SsrNode};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::testpage::get_template)
        .template(crate::templates::book_list::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .global_state_creator(crate::global_state::get_global_state_creator())
        .static_alias("/favicon.ico", "static/favicon.ico")
        .static_alias("/test.txt", "static/test.txt")
        .index_view(|| index_view())
}

pub fn index_view() -> sycamore::prelude::View<SsrNode> {
    sycamore::view! {
        html() {
            head {
                link(rel = "stylesheet", href = ".perseus/static/normalize.css")
                link(rel = "stylesheet", href = ".perseus/static/skeleton.css")
                link(rel = "stylesheet", href = ".perseus/static/styles.css")
            }
            body(class = "container") {
                PerseusRoot()
                footer {
                    ul(class="footer-container") { 
                        li(class="footer-item") { "Maintained By Quinton Dean" }
                        li(class="footer-item") { 
                            a(href="https://github.com/qrdean/Perseus-Boilerplate", target="_blank", rel="noopener") { "Github" }
                        }
                    }
                }
            }
        }
    }
}
