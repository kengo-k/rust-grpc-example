mod hello {
    tonic::include_proto!("hello");
}

use hello::{
    hello_service_server::{HelloService, HelloServiceServer},
    HelloReply, HelloRequest,
};
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

#[derive(Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let hello_service = MyHelloService::default();

    tonic::transport::Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .add_service(
            Builder::configure()
                .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                    "hello_descriptor"
                ))
                .build()
                .unwrap(),
        )
        .serve(addr)
        .await?;

    Ok(())
}
