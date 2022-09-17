use std::path::PathBuf;
use clipstash::data::Db;
use clipstash::web::{renderer::Renderer};
use dotenv::dotenv;
use rocket::{Ignite, Rocket};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:db/data.db")]
    db_uri: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_dir: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spaw tokio runtime");

    let handle = rt.handle().clone();

    let _ = rt.block_on(async move {
        let renderer = Renderer::new(opt.template_dir);
        let db = Db::new(&opt.db_uri).await;

        let config = clipstash::RocketConfig { renderer, db };

        clipstash::rocket(config).launch().await.expect("failed to launch rocket server")
    });
}