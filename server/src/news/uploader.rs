use crate::{s3::public, AppState};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::Error;
use actix_web::{error::ErrorInternalServerError, web, HttpResponse, Responder};
use uuid::Uuid;

use super::news_repository;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

pub async fn save_news_img(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let news_id = path.into_inner();
    let news = news_repository::find_by_pk(news_id, &data.db).await;

    match news {
        Ok(news) => {
            for f in form.files {
                let path = format!(
                    "./tmp/{}",
                    f.file_name.clone().unwrap_or_else(|| "unknown".to_string())
                );

                let s3_key = format!(
                    "news/img/{}/{}",
                    news.news_id,
                    f.file_name.clone().unwrap_or_else(|| "unknown".to_string())
                );

                if let Err(_) = f.file.persist(&path) {
                    return Err(ErrorInternalServerError("file save error"));
                }

                if let Err(_) = public::PublicBucket::put_object(&path, &s3_key).await {
                    return Err(ErrorInternalServerError("s3 error"));
                } else {
                    let bucket_name = public::PublicBucket::get_name();
                    let full_path = format!("{}/{}", bucket_name, s3_key);
                    if let Err(_) =
                        news_repository::update_img_by_pk(news_id, full_path, &data.db).await
                    {
                        return Err(ErrorInternalServerError("save image url error"));
                    }
                }
            }

            Ok(HttpResponse::Ok())
        }
        Err(_) => Err(ErrorInternalServerError("database error")),
    }
}
