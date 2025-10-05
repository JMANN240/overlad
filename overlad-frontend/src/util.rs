use gloo::net::http::RequestBuilder;
use image::Rgba;

pub trait WithToken {
    fn with_token(self, token: impl AsRef<str>) -> Self;
}

impl WithToken for RequestBuilder {
    fn with_token(self, token: impl AsRef<str>) -> Self {
        self.header("Authorization", &format!("Bearer {}", token.as_ref()))
    }
}
