# Parsing Human Protein Atlas Files - Currently a POC

grabbing the tsv files from https://www.proteinatlas.org/about/download and parsing them into structs for rust.
to download the data files pass an arg
`cargo run -- download`
`cargo run` will read and parse the files into an app struct - see main.rs/main

```rust
    pub struct App
        tissues: Vec::<Gene>,
        pathogens: Vec::<Pathogen>,
        rna_immune_cell_schmiedel: Vec<RNAImmuneCellSchmiedel>,
        rna_celline_cancer: Vec<RNACellLineCancer>,
        rna_immune_cell: Vec<RNAImmuneCell>,
        rna_immune_cell_monaco: Vec<RNAImmuneCellMonaco>,
        rna_mouse_brain_hpa: Vec<RNAMouseBrainHPA>,
        rna_mouse_brain_allen: Vec<RNAMouseBrainAllen>,
        pig_brain: Vec<AnimalSample>,
        mouse_brain: Vec<AnimalSample>