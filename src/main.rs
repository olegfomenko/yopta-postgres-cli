#![feature(in_band_lifetimes)]

mod command;

use clap::Parser;
use std::iter;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::config::OutputStreamType;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::Validator;
use rustyline::validate::{ValidationContext, ValidationResult};
use rustyline::{CompletionType, Config, Context, EditMode};
use postgres::{Client, NoTls};
use postgres_types::{Type};

use std::fmt::Debug;

#[derive(Parser)]
struct Cli {
    host: String,
    user: String,
    password: String,
    database: String,
}

fn main() {
    let args = Cli::parse();

    let mut client = Client::configure()
        .host(args.host.as_str())
        .user(args.user.as_str())
        .password(args.password.as_str())
        .dbname(args.database.as_str())
        .connect(postgres::NoTls).unwrap();

    let mut repl = Editor::<()>::with_config(Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build()
    );

    let commands = command::Commands::new();
    let p = format!("yopta-postgresql-cli> ");
    let mut full_command = String::new();

    loop {
        let readline = repl.readline(&p);

        match readline {
            Ok(command) => {
                if command::Commands::is_exit(&command) {
                    break;
                }

                full_command.push_str(&command);

                if full_command.ends_with(";") {
                    let result = client.query(
                        commands.compile_sql(full_command).as_str(),
                        &[],
                    );

                    match result {
                        Ok(result) => {
                            let mut column_printed = false;

                            for row in result {
                                if !column_printed {
                                    column_printed = true;
                                    for col in row.columns() {
                                        print!("{:<10} |", col.name());
                                    }
                                    println!();
                                }

                                for i in 0..row.len() {
                                    match row.columns()[i].type_() {
                                        &Type::BOOL => {
                                            let field: bool = row.get(i);
                                            print!("{:<10} |", field);
                                        }
                                        &Type::TEXT | &Type::VARCHAR | &Type::CHAR_ARRAY => {
                                            let field: &str = row.get(i);
                                            print!("{:<10} |", field);
                                        }
                                        &Type::INT4 => {
                                            let field: i32 = row.get(i);
                                            print!("{:<10} |", field);
                                        }
                                        &Type::INT2 => {
                                            let field: i16 = row.get(i);
                                            print!("{:<10} |", field);
                                        }
                                        &Type::INT8 => {
                                            let field: i64 = row.get(i);
                                            print!("{:<10} |", field);
                                        }

                                        tp => print!("*Unknown field type: {}* ", tp)
                                    };
                                }
                                println!();
                            }
                        }
                        Err(err) => {
                            eprintln!("Ты че охуел?!!!!: {:?}", err);
                        }
                    }

                    full_command = String::new();
                }
            }

            Err(ReadlineError::Interrupted) => {
                eprintln!("Ты че охуел?!!!!");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("Ты че охуел?!!!!");
                break;
            }
            Err(err) => {
                eprintln!("Ты че охуел?!!!!: {:?}", err);
                break;
            }
        }
    }
}