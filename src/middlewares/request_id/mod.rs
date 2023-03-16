use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Data, Request, Response};

pub struct RequestId {}

#[rocket::async_trait]
impl Fairing for RequestId {
    // This is a request and response fairing named "Request ID".
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Add a unique ID to each request as a header.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // add a header to the request
        let uuid = uuid::Uuid::new_v4();
        let header = Header::new("x-request-id", uuid.to_string());
        request.add_header(header)
    }

    // Add the hader x-request-id to each response.
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        //  get header from request
        let header = request.headers().get_one("x-request-id").unwrap();
        // add header to response
        let header = Header::new("x-request-id", header);
        response.set_header(header);
    }
}
