use std::{process,io};

enum MetaCmdResult {
    META_CMD_SUCCESS,
    META_CMD_UNRECOGNISED_CMD,
}

enum PrepareResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNISED_STATEMENT
}

enum StatementType{
    STATEMENT_INSERT,
    STATEMENT_SELECT
}



#[derive(Default)]
struct Statement{
    stype:Option<StatementType>,
}






fn print_prompt(){
    print!("db > ");
    io::Write::flush(&mut io::stdout()).expect("flush failed");
}

fn read_input(buffer: &mut String){
    buffer.clear();
    io::stdin().read_line(buffer).expect("error reading input");
}

fn do_meta_cmd(buffer: &str) -> MetaCmdResult{
    if(buffer == ".exit"){
        process::exit(0x0100)
    }else{
        MetaCmdResult::META_CMD_UNRECOGNISED_CMD
    }
}




// This function acts as the parser which parses commands into statements understandable by the virtual machine
fn prepare_statement(buffer: &str , statement:&mut Statement) -> PrepareResult{
    if buffer == "insert"{
        statement.stype = Some(StatementType::STATEMENT_INSERT);
        return PrepareResult::PREPARE_SUCCESS;

    }else if buffer == "select" {
        statement.stype = Some(StatementType::STATEMENT_SELECT);
        return PrepareResult::PREPARE_SUCCESS;
    }

    PrepareResult::PREPARE_UNRECOGNISED_STATEMENT
}



// This function acts as the virtual machine which interprets statements made by prepare_statements
fn execute_statement(statement:&mut Statement){
    match statement.stype{
        Some(StatementType::STATEMENT_INSERT) => println!("insert value function"),
        Some(StatementType::STATEMENT_SELECT) => println!("select value function"),
        _ => println!("error in statement type")
    }
}







fn main() {
    
    let mut buffer = String::new();
    
    loop{
        print_prompt();
        read_input(&mut buffer);

        let input = buffer.trim();

        if &input[0..1] == "." {
            match do_meta_cmd(input){

                MetaCmdResult::META_CMD_SUCCESS => continue,
                MetaCmdResult::META_CMD_UNRECOGNISED_CMD => {
                    println!("Unrecognised command {input}");
                    continue
                }
            }
        }

        let mut statement:Statement = Default::default();

        match prepare_statement(input, &mut statement){
            PrepareResult::PREPARE_SUCCESS =>{} ,
            PrepareResult::PREPARE_UNRECOGNISED_STATEMENT => {
                println!("unrecognised keyword at start of {buffer}");
                continue
            }
        }

        execute_statement(&mut statement);
        println!("Executed!")
    }

}
