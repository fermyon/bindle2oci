use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// ID of application bindle.
    #[clap(long = "bindle")]
    pub bindle_id: String,

    /// URL of bindle server.
    #[clap(long = "bindle-url", env = "BINDLE_URL")]
    pub bindle_url: String,

    /// Basic http auth username for the bindle server
    #[clap(
        long = "bindle-username",
        env = "BINDLE_USERNAME",
        requires = "bindle_password"
    )]
    pub bindle_username: Option<String>,

    /// Basic http auth password for the bindle server
    #[clap(
        long = "bindle-password",
        env = "BINDLE_PASSWORD",
        requires = "bindle_username"
    )]
    pub bindle_password: Option<String>,

    /// Reference of the Spin application
    #[clap(long)]
    pub reference: String,

    /// Ignore server certificate errors from bindle server or registry
    #[clap(long, short = 'k')]
    pub insecure: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let tmpdir = tempfile::tempdir()?.path().canonicalize()?;

    let bindle_connection_info = spin_loader::bindle::BindleConnectionInfo::new(
        cli.bindle_url,
        cli.insecure,
        cli.bindle_username,
        cli.bindle_password,
    );
    let app = spin_loader::from_bindle(cli.bindle_id.as_str(), bindle_connection_info, tmpdir)
        .await
        .context("failed to load app from bindle")?;

    // TODO: OCI client auth
    let mut oci_client = spin_oci::Client::new(cli.insecure, None)
        .await
        .context("failed to build an OCI client")?;

    oci_client
        .push(&app, cli.reference)
        .await
        .context("failed to push to registry")?;

    Ok(())
}
