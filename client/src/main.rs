use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "7roxy Client",
    about = "A personal AI proxy agent terminal client."
)]
struct Opt {}

fn main() {
    let Opt {} = Opt::from_args();
}
