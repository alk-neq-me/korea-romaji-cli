mod color;
mod conveter;
mod banner;

use std::io::Write;
use conveter::Conveter;
use color::Color;


fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let mut result = Conveter::new();
    
    banner::show();

    loop {
        print!("\n\nhangul: ");
        std::io::stdout().flush().expect("failed flushing output");
        std::io::stdin().read_line(&mut input).expect("failed reading input");

        if input.trim() == "q" {
            break
        };

        if input.trim() == "help" {
            banner::show();
        }

        match result.convert(&input.trim()) {
            Ok(txt) => {
                let log = Color::Purple("[ Ok ]");
                println!("{log} {:#?}", txt.romaji);
            },
            Err(e) => {
                let log = Color::Red("[ Err ]");
                println!("{log} {e:?}");
            }
        };
        input.clear();
        result.clear();
    }

    Ok(())
}
