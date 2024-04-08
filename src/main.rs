#[cfg(feature = "cli")]
mod cli {
    use clap::{Parser, Subcommand};
    use respicta::server::app;
    use std::path::PathBuf;
    use tokio::{net::TcpListener, signal};
    #[derive(Parser)]
    #[command(version, about, long_about = None, arg_required_else_help = true)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand)]
    pub enum Commands {
        #[clap(
            arg_required_else_help = true,
            disable_help_flag = true,
            after_help = "
    Examples:
    
      respicta convert --width 100 --height 100 input.jpg output.jpg
    
    "
        )]
        /// Convert images from one format to another
        Convert {
            /// Input image path
            input_path: PathBuf,
            /// Output image path
            output_path: PathBuf,
            /// Width of the output image
            /// If not set, the width will be the same as the input image
            #[clap(short, long)]
            width: Option<u32>,
            /// Height of the output image
            /// If not set, the height will be the same as the input image
            #[clap(short, long)]
            height: Option<u32>,
            #[clap(long, action = clap::ArgAction::HelpLong)]
            help: Option<bool>,
        },
        /// Start a server to convert images
        Server {
            /// Address to bind the server to (default: 0.0.0.0:3000)
            #[clap(short, long)]
            address: Option<String>,
            /// Maximum file size in bytes (default: 10MB)
            #[clap(short, long)]
            limit: Option<usize>,
        },
    }

    pub async fn start_server(
        address: Option<String>,
        limit: Option<usize>,
    ) -> std::io::Result<()> {
        let address = address.unwrap_or_else(|| "0.0.0.0:3000".to_string());
        let app = app(limit);
        let listener = TcpListener::bind(address.clone()).await;
        match listener {
            Ok(listener) => {
                println!("Server started at http://{address}");
                axum::serve(listener, app)
                    .with_graceful_shutdown(shutdown_signal())
                    .await
            }
            Err(error) => Err(error),
        }
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
    use crate::cli::Commands;
    use clap::Parser;
    use respicta::{convert, Config};

    let cli = cli::Cli::parse();

    match cli.command {
        Some(Commands::Convert {
            input_path,
            output_path,
            width,
            height,
            ..
        }) => {
            convert(&Config {
                input_path: &input_path,
                output_path: &output_path,
                width,
                height,
            })
            .unwrap();
        }
        Some(Commands::Server { address, limit }) => {
            cli::start_server(address, limit).await.unwrap();
        }
        None => unreachable!(),
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    unimplemented!("Please enable the `cli` feature to use the CLI")
}
