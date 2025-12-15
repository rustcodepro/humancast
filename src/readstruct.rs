#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ReadSNP {
    pub rsid: String,
    pub chromosome: String,
    pub position: String,
    pub genotype: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ReadAnnotate {
    pub rsid: String,
    pub chromosome: String,
    pub position: String,
    pub genotype: String,
    pub annotate: String,
}
