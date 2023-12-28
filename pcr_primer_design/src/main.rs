use clap::Parser;
use colored::Colorize;
#[derive(Parser, Debug)]
#[command(author = "Pranav Bhedsurkar", about, long_about = None)]
struct Cli {
    #[arg(short = 's', long = None)]
    sequence:String,
    #[arg(short = 'a', long = "all", default_value_t = false)]
    get_all:bool,
}

fn main() {
    //getting the sequence 
    let args:Cli = Cli::parse();
    let seq = &args.sequence;

    //Validates input sequence
    for n in seq.chars(){
        match n {
            'A' => (),
            'T' => (),
            'G' => (),
            'C' => (),
            _ => panic!("ERROR sequence contains chatacters other than 'A', 'T', 'G', 'C'")
        }
    }

    let all_primers:(Vec<String>, Vec<String>) = get_all_primers(seq, 14, 40);
    for p in all_primers.0 { primer_stats(&p, "Forward"); }
    for p in all_primers.1 { primer_stats(&p, "Reverse"); }
    println!("____________________________________________________");
}

fn get_all_primers(seq:&str, min_len:i8, max_len:i8) -> (Vec<String>, Vec<String>) {
    let mut complement: String = String::new();
    for found in  seq.chars() {
        if found == 'A' {
            complement.push('T');
        } else if found == 'T' {
            complement.push('A');
        } else if found == 'G' {
            complement.push('C');
        } else if found == 'C' {
            complement.push('G');
        }
    };
    let reverse_complement:String = complement.chars().rev().collect();

    let mut fwd_primers:Vec<String> = Vec::new();
    let mut rev_primers:Vec<String> = Vec::new();
    for len in  min_len..max_len{
        fwd_primers.push(seq[..len.try_into().unwrap()].to_string()); 
        rev_primers.push(reverse_complement[..len.try_into().unwrap()].to_string());
    }

    return (fwd_primers, rev_primers);
}
fn primer_stats(seq:&str, primer_type:&str) {
    println!("____________________________________________________");
    println!("Primer stats ->");
    println!("Primer           - 5`{seq}");
    println!("Length and type  - {} {}", seq.len(), primer_type);

    if melting_temperatuure(seq) < 50.0 || melting_temperatuure(seq) > 58.0{
        println!("Melting temp(\u{b0}C) - {}", melting_temperatuure(seq).to_string().red());
    } else {
        println!("Melting temp(\u{b0}C) - {}", melting_temperatuure(seq));
    }

    if gc_content(seq) < 50.0 {
        println!("GC content       - {} {}", gc_content(seq).to_string().red(), "%".red());
    } else {
        println!("GC content       - {}{}", gc_content(seq), "%");
    }

    if gc_clamp_check(&seq.to_string()) == true {
        println!("GC clamp present in the primer");
    } else {
        println!("{}", "GC clamp absent in this primer".red());
    }
}
fn melting_temperatuure(seq:&str) -> f32 {
    // Assumptions :
    // Tm= 64.9 +41*(nG+nC-16.4)/(seq_len)
    // annealing occurs under the standard conditions 
    // 50 nM primer 
    // 50 mM Na+ 
    // pH 7.0
    let melting_temp:f32 = 64.9 + 
        (41 as f32) * (seq.matches("G").count() as f32 + seq.matches("C").count() as f32 -16.4) / 
        seq.len() as f32;
    //println!(" Basic Melting temp(\u{b0}C) : {melting_temp} \u{b0}C");
    return melting_temp
}
fn gc_content(seq:&str) -> f32 {
    let gc_content:f32 = (seq.matches("G").count() + seq.matches("C").count()) as f32 / 
    seq.len() as f32 * 100 as f32;
    return gc_content
}
fn gc_clamp_check(primer:&String) -> bool {
    //checks if the primer has at least one G or C in the last 5 bases of the primer
    let s:String = primer.chars().rev().collect();
    if s[..5].contains("G") || s[..5].contains("C") == true {
        return true;
    } else{
        return false;
    }
}
