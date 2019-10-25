use viewers::*;

fn main() {
    let args = args::Args::parse();

    flexi_logger::Logger::with_env_or_str("viewers=info")
        .start()
        .unwrap();

    let api_key = util::get_api_key(util::CLIENT_ID_ENV_VAR);
    ui::App::new(api_key, args.channel, args.timeout).run();
}
