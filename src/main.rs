mod math;
mod models;
mod square;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(hello_handler);
    let bye = warp::path!("bye" / String).map(|name| format!("Bye, {}!", name));
    let request = warp::path!("request").and_then(makehttprequest);
    let routes = hello.or(bye).or(request);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn hello_handler(name: String) -> String {
    format!("Hello, {}!", name)
}

async fn makehttprequest() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header("User-Agent", "warp")
        .send()
        .await
        .unwrap();

    println!("Status: {}", res.status());
    return Ok(Box::new(res.text().await.unwrap()));
}
