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
    submitted: String,
    data: String
}

#[perseus::template_rx]
pub fn add_book_page(state: AddBookStateRx) -> View<G> {
    let title = state.title.get();
    let author = state.author.get();
    let lccn = state.lccn.get();
    let isbn = state.isbn.get();
    let publish_date = state.publish_date.get();
    let state_2 = state.data.clone();
    /*
    let handler = |_| {
        web_log!("I submit here");
    };*/
    let req = move |_| {
        if G::IS_BROWSER {
            web_log!("Hello im in browser");
            let body = "{\"title\":\"foo\"}"; 
            perseus::spawn_local(cloned!(state_2 => async move {
                web_log!("try to call server");
                let res = reqwasm::http::Request::post("https://jsonplaceholder.typicode.com/posts")
                    .header("Content-Type", "application/json; charset=UTF-8")
                    .body(body)
                    .send()
                    .await
                    .expect("expect to work")
                    .text()
                    .await
                    .expect("expect to work2");
                    
                    
                    //.send_json(ureq::json!({
                     //   "name": "new name",
                      //  "email": "new_name@gmail.com"
                    //})).expect("did not get post back")
                    //.into_string().expect("did not go to string");
                state_2.set(res.to_string());
            }))
        }
    }; 
    view! {
        NavComponent()
        h2 {"Add Book Page"}
        input(value=title)
        input(value=author)
        input(value=lccn)
        input(value=isbn)
        input(value=publish_date)
        button(class="button", on:click=req)
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
        data: "".to_string(),
    })
}
