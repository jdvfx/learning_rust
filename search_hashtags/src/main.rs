use argparse::ArgumentParser;
use argparse::Store;
use std::fs;
use walkdir::WalkDir;

fn main() {
    // set default args
    let mut path = ".".to_string();
    let mut query = "query".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Recursive string locater in files");
        ap.refer(&mut path)
            .add_option(&["-p", "--path"], Store, "Path to folder");
        ap.refer(&mut query)
            .add_option(&["-q", "--query"], Store, "Query string to find")
            .required();
        ap.parse_args_or_exit();
    }
    search_hashtags(&path, &query);
}
/// recursively search through all .rs files
/// and find hashtags eg:"#myhashtag"
/// then print out the code in context with 10 lines before and 10 lines after
///
fn search_hashtags(search_path: &str, query: &str) {
    //
    for (_index, path) in WalkDir::new(search_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .enumerate()
    {
        if path.metadata().unwrap().is_file() {
            let file = path.file_name().to_str().unwrap_or("");
            if file.ends_with(".rs") {
                let contents = match fs::read_to_string(path.path()) {
                    Ok(t) => t,
                    Err(_e) => "".to_string(),
                };

                if contents.chars().count() > 0 {
                    // keep 20 lines in an array (buffer)
                    let mut buff: Vec<String> = Vec::new();
                    // when hashtag found, add=1, chunk+=add,
                    let mut add: u32 = 0;
                    let mut chunk: u32 = 0;
                    let lines: Vec<&str> = contents.split("\n").collect();

                    for line in lines {
                        if line.chars().count() > 1 {
                            //
                            // add to buffer, but keep only 20 lines
                            buff.push(line.to_string());
                            buff.reverse();
                            buff.truncate(20);
                            buff.reverse();
                            //
                            if line.contains("#") && line.contains(&query) {
                                add = 1;
                            }
                            chunk += add;
                            // reached 10 lines after #, print buffer
                            if chunk == 10 {
                                println!("..................");
                                println!("{:?}", path.path());
                                for i in &buff {
                                    println!("{}", &i);
                                }
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
