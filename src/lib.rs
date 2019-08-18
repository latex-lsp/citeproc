use ducc::{Ducc, ExecSettings, Function};

const JS: &str = include_str!("../js/dist/citeproc.js");

pub fn render(code: &str) -> Option<String> {
    let ducc = Ducc::new();
    let _: () = ducc.exec(JS, None, ExecSettings::default()).unwrap();
    let js = "(function execute(code) { return citeproc.default(code); })";
    let func: Function = ducc.compile(js, None).unwrap().call(()).unwrap();
    let code = ducc.create_string(&code).unwrap();
    let html: String = func.call((code, 0)).ok()?;
    let markdown = html2md::parse_html(&html);
    Some(markdown.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let citation = render("@article{foo, author = {Foo Bar}, title = {Baz Qux}, year = 1337}").unwrap();
        assert_eq!(citation, "Bar, F. (1337). Baz Qux.");
    }

    #[test]
    fn failure() {
        let citation = render("@article}{");
        assert_eq!(citation, None);
    }
}
