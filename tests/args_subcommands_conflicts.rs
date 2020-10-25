use clap::Clap;

#[derive(Clap, Debug)]
struct Opts {
    #[clap(global = true, long)]
    global_flag: bool,

    #[clap(global = true, long)]
    global_str: Option<String>,

    #[clap(subcommand)]
    first_subcommand: FirstSubCommandEnum,
}

#[derive(Clap, Debug)]
enum FirstSubCommandEnum {
    First(FirstSubCommand),
}

#[derive(Clap, Debug)]
struct FirstSubCommand {
    #[clap(global = true, long)]
    first_flag: bool,

    #[clap(global = true, long)]
    first_str: Option<String>,

    #[clap(subcommand)]
    second_subcommand: SecondSubCommandEnum,
}

#[derive(Clap, Debug)]
enum SecondSubCommandEnum {
    Second(SecondSubCommand),
}

#[derive(Clap, Debug)]
struct SecondSubCommand {}

#[test]
fn args_subcommands_conflicts() {
    let parsed: Opts = Opts::parse_from(&[
        "_demo_",
        "first",
        "second",
        "--global-flag",
        "--global-str=hello",
        "--first-flag",
        "--first-str=world",
    ]);

    println!("{:#?}", parsed);

    assert!(parsed.global_flag, "--global-flag hasn't been detected");

    assert!(
        matches!(parsed.global_str, Some(_)),
        "--global-str argument hasn't been detected"
    );

    assert_eq!(
        parsed.global_str.unwrap(),
        "hello",
        "Wrong value detected for --global-str"
    );

    let first_subcmd = match parsed.first_subcommand {
        FirstSubCommandEnum::First(cmd) => cmd,
    };

    assert!(first_subcmd.first_flag, "--first-flag hasn't been detected");

    assert!(
        matches!(first_subcmd.first_str, Some(_)),
        "--first-str argument hasn't been detected"
    );

    assert_eq!(
        first_subcmd.first_str.unwrap(),
        "hello",
        "Wrong value detected for --first-str"
    );

    let _second_subcmd = match first_subcmd.second_subcommand {
        SecondSubCommandEnum::Second(cmd) => cmd,
    };
}
