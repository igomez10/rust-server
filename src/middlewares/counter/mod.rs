use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    pub get: AtomicUsize,
    pub post: AtomicUsize,
}

#[rocket::async_trait]
impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Get => {
                self.get.fetch_add(1, Ordering::Relaxed);
                println!("GET request");
            }
            Method::Post => {
                self.post.fetch_add(1, Ordering::Relaxed);
                println!("POST request");
            }
            _ => return,
        };
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Don't change a successful user's response, ever.
        if response.status() != Status::NotFound {
            return;
        }

        // Rewrite the response to return the current counts.
        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("Get: {}\nPost: {}", get_count, post_count);

            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), Cursor::new(body));
        }
    }
}