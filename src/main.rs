use rpc::*;
use rpc_example::Greeter;
use std::io;

fn codegen_init() {
    #[cfg(debug_assertions)]
    {
        use frpc_codegen_client::{typescript, Config};
        Config {
            typescript: Some(typescript::Config {
                out_dir: "./target/codegen/".into(),
                preserve_import_extension: true,
                ..Default::default()
            }),
            ..Default::default()
        }
        .generate_binding(&[&Greeter.into()])
        .expect("failed to generate bindings");
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    codegen_init();
    println!("server runing at 127.0.0.1:4433\n");

    Server::new("./.secret/key.pem", "./.secret/cert.pem")?
        .bind("127.0.0.1:4433", |_addr, _| async { App {} })
        .await
}

#[derive(Clone)]
struct App {}

impl Application for App {
    async fn stream(self, mut ctx: Ctx) {
        serve! {ctx:
            "/greeter" => Greeter; ()
        }
    }
}
