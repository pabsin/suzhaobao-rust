use redis;
use salvo::conn::TcpListener;
use salvo::cors::Cors;
use salvo::jwt_auth::{ConstDecoder, CookieFinder, FormFinder, HeaderFinder, QueryFinder};
use salvo::prelude::JwtAuth;
use salvo::{Listener, Router, Server, Service};
use salvo_extra::affix_state;
use salvo_extra::caching_headers::CachingHeaders;
use salvo_extra::logging::Logger;
use salvo_extra::request_id::RequestId;
use sea_orm::ConnectOptions;
use std::sync::Arc;
use vegar_core::AppState;
use vegar_core::JwtClaims;
use vegar_core::settings::SETTINGS;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    let settings = SETTINGS.read().await;

    let cors = Cors::very_permissive().into_handler();

    let server_url = "0.0.0.0:5800";

    let mut opt = ConnectOptions::new(&settings.database.dsn);
    opt.sqlx_logging(false);

    let conn = sea_orm::Database::connect(opt).await.expect("Failed to connect to database");

    let jwthandler: JwtAuth<JwtClaims, _> =
        JwtAuth::new(ConstDecoder::from_secret(&settings.auth.jwt_secret.as_bytes()))
            .finders(vec![
                Box::new(HeaderFinder::new()),
                Box::new(QueryFinder::new("token")),
                Box::new(CookieFinder::new("token")),
                Box::new(FormFinder::new("token")),
            ])
            .force_passed(true);

    let redis_client = redis::Client::open(settings.redis.dsn.as_str()).unwrap();

    let settings = Arc::new(settings.clone());
    let state = AppState { conn, settings, redis: redis_client };

    let router = Router::with_hoop(CachingHeaders::new())
        .hoop(RequestId::new())
        .hoop(jwthandler)
        .hoop(affix_state::inject(state))
        .push(id_photo::routes());

    let service = Service::new(router).hoop(cors).hoop(Logger::new());

    let acceptor = TcpListener::new(&server_url).bind().await;
    Server::new(acceptor).serve(service).await;
}
