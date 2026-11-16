use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Message {
    role: String,
    content: String,
}

fn append_session_message(history: &mut Vec<Message>, role: &str, content: &str) -> bool {
    let allowed_roles = ["user", "assistant", "system", "tool"];
    if !allowed_roles.contains(&role) {
        return false;
    }
    let content = content.trim();
    if content.is_empty() {
        return false;
    }
    let message = Message {
        role: role.to_string(),
        content: content.to_string(),
    };
    history.push(message);
    true
}

fn select_model_context(history: &[Message], max_recent: usize) -> Vec<Message> {
    let mut remaining = max_recent;
    let mut selected = HashSet::new();
    for i in (0..history.len()).rev() {
        let message = &history[i];
        if message.role == "system" {
            selected.insert(i);
        } else if remaining > 0 {
            selected.insert(i);
            remaining -= 1;
        }
    }
    history
        .iter()
        .enumerate()
        .filter(|(i, _)| selected.contains(i))
        .map(|(_, message)| message.clone())
        .collect()
}

fn generate_mock_reply(context: &[Message]) -> Option<Message> {
    for message in context.iter().rev() {
        if message.role == "user" {
            return Some(Message {
                role: "assistant".to_string(),
                content: String::from(format!("Echo: {}", message.content)),
            });
        }
    }
    None
}

fn main() {
    let mut history = Vec::new();
    append_session_message(&mut history, "user", "Hello, world!");
    append_session_message(&mut history, "assistant", "Hello, how can I help you?");
    append_session_message(&mut history, "system", "You are a helpful assistant.");
    append_session_message(
        &mut history,
        "tool",
        "You can use the following tools: [tool1, tool2, tool3]",
    );
    println!("{:?}", history);
    println!("{:?}", select_model_context(&history, 2));
    println!(
        "{:?}",
        generate_mock_reply(&select_model_context(&history, 2))
    );
}
