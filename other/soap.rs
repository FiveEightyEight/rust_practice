// A SOAP server is a type of web service that uses the Simple Object Access Protocol (SOAP) for communication.
// In Rust, you can implement a SOAP server by using a library like tower-soap or rouille.
// To use tower-soap, you would first need to add it to your Cargo.toml file as a dependency:

[dependencies]
tower-soap = "0.1"

use tower_soap::{
    soap12,
    soap12::{Envelope, Fault, Response},
    soap12::fault::{Code, Data, FaultCode, FaultReason},
    util::{self, xml},
};

struct MyService;

#[derive(Debug, Clone)]
struct HelloRequest {
    name: String,
}

#[derive(Debug, Clone)]
struct HelloResponse {
    greeting: String,
}

impl MyService {
    fn hello(&self, request: HelloRequest) -> Result<HelloResponse, Fault> {
        Ok(HelloResponse {
            greeting: format!("Hello, {}!", request.name),
        })
    }
}


// Next, you can use the tower-soap crate to define the structure of your SOAP service and its methods.
// Here is an example of a simple SOAP service that has a hello method that takes a name parameter and returns a greeting message:

use tower_soap::{
    soap12,
    soap12::{Envelope, Fault, Response},
    soap12::fault::{Code, Data, FaultCode, FaultReason},
    util::{self, xml},
};

struct MyService;

#[derive(Debug, Clone)]
struct HelloRequest {
    name: String,
}

#[derive(Debug, Clone)]
struct HelloResponse {
    greeting: String,
}

impl MyService {
    fn hello(&self, request: HelloRequest) -> Result<HelloResponse, Fault> {
        Ok(HelloResponse {
            greeting: format!("Hello, {}!", request.name),
        })
    }
}


// Once you have defined your SOAP service and its methods, you can use the tower-soap crate to create an HTTP server that can handle SOAP requests and responses.
// Here is an example of how you can do this using the hyper crate:

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use tower_soap::{
    soap12,
    soap12::{Envelope, Fault, Response},
    soap12::fault::{Code, Data, FaultCode, FaultReason},
    util::{self, xml},
};

struct MyService;

#[derive(Debug, Clone)]
struct HelloRequest {
    name: String,
}

#[derive(Debug, Clone)]
struct HelloResponse {
    greeting: String,
}

impl MyService {
    fn hello(&self, request: HelloRequest) -> Result<HelloResponse, Fault> {
        Ok(HelloResponse {
            greeting: format!("Hello, {}!", request.name),
        })
    }
}

async fn handle_request(req: Request<Body>, service: MyService) -> Result<Response<Body>, Fault> {
    let (parts, body) = req.into_parts();
    let method = match parts.method {
        Method::POST => util::http::Method::Post,
        _ => return Err(Fault::new(Code::Sender, FaultReason::Text("Method not allowed"))),
    };

    let (response, _status) = soap12::dispatch(method, body, |request: Envelope| {
        let body = request.body.first().expect("request has no body");

        match body.name.local_name.as_str() {
            "hello" => {
                let request = util::xml::from_str(&body.data).expect("failed to parse request");
                let response = service.hello(request)?;
                let response = util::xml::to_string(&response).expect("failed to serialize response");
                Ok(Response::new(response))
            }
            _ => Err(Fault::new(Code::Sender, FaultReason::Text("Method not found"))),
        }
    })
}