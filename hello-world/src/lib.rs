pub fn hello(a: Option<&str>) -> String
{

  //return "Hello, Alice!".to_string()
  match a {
        Some (ref str) => format!("Hello, {}!",str),
        None => "Hello, World!".to_string(),
  }
}
