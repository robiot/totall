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
    Ok(all_files)
}

fn read_lines(file_name: String) -> Result<usize, std::io::Error> {
    Ok(linecount::count_lines(std::fs::File::open(file_name)?)? + 1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_error("No path given.");
        return;
    }

    let mut path: &str = &args[1];
    if Path::new(path).exists() {
        let md = metadata(path).unwrap();
        if md.is_dir() {
            let current_dir = std::env::current_dir().unwrap();
            if path == "." {
                path = current_dir.to_str().unwrap();
            }

            let files: Vec<String> = match get_all_files(path) {
                Ok(m) => m,
                Err(err) => {
                    print_error(err);
                    return;
                }
            };

            let mut count: usize = 0;
            for file in files {
                count += read_lines(file).unwrap();
            }
            println!("{}", count);

        } else if md.is_file() {
            println!("{}", read_lines(path.to_string()).unwrap());
        }
    } else {
        print_error("Given path does not exist.")
    }
}