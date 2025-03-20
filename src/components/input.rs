use leptos::*;

#[component]
pub fn ChatInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    on_send: Callback<String>,
    is_disabled: ReadSignal<bool>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Textarea>();

    // Handle text input change
    let handle_input = move |ev| {
        let new_value = event_target_value(&ev);
        set_value.set(new_value);

        // Auto resize the textarea
        if let Some(textarea) = input_ref.get() {
            // Set new height based on scroll height
            let textarea = textarea.style("heigh", "auto");
            let scroll_height = textarea.scroll_height();
            let _ = textarea.style("height", format!("{}px", scroll_height));
        }
    };

    // Handle sending on Enter (but allow Shift+Enter for new lines)
    let handle_key_down = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();

            if !is_disabled.get() && !value.get().trim().is_empty() {
                on_send.call(value.get());

                // Reset the textarea height
                if let Some(textarea) = input_ref.get() {
                    let _ = textarea.style("height", "56px");
                }
            }
        }
    };

    // Handle submit button click
    let handle_submit = move |_| {
        if !is_disabled.get() && !value.get().trim().is_empty() {
            on_send.call(value.get());

            // Reset the textarea height
            if let Some(textarea) = input_ref.get() {
                let _ = textarea.style("height", "56px");
            }
        }
    };

    view! {
        <div class="chat-input-container">
            <textarea
                ref=input_ref
                class="chat-input"
                placeholder="Type a message..."
                value=move || value.get()
                on:input=handle_input
                on:keydown=handle_key_down
                disabled=move || is_disabled.get()
                rows="1"
            ></textarea>

            <button
                class="send-button"
                on:click=handle_submit
                disabled=move || is_disabled.get() || value.get().trim().is_empty()
            >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="22" y1="2" x2="11" y2="13"></line>
                    <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                </svg>
            </button>
        </div>
    }
}
