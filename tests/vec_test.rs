use noserde::{ToJson, impl_to_json};

struct Article {
    title: String,
    tags: Vec<String>,
}

impl_to_json!(Article { title, tags });

#[test]
fn serializes_vec_field() {
    let article = Article {
        title: "Rust JSON".to_string(),
        tags: vec![
            "rust".to_string(),
            "json".to_string(),
            "no-deps".to_string(),
        ],
    };

    assert_eq!(
        article.to_json(),
        r#"{"title":"Rust JSON","tags":["rust","json","no-deps"]}"#
    );
}
