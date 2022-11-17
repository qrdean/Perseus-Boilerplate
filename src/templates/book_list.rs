use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, SsrNode, View, Keyed, KeyedProps};
use serde::{Deserialize, Serialize};

use crate::global_state::AppStateRx;
use crate::components::nav::NavComponent;

#[derive(Deserialize,Serialize,Clone,PartialEq,Eq,Hash)]
pub struct BookData {
    pub id: i32,
    created_at: String,
    lccn: String,
    isbn: String,
    pub title: String,
    pub author: String,
    publishDate: String,
}

#[perseus::make_rx(BookListStateRx)]
pub struct BookListState {
    data: Vec<BookData>,
}

// This macro will make our state reactive *and* store it in the page state store, which means it'll be the same even if we go to the about page and come back (as long as we're in the same session)
#[perseus::template_rx]
pub fn book_list_page(state: BookListStateRx, global_state: AppStateRx) -> View<G> {
    let data = state.data;

    view! {
        NavComponent()
        label {"Search"}
        input(type="text")
        Keyed(KeyedProps {
            iterable: data.handle(),
            template: |x| view! {
                div {
                    p {
                        span{ (x.title) }
                        span{ (x.author) }
                    }
                } 
            },
            key: |x| x.id,
        })

        div { (global_state.test.get()) }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Book List" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("book_list")
        .template(book_list_page)
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


