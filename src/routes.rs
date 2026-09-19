use crate::api_contract::{GreetingResponse, HealthResponse};
use crate::service::SharedApiService;
use actix_web::{HttpResponse, Responder, web};
use std::sync::Arc;

const OPENAPI_SPEC: &str = include_str!("../openapi/api.yaml");

async fn health(service: web::Data<SharedApiService>) -> impl Responder {
    web::Json(service.health())
}

async fn greet(name: web::Path<String>, service: web::Data<SharedApiService>) -> impl Responder {
    web::Json(service.greeting(&name.into_inner()))
}

async fn openapi_contract() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/yaml")
        .body(OPENAPI_SPEC)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/v1/greetings/{name}", web::get().to(greet))
        .route("/openapi", web::get().to(openapi_contract));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::ApiService;
    use actix_web::{App, http::StatusCode, test};
    use serde_json::Value;
    use serde_yaml::Value as YamlValue;

    struct MockApiService;

    impl ApiService for MockApiService {
        fn health(&self) -> HealthResponse {
            HealthResponse { status: "ok" }
        }

        fn greeting(&self, name: &str) -> GreetingResponse {
            GreetingResponse {
                message: format!("Hello, {}!", name),
            }
        }
    }

    fn test_service() -> SharedApiService {
        Arc::new(MockApiService)
    }

    fn test_app() -> App<
        impl actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        App::new()
            .app_data(web::Data::new(test_service()))
            .configure(configure)
    }

    fn openapi_spec() -> YamlValue {
        serde_yaml::from_str(OPENAPI_SPEC).expect("openapi contract should parse")
    }

    fn response_schema_for(path: &str, content_type: &str) -> YamlValue {
        let spec = openapi_spec();
        let schema_ref = spec["paths"][path]["get"]["responses"]["200"]["content"][content_type]["schema"]
            ["$ref"]
            .as_str()
            .expect("response should reference a component schema");

        let schema_name = schema_ref
            .rsplit('/')
            .next()
            .expect("schema reference should contain a name");

        spec["components"]["schemas"][schema_name].clone()
    }

    fn assert_body_matches_contract(body: &Value, schema: &YamlValue) {
        let body_object = body
            .as_object()
            .expect("response body should be a JSON object");

        let required_fields = schema["required"]
            .as_sequence()
            .expect("schema should define required fields");

        for field in required_fields {
            let field_name = field
                .as_str()
                .expect("required field names should be strings");
            assert!(
                body_object.contains_key(field_name),
                "missing required field `{field_name}` in response body"
            );

            let expected_type = schema["properties"][field_name]["type"]
                .as_str()
                .expect("property type should be defined");
            let actual_value = &body_object[field_name];

            match expected_type {
                "string" => assert!(
                    actual_value.is_string(),
                    "field `{field_name}` should be a string"
                ),
                other => panic!("unsupported schema type in test: {other}"),
            }

            if let Some(example) = schema["properties"][field_name]["example"].as_str() {
                assert_eq!(
                    actual_value, example,
                    "field `{field_name}` does not match contract example"
                );
            }
        }
    }

    #[actix_web::test]
    async fn health_response_matches_contract() {
        let app = test::init_service(test_app()).await;
        let request = test::TestRequest::get().uri("/health").to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = test::read_body_json(response).await;
        let schema = response_schema_for("/health", "application/json");

        assert_body_matches_contract(&body, &schema);
    }

    #[actix_web::test]
    async fn greeting_response_matches_contract() {
        let app = test::init_service(test_app()).await;
        let request = test::TestRequest::get()
            .uri("/v1/greetings/Ada")
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = test::read_body_json(response).await;
        let schema = response_schema_for("/v1/greetings/{name}", "application/json");

        assert_body_matches_contract(&body, &schema);
    }

    #[actix_web::test]
    async fn openapi_endpoint_returns_contract_document() {
        let app = test::init_service(test_app()).await;
        let request = test::TestRequest::get().uri("/openapi").to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .and_then(|value| value.to_str().ok()),
            Some("application/yaml")
        );

        let body = test::read_body(response).await;

        assert_eq!(
            std::str::from_utf8(&body).expect("openapi body should be utf-8"),
            OPENAPI_SPEC
        );
    }

    #[actix_web::test]
    async fn routes_use_injected_service() {
        struct CustomGreetingService;

        impl ApiService for CustomGreetingService {
            fn health(&self) -> HealthResponse {
                HealthResponse { status: "healthy" }
            }

            fn greeting(&self, name: &str) -> GreetingResponse {
                GreetingResponse {
                    message: format!("Welcome, {}.", name),
                }
            }
        }

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(
                    Arc::new(CustomGreetingService) as SharedApiService
                ))
                .configure(configure),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/v1/greetings/Grace")
            .to_request();

        let response = test::call_service(&app, request).await;
        let body: Value = test::read_body_json(response).await;

        assert_eq!(body["message"], "Welcome, Grace.");
    }
}
