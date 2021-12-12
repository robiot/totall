use std::path::Path;
use std::fs::metadata;
use path_clean::PathClean;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn print_error<T: std::string::ToString>(err: T) {
    eprintln!("totall: {}", err.to_string());
}

fn print_help(args: Vec<String>) {
    println!(
        "totall v{} \
        \nUsage: {} <file/folder>", VERSION, args[0]
    )
}

fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<std::path::PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

fn get_all_files(dir_name: &str) -> std::io::Result<Vec<String>>
{
    let mut all_files: Vec<String> = vec!();
    let paths;
    if let Some(var) = std::fs::read_dir(dir_name).ok() {
        paths = var;
    } else {
        return Ok(all_files);
    }

    for path in paths {
        let full_path = absolute_path(path.unwrap().path()).unwrap();
        let full_path_str: &str = full_path.to_str().unwrap();
        let md;
        if let Some(var) = metadata(full_path.clone()).ok() {
            md = var;
        } else {
            continue;
        }

        if md.is_dir() {
            all_files.append(&mut get_all_files(full_path_str)?);
        } else if md.is_file() {
            all_files.append(&mut vec!(full_path_str.to_string()));
        }
    }
    Ok(all_files)
}

fn read_lines(file_name: String) -> std::io::Result<usize> {
    Ok(linecount::count_lines(std::fs::File::open(file_name)?)? + 1)
}

fn path_count(path: &str) -> std::io::Result<usize> {
    let md = metadata(path).unwrap();
    if md.is_dir() {
        let files: Vec<String> = match get_all_files(path) {
            Ok(f) => f,
            Err(err) => return Err(err)
        };

        let mut count: usize = 0;
        for file in files {
            count += match read_lines(file) {
                Ok(m) => m,
                Err(err) => return Err(err)
            };
        }
        return Ok(count);
    } else if md.is_file() {
        return read_lines(path.to_string())
    }
    Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help(args);
        return;
    }

    let mut count: usize = 0;
    for i in 1..args.len() {
        let path: &str = &args[i];
        if !Path::new(path).exists() {
            print_error(format!("path does not exist: {}", path));
            continue;
        }

        count += match path_count(path) {
            Ok(s) => s,
            Err(err) => {
                print_error(err);
                continue;
            }
        }
    }

    println!("Total lines: {}", count);
}