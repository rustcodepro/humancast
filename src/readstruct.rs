use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialOrd, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ReadSNP {
    pub rsid: String,
    pub chromosome: String,
    pub position: String,
    pub genotype: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ReadAnnotate {
    pub rsid: String,
    pub chromosome: String,
    pub position: String,
    pub genotype: String,
    pub annotate: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct AnnotateID {
    pub rsid: String,
    pub idinformation: String,
}
