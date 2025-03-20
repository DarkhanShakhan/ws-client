use leptos::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Clone, Copy, PartialEq)]
pub enum MessageRole {
    Human,
    Assistant,
    System,
}

#[derive(Clone)]
pub struct MessageData {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub animated: bool, // Whether to show the animation effect
}

#[component]
pub fn Message(role: MessageRole, content: String, animated: bool) -> impl IntoView {
    // Create signals for the typing animation
    let (displayed_text, set_displayed_text) = create_signal(if animated {
        "".to_string()
    } else {
        content.clone()
    });
    let (is_typing, set_is_typing) = create_signal(animated);

    // Start animation if needed
    if animated {
        let content_clone = content.clone();
        window_event_listener(move || {
            let content = content_clone.clone();

            // Set up the typing animation using JavaScript
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let message_content = document.create_element("script").unwrap();

            let script = format!(
                r#"
                (function() {{
                    const text = {};
                    let displayed = "";
                    let index = 0;
                    const targetId = "msg-{}";
                    const delay = 30;
                    
                    function typeNextChar() {{
                        if (index < text.length) {{
                            const char = text.charAt(index);
                            displayed += char;
                            
                            const element = document.getElementById(targetId);
                            if (element) {{
                                element.textContent = displayed;
                                
                                // Add cursor to the end
                                element.innerHTML = displayed + '<span class="cursor">|</span>';
                                
                                // Additional delay for punctuation and spaces
                                let timeout = delay;
                                if ([' ', '.', ',', '!', '?', ';', ':', '\n'].includes(char)) {{
                                    timeout += 20;
                                }}
                                
                                index++;
                                setTimeout(typeNextChar, timeout);
                            }}
                        }} else {{
                            // Animation complete
                            const element = document.getElementById(targetId);
                            if (element) {{
                                element.innerHTML = displayed;
                                element.classList.remove("typing");
                            }}
                        }}
                    }}
                    
                    // Start typing animation after a small delay
                    setTimeout(typeNextChar, 50);
                }})();
                "#,
                serde_json::to_string(&content).unwrap(),
                content.clone().len() // Use content length as a unique ID
            );

            message_content.set_text_content(Some(&script));
            let _ = document.body().unwrap().append_child(&message_content);
        });
    }

    // Function to convert markdown to HTML
    let markdown_to_html = move |markdown: String| {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    };

    let role_class = match role {
        MessageRole::Human => "message-human",
        MessageRole::Assistant => "message-assistant",
        MessageRole::System => "message-system",
    };

    let role_name = match role {
        MessageRole::Human => "You",
        MessageRole::Assistant => "Assistant",
        MessageRole::System => "System",
    };

    // Create a unique display ID for this message
    let display_id = format!("msg-{}", content.len());

    view! {
        <div class=format!("message {}", role_class)>
            <div class="message-header">
                <div class="message-role">{role_name}</div>
            </div>
            <div
                class=format!("message-content markdown-content {}", if animated {"typing"} else {""})
                id=display_id
                inner_html=if animated {
                    "".to_string()
                } else {
                    markdown_to_html(content)
                }
            >
            </div>
        </div>
    }
}

// Helper to run code once after component mount
fn window_event_listener<F>(f: F)
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().unwrap();

    // Use requestAnimationFrame to run the code after component is mounted
    let closure = Closure::once(f);
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
