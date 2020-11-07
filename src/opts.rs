extern crate selog;

use crate::{Format, Target};

selog::opts! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ClapOpts {
        #[clap(long, short = 'F', about = "The output format", possible_values = Format::VARIANTS,
               default_value = "term")]
        format: Format,
        #[clap(long, short, about = "The output file.")]
        output: Option<String>,
        #[clap(short, long, about = "The string to convert to QR code.")]
        text: String
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Opts {
    pub format: Format,
    pub target: Target,
    pub text: String,
}

impl From<ClapOpts> for Opts {
    fn from(opts: ClapOpts) -> Self {
        Self {
            format: opts.format,
            target: opts.output.into(),
            text: opts.text,
        }
    }
}

/// Initialize the application.
pub fn init() -> Opts {
    let opts = ClapOpts::parse();
    opts.init_log().expect("Failed to initialize logger.");

    let opts = Opts::from(opts);

    log::debug!("Output string: {}", opts.text);
    log::debug!("Output format: {}", opts.format);
    log::debug!("Output target: {}", opts.target);

    opts
}
