use ik_config::ik;

mod ik_config;
mod urdf_parser;

fn main() {
    ik_config::ik();
    urdf_parser::parse();
}