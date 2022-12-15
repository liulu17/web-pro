use std::io::{Error, ErrorKind};
use std::str::FromStr;
// import the Filter trait from warp
use warp::Filter;
use serde::Serialize;

#[derive(Debug,Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug,Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

//async fn get_question() -> Result<impl warp::Reply,warp::Rejection> {
//
//}
#[tokio::main]
async fn main() {
    // create a path Filter
    let hello = warp::path("hello").map(|| format!("Hello, World!"));

    // start the server and pass the route filter to it
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
