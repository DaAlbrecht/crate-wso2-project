use clap::Command;

fn cli() -> Command {
    Command::new("wso2")
        .about("WSO2 project scaffolding tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("new").about("Create a new WSO2 project"))
}

fn main() {
    let matches = cli().get_matches();

    if matches.subcommand_matches("new").is_some() {
        create_wso2_project::run().unwrap();
    }
}
