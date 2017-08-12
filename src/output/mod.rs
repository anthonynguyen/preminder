pub mod stdout;

trait OutputPlugin {}

pub fn init(block: Vec<settings::OutputBlock>) {
    println!("{:?}", block)
}
