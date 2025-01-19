fn main() {
    let my_string: String = String::from("Ei je duniya!");

    // create a Vec<char> which is an array type
    let arr : Vec<char> = my_string.chars().collect();

    for i in 0..arr.len(){
        print!("{}", arr[i]);
    }

}
