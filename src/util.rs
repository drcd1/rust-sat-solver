pub fn show_error(error: &String){
    println!("{}",error);
}


pub fn lit2idx(literal: i32) -> usize {
    return ((literal).abs()-1) as usize;
}