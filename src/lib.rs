#![allow(non_snake_case)]
use frpc::*;

/// The request message containing the user's name.
#[derive(Input)]
struct HelloRequest {
    name: String,
}

/// The response message containing the greetings.
#[derive(Output)]
struct HelloReply {
    message: String,
}

async fn SayHello(req: HelloRequest) -> HelloReply {
    HelloReply {
        message: format!("Hello {}", req.name),
    }
}

async fn SayHelloAgain(req: HelloRequest) -> HelloReply {
    HelloReply {
        message: format!("Hello Again, {}", req.name),
    }
}

declare! {
    /// The greeting service definition.
    pub service Greeter {
        /// Sends a greeting
        rpc SayHello = 1;

        /// Sends another greeting
        rpc SayHelloAgain = 2;
    }
}
