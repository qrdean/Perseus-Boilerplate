use perseus::{Html, RenderFnResultWithCause, SsrNode, Template, web_log};
use sycamore::prelude::{view, View, cloned};

use crate::global_state::AppStateRx;
use crate::components::nav::NavComponent;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub data: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx, global_state: AppStateRx) -> View<G> {
    let test = global_state.test;
    let test_2 = test.clone();
    let state_2 = state.data.clone();

    let req = move |_| {
        if G::IS_BROWSER {
            web_log!("Hello im in browser");
            let body ="name=newperson&email=anotherperson%40gmail.com"; 
            perseus::spawn_local(cloned!(state_2 => async move {
                web_log!("try to call server");
                let res = reqwasm::http::Request::post("http://localhost:8000/subscriptions")
                    .header("Content-Type", "application/x-www-form-urlencoded")
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
        p { (state.greeting.get()) (state.data.get()) }
        p { (test.get()) }
        input(bind:value = test_2)
        br()
        button(id = "test button", class="button button-primary", on:click=req) 
        { "Test Button" }
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

    Ok(IndexPageState {
        greeting: "Welcome To The Library!".to_string(),
        data: body.to_string(),
    })
}


