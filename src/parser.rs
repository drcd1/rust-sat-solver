use crate::sat_solver::SatSolver;
use std::fs;

pub fn parse_file(filename: &String) -> SatSolver {

    let vc;
    let cc;

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut help = contents.split('\n');
    let mut a = help.next();
    let mut no_line = false;

    //match comments
    loop{
        match a {
            Some(x) => {
                let y = x.trim();

                if y.chars().nth(0).unwrap() != 'c' {
                    break;
                }
                
                println!("Comment: {}",x);

                a= help.next();
            }

            None =>{
                no_line = true;
                break;
            }
        }
    }

    if no_line{
        println!("Empty file!");
        return SatSolver::new(0);
    }

    //first line
    let mut help2 = a.unwrap().split(char::is_whitespace);
    let mut b = help2.next();
    match b {
        Some(x) => {
            if x!="p" {
                println!("Wrong first line");
                return SatSolver::new(0);
            }
        }
        None => {                
            println!("Empty file!");
            return SatSolver::new(0);
        }
    }
    
    b = help2.next();

    match b {
        Some(x) => {
            if x!="cnf" {
                println!("Wrong first line2");
                return SatSolver::new(0);
            }
        }
        None => {                
            println!("Not enough in first line!");
            return SatSolver::new(0);
        }
    }

    
    
    b = help2.next();

    match b {
        Some(x) => {
            match x.parse::<usize>(){
                Ok(y) => {
                    vc = y;
                }

                Err(e) => {       
                    println!("Error parsing nv: {}",e);
                    return SatSolver::new(0);
                }
            }
        }
        None => {                
            println!("Empty file!");
            return SatSolver::new(0);
        }
    }
    
    b = help2.next();

    match b {
        Some(x) => {
            match x.parse::<usize>(){
                Ok(y) => {
                    cc = y;
                }

                Err(e) => {       
                    println!("Error parsing nc: {}",e);
                    return SatSolver::new(0);
                }
            }
        }
        None => {                
            println!("Empty file!");
            return SatSolver::new(0);
        }
    }

    a = help.next();
    println!("We have a sat solver with: {} var and {} clauses", vc,cc);
    let mut tmp = SatSolver::new(vc);

    let mut i = 0;
    let mut current_clause : Vec<i32> = Vec::new();
    while i<cc{
        match a {
            Some(x) => {
                help2 = x.split(char::is_whitespace);
                b = help2.next();
                loop{
                    match b {
                        Some(y) => {
                            //println!("Trying to parse num: '{}'",y);
                            match y.parse::<i32>() {
                                
                                Ok(z) => {
                                    if z.abs() as usize > vc {
                                        println!("Warning: variable index is higher than max! : {}", z.abs());
                                    }

                                    if z!=0{
                                        current_clause.push(z);
                                    } else {
                                        tmp.add_clause(current_clause);
                                        current_clause = Vec::new();
                                        i+=1;
                                    }
                                }
                                Err(_e) => {       
                                    
                                }

                            }
                            
                            b = help2.next();
                        }
                        None => {
                            break;
                        }

                    }
                }             
                
                a= help.next();
            }

            None =>{
                println!("Not enough clauses!");
                return SatSolver::new(0);
            }
        }
    }



    

    return tmp;
}
/*
fn parse_error(msg: &String) -> SatSolver {
    println!("{}",msg);
    return SatSolver::new(0);
}*/