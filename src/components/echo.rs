use dioxus::fullstack::{use_websocket, WebSocketOptions, Websocket};
use dioxus::prelude::*;
const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");
/// Echo component that sends messages to the server over a websocket and shows the uppercased replies.
#[component]
pub fn Echo() -> Element {
    let mut socket = use_websocket(|| uppercase_ws(WebSocketOptions::new()));
    let mut input = use_signal(String::new);
    let mut messages = use_signal(Vec::<String>::new);
    use_future(move || async move {
        while let Ok(msg) = socket.recv().await {
            messages.push(msg);
        }
    });
    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { id: "echo",
            h4 { "Websocket Echo" }
            form {
                onsubmit: move |event| async move {
                    event.prevent_default();
                    let msg = input.take();
                    if !msg.is_empty() {
                        _ = socket.send(msg).await;
                    }
                },
                input {
                    placeholder: "Type a message and press Enter...",
                    value: "{input}",
                    oninput: move | event |
                            input.set(event.value()),
                }
            }
            for msg in messages.iter() {
                p {
                    i { "{msg}" }
                }
            }
        }
    }
}
#[get("/api/uppercase_ws")]
async fn uppercase_ws(options: WebSocketOptions) -> Result<Websocket> {
    Ok(
        options
            .on_upgrade(move |
                mut socket: dioxus::fullstack::TypedWebsocket<String, String>|
            async move {
                while let Ok(msg) = socket.recv().await {
                    _ = socket.send(msg.to_uppercase()).await;
                }
            }),
    )
}
