use crate::middleware::must_login;
use salvo::Router;
use salvo::http::request::SecureMaxSize;

mod other;
mod photo_item;
mod photos;
mod user;
mod wechat;

pub fn routes() -> Router {
    Router::new()
        .push(
            Router::with_path("id-photo")
                .push(Router::with_path("login").post(wechat::miniapp_login))
                .push(
                    Router::with_path("update-avatar")
                        .hoop(must_login)
                        .post(wechat::update_user_avatar),
                )
                .push(
                    Router::with_path("update-nickname")
                        .hoop(must_login)
                        .post(wechat::update_user_nickname),
                )
                .push(Router::with_path("upload").hoop(must_login).post(wechat::upload))
                .push(Router::with_path("spec-list").get(photo_item::photo_item_list))
                .push(Router::with_path("spec-detail").get(photo_item::photo_item))
                .push(Router::with_path("photo-list").hoop(must_login).get(photos::photo_list))
                .push(Router::with_path("photo-detail").hoop(must_login).get(photos::photo_detail))
                .push(
                    Router::with_path("create-id-photo")
                        .hoop(must_login)
                        .post(photos::create_id_photo),
                )
                .push(
                    Router::with_path("update-photo")
                        .hoop(must_login)
                        .hoop(SecureMaxSize(512 * 1024))
                        .post(photos::update_id_photo),
                )
                .push(
                    Router::with_path("create-photo-spec")
                        .hoop(must_login)
                        .post(photo_item::create_photo_item),
                )
                .push(Router::with_path("get-video-unit").get(wechat::get_video_unit))
                .push(
                    Router::with_path("download-photo")
                        .hoop(must_login)
                        .post(photos::download_photo),
                )
                .push(
                    Router::with_path("delete-photo").hoop(must_login).delete(photos::delete_photo),
                )
                .push(Router::with_path("colourize").hoop(must_login).post(other::colourize))
                .push(
                    Router::with_path("generate-layout-photo")
                        .hoop(must_login)
                        .post(other::generate_layout_photo),
                )
                .push(Router::with_path("matting").hoop(must_login).post(other::matting))
                .push(Router::with_path("cartoon").hoop(must_login).post(other::cartoon)),
        )
        .push(
            Router::with_path("user")
                .push(Router::with_path("register").post(user::register))
                .push(Router::with_path("me").hoop(must_login).patch(user::update).get(user::me)),
        )
}
