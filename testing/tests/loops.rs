use askama::Template;

#[derive(Template)]
#[template(path = "for.html")]
struct ForTemplate<'a> {
    strings: Vec<&'a str>,
    tuple_strings: Vec<(&'a str, &'a str)>,
}

#[test]
fn test_for() {
    let s = ForTemplate {
        strings: vec!["A", "alfa", "1"],
        tuple_strings: vec![("B", "beta")],
    };
    assert_eq!(
        s.render().unwrap(),
        "0. A (first)\n1. alfa\n2. 1\n\n0. B,beta (first)\n"
    );
}

#[derive(Template)]
#[template(path = "nested-for.html")]
struct NestedForTemplate<'a> {
    seqs: Vec<&'a [&'a str]>,
}

#[test]
fn test_nested_for() {
    let alpha = vec!["a", "b", "c"];
    let numbers = vec!["one", "two"];
    let s = NestedForTemplate {
        seqs: vec![&alpha, &numbers],
    };
    assert_eq!(s.render().unwrap(), "1\n  0a1b2c2\n  0one1two");
}

#[derive(Template)]
#[template(path = "precedence-for.html")]
struct PrecedenceTemplate<'a> {
    strings: Vec<&'a str>,
}

#[test]
fn test_precedence_for() {
    let s = PrecedenceTemplate {
        strings: vec!["A", "alfa", "1"],
    };
    assert_eq!(
        s.render().unwrap(),
        "0. A2 (first)\n1. alfa4\n2. 16 (last)\n"
    );
}

#[derive(Template)]
#[template(path = "for-range.html")]
struct ForRangeTemplate {
    init: i32,
    end: i32,
}

#[test]
fn test_for_range() {
    let s = ForRangeTemplate { init: -1, end: 1 };
    assert_eq!(
        s.render().unwrap(),
        "foo (first)\nfoo (last)\nbar\nbar\nfoo\nbar\nbar\n"
    );
}

#[derive(Template)]
#[template(source = "{% for i in [1, 2, 3] %}{{ i }}{% endfor %}", ext = "txt")]
struct ForArrayTemplate;

#[test]
fn test_for_array() {
    let t = ForArrayTemplate;
    assert_eq!(t.render().unwrap(), "123");
}

#[derive(Template)]
#[template(
    source = "{% for i in [1, 2, 3].iter() %}{{ i }}{% endfor %}",
    ext = "txt"
)]
struct ForMethodCallTemplate;

#[test]
fn test_for_method_call() {
    let t = ForMethodCallTemplate;
    assert_eq!(t.render().unwrap(), "123");
}

#[derive(Template)]
#[template(
    source = "{% for i in ::std::iter::repeat(\"a\").take(5) %}{{ i }}{% endfor %}",
    ext = "txt"
)]
struct ForPathCallTemplate;

#[test]
fn test_for_path_call() {
    assert_eq!(ForPathCallTemplate.render().unwrap(), "aaaaa");
}

#[derive(Template)]
#[template(
    source = "{% for i in [1, 2, 3, 4, 5][3..] %}{{ i }}{% endfor %}",
    ext = "txt"
)]
struct ForIndexTemplate;

#[test]
fn test_for_index() {
    let t = ForIndexTemplate;
    assert_eq!(t.render().unwrap(), "45");
}

#[derive(Template)]
#[template(
    source = "{% for (i, j) in (0..10).zip(10..20).zip(30..40) %}{{ i.0 }} {{ i.1 }} {{ j }} {% endfor %}",
    ext = "txt"
)]
struct ForZipRangesTemplate;

#[test]
fn test_for_zip_ranges() {
    let t = ForZipRangesTemplate;
    assert_eq!(
        t.render().unwrap(),
        "0 10 30 1 11 31 2 12 32 3 13 33 4 14 34 5 15 35 6 16 36 7 17 37 8 18 38 9 19 39 "
    );
}
