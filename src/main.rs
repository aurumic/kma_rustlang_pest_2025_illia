use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {
  let got: pest::iterators::Pairs<'_, Rule> = Grammar::parse(Rule::file, "-273.15,-15\n")?;
  println!("{:?}", got);

  Ok(())
}

/*

> cargo run

> [Pair { rule: file, span: Span { str: "-273.15,-15\n", start: 0, end: 12 },
    inner: [
      Pair { rule: record, span: Span { str: "-273.15,-15", start: 0, end: 11 },
        inner: [
          Pair { rule: field, span: Span { str: "-273.15", start: 0, end: 7 }, inner: [] },
          Pair { rule: field, span: Span { str: "-15", start: 8, end: 11 }, inner: [] }
        ] 
      },
      Pair { rule: EOI, span: Span { str: "", start: 12, end: 12 }, inner: [] }
    ]
  }]

*/