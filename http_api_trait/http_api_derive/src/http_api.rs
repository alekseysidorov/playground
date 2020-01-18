use darling::{FromMeta, self};

#[derive(Debug)]
enum SupportedHttpMethod {
    Get,
    Post,
}

impl FromMeta for SupportedHttpMethod {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
         match value {
             "get" => Ok(SupportedHttpMethod::Get),
             "post" => Ok(SupportedHttpMethod::Post),
             other => Err(darling::Error::unknown_value(other))
         }
    }
}

#[derive(Debug, FromMeta)]
pub struct EndpointAttrs {
    method: SupportedHttpMethod,
    #[darling(default)]
    rename: Option<String>,
}