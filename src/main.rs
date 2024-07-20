mod data_string;
mod program_string;
mod bct;
mod dropper;

fn main() {
    let bct = BCT {
        data: data_string::DataString::new(vec![
            true
        ]),
        program: program_string::ProgramString::new(vec![
            true
        ]),
    };

    let program = bct.program.iter_loop();
    while let Some(command) =  {
        match command {
            false => bct.data.delete_leftmost(),
            true => {
            
            }
        }
    }
}
