pub enum Kind {
  Copilot,
  Command,
  Code,
  Url
}

pub fn prefix(kind: Kind) -> &'static str {
  let prompt = match kind {
    Copilot => "",
    Command => "",
    Code => "",
    Url => "",
  };
  format!("Given the user request, only produce {}", prompt)
}
