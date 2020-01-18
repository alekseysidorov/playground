use http_api_derive::FromUrlQuery;
use http_api::FromUrlQuery;

#[derive(FromUrlQuery)]
struct MyQuery {
    first: String,
    second: u64,
}

#[test]
fn test_from_url_query() {
    let query = "first=abacaba&second=10";

    let parsed = MyQuery::from_str(query).unwrap();
    assert_eq!(parsed.first, "abacaba");
    assert_eq!(parsed.second, 10);
}