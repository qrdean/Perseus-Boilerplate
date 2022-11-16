use perseus::{SsrNode, Template, Html, RenderFnResultWithCause, web_log};
use sycamore::prelude::{View, view, cloned};

use crate::components::nav::NavComponent;

#[perseus::make_rx(AddBookStateRx)]
pub struct AddBookState {
    title: String,
    author: String,
    lccn: String,
    isbn: String,
    publish_date: String,
    submitted: String
}

#[perseus::template_rx]
pub fn add_book_page(state: AddBookStateRx) -> View<G> {
    let title = state.title.get();
    let author = state.author.get();
    let lccn = state.lccn.get();
    let isbn = state.isbn.get();
    let publish_date = state.publish_date.get();
    let submitted = state.submitted.clone().get();
    let handler = move |params| {
        web_log!("I submit here");
        web_log!("{:?}", params);
        cloned!(state => {
            state.submitted.set("true".to_string());
        })       
    };
    view! {
        NavComponent()
        p {"Add Book Page"}
        p { (submitted) }
        form(on:click=handler, accept-charset="utf-8") {
            input(value=title)
            input(value=author)
            input(value=lccn)
            input(value=isbn)
            input(value=publish_date)
            input(type="submit", value="Submit")
        }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Add Book" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("add_book")
        .head(head)
        .template(add_book_page)
        .build_state_fn(get_build_state)
}

#[perseus::autoserde(build_state)]
pub fn get_build_state(
    _page: String,
    _locale: String
) -> RenderFnResultWithCause<AddBookState> {
    Ok(AddBookState {
        title: "".to_string(),
        author: "".to_string(),
        isbn: "".to_string(),
        lccn: "".to_string(),
        publish_date: "".to_string(),
        submitted: "false".to_string(),
    })
}
