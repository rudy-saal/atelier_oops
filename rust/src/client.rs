use crate::response::PingResponse;

pub trait PingClient {
    fn ping(&self, url: &str) -> PingResponse;
}

pub struct HttpPingClient {}

impl PingClient for HttpPingClient {
    fn ping(&self, url: &str) -> PingResponse {
        let response = reqwest::blocking::get(url).unwrap();
        let status = response.status().as_u16();

        PingResponse { status }
    }
}
