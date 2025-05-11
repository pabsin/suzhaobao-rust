use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct MiniAppLoginResponse {
    pub token: String,
    pub open_id: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct MiniAppLoginRequest {
    pub code: String,
    pub app_id: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct UpdateUsernameRequest {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct ItemListRequest {
    pub page: u64,
    pub page_size: u64,
    pub class: Option<u8>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct PhotoListRequest {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct CreatePhotoSpecRequest {
    pub name: String,
    pub width_px: u32,
    pub height_px: u32,
    pub width_mm: u32,
    pub height_mm: u32,
    pub dpi: u32,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct CreateIdPhotoRequest {
    pub image_id: String,
    pub item_id: String,
    pub is_beauty_on: u8,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreateIdPhotoResult {
    pub status: bool,
    pub image_base64_standard: String,
    pub image_base64_hd: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AddBackgroundResult {
    pub status: bool,
    pub image_base64: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColourizeResult {
    pub status: u8,
    pub processed_image: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct UpdateIdPhotoRequest {
    pub color: String,
    pub image_id: String,
    pub render: u32,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct GenerateLayoutPhotoRequest {
    pub height: String,
    pub dpi: String,
    pub image_id: String,
    pub width: String,
    pub kb: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Deserialize)]
pub struct MattingRequest {
    pub dpi: String,
    pub image_id: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetVideoUnitResponse {
    pub video_unit_id: String,
    pub download_hd: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IdPhotoResponse {
    pub image_id: String,
    pub image_base64: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImageIdRequest {
    pub image_id: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreatePhotoDto {
    pub user_id: String,
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub processing_key: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UpdatePhotoDto {
    pub name: Option<String>,
    pub file_path: Option<String>,
    pub colorize_key: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
