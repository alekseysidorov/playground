use http_api::FromUrlQuery;
use http_api_derive::FromUrlQuery;

#[derive(FromUrlQuery)]
struct MyQuery {
    first: String,
    second: u64,
}

#[derive(FromUrlQuery)]
struct OptionalQuery {
    first: String,
    opt_value: Option<u64>,
}

#[test]
fn test_from_url_simple() {
    let query = "first=abacaba&second=10";

    let parsed = MyQuery::from_query_str(query).unwrap();
    assert_eq!(parsed.first, "abacaba");
    assert_eq!(parsed.second, 10);
}

#[test]
fn test_from_url_with_option() {
    let query_1 = "first=abacaba";
    let parsed = OptionalQuery::from_query_str(query_1).unwrap();
    assert_eq!(parsed.first, "abacaba");
    assert_eq!(parsed.opt_value, None);

    let query_2 = "first=cababa&opt_value=10";
    let parsed = OptionalQuery::from_query_str(query_2).unwrap();
    assert_eq!(parsed.first, "cababa");
    assert_eq!(parsed.opt_value, Some(10));
}
