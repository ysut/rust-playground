// use vcf::{VCFReader, U8Vec, VCFHeaderFilterAlt, VCFError, VCFRecord};
use vcf::{VCFReader, VCFError, VCFRecord};
use flate2::read::MultiGzDecoder;
use std::fs::File;
use std::io::BufReader;


fn main() -> Result<(), VCFError> {
    let input_vcf: &str = "resources/examples/nf.vep.vcf.gz";
    // let file = File::open(input_vcf,)?;

    let mut reader = VCFReader::new(BufReader::new(MultiGzDecoder::new(File::open(
        input_vcf,
    )?)))?;

    let mut vcf_record: VCFRecord = reader.empty_record();
    // println!("{:?}", vcf_record);

    let header: &vcf::VCFHeader = vcf_record.header();
    
    let info_list: std::collections::hash_map::Keys<'_, Vec<u8>, usize> = header.info_list();

    // Print the INFO list
    for info in info_list {
        println!("{:?}", info);
    }


    // let record: bool = reader.next_record(&mut vcf_record)?;
    
    Ok(())
}