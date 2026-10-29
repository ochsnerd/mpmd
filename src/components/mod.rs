//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define an
//! Echo component that talks to the server over a websocket.


mod echo;
pub use echo::Echo;
