extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

fn is_timecode(line: &str) -> bool {
    line.contains("-->")
}

fn delete_cue_settings(line: &str) -> String {
    let mut output = String::new();
    let comma_line = line.replace(".", ",");
    
    for ch in comma_line.chars() {
        let ch_lower = ch.to_ascii_lowercase();
        if ch_lower >= 'a' && ch_lower <= 'z' {
            break;
        }
        output.push(ch_lower)
    }

    output.trim().to_string()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing vtt file");
    let input = File::open(&path).expect("Can't open file");
    let reader = BufReader::new(input);

    let re = Regex::new(r"<[^>]*>").unwrap();

    let mut block : Vec<String>= [].to_vec();
    let mut count: i32 = 1;
    let mut srt_file: String = String::new();

    for line_op in reader.lines() {
        let line = line_op.unwrap();
        
        if is_timecode(&line) {
            block = [].to_vec();
            block.push(count.to_string());
            block.push(delete_cue_settings(&line));
        }
        
        if line.contains('<') && block.len() > 0 {
            let l = re.replace_all(line.as_str(), "");
            block.push(l.to_string());
            srt_file = srt_file + &block.join("\n") + "\n\n";
            count = count + 1;
        }
    }

    let mut output = File::create(path.replace("vtt", "srt")).expect("Can't write file");
    write!(output, "{}", srt_file).expect("Can't write file");

    println!("Done.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_timecode_validation() {
        let r = is_timecode("00:00:00.079 --> 00:00:03.370 align:start position:0%");
        assert!(r);
    }

    #[test]
    fn clean_cue_settings() {
        let r = delete_cue_settings("00:00:00.079 --> 00:00:03.370 align:start position:0%");
        println!("{}", r);
        assert_eq!(r, "00:00:00,079 --> 00:00:03,370");
    }

}