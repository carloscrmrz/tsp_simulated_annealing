use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long)]
    pub instance: Option<String>,

    #[clap(short('p'), long)]
    pub instance_path: Option<String>,

    #[arg(short, long, default_value_t = 50000.0)]
    pub temperature: f64,

    #[arg(short, long, default_value_t = 0.98)]
    pub decay_factor: f64,

    #[arg(short, long)]
    pub seed: Option<u64>,

    #[arg(short, long, default_value_t = 5000)]
    pub lot_size: usize,

    #[arg(short, long, default_value_t = 10000)]
    pub max_lots: usize,

    #[arg(long)]
    pub db_path: Option<String>,

    #[arg(short('c'), long)]
    pub threads: Option<usize>,
}