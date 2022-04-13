// cli args
// use std::env;
// exit if cli args incorrect
// use std::process;
// read large file
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// xml parse
use roxmltree;

fn main() {
    //
    // get CLI args
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Error: {}", err);
    //     process::exit(1);
    // });
    // let file_in: String = config.input_file;
    // let file_out: String = config.output_file;

    let file_in: String = "applewatch_data_export.xml".to_string();
    let file_out: String = "out.txt".to_string();
    sort_data(file_in, file_out);
}

// struct Config {
//     input_file: String,
//     output_file: String,
// }

// impl Config {
//     fn new(args: &Vec<String>) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let i = &args[1];
//         let o = &args[2];
//         Ok(Config {
//             input_file: i.to_string(),
//             output_file: o.to_string(),
//         })
//     }
// }

// fn timestamp_reformat(String) -> String{

// }

struct Timestamp {
    date: String,
    seconds: String,
}

impl Timestamp {
    fn new(raw_timestamp: String) -> Option<Timestamp> {
        let b: Vec<&str> = raw_timestamp.split_whitespace().collect();
        let hms: Vec<u32> = b[1].split(":").filter_map(|a| a.parse().ok()).collect();
        let secs: u32 = hms[0] * 3600 + hms[1] * 60 + hms[2];

        // raw_timestamp's format: "2020-10-01 12:26:51 -0700"
        // converted to
        // Timestamp.date:String = "2020-10-01"
        // Timestamp.seconds:String = "44811"
        //
        Some(Timestamp {
            date: b[0].to_string(),
            seconds: secs.to_string(),
        })
    }
}

fn sort_data(input_file: String, output_file: String) -> std::io::Result<()> {
    let file = File::open(&input_file)?;
    let reader = BufReader::new(file);
    //
    // let mut nlines = 0;
    let mut pulse_measures: Vec<String> = Vec::new();
    //
    'loop_lines: for line in reader.lines() {
        let l = line.as_ref().unwrap();
        let tree = roxmltree::Document::parse(&l); //
        if tree.is_ok() {
            for xml_element in tree.unwrap().descendants() {
                //
                let typ = &xml_element.attribute("type");
                if typ.is_some() {
                    if typ.unwrap().contains("HeartRate") == false {
                        continue;
                    }
                }
                let val = &xml_element.attribute("value");
                let date = &xml_element.attribute("startDate");

                if date.is_some() && val.is_some() {
                    let val_ = val.unwrap().parse::<f32>();
                    if val_.is_ok() {
                        if val_.as_ref().unwrap() > &30.0 {
                            let date = date.unwrap().to_string();
                            let timestamp = Timestamp::new(date);

                            if timestamp.is_some() {
                                let t = timestamp.unwrap();
                                let pulse_measure: String = format!(
                                    "{} {} {}",
                                    &t.date.to_string(),
                                    &t.seconds.to_string(),
                                    val_.unwrap().to_string(),
                                );
                                pulse_measures.push(pulse_measure);
                            }
                        }
                    }
                }
            }
            // nlines += 1;
            // if nlines % 1000 == 0 {
            //     println!("{}", nlines);
            // }
            // if nlines > 1000 {
            //     break 'loop_lines;
            // }
        }
    }

    pulse_measures.sort();

    let mut output = File::create(&output_file)?;
    //
    let mut lastdate: &str = "";

    for i in &pulse_measures {
        let sp: Vec<&str> = i.split_whitespace().collect();
        let date: &str = sp[0];
        //
        if date != lastdate {
            lastdate = date;
            let mut s: String = "# ".to_string();
            s.push_str(lastdate);
            s.push_str("\n");
            write!(output, "{}", s).ok();
        }

        let mut st: String = "".to_string();

        st.push_str(&sp[1].to_string());
        st.push_str(" ");
        st.push_str(&sp[2]);
        st.push_str("\n");
        write!(output, "{}", st).ok();
    }

    Ok(())
}
