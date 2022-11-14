use sycamore::prelude::{View, view, component};

/// a top level navigation component used throughout the app. 
/// Could extend to have state and have different links on different pages
#[component(NavComponent<G>)]
pub fn nav_component() -> View<G> {
    view! {
        nav(class="navbar") {
            ul(class="navbar-nav") {
                li(class="nav-item") {
                    a(class="button", href = "", id = "home-link") { "Home" }
                }
                li(class="nav-item") {
                    a(class="button", href = "about", id = "about-link") { "About!" }
                }
                li(class="nav-item") {
                    a(class="button", href = "testpage", id = "test-link") { "TestPage!" }
                }
                li(class="nav-item") {
                    a(class="button", href = "book_list", id = "book-list-link") { "Book List!" }
                }
                /*
                  li(class="nav-item has-dropdown") {
                    a(class="button",id="menu-button") {"Menu"}
                    MenuComponent()
                } */
            }
        }
    }
}

/* Need to do some more research into this. Not quite working as I want 
#[component(MenuComponent<G>)]
fn menu_component() -> View<G> {
    view! {
        div(class="dropdown") {
            div(class="menu") {
                a(href="about", class="menu-item") {
                    "Link to page"
                }
            }
        }
    }
}
*/
