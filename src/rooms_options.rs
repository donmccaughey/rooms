extern crate getopts;

use self::getopts::Options;
use std::env;
use std::io;
use std::io::Write;
use std::process;


pub struct RoomsOptions {
    options: Options,
    pub command: String,
    pub roomsfile: Option<String>,
}


impl RoomsOptions {
    fn print_usage_and_exit(&self, status: i32) -> ! {
        let brief = format!("Usage: {} [roomsfile]", self.command);
        print!("{}", self.options.usage(&brief));
        process::exit(status);
    }

    pub fn new() -> RoomsOptions {
        let args: Vec<_> = env::args().collect();
        let mut rooms_options = RoomsOptions {
            options: Options::new(),
            command: args[0].clone(),
            roomsfile: None,
        };
        rooms_options.options.optflag("h", "help", "print this help message");

        let matches;
        match rooms_options.options.parse(&args[1..]) {
            Ok(found_matches) => matches = found_matches,
            Err(error) => {
                writeln!(&mut io::stderr(), "{}", error).unwrap();
                rooms_options.print_usage_and_exit(2);
            },
        };

        if matches.opt_present("h") {
            rooms_options.print_usage_and_exit(0);
        }

        rooms_options.roomsfile = matches.free.first().map(|s| s.clone());
        rooms_options
    }
}

