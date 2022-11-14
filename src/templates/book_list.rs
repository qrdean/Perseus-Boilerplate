use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, SsrNode, View};

use crate::global_state::AppStateRx;
use crate::components::nav::NavComponent;
use crate::templates::index::BookData;

#[perseus::make_rx(BookListStateRx)]
pub struct BookListState {
    data: Vec<BookData>,
}

// This macro will make our state reactive *and* store it in the page state store, which means it'll be the same even if we go to the about page and come back (as long as we're in the same session)
#[perseus::template_rx]
pub fn test_page(state: BookListStateRx, global_state: AppStateRx) -> View<G> {
    // Bit unergonomic here. Needed to bind:value to the username
    // This will be unnecessary though with newer sycamore version, make the 
    // switch when 0.4.0 of Perseus and 0.8.x sycamore releases stable
    let data = state.data;
    let (title, author) = match data.get().first() {
        Some(book) => (book.title.clone(), book.author.clone()),
        None => ("".to_string(), "".to_string()),
    };

    view! {
        NavComponent()
        p { (format!("Greetings, {}!", title)) }
        p { (format!("Greetings, {}!", author)) }
        div { (global_state.test.get()) }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Test Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("book_list")
        .template(test_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<BookListState> {
    let book = perseus::cache_fallible_res(
        "book",
        || async {
            //let res = reqwest::get("https://api.ipify.org").await?.text().await?;
            //Ok::<String, reqwest::Error>(res)
            // let res = reqwest::blocking::get("https://api.ipify.org")?.text()?;
            // Ok::<String, reqwest::Error>(res)
            let res: Vec<BookData> = ureq::get("https://mlibraryapi.xyz/dbconnect").call()?.into_json()?;
            Ok::<Vec<BookData>, ureq::Error>(res)
        }, true
    ).await?;

    Ok(BookListState {
        data: book,
    })
}


