use perseus::{Html, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, SsrNode, View};

#[perseus::make_rx(TestPageStateRx)]
pub struct TestPageState {
    pub username: String,
}

// This macro will make our state reactive *and* store it in the page state store, which means it'll be the same even if we go to the about page and come back (as long as we're in the same session)
#[perseus::template_rx]
pub fn test_page(state: TestPageStateRx) -> View<G> {
    // Bit unergonomic here. Needed to bind:value to the username
    // This will be unnecessary though with newer sycamore version, make the 
    // switch when 0.4.0 of Perseus and 0.8.x sycamore releases stable
    let username = state.username;
    let username_2 = username.clone();

    view! {
        p { (format!("Greetings, {}!", username.get())) }
        input(bind:value = username_2, placeholder = "Username")
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Test Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("testpage")
        .template(test_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<TestPageState> {
    Ok(TestPageState {
        username: "".to_string(),
    })
}


