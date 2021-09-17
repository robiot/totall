use std::path::Path;
use std::fs::metadata;

fn print_error<T: std::string::ToString>(err: T) {
    eprintln!("totall: {}", err.to_string());
}

fn get_all_files(dir_name: &str) -> Result<Vec<String>, std::io::Error>
{
    let mut all_files: Vec<String> = vec!();
    let paths = std::fs::read_dir(dir_name)?;

    for path in paths {
        let md = metadata(path.as_ref().unwrap().path())?;
        if md.is_dir() {
            all_files.append(&mut get_all_files(path.unwrap().path().to_str().unwrap())?);
        } else if md.is_file() {
            all_files.append(&mut vec!(path.unwrap().path().to_str().unwrap().to_string()));
        }
    }
    return Ok(all_files)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_error("No path given.");
        return;
    }

    let path: &str = &args[1];
    if Path::new(path).exists() {
        let md = metadata(path).unwrap();
        if md.is_dir() {
            let files: Vec<String> = match get_all_files(path) {
                Ok(m) => m,
                Err(err) => {
                    print_error(err);
                    return;
                }
            };

            let mut count: i32 = 0;
            for file in files {
                count += 1;
                println!("{}", file)
            }
            println!("{}", count);

        } else if md.is_file() {

        }
    } else {
        print_error("Given path does not exist.")
    }
}