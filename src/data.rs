use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAMouseBrainHPA {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Brain region")]
    pub brain_region: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
    #[serde(rename = "nTPM")]
    pub n_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pathogen {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Cancer")]
    pub cancer: String,
    #[serde(rename = "High")]
    pub high: String,
    #[serde(rename = "Medium")]
    pub medium: String,
    #[serde(rename = "Low")]
    pub low: String,
    #[serde(rename = "Not detected")]
    pub not_detected: String,
    #[serde(rename = "prognostic - favorable")]
    pub prognostic_favorable: String,
    #[serde(rename = "unprognostic - favorable")]
    pub unprognostic_favorable: String,
    #[serde(rename = "prognostic - unfavorable")]
    pub prognostic_unfavorable: String,
    #[serde(rename = "unprognostic - unfavorable")]
    pub unprognostic_unfavorable: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNACellLineCancer {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Cancer")]
    pub cancer: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
    #[serde(rename = "nTPM")]
    pub n_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAImmuneCellSchmiedel {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Immune cell")]
    pub immune_cell: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAImmuneCellMonaco {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Immune cell")]
    pub immune_cell: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAImmuneCellSample {
    #[serde(rename = "Sample ID")]
    pub sample_id: String,
    #[serde(rename = "Donor")]
    pub donor: String,
    #[serde(rename = "Immune cell")]
    pub immune_cell: String,
    #[serde(rename = "ENSG ID")]
    pub ensg_id: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
    #[serde(rename = "nTPM")]
    pub n_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAMouseBrainAllen {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Brain region")]
    pub brain_region: String,
    #[serde(rename = "Expression energy")]
    pub expression_energy: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RNAImmuneCell {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene name")]
    pub gene_name: String,
    #[serde(rename = "Immune cell")]
    pub immune_cell: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
    #[serde(rename = "nTPM")]
    pub n_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimalSample {
    #[serde(rename = "ENSMUSG_ID")]
    pub ensmusg_id: String,
    #[serde(rename = "Main_region")]
    pub main_region: String,
    #[serde(rename = "Subregion")]
    pub subregion: String,
    #[serde(rename = "Animal")]
    pub animal: String,
    #[serde(rename = "TPM")]
    pub tpm: String,
    #[serde(rename = "pTPM")]
    pub p_tpm: String,
    #[serde(rename = "nTPM")]
    pub n_tpm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gene {
    #[serde(rename = "Gene")]
    pub gene: String,
    #[serde(rename = "Gene synonym")]
    pub gene_synonym: Option<String>,
    #[serde(rename = "Ensembl")]
    pub ensembl: Option<String>,
    #[serde(rename = "Gene description")]
    pub gene_description: Option<String>,
    #[serde(rename = "Uniprot")]
    pub uniprot: Option<String>,
    #[serde(rename = "Chromosome")]
    pub chromosome: Option<String>,
    #[serde(rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Protein class")]
    pub protein_class: Option<String>,
    #[serde(rename = "Biological process")]
    pub biological_process: Option<String>,
    #[serde(rename = "Molecular function")]
    pub molecular_function: Option<String>,
    #[serde(rename = "Disease involvement")]
    pub disease_involvement: Option<String>,
    #[serde(rename = "Evidence")]
    pub evidence: Option<String>,
    #[serde(rename = "HPA evidence")]
    pub hpa_evidence: Option<String>,
    #[serde(rename = "UniProt evidence")]
    pub uni_prot_evidence: Option<String>,
    #[serde(rename = "NeXtProt evidence")]
    pub ne_xt_prot_evidence: Option<String>,
    #[serde(rename = "RNA tissue specificity")]
    pub rna_tissue_specificity: Option<String>,
    #[serde(rename = "RNA tissue distribution")]
    pub rna_tissue_distribution: Option<String>,
    #[serde(rename = "RNA tissue specificity score")]
    pub rna_tissue_specificity_score: Option<String>,
    #[serde(rename = "RNA tissue specific nTPM")]
    pub rna_tissue_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA single cell type specificity")]
    pub rna_single_cell_type_specificity: Option<String>,
    #[serde(rename = "RNA single cell type distribution")]
    pub rna_single_cell_type_distribution: Option<String>,
    #[serde(rename = "RNA single cell type specificity score")]
    pub rna_single_cell_type_specificity_score: Option<String>,
    #[serde(rename = "RNA single cell type specific nTPM")]
    pub rna_single_cell_type_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA cancer specificity")]
    pub rna_cancer_specificity: Option<String>,
    #[serde(rename = "RNA cancer distribution")]
    pub rna_cancer_distribution: Option<String>,
    #[serde(rename = "RNA cancer specificity score")]
    pub rna_cancer_specificity_score: Option<String>,
    #[serde(rename = "RNA cancer specific FPKM")]
    pub rna_cancer_specific_fpkm: Option<String>,
    #[serde(rename = "RNA brain regional specificity")]
    pub rna_brain_regional_specificity: Option<String>,
    #[serde(rename = "RNA brain regional distribution")]
    pub rna_brain_regional_distribution: Option<String>,
    #[serde(rename = "RNA brain regional specificity score")]
    pub rna_brain_regional_specificity_score: Option<String>,
    #[serde(rename = "RNA brain regional specific nTPM")]
    pub rna_brain_regional_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA blood cell specificity")]
    pub rna_blood_cell_specificity: Option<String>,
    #[serde(rename = "RNA blood cell distribution")]
    pub rna_blood_cell_distribution: Option<String>,
    #[serde(rename = "RNA blood cell specificity score")]
    pub rna_blood_cell_specificity_score: Option<String>,
    #[serde(rename = "RNA blood cell specific nTPM")]
    pub rna_blood_cell_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA blood lineage specificity")]
    pub rna_blood_lineage_specificity: Option<String>,
    #[serde(rename = "RNA blood lineage distribution")]
    pub rna_blood_lineage_distribution: Option<String>,
    #[serde(rename = "RNA blood lineage specificity score")]
    pub rna_blood_lineage_specificity_score: Option<String>,
    #[serde(rename = "RNA blood lineage specific nTPM")]
    pub rna_blood_lineage_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA cell line specificity")]
    pub rna_cell_line_specificity: Option<String>,
    #[serde(rename = "RNA cell line distribution")]
    pub rna_cell_line_distribution: Option<String>,
    #[serde(rename = "RNA cell line specificity score")]
    pub rna_cell_line_specificity_score: Option<String>,
    #[serde(rename = "RNA cell line specific nTPM")]
    pub rna_cell_line_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA tissue cell type enrichment")]
    pub rna_tissue_cell_type_enrichment: Option<String>,
    #[serde(rename = "RNA mouse brain regional specificity")]
    pub rna_mouse_brain_regional_specificity: Option<String>,
    #[serde(rename = "RNA mouse brain regional distribution")]
    pub rna_mouse_brain_regional_distribution: Option<String>,
    #[serde(rename = "RNA mouse brain regional specificity score")]
    pub rna_mouse_brain_regional_specificity_score: Option<String>,
    #[serde(rename = "RNA mouse brain regional specific nTPM")]
    pub rna_mouse_brain_regional_specific_n_tpm: Option<String>,
    #[serde(rename = "RNA pig brain regional specificity")]
    pub rna_pig_brain_regional_specificity: Option<String>,
    #[serde(rename = "RNA pig brain regional distribution")]
    pub rna_pig_brain_regional_distribution: Option<String>,
    #[serde(rename = "RNA pig brain regional specificity score")]
    pub rna_pig_brain_regional_specificity_score: Option<String>,
    #[serde(rename = "RNA pig brain regional specific nTPM")]
    pub rna_pig_brain_regional_specific_n_tpm: Option<String>,
    #[serde(rename = "Antibody")]
    pub antibody: Option<String>,
    #[serde(rename = "Reliability (IH)")]
    pub reliability_ih: Option<String>,
    #[serde(rename = "Reliability (Mouse Brain)")]
    pub reliability_mouse_brain: Option<String>,
    #[serde(rename = "Reliability (IF)")]
    pub reliability_if: Option<String>,
    #[serde(rename = "Subcellular location")]
    pub subcellular_location: Option<String>,
    #[serde(rename = "Secretome location")]
    pub secretome_location: Option<String>,
    #[serde(rename = "Secretome function")]
    pub secretome_function: Option<String>,
    #[serde(rename = "CCD Protein")]
    pub ccd_protein: Option<String>,
    #[serde(rename = "CCD Transcript")]
    pub ccd_transcript: Option<String>,
    #[serde(rename = "Blood concentration - Conc. blood IM [pg/L]")]
    pub blood_concentration_conc_blood_im_pg_l: Option<String>,
    #[serde(rename = "Blood concentration - Conc. blood MS [pg/L]")]
    pub blood_concentration_conc_blood_ms_pg_l: Option<String>,
    #[serde(rename = "Blood expression cluster")]
    pub blood_expression_cluster: Option<String>,
    #[serde(rename = "Tissue expression cluster")]
    pub tissue_expression_cluster: Option<String>,
    #[serde(rename = "Brain expression cluster")]
    pub brain_expression_cluster: Option<String>,
    #[serde(rename = "Cell line expression cluster")]
    pub cell_line_expression_cluster: Option<String>,
    #[serde(rename = "Single cell expression cluster")]
    pub single_cell_expression_cluster: Option<String>,
    #[serde(rename = "Interactions")]
    pub interactions: Option<String>,
    #[serde(rename = "Subcellular main location")]
    pub subcellular_main_location: Option<String>,
    #[serde(rename = "Subcellular additional location")]
    pub subcellular_additional_location: Option<String>,
    #[serde(rename = "Antibody RRID")]
    pub antibody_rrid: Option<String>,
    #[serde(rename = "Pathology prognostics - Breast cancer")]
    pub pathology_prognostics_breast_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Cervical cancer")]
    pub pathology_prognostics_cervical_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Colorectal cancer")]
    pub pathology_prognostics_colorectal_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Endometrial cancer")]
    pub pathology_prognostics_endometrial_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Glioma")]
    pub pathology_prognostics_glioma: Option<String>,
    #[serde(rename = "Pathology prognostics - Head and neck cancer")]
    pub pathology_prognostics_head_and_neck_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Liver cancer")]
    pub pathology_prognostics_liver_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Lung cancer")]
    pub pathology_prognostics_lung_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Melanoma")]
    pub pathology_prognostics_melanoma: Option<String>,
    #[serde(rename = "Pathology prognostics - Ovarian cancer")]
    pub pathology_prognostics_ovarian_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Pancreatic cancer")]
    pub pathology_prognostics_pancreatic_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Prostate cancer")]
    pub pathology_prognostics_prostate_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Renal cancer")]
    pub pathology_prognostics_renal_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Stomach cancer")]
    pub pathology_prognostics_stomach_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Testis cancer")]
    pub pathology_prognostics_testis_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Thyroid cancer")]
    pub pathology_prognostics_thyroid_cancer: Option<String>,
    #[serde(rename = "Pathology prognostics - Urothelial cancer")]
    pub pathology_prognostics_urothelial_cancer: Option<String>,
}
