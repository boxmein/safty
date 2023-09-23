use hyper::body::Bytes;

#[allow(unused)]
pub fn get_form_field_value(body: &Bytes, field_name: &str) -> Option<String> {
    form_urlencoded::parse(body)
        .find(|(k, _)| k == field_name)
        .map(|(_, v)| v.to_string())
}
