use std::fs;
use std::str::FromStr;
use std::net::{Ipv4Addr, SocketAddrV4};
use serde_json::Value;
mod process_file;
use process_file::handle_sending_file;
mod custom_ip_utils;
use custom_ip_utils::{get_ip, calculate_broadcast_address, fetch_device_ips_from_broadcast};
use hyper::{Body, Request, Response};
use hyper::header::HeaderValue;

pub use super::custom_file::FileObject;

pub async fn handle_request(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Request");
    match (request.method(), request.uri().path()) {
        (&hyper::Method::GET, "/") => send_file_to_client("index.html"),
        (&hyper::Method::GET, "/ping") => send_json_object("{\"message\": \"pong\"}"),
        (&hyper::Method::GET, "/devices") => {
            let devices = get_local_devices();
            println!("devices {:?}", devices);
            send_json_object("{\"devices\": [] }")
        },
        (&hyper::Method::POST, "/send-file") => {
            let file_body = get_request_body(request).await;
            handle_sending_file(file_body)
        },
        _ => send_file_to_client("404.html")
    }
}

fn get_local_devices() -> Option<Vec<SocketAddrV4>> {
    let ip_address: Ipv4Addr = get_ip().unwrap();
    let subnet_mask: Ipv4Addr = Ipv4Addr::from_str("255.255.255.255").unwrap();

    let broadcast_address: Option<Ipv4Addr> = calculate_broadcast_address(ip_address, subnet_mask);
    println!("NETWORK {:?}", ip_address);

    let devices: Option<Vec<SocketAddrV4>> = match broadcast_address {
        Some(address) => fetch_device_ips_from_broadcast(address),
        _ => None
    };

    devices
}

async fn get_request_body(request: Request<Body>) -> Value {
    let body = hyper::body::to_bytes(request.into_body()).await.unwrap();
    let body_string = String::from_utf8_lossy(&body).to_string();

    let body_json: Value = match serde_json::from_str(&body_string) {
        Ok(value) => value,
        Err(error) => Value::Null
    };

    body_json
}

fn send_json_object(json_string: &str) -> Result<Response<Body>, hyper::Error> {
    let response = Response::builder()
        .header(
            hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("http://localhost:5173")
        )
        .header(
            hyper::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true")
        )
        .header("Content-Type", "text/json")
        .header("content-length", json_string.len())
        .body(Body::from(json_string.to_string()))
        .unwrap();

    Ok(response)
}

fn send_file_to_client(file_path: &str) -> Result<Response<Body>, hyper::Error> {
    let mut contents = String::new();

    match fs::read_to_string(file_path) {
        Ok(file_contents) => {
            contents = file_contents;
        }
        Err(_error) => {
            // If the file couldn't be found, the value is probably JSON object and should be kept as is.
            // println!("The file doesn't exist: {}", _error);
        }
    }

    let response = Response::builder()
        .header(
            hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("http://localhost:5173")
        )
        .header(
            hyper::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true")
        )
        .header("Content-Type", "text/json")
        .header("content-length", contents.len())
        .body(Body::from(contents))
        .unwrap();

    Ok(response)
}

// fn set_response_common_headers(response: , content_type: String) -> Response<Body> {
//     let response = response
//         .header(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173")
//         .header(hyper::header::CONTENT_TYPE, "text/json");

//     response
// }