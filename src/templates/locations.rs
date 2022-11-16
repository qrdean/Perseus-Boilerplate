use perseus::{SsrNode, Html, Template, RenderFnResultWithCause};
use sycamore::prelude::{View, view, Keyed, KeyedProps};
use serde::{Deserialize, Serialize};

use crate::components::nav::NavComponent;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Location {
    id: i32,
    location_name: String,
    enabled: bool,
}

#[perseus::make_rx(LocationStateRx)]
pub struct LocationState {
    locations: Vec<Location>
}

#[perseus::template_rx]
pub fn location_page(state: LocationStateRx) -> View<G> {
    let locations = state.locations;
    view! {
        NavComponent()
        Keyed(KeyedProps {
            iterable: locations.handle(),
            template: |location| view! {
                p {
                    span {(location.location_name)}
                    span {(location.enabled)}
                }
            },
            key: |location| location.id,
        })

    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Locations" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("locations")
        .template(location_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[perseus::autoserde(build_state)]
pub fn get_build_state(_page: String, _locale: String) -> RenderFnResultWithCause<LocationState> {
    let location = Location {
        id: 1,
        location_name: "Library Shelf".to_string(),
        enabled: true
    };

    let location2 = Location {
        id: 2,
        location_name: "Bedside Table".to_string(),
        enabled: true
    };

    let locations = vec!(location,location2);

    Ok(LocationState { locations })
}
