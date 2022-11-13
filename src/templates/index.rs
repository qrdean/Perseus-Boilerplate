use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, View};
use serde::{Deserialize, Serialize};

use crate::global_state::AppStateRx;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub data: String,
    pub book: String,
    pub author: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct BookData {
    id: i32,
    created_at: String,
    lccn: String,
    isbn: String,
    title: String,
    author: String,
    publishDate: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx, global_state: AppStateRx) -> View<G> {
    let test = global_state.test;
    let test_2 = test.clone();
    view! {
        p { (state.greeting.get()) (state.data.get()) }
        div { (state.book.get())}
        div { (state.author.get()) }
        p { (test.get()) }
        input(bind:value = test_2)
        a(href = "about", id = "about-link") { "About!" }
        a(href = "testpage", id = "test-link") { "Test Page!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Browsing Test" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    let body = perseus::cache_fallible_res(
        "ipify",
        || async {
            //let res = reqwest::get("https://api.ipify.org").await?.text().await?;
            //Ok::<String, reqwest::Error>(res)
            // let res = reqwest::blocking::get("https://api.ipify.org")?.text()?;
            // Ok::<String, reqwest::Error>(res)
            let res = ureq::get("https://api.ipify.org").call()?.into_string()?;
            Ok::<String, ureq::Error>(res)
        }, true
    ).await?;

    let _book = perseus::cache_fallible_res(
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

    let (title, author) = match _book.first() {
        Some(book) => (book.title.clone(), book.author.clone()),
        None => ("".to_string(), "".to_string()),
    };

    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
        data: body.to_string(),
        book: title.to_string(),
        author: author.to_string(),
    })
}

