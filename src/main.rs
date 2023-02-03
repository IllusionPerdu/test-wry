//#![windows_subsystem = "windows"]
#![windows_subsystem = "console"]

//If is commented it's work with with_url(&uri)
#[cfg(all(
    any(windows, unix),
    any(target_arch = "x86_64", target_arch = "wasm32"),
    not(target_env = "musl"),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::collections::HashMap;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget};
use wry::application::window::{Window, WindowBuilder, WindowId};
use wry::webview::{WebView, WebViewBuilder};

enum UserEvents {
    CloseWindow(WindowId),
    NewWindow(),
}

fn create_new_window(
    title: String,
    uri: String,
    event_loop: &EventLoopWindowTarget<UserEvents>,
    proxy: EventLoopProxy<UserEvents>,
) -> (WindowId, WebView) {
    let window = WindowBuilder::new()
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let window_id = window.id();
    let handler = move |window: &Window, req: String| match req.as_str() {
        "new-window" => {
            let _ = proxy.send_event(UserEvents::NewWindow());
        }
        "close" => {
            let _ = proxy.send_event(UserEvents::CloseWindow(window.id()));
        }
        _ if req.starts_with("change-title") => {
            let title = req.replace("change-title:", "");
            window.set_title(title.as_str());
        }
        _ => {}
    };

    let webview = WebViewBuilder::new(window)
        .unwrap()
        /*.with_html(
            format!(
                "<!DOCTYPE html><head><script>window.location.href = '{}';</script></head></html>",
                &uri.replace("app://", "https://app.")
            )
        ) // !OK
        */
        .with_url(&uri)  // !Crash with Result::unwrap() on an Err value: WebView2Error(WindowsError(Error { code: 0x80070057, message: Paramètre incorrect. }))
        //.with_url_and_headers(&uri, wry::http::HeaderMap::new()) // !OK
        .unwrap()
        .with_ipc_handler(handler)
        .build()
        .unwrap();
    (window_id, webview)
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::<UserEvents>::with_user_event();
    let mut webviews = HashMap::new();
    let proxy = event_loop.create_proxy();

    let new_window = create_new_window(
        format!("e-Dico {}", webviews.len() + 1),
        "https://www.poeme-france.com/".to_string(),
        &event_loop,
        proxy.clone(),
    );
    //new_window.1.load_url("https://poeme-france.com/");
    webviews.insert(new_window.0, new_window.1);

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("DicoGui has started!"),
            #[allow(clippy::single_match)]
            Event::WindowEvent {
                event, window_id, ..
            } => match event {
                // TODO more event to support ?
                WindowEvent::CloseRequested => {
                    webviews.remove(&window_id);
                    if webviews.is_empty() {
                        *control_flow = ControlFlow::Exit
                    }
                }
                WindowEvent::Destroyed => (),
                _ => (),
            },
            Event::UserEvent(user_event) => match user_event {
                UserEvents::NewWindow() => {
                    let new_window = create_new_window(
                        format!("e-Dico {}", webviews.len() + 1),
                        "https://www.poeme-france.com/".to_string(),
                        event_loop,
                        proxy.clone(),
                    );
                    webviews.insert(new_window.0, new_window.1);
                }
                UserEvents::CloseWindow(id) => {
                    webviews.remove(&id);
                    if webviews.is_empty() {
                        *control_flow = ControlFlow::Exit
                    }
                }
            },
            _ => (),
        }
    });
}
