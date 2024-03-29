use slimr::SlimR;

#[test]
fn renders_single_var() {
    #[derive(SlimR)]
    #[template(source = "h1 {{arg}}")]
    struct Title {
        arg: String,
    }
    let t = Title {
        arg: "hello".into(),
    };
    assert_eq!(t.render(), "<h1>hello</h1>");
}

#[test]
fn renders_two_vars() {
    #[derive(SlimR)]
    #[template(source = "h1 Hello Mr. {{first}} {{last}}")]
    struct Name {
        first: String,
        last: String,
    }
    let n = Name {
        first: "John".into(),
        last: "Smith".into(),
    };
    assert_eq!(n.render(), "<h1>Hello Mr. John Smith</h1>");
}

#[test]
fn handles_lifetimes() {
    #[derive(SlimR)]
    #[template(source = "h1 {{arg}}")]
    struct Title<'a> {
        arg: &'a str,
    }
    let t = Title { arg: "hello" };
    assert_eq!(t.render(), "<h1>hello</h1>");
}

#[test]
fn reads_slimr_file() {
    #[derive(SlimR)]
    #[template(path = "testing/templates/basic_tag.slimr")]
    struct Title<'a> {
        name: &'a str,
    }
    let t = Title { name: "World" };
    assert_eq!(t.render(), "<h1>Hello World</h1>");
}
