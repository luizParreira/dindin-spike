use dindin_spike::models::User;
use dindin_spike::server;
use pretty_assertions::assert_eq;
use rocket::http::Status;
use rocket::local::Client;
use serde_json::json;

#[test]
fn test_email_auth_input() {
    let client = Client::new(server::init()).unwrap();

    let request_body = json!({
      "auth_input": {
        "type": "Email",
        "attributes": {
          "email": "luiz@dindin.app"
        }
      }
    });

    let request = client.post("/authenticate").body(request_body.to_string());
    let mut response = request.dispatch();

    let response_body: User = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response_body.email, Some("luiz@dindin.app".to_string()));
    assert_eq!(response_body.phone_number, None);
}

#[test]
fn test_phone_number_auth_input() {
    let client = Client::new(server::init()).unwrap();

    let request_body = json!({
      "auth_input": {
        "type": "PhoneNumber",
        "attributes": {
          "phone": "11969493020"
        }
      }
    });

    let request = client.post("/authenticate").body(request_body.to_string());
    let mut response = request.dispatch();

    let response_body: User = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response_body.email, None);
    assert_eq!(response_body.phone_number, Some("11969493020".to_string()));
}
