mod data;

use std::{env, fs};

use data::{
    AnimalSample, Pathogen, RNACellLineCancer, RNAImmuneCell, RNAImmuneCellMonaco,
    RNAImmuneCellSample, RNAImmuneCellSchmiedel, RNAMouseBrainAllen, RNAMouseBrainHPA,
};
use serde_derive::{Deserialize, Serialize};

use crate::data::Gene;

fn get_records() -> Vec<Gene> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/normal_tissue.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Gene = result.expect("get");
        records.push(record);
    }
    records
}
fn get_pathogens() -> Vec<Pathogen> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/pathology.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Pathogen = result.expect("get");
        records.push(record);
    }
    records
}

fn get_rna_immune_cell_monaco() -> Vec<RNAImmuneCellMonaco> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_immune_cell_monaco.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAImmuneCellMonaco = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_mouse_brain_hpa() -> Vec<RNAMouseBrainHPA> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_mouse_brain_hpa.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAMouseBrainHPA = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_mouse_brain_allen() -> Vec<RNAMouseBrainAllen> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_mouse_brain_allen.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAMouseBrainAllen = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_immune_cell_sample() -> Vec<RNAImmuneCellSample> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_immune_cell_sample.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAImmuneCellSample = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_immune_cell() -> Vec<RNAImmuneCell> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_immune_cell.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAImmuneCell = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_celline_cancer() -> Vec<RNACellLineCancer> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_celline_cancer.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNACellLineCancer = result.expect("get");
        records.push(record);
    }
    records
}

fn get_rna_immune_cell_schmiedel() -> Vec<RNAImmuneCellSchmiedel> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_immune_cell_schmiedel.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: RNAImmuneCellSchmiedel = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_mouse_brain_sample_hpa() -> Vec<AnimalSample> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_mouse_brain_sample_hpa.tsv/rna_mouse_brain_sample_hpa.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: AnimalSample = result.expect("get");
        records.push(record);
    }
    records
}

fn rna_pig_brain_sample_hpa() -> Vec<AnimalSample> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/rna_pig_brain_sample_hpa.tsv/rna_pig_brain_sample_hpa.tsv")
        .expect("read TSV");
    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: AnimalSample = result.expect("Get record");
        records.push(record);
    }
    records
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    tissues: Vec<Gene>,
    pathogens: Vec<Pathogen>,
    rna_immune_cell_schmiedel: Vec<RNAImmuneCellSchmiedel>,
    rna_celline_cancer: Vec<RNACellLineCancer>,
    rna_immune_cell: Vec<RNAImmuneCell>,
    rna_immune_cell_sample: Vec<RNAImmuneCellSample>,
    rna_immune_cell_monaco: Vec<RNAImmuneCellMonaco>,
    rna_mouse_brain_hpa: Vec<RNAMouseBrainHPA>,
    rna_mouse_brain_allen: Vec<RNAMouseBrainAllen>,

    pig_brain: Vec<AnimalSample>,
    mouse_brain: Vec<AnimalSample>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let app = crate::App {
            tissues: crate::get_records(),
            pathogens: crate::get_pathogens(),
            rna_celline_cancer: crate::rna_celline_cancer(),
            rna_immune_cell: crate::rna_immune_cell(),
            rna_immune_cell_sample: crate::rna_immune_cell_sample(),
            rna_immune_cell_schmiedel: crate::get_rna_immune_cell_schmiedel(),
            rna_immune_cell_monaco: crate::get_rna_immune_cell_monaco(),
            rna_mouse_brain_allen: crate::rna_mouse_brain_allen(),
            rna_mouse_brain_hpa: crate::rna_mouse_brain_hpa(),

            mouse_brain: crate::rna_pig_brain_sample_hpa(),
            pig_brain: crate::rna_mouse_brain_sample_hpa(),
        };
        for pb in app.pig_brain {
            print!("{} - {}", pb.animal, pb.tpm);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    println!("{}", arg);
    if arg == "download" {
        let response = reqwest::blocking::get("https://www.proteinatlas.org/about/download")
            .unwrap()
            .text()
            .unwrap();
        let document = scraper::Html::parse_document(&response);
        let url_selector = scraper::Selector::parse("a").unwrap();
        let zipped_to_download:Vec<String> = document.select(&url_selector)
            .map(|x| x.value().attr("href").expect("get href").to_string())
            .filter(|href| href.contains("/download/"))
            .collect();
        for z in zipped_to_download {
            let path = z.clone();
            let splitpath = path.split("/");
            let fname = splitpath.last().expect("name");
            let mut file = std::fs::File::create(format!("data/{}", fname)).expect("Create file handler");
            let response = reqwest::blocking::get(format!("https://www.proteinatlas.org{}", path))
                .unwrap()
                .copy_to(&mut file)
                .expect("Write out to file");
            println!("{} {} bytes", path, response);
            
        }
        return;
    }

}
