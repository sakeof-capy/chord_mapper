// use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/chord.pest"]
pub struct ChordParser;

// pub fn parse_chord(input: &str) -> Result<(), pest::error::Error<Rule>> {
//     let pairs = ChordParser::parse(Rule::root, input)?;
//     println!("{:?}", pairs);
//     // for pair in pairs {
//     //     match pair.as_rule() {
//     //         Rule::root => {
//     //           println!("Found root: {}", pair.as_str());

//     //         }
//     //         Rule::chord => {
//     //             println!("Found chord: {}", pair.as_str());
//     //         }
//     //         Rule::delimitor => {
//     //             // Ignore delimiters.
//     //         }
//     //         _ => {
//     //           eprintln!("Encountered an unexpected rule: {:?}", pair.as_rule());
//     //       }
//     //     }
//     // }
//     Ok(())
// }
