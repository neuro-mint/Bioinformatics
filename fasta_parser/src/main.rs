use std::fs::read;
use std::path::PathBuf;

fn main() {
    read_from_fasta("paste path here".into());
}

fn read_from_fasta(path: PathBuf) {
    let file_data: Vec<String> = String::from_utf8(read(path).expect("Failed to read data from the FASTA file"))
        .expect("Failed to convert utf-8 to String").split("\n")
        .map(|s: &str| s.to_string()).collect();
    
    parse_fasta(file_data);
}

fn parse_fasta(file_data: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut headers = Vec::new();
    let mut sequences = Vec::new();
    let mut seq = String::new();

    //splits the contents of the FASTA file into sequences and headers
    for i in 0..file_data.len() {
        if i == 0 {
            if file_data[i].starts_with(">") {
                headers.push(file_data[i].to_owned());
            } else {
                continue;
            }
        } else {
            if file_data[i].starts_with(">") {
                headers.push(file_data[i].to_owned());
                sequences.push(seq.to_owned());
                seq = String::new();
            } else {
            seq.push_str(&file_data[i]);
            }
        }
    }

    return (headers, sequences);
}
