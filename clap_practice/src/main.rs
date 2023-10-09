// SPDX-License-Identifier: Apache-2.0

const SUBCOMMAND_A: &'static str = "subcommand_a";
const SUBCOMMAND_B: &'static str = "subcommand_b";
const SUBCOMMAND_C: &'static str = "subcommand_c";

fn main() {
    let matches = clap::Command::new("clap_practice")
        .about("Practicing clap")
        .subcommand_required(true)
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .global(true)
                .action(clap::ArgAction::Count)
                .help("Set verbose level"),
        )
        .subcommand(clap::Command::new(SUBCOMMAND_A).about(SUBCOMMAND_A))
        .subcommand(
            clap::Command::new(SUBCOMMAND_B).about(SUBCOMMAND_B).arg(
                clap::Arg::new("arg1")
                    .long("arg1")
                    .help("arg1")
                    .action(clap::ArgAction::Set),
            ),
        )
        .subcommand(
            clap::Command::new(SUBCOMMAND_C).about(SUBCOMMAND_C).arg(
                clap::Arg::new("arg2")
                    .help("arg2")
                    .long("arg2")
                    .action(clap::ArgAction::Set),
            ),
        )
        .get_matches();
    let verbose_level = matches.get_count("verbose");
    println!("Verbose level {}", verbose_level);
    if matches.subcommand_matches(SUBCOMMAND_A).is_some() {
        println!("Got {}", SUBCOMMAND_A);
    } else if let Some(m) = matches.subcommand_matches(SUBCOMMAND_B) {
        println!("Got {}", SUBCOMMAND_B);
        if let Some(val) = m.get_one::<String>("arg1") {
            println!("Got arg1: {}", val);
        }
    } else if let Some(m) = matches.subcommand_matches(SUBCOMMAND_C) {
        println!("Got {}", SUBCOMMAND_C);
        if let Some(val) = m.get_one::<String>("arg2") {
            println!("Got arg2: {}", val);
        }
    }
}
