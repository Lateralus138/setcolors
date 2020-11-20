use std::{
    env::args,
    process::exit,
    process::Command
};
use colored::Colorize;
fn main() {
    let args_string_vec: Vec<String> = args().collect();
    let args_string_vec = args_string_vec[1..].to_vec();
    let mut args_string = String::new();
    args_string.push_str(&args_string_vec.join(";"));
    let args_string = args_string;
    let help_closure = |err_num| {
        let opts_help_txt = String::from("OPTIONS").green().bold();
        let usage_help_txt = String::from("USAGE").cyan().bold(); 
        let colors_help_txt = String::from("COLORS").blue().bold();
        let err_help_txt = String::from("ERRORS").red().bold();
        print!("\
            \n Set console colors.\
            \n E.g. setcolors 1 31 107 \
            \n      setcolors 4;echo \"file:///{}/Documents\"\
            \n\n @{}:\
            \n\tsetcolors [{}...]\
            \n\tsetcolors [{}...]\
            \n\n @{}:\n\t-h,--help\tThis HELP message.\
            \n\n @{}:\n\tIntegers\tAny amount of numbers\
            \n\t\t\tfrom the list below.\
            \n\t0-5\t\tVarious styles.\
            \n\t30-37\t\tDarker foreground colors.\
            \n\t90-97\t\tLighter foreground colors.\
            \n\t40-47\t\tDarker background colors.\
            \n\t100-107\t\tLighter background colors.\
            \n\n @{}: Integers - Exit Codes.\
            \n\t0\t\tNo errors.\
            \n\t1\t\tPassed arguments are not integers\
            \n\t\t\tbetween 0-255.\
            \n\t2\t\tError in 'printf'.\n\n",
                "${HOME}",usage_help_txt,
                opts_help_txt,colors_help_txt,
                opts_help_txt,colors_help_txt,
                err_help_txt
            );
        exit(err_num);
    };
    for arg_item in &args_string_vec {
        let arg_case_conv = &arg_item.to_lowercase();
        if arg_case_conv == "--help" || arg_case_conv == "-h" {
            help_closure(0);
        }
        match arg_item.parse::<u8>() {
            Ok(_)=> (),
            Err(error)=> {
                println!(" {}",
                    error
                    .to_string()
                    .red()
                    .bold());
                exit(1);
            }
        };
    }
    let mut full_arg_str = String::new();
    full_arg_str.push_str("\\033[");
    full_arg_str.push_str(&args_string);
    full_arg_str.push_str("m");
    let full_arg_str = full_arg_str;
    let mut color_command = Command::new("printf");
    match color_command
        .args(&[full_arg_str])
        .status() {
            Ok(_)=>(),
            Err(command_error)=> {
                println!(" {}",
                    &command_error
                    .to_string()
                    .red()
                    .bold()
                );
                help_closure(2);
            }
    }
}