use crate::command_prelude::*;

use cargo::ops;

pub fn cli() -> Command {
    subcommand("init")
        .about("Create a new cargo package in an existing directory")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .action(ArgAction::Set)
                .default_value("."),
        )
        .arg(
            Arg::new("workspace")
                .long("workspace")
                .short('w')
                .help("Initialize a new workspace")
                .action(ArgAction::SetTrue),
        )
        .arg_new_opts()
        .arg_registry("Registry to use")
        .arg_silent_suggestion()
        .after_help(color_print::cstr!(
            "Run `<cyan,bold>cargo help init</>` for more detailed information.\n"
        ))
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    if args.get_flag("workspace") {
        let crates_list = ops::init_workspace(args.value_of_path("path", config).unwrap(), config)?;
        config
            .shell()
            .status("Created", format!("workspace with [{}]", crates_list))?;
        return Ok(());
    }
    let opts = args.new_options(config)?;
    let project_kind = ops::init(&opts, config)?;
    config
        .shell()
        .status("Created", format!("{} package", project_kind))?;
    Ok(())
}
