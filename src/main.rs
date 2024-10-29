mod sudoku;
mod puzzle;
mod solution;

use csv::Reader;
use std::error::Error;
use sudoku::Sudoku;

fn main() {
    let solution = "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
    let puzzle = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
    //let s = Sudoku::new(solution, puzzle);
    let c: char = '8';
    let b: u8 = c as u8;
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("../sudoku.csv_save")?;
    let mut i = 0;
    for (i, result) in rdr.records().enumerate() {
        if i > 10 {
            break;
        }
        let record = result?;
        record.as_slice();
        println!("{:?}", record);

    }
    Ok(())
}

