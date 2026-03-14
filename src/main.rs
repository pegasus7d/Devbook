use devbook::app::App;
use devbook::cli::{parse, Intent};

fn main() {
    let intent = parse();
    let result = match intent {
        Intent::List => App::list_actions(),
        Intent::Init => App::init_config(),
        Intent::Run(name) => App::run_action(&name),
    };
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
