mod path_sort;

use clap::{App, AppSettings, Arg, SubCommand};

pub type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
  let res = App::new("path_sort")
    .version("1.0.0")
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
      let path_sort = crate::path_sort::PathSort::new();
      let sort_file = path_sort.get_sort_file(s.value_of("sort_file"))?;
      let handler = crate::path_sort::OsPathHandler::new(&sort_file)?;
      let mut full_path = handler.create_full_path().ok_or_else(|| anyhow::anyhow!("could not create full path"))?;
      full_path.sort(&sort_file);
      println!("{}", full_path.to_string());
    },
    ("view", Some(_)) => {
      let view = crate::path_sort::view::View::new();
      view.print_view();
    },
    _ => unreachable!(),
  }

  Ok(())
}
