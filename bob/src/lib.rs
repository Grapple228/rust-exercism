pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with('?');
    let is_yelled = message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic());

    if is_question && is_yelled {
        return "Calm down, I know what I'm doing!";
    }

    if is_yelled {
        return "Whoa, chill out!";
    }

    if is_question {
        return "Sure.";
    }

    "Whatever."
}
