#![allow(dead_code)]
use std::io;
use rand::Rng;

fn main() {
    point_mutations("ATTGCGCTAGhACTGC", "TTGGCGCCAGTCATC");
}

//Try writing these functions with fewer variables
fn genseq() -> String{
    const DNA_NUCLEOTIDES: &[u8] = b"ATGC";
    const RNA_NUCLEOTIDES: &[u8] = b"AUGC";
    let mut rng = rand::thread_rng();

    let mut nuc_type:String = String::new();
    println!("DNA or RNA");
    io::stdin()
        .read_line(&mut nuc_type)
        .expect("Failed to read line");

    let mut len:String  = String::new();
    println!("Enter length of DNA sequence");
    io::stdin() 
        .read_line(&mut len)
        .expect("Failed to readd line");    
    let len: u32 = len.trim().parse().expect("Please enter a whole number");

    let n: &str = nuc_type.trim();
    match n {
        "DNA" => {
            let random_dna_seq: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..DNA_NUCLEOTIDES.len());
                DNA_NUCLEOTIDES[idx] as char
            }).collect();
            return random_dna_seq;
        } 
        "RNA" => {
            let random_rna_seq: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..RNA_NUCLEOTIDES.len());
                RNA_NUCLEOTIDES[idx] as char
            }).collect();
        return random_rna_seq;
        } 
        _ => "Please enter a valid nucleic acid".to_string()
    }
}
fn nucleotide_count_and_gc_content(sequence:&str) -> f32{
    println!("Nucleotide counts:");
    println!("A : {}", sequence.matches("A").count());
    println!("T : {}", sequence.matches("T").count());
    println!("G : {}", sequence.matches("G").count());
    println!("C : {}", sequence.matches("C").count());
    println!("U : {}", sequence.matches("U").count());

    let sequence_len:u32 = sequence.chars().count().try_into().unwrap();
    let total_gc: u32 = (sequence.matches("G").count() + sequence.matches("C").count()).try_into().unwrap();
    let gc_content:f32 = total_gc as f32/sequence_len as f32;
    println!("GC content : {}", gc_content);
    return gc_content;
}
fn transcribe_dna_sequence(sequence:&str) -> String{
    let transcribed_sequence:String = sequence.to_string().replace("T", "U");
    println!("Transcribed sequence : 5` {}", transcribed_sequence);
    return transcribed_sequence;
}
fn sequence_complement(sequence:&str) -> String {
    let mut complement = String::new();
    for found in  sequence.chars(){
        if found == 'A' {
            complement.push('T');
        } else if found == 'T' {
            complement.push('A');
        } else if found == 'G' {
            complement.push('C');
        } else if found == 'C' {
            complement.push('G');
        }
    }
    return complement
} 
fn reverse_complement(complement:String) -> String {
    let reverse_complement:String = complement.chars().rev().collect();
    return reverse_complement;
}
fn is_reverse_palindrome(sequence:&str) -> bool {
    // if seq_slice == reverse complement ->  it is a restriction site
    let rev = reverse_complement(sequence_complement(&sequence));
    if sequence == rev  {
        return true;
    } else {
        return false;
    }
}
fn point_mutations(reference:&str, to_compare:&str) -> Vec<i32>{
    let mut indices:Vec<i32> = Vec::new();
    let ref_seq:Vec<char> = reference.chars().collect();
    let comp_seq:Vec<char> = to_compare.chars().collect();

    if ref_seq.len() == comp_seq.len() {
        for i in 0..reference.len() {
            if ref_seq[i] != comp_seq[i] {
                println!(" {} -> {} | mutation index : {i}", ref_seq[i], comp_seq[i]);
                indices.push(i.try_into().unwrap());
            } 
        }
    } else {
        println!("Please provide sequences of equal length");
    }
    return indices;
}

