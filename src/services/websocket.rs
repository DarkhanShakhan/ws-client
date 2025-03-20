use leptos::logging::log;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

pub enum WebSocketMessage {
    Text(String),
    Error(String),
}

#[derive(Clone)]
pub struct WebSocketService {
    url: String,
    ws: Rc<RefCell<Option<WebSocket>>>,
}

impl WebSocketService {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ws: Rc::new(RefCell::new(None)),
        }
    }

    pub fn connect<F, G, H, I>(
        &self,
        mut on_open: F,
        mut on_message: G,
        mut on_close: H,
        mut on_error: I,
    ) where
        F: FnMut() + 'static,
        G: FnMut(WebSocketMessage) + 'static,
        H: FnMut() + 'static,
        I: FnMut(String) + 'static,
    {
        let ws = WebSocket::new(&self.url);

        if let Err(err) = ws {
            log!("Error creating WebSocket: {:?}", err);
            return;
        }

        let ws = ws.unwrap();

        // Binary type (we only handle text for markdown)
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        // Set up the on open callback
        let on_open = Closure::wrap(Box::new(move || {
            on_open();
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();

        // Set up the on message callback
        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let message = String::from(txt);
                on_message(WebSocketMessage::Text(message));
            } else {
                on_message(WebSocketMessage::Error(
                    "Received non-text message".to_string(),
                ));
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();

        // Set up the on close callback
        let on_close = Closure::wrap(Box::new(move |_: CloseEvent| {
            on_close();
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();

        // Set up the on error callback
        let on_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            let msg = e.message();
            on_error(msg);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        on_error.forget();

        // Store the WebSocket instance
        *self.ws.borrow_mut() = Some(ws);
    }

    pub fn send_text(&self, text: &str) {
        if let Some(ws) = self.ws.borrow().as_ref() {
            if let Err(err) = ws.send_with_str(text) {
                log!("Error sending message: {:?}", err);
            }
        } else {
            log!("WebSocket not connected");
        }
    }

    pub fn close(&self) {
        if let Some(ws) = self.ws.borrow().as_ref() {
            if let Err(err) = ws.close() {
                log!("Error closing WebSocket: {:?}", err);
            }
        }
    }
}
