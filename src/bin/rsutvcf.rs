use rust_htslib::bcf::{Reader, Read};

// iterate through each row of the vcf body.

fn main () {
	let path: &str = "resources/examples/nf.vep.vcf.gz";
	
	let mut bcf = Reader::from_path(path).expect("Error opening file.");
	for (_, record_result) in bcf.records().enumerate() {
		let record = record_result.expect("Fail to read record");
		let mut s = String::new();

		for allele in record.alleles() {
			for c in allele {
				s.push(char::from(*c))
			}
			s.push(' ')
		}

		// println!("Locus: {}, Alleles: {}", record.pos(), s);
		
		// start
		let start: i64 = record.pos();
		// end
		let end: i64 = record.end();		
		// chromosome
		let chr = record.data();

		println!("{}\t{}\t{}", chr, start, end);
		
	}
}