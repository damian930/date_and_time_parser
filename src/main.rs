use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let result = Grammar::parse(Rule::date_time, "06.11.2024 19:34:00");
    println!("{:?}", result); 

    return Ok(());
}
