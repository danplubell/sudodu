mod sudoku;
mod solution;
mod model_inner;
mod solvers;
mod model;

fn main() {
    let _solution = "318457962572986143946312578639178425157294836284563791425731689761829354893645217";
    let _puzzle_data = "310450900072986143906010508639178020150090806004003700005731009701829350000645010";
    //let s = Sudoku::new(solution, puzzle);
//    let p = puzzle_save::Puzzle::try_from(_solution);
}
/*
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




 */