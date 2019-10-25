use std::time::Duration;

pub struct Args {
    pub channel: String,
    pub timeout: Duration,
}

impl Args {
    fn usage(name: &str) -> ! {
        eprintln!("{} {}", name, env!("CARGO_PKG_VERSION"));
        eprintln!("USAGE:");
        eprintln!("\t{} [options]", name);
        eprintln!("OPTIONS:");
        eprintln!("\t-c <channel>  <-- twitch channel to get the viewers for");
        eprintln!("\t-t (timespec) <-- how many seconds to wait between each update");
        eprintln!("\t\t- format is \\d+h?\\s?\\d+m?\\s?\\d+s. such as:");
        eprintln!("\t\t-- \"1h 1m 1s\" or 1h1m1s without spaces");
        std::process::exit(1);
    }

    pub fn parse() -> Self {
        let name = std::env::args().next().unwrap();
        let mut args = pico_args::Arguments::from_env();
        if args.contains("-h") {
            Self::usage(&name)
        }

        let this = Self {
            channel: args
                .value_from_str("-c")
                .unwrap_or_else(|_| Self::usage(&name)),
            timeout: args
                .opt_value_from_fn("-t", |s| crate::parse::timeout(s).map(Duration::from_secs))
                .unwrap_or_else(|_| Self::usage(&name))
                .unwrap_or_else(|| Duration::from_secs(30)),
        };
        args.finish().expect("valid args");
        this
    }
}
