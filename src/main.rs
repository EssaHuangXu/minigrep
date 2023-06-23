use std::env;
use std::process;

fn main() {
    let collect : Vec<String> = env::args().collect();
    //get it wit tuple
    //let (opt, filename) = get_opt_and_filename(&collect);

    //get it with ref
    let config = minigrep::OptConfig::new(&collect).unwrap_or_else(|err| {
        eprintln!("User Input not valid, {:?}", err);
        process::exit(1);
    });

    if let Err(error) = minigrep::process_file(&config){
        eprintln!("Application error : {}", error);
        process::exit(1);
    }
}


// fn get_opt_and_filename(collect : &Vec<String>) -> (&String, &String){
//     return (&collect[1], &collect[2]);
// }