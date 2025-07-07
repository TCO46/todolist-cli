use clap;

#[derive(clap::ValueEnum, Clone)]
pub enum Sort {
    Name,
    Priority
}
