use http_api_derive::{http_api, http_api_endpoint};

#[http_api(warp = "impl_api_interface")]
trait ApiInterface {
    #[http_api_endpoint(method = "get")]
    fn ping(&self) -> Result<(), String>;

    #[http_api_endpoint(method = "post")]
    fn set_type(&self) -> Result<(), String>;

    #[http_api_endpoint(method = "get", rename = "type")]
    fn get_type(&self) -> Result<u64, String>;
}
