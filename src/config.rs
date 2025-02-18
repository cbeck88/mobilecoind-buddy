use clap::Parser;
use deqs_api::DeqsClientUri;
use mc_mobilecoind_api::MobilecoindUri;
use std::path::PathBuf;

/// Command line config, set with defaults that will work with
/// a standard mobilecoind instance
#[derive(Clone, Debug, Parser)]
#[clap(name = "mobilecoind-buddy", about = "A front-end for mobilecoind")]
pub struct Config {
    /// Path to json-formatted key file, containing mnemonic or root entropy.
    #[clap(long, env = "MC_KEYFILE")]
    pub keyfile: PathBuf,

    /// MobileCoinD URI.
    #[clap(
        long,
        default_value = "insecure-mobilecoind://127.0.0.1/",
        env = "MC_MOBILECOIND_URI"
    )]
    pub mobilecoind_uri: MobilecoindUri,

    /// Deqs URI. (Optional)
    #[clap(long, env = "MC_DEQS_URI")]
    pub deqs_uri: Option<DeqsClientUri>,

    /// Pixels per point. Set this if the display is too large or too small for your monitor. Default is 2 but on a high res monitor 4 is better.
    #[clap(long, env)]
    pub pixels_per_point: Option<u16>,
}
