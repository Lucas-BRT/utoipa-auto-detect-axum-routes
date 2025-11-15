use axum::{Router, extract::Path, http::StatusCode, serve};
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

const PORT: u16 = 3000;

#[derive(OpenApi)]
#[openapi(info(
    description = "uma aplicação simples para testar as capacidades de detecção automática de rotas do Utoipa com Axum"
))]
struct ApiDoc;

#[utoipa::path(get, path = "/hello")]
async fn hello() -> (StatusCode, String) {
    (StatusCode::OK, "Hello, world!".to_string())
}

#[utoipa::path(
    get,
    path = "/say/{text}",
    params(
        ("text" = String, Path, description = "O texto a ser repetido pelo servidor")
    ),
)]
async fn say(Path(text): Path<String>) -> (StatusCode, String) {
    println!("{text}");
    (StatusCode::OK, text)
}

fn main_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(say))
        .routes(routes!(hello))
}

async fn setup_tcp_listener() -> TcpListener {
    TcpListener::bind((Ipv4Addr::LOCALHOST, PORT))
        .await
        .expect("failed to bind to TCP port {PORT}")
}

async fn launch_server(listener: TcpListener, router: Router) {
    serve(listener, router).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = setup_tcp_listener().await;

    let openapi_router = OpenApiRouter::with_openapi(ApiDoc::openapi()).merge(main_router());

    let (router, doc) = openapi_router.split_for_parts();

    println!("api docs: {:#?}", doc.paths.paths.keys());

    println!(
        "server launched at http://{}:{}/",
        Ipv4Addr::LOCALHOST,
        PORT
    );
    launch_server(listener, router).await;
}
