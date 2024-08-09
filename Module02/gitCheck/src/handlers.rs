use crate::model::RepoInfo;
use actix_web::{web, HttpResponse, Responder};
use reqwest;
use serde_json::from_str;

pub async fn get_repo_info(path: web::Path<(String, String)>) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.text().await.unwrap();
                let repo_info: RepoInfo = from_str(&body).unwrap();
                HttpResponse::Ok().json(repo_info)
            } else {
                HttpResponse::NotFound().body("Repository not Found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get_repo_info() {
        let mut app =
            test::init_service(App::new().route("/{owner}/{repo}", web::get().to(get_repo_info)))
                .await;

        let req = test::TestRequest::get().uri("/rust-lang/rust").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
