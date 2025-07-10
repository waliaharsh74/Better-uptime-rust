use poem::{get, handler, http::response, listener::TcpListener, post, web::{Json, Path}, Route, Server};

use crate::{request_input::CreateWebsiteInput, request_outputs::CreateWebsiteOutput};

pub mod request_input;
pub mod request_outputs;
#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[handler]

fn create_website(Json(data):Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url=data.url;
    let response=CreateWebsiteOutput{
        id:String::from("ID")
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
