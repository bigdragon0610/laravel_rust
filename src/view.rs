pub mod __export {
    pub use ::regex::Regex;
}

use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::Error;
use std::fs;

#[macro_export]
macro_rules! compact {
  ($content:ident, $($x:ident),+) => {
      $(
        let re_string = format!(r"\{{\{{\s*\${}\s*\}}\}}", stringify!($x));
        let re = $crate::view::__export::Regex::new(&re_string).unwrap();
        $content = re.replace_all(&$content, $x).into_owned();
      )*
  };
}

pub trait View {
    fn return_view(&self) -> HttpResponse;
}

impl View for String {
    fn return_view(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(self.to_string())
    }
}

pub fn view(file_path: &str) -> String {
    fs::read_to_string(format!("./view/{}", file_path))
        .expect("Something went wrong reading the file")
}

pub fn json<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + Serialize,
{
    let json = serde_json::to_string(value)?;
    Ok(format!("JSON.parse(`{}`)", json))
}
