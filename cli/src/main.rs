use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use strip_shared::compiler::compile;
use strip_shared::parser::parse;

mod debug;
use debug::Trace;

fn main() -> io::Result<()> {
  let mut app = App::new("strip")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Vitaly Domnikov <oss@vitaly.codes>")
    .about("StripVM development tools.")
    .subcommand(
      App::new("compile")
        .about("Compiles program")
        .arg(
          Arg::with_name("INPUT")
            .help("Sets the assembly file")
            .value_name("INPUT")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("OUTPUT")
            .help("Sets the output file")
            .value_name("OUTPUT")
            .required(true)
            .index(2),
        ),
    )
    .subcommand(
      App::new("trace")
        .about("Starts program tracing")
        .arg(
          Arg::with_name("INPUT")
            .help("Sets the assembly file")
            .value_name("INPUT")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("MEMORY")
            .short("m")
            .help("Trace memory access"),
        )
        .arg(
          Arg::with_name("RAM")
            .short("ram")
            .default_value("8")
            .help("Sets RAM size"),
        )
        .arg(
          Arg::with_name("SPINS")
            .short("spins")
            .default_value("1")
            .help("Sets VM spins"),
        )
        .arg(
          Arg::with_name("MAX_OPS")
            .short("ops")
            .takes_value(true)
            .help("Sets VM ops quota"),
        ),
    );

  match app.clone().get_matches().subcommand() {
    ("compile", Some(args)) => {
      let input = args.value_of("INPUT").unwrap();
      let mut file = File::open(input).unwrap();
      let mut code = String::new();
      file.read_to_string(&mut code)?;

      let exprs = parse(&code).unwrap();
      let bytecode = compile(&exprs).unwrap();

      let out_path = args.value_of("OUTPUT").unwrap();
      let mut file = File::create(out_path).unwrap();
      file.write_all(&bytecode).unwrap();
    }
    ("trace", Some(args)) => {
      let input = args.value_of("INPUT").unwrap();
      let mut file = File::open(input).unwrap();
      let mut code = String::new();
      file.read_to_string(&mut code)?;

      let exprs = parse(&code).unwrap();
      let bytecode = compile(&exprs).unwrap();
      let spins = u16::from_str_radix(args.value_of("SPINS").unwrap(), 10).unwrap();
      let ram = u16::from_str_radix(args.value_of("RAM").unwrap(), 10).unwrap();
      let trace_mem = args.is_present("MEMORY");
      let max_ops = args
        .value_of("MAX_OPS")
        .map(|s| u32::from_str_radix(s, 10).unwrap());
      let mut trace = Trace::new(spins, max_ops, ram, trace_mem, &bytecode).unwrap();
      trace.start().unwrap();
    }
    _ => {
      app.print_long_help().unwrap();
    }
  }

  Ok(())
}
