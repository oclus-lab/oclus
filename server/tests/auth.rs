use actix_http::Request;
use actix_web::test;
use oclus_server::app;
use oclus_server::dto::auth::{LoginRequest, RegisterRequest};
use oclus_server::dto::error::ErrorDto;

mod util;

fn register_request(email: &str, username: &str, password: &str) -> Request {
    let request_body = RegisterRequest {
        email: email.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };

    test::TestRequest::post()
        .uri("/auth/register")
        .set_json(request_body)
        .to_request()
}

#[actix_web::test]
pub async fn test_register() {
    let db_pool = util::setup_test_db();
    let app = test::init_service(app(db_pool)).await;

    let request = register_request("test@test.com", "test_username", "test_password");

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
pub async fn test_register_data_validation() {
    let db_pool = util::setup_test_db();
    let app = test::init_service(app(db_pool)).await;

    // use a valid email
    let request = register_request("test@test.com", "test_username", "test_password");
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    // use an invalid email
    let request = register_request("invalid_email", "test_username", "test_password");
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_client_error());

    let error: ErrorDto = test::read_body_json(response).await;
    assert!(matches!(error, ErrorDto::InvalidData));
}

#[actix_web::test]
pub async fn test_register_email_conflict() {
    let db_pool = util::setup_test_db();
    let app = test::init_service(app(db_pool)).await;

    let request = register_request("test@test.com", "test_username", "test_password");
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    // redo a request with the same email
    let request = register_request(
        "test@test.com",
        "another_test_username",
        "another_test_password",
    );
    let response = test::call_service(&app, request).await;

    // should return an email conflict error
    assert!(response.status().is_client_error());
    let error: ErrorDto = test::read_body_json(response).await;
    match error {
        ErrorDto::Conflict(field) => {
            assert_eq!(field, "email");
        }
        _ => {
            panic!("Expected Conflict error, got something else");
        }
    }
}

fn login_request(email: &str, password: &str) -> Request {
    let request_body = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    test::TestRequest::post()
        .uri("/auth/login")
        .set_json(request_body)
        .to_request()
}

#[actix_web::test]
pub async fn test_login() {
    let db_pool = util::setup_test_db();
    let app = test::init_service(app(db_pool)).await;

    let request = register_request("test@test.com", "test_username", "test_password");
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    // with good credentials
    let request = login_request("test@test.com", "test_password");
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    // with wrong credentials
    let request = login_request("test@test.com", "wrong_password");
    let response = test::call_service(&app, request).await;
    
    assert!(response.status().is_client_error());

    let error: ErrorDto = test::read_body_json(response).await;
    assert!(matches!(error, ErrorDto::InvalidCredentials));
}
