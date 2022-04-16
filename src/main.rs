use ansi_term::{self, Colour};
use clap::{ArgEnum, Parser};

fn draw(quote: &str, color: &Colour) {
    const CAT: &'static str = r"
    |              |
    .' `.          .' `.
   ; :   \_..--.._/   : .
   | . '            ` . |
   '   ___        ___   `
   '  `.  `.    .'  .'  `
  :     `-.|    |.-'     :
  .     .  `    '  .     ,
  /      `. \  / .'      \
 `,'  . . .` `' '. . .  `.'
  `,'    .__.--.__.    `.'
   `,'                `.'
    `,'-`;::....::;'-`.'
     `    ''::::``    '
    ";
    println!("{}", format!("\"{}\"{}", quote, color.paint(CAT)));
}

fn input() -> (String, Colour) {
    #[derive(Parser, Debug)]
    #[clap(author, version, about = "catsay")]
    struct Args {
      #[clap(short, long)]
      quote: String,
      #[clap(arg_enum)]
      color: Colors,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
    enum Colors {
      Red, 
      Green,
      Blue,
      Purple,
      Yellow,
      Cyan,
      White,
    }

    let args = Args::parse();

    let color_matched = match args.color {
      Colors::Red => Colour::Red,
      Colors::Green => Colour::Green,
      Colors::Blue => Colour::Blue,
      Colors::Purple => Colour::Purple,
      Colors::Yellow => Colour::Yellow,
      Colors::Cyan => Colour::Cyan,
      Colors::White => Colour::White,
    };
    (args.quote, color_matched)
}

fn main() {
  let (q, c) = input();
  draw(&q, &c)
}
