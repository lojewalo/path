#![feature(proc_macro)]

extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate number_prefix;
extern crate separator;

#[macro_use]
mod path_sort;

use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;

fn inner() -> i32 {
  let res = App::new("path_sort")
    .version("0.2.0")
    .about("Manages the PATH variable")
    .setting(AppSettings::SubcommandRequired)
    .subcommand(SubCommand::with_name("sort")
      .about("Cleans and sorts the PATH according to a rule file")
      .arg(Arg::with_name("sort_file")
        .short("s")
        .long("sort-file")
        .help("the custom sort file to override default rules")
        .takes_value(true)
        .value_name("file")))
    .subcommand(SubCommand::with_name("view").about("Views each element in the PATH"))
    .after_help("path_sort will use a default set of rules if it is not given a custom sort file.")
    .get_matches();
  match res.subcommand() {
    ("sort", Some(s)) => {
      let path_sort = path_sort::PathSort::new();
      let sort_file = try_or_err_code!(path_sort.get_sort_file(s.value_of("sort_file")));
      let handler = path_sort::OsPathHandler::new(&sort_file).unwrap();
      let mut full_path = match handler.create_full_path() {
        Some(f) => f,
        None => return 1,
      };
      full_path.sort(&sort_file);
      println!("{}", full_path.to_string());
      0
    }
    ("view", Some(_)) => {
      let view = path_sort::view::View::new();
      view.print_view();
      0
    }
    _ => {
      unreachable!();
    }
  }
}

fn main() {
  let exit_code = inner();
  std::process::exit(exit_code);
}
