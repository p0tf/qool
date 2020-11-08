extern crate selog;

use crate::{Format, Source, Target};
use std::path::Path;

selog::opts! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ClapOpts {
        #[clap(long, short, about = "The output format.",
               possible_values = Format::VARIANTS)]
        format: Option<Format>,
        #[clap(long, short, about = "The output file.")]
        output: Option<String>,
        #[clap(long, short = 'T', about = "The string to convert to QR code.",
               conflicts_with = "file")]
        text: Option<String>,
        #[clap(long, short = 'F', about = "The input file.",
               conflicts_with = "text")]
        file: Option<String>
    }
}

/// Configuration with the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Opts {
    pub format: Format,
    pub target: Target,
    pub source: Source,
}

impl From<ClapOpts> for Opts {
    fn from(opts: ClapOpts) -> Self {
        let opt_format = opts.format;
        let output = opts.output;
        let source = Source::new(opts.text, opts.file);

        let (format, target) = match (source.clone(), output, opt_format) {
            (Source::File(_), None, None) => (Format::Png, Target::File("a.png".to_string())),
            (_, None, None) => (Format::Term, Target::Stdout),
            (_, Some(s), None) => (
                match Path::new(&s).extension().map(|i| i.to_str()).flatten() {
                    Some("png") | Some("PNG") => Format::Png,
                    Some("jpg") | Some("JPG") | Some("jpeg") | Some("JPEG") => Format::Jpeg,
                    Some("gif") | Some("GIF") => Format::Gif,
                    Some("bmp") | Some("BMP") => Format::Bmp,
                    Some("svg") | Some("SVG") => Format::Svg,
                    _ => Format::Png,
                },
                Target::File(s),
            ),
            (_, None, Some(f @ Format::Term)) => (f, Target::Stdout),
            (_, None, Some(f @ _)) => (f.clone(), Target::File(format!("a.{}", f))),
            (_, Some(s), Some(f)) => (f, Target::File(s)),
        };

        Self {
            format,
            target,
            source,
        }
    }
}

/// Initialize the application.
pub fn init() -> Opts {
    let opts = ClapOpts::parse();
    opts.init_log().expect("Failed to initialize logger.");

    let opts = Opts::from(opts);

    log::debug!("Source: {}", opts.source);
    log::debug!("Output format: {}", opts.format);
    log::debug!("Output target: {}", opts.target);

    opts
}
