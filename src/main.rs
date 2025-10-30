use my_parser_kma_lol1337::*;

use pest::Parser;

pub fn main() -> anyhow::Result< () >{
    // println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
    let got = Grammar::parse(Rule::file, "-273.15,-15\n")?;
    println!("{:?}", got);
    Ok(())
}
