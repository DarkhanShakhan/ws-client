use crate::components::input::ChatInput;
use crate::components::message::{Message, MessageData, MessageRole};
use crate::services::websocket::{WebSocketMessage, WebSocketService};
use leptos::*;
use std::collections::VecDeque;
use uuid::Uuid;

#[component]
pub fn ChatView() -> impl IntoView {
    let (messages, set_messages) = create_signal(VecDeque::<MessageData>::new());
    let (input_value, set_input_value) = create_signal(String::new());
    let (is_sending, set_is_sending) = create_signal(false);
    let (ws_status, set_ws_status) = create_signal("Disconnected".to_string());

    // Create a WebSocket service
    let ws_service = WebSocketService::new("ws://localhost:8000/ws");
    let ws_service = store_value(ws_service);

    // Connect to WebSocket on component mount
    create_effect(move |_| {
        let ws = ws_service.get_value();
        set_ws_status.update(|s| *s = "Connecting...".to_string());

        ws.connect(
            // On open
            move || {
                set_ws_status.update(|s| *s = "Connected".to_string());
            },
            // On message
            move |msg: WebSocketMessage| {
                match msg {
                    WebSocketMessage::Text(text) => {
                        set_messages.update(|msgs| {
                            // Add the message from the assistant
                            msgs.push_back(MessageData {
                                id: Uuid::new_v4().to_string(),
                                role: MessageRole::Assistant,
                                content: text,
                            });
                        });
                        set_is_sending.set(false);
                    }
                    WebSocketMessage::Error(err) => {
                        set_messages.update(|msgs| {
                            msgs.push_back(MessageData {
                                id: Uuid::new_v4().to_string(),
                                role: MessageRole::System,
                                content: format!("Error: {}", err),
                            });
                        });
                        set_is_sending.set(false);
                    }
                }
            },
            // On close
            move || {
                set_ws_status.update(|s| *s = "Disconnected".to_string());
            },
            // On error
            move |err| {
                set_ws_status.update(|s| *s = format!("Error: {}", err));
            },
        );
    });

    // Function to send a message
    let send_message = Callback::new(move |content: String| {
        if content.trim().is_empty() || is_sending.get() {
            return;
        }

        let message = MessageData {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::Human,
            content: content.clone(),
        };

        // Add the user message to the list
        set_messages.update(|msgs| {
            msgs.push_back(message);
        });

        // Send to WebSocket
        let ws = ws_service.get_value();
        ws.send_text(&content);

        // Clear the input and set sending state
        set_input_value.set(String::new());
        set_is_sending.set(true);
    });
    view! {
        <div class ="chat-container">
            <div class="chat-status">
                <span class=move || {
                    if ws_status.get() == "Connected" { "status-indicator connected" }
                    else if ws_status.get().starts_with("Error") { "status-indicator error" }
                    else { "status-indicator disconnected" }
                }></span>
                <span>{move || ws_status.get()}</span>
            </div>
            <div class = "messages-container">
                <For
                    each = move || messages.get()
                    key = |message| message.content.clone()
                    children = move |content| {
                        view! {
                            <Message
                                role=content.role
                                content = content.content
                            />
                        }
                    }
                />
                <ChatInput
                        value=input_value
                        set_value=set_input_value
                        on_send=send_message
                        is_disabled=is_sending
                    />
            </div>
        </div>
    }
    // view! {
    //     <div class="chat-container">
    //         <div class="chat-status">
    //             <span class=move || {
    //                 if ws_status.get() == "Connected" { "status-indicator connected" }
    //                 else if ws_status.get().starts_with("Error") { "status-indicator error" }
    //                 else { "status-indicator disconnected" }
    //             }></span>
    //             <span>{move || ws_status.get()}</span>
    //         </div>

    //         <div class="messages-container">
    //             <For
    //                 each=move || messages.get().into_iter().collect::<Vec<_>>()
    //                 key=|message| message.id.clone()
    //                 children=move |message| {
    //                     view! {
    //                         <Message
    //                             role=message.role
    //                             content=message.content
    //                         />
    //                     }
    //                 }
    //             />

    //             // Show a loading indicator when waiting for a response
    //             {move || if is_sending.get() {
    //                 view! {
    //                     <div class="message-loading">
    //                         <div class="loading-dot"></div>
    //                         <div class="loading-dot"></div>
    //                         <div class="loading-dot"></div>
    //                     </div>
    //                 }
    //             } else {
    //                 view! { <div></div> }
    //             }}
    //         </div>

    //         <ChatInput
    //             value=input_value
    //             set_value=set_input_value
    //             on_send=send_message
    //             is_disabled=is_sending
    //         />
    //     </div>
    // }
}
