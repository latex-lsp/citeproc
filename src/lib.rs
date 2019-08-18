use ducc::{Ducc, ExecSettings, Function};
use once_cell::sync::Lazy;
use std::sync::Mutex;

const JS: &str = include_str!("../js/dist/citeproc.js");

const WRAPPER: &str = "(function execute(code) { return citeproc.default(code); })";

struct JavaScriptEngine {
    ducc: Ducc,
}

impl JavaScriptEngine {
    pub fn new() -> Self {
        let ducc = Ducc::new();
        let _: () = ducc.exec(JS, None, ExecSettings::default()).unwrap();
        Self { ducc }
    }
}

unsafe impl Send for JavaScriptEngine {}

static ENGINE: Lazy<Mutex<JavaScriptEngine>> = Lazy::new(|| Mutex::new(JavaScriptEngine::new()));

pub fn render(code: &str) -> Option<String> {
    let engine = ENGINE.lock().unwrap();
    let code = engine.ducc.create_string(&code).unwrap();
    let func: Function = engine
        .ducc
        .compile(WRAPPER, None)
        .unwrap()
        .call(())
        .unwrap();
    let html: String = func.call((code, 0)).ok()?;
    let markdown = html2md::parse_html(&html);
    Some(markdown.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let citation =
            render("@article{foo, author = {Foo Bar}, title = {Baz Qux}, year = 1337}").unwrap();
        assert_eq!(citation, "Bar, F. (1337). Baz Qux.");
    }

    #[test]
    fn failure() {
        let citation = render("@article}{");
        assert_eq!(citation, None);
    }
}
