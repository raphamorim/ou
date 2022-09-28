extern crate clap;
use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use clap::Parser;
use std::fs;
use std::{io, net::SocketAddr};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

const OPTSET_COMMAND: &str = "COMMAND";
const OPTSET_OUTPUT: &str = "OUTPUT";
const OPTSET_DEBUGGING: &str = "DEBUGGING";
const OPTSET_BEHAVIOUR: &str = "BEHAVIOUR";

#[derive(Debug, Clone, clap::Parser)]
#[clap(author, bin_name = "cargo", name = "cargo-server", version, about)]
struct Args {
    /// Full command to run. -x and -s will be ignored!
    #[clap(
        hide = true,
        help_heading = OPTSET_COMMAND,
    )]
    pub cmd_trail: String,

    /// Path
    #[clap(
        long,
        value_name = "path",
        required = false,
        default_value = "",
        help_heading = OPTSET_BEHAVIOUR,
    )]
    pub path: String,

    /// Show the help
    #[clap(
        short = 'h',
        long = "help",
        help_heading = OPTSET_DEBUGGING,
    )]
    pub help: bool,

    /// Version
    #[clap(
        short = 'V',
        long = "version",
        help_heading = OPTSET_DEBUGGING,
    )]
    pub version: bool,

    /// Port
    #[clap(short, long, help_heading = OPTSET_BEHAVIOUR, value_parser, default_value_t = 8000)]
    pub port: u16,

    /// Open
    #[clap(short, long, value_parser, default_value_t = false)]
    pub open: bool,

    /// Quiet
    #[clap(
        short = 'q',
        long = "quiet",
        value_parser,
        default_value_t = false,
        help_heading = OPTSET_OUTPUT,
    )]
    pub quiet: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = &args.port;
    let path = &args.path;
    let quiet = &args.quiet;
    let open = &args.open;
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|_, _| true))
        .allow_credentials(true);

    let mut server_path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    if !path.is_empty() {
        server_path = path.to_string();
    }

    let app: _ = Router::new()
        .fallback(get_service(ServeDir::new(&server_path)).handle_error(handle_error))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], *port as u16));
    let server = format!("{}{}{}", "\x1b[93m", "[cargo-server]", "\x1b[0m");

    let files = fs::read_dir(&server_path).unwrap();
    let mut files_str = String::new();
    for file in files {
        files_str = files_str
            + " "
            + &file
                .as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .ok()
                .unwrap();
    }

    if !*quiet {
        println!("{} path: {}", server, server_path);

        if !files_str.contains("./index.html") {
            println!("{} hint: consider to add an 'index.html' file", server);
        }

        println!("{} listening on: {}", server, addr);
    }

    if open == &true {
        let url: String = format!("http://{}", addr);
        match open::that(&url) {
            Ok(()) => {
                if !*quiet {
                    println!("{} opened '{}' successfully on browser.", server, url)
                }
            }
            Err(err) => {
                if !*quiet {
                    eprintln!(
                        "{} an error occurred when opening {} on browser: {}",
                        server, url, err
                    )
                }
            }
        }
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
