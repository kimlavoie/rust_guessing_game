extern crate rand;

#[derive(Debug)]
struct Answer {
    message: String,
    continu: bool
}

impl Answer {
     pub fn new(message: String, continu: bool) -> Answer{
          Answer{continu: continu, message:message, }   
     }
     pub fn print_message(&self){
         println!("{}", self.message);
     }
}

impl PartialEq for Answer {
    fn eq(&self, other: &Answer) -> bool{
        self.message == other.message && self.continu == other.continu
    }
}

fn read_line() -> Result<String, std::io::Error> {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).map(|_| input)
}

fn read_number() -> u32 {
    loop {
        let trimmed = read_trimmed_line();   
        match trimmed.parse::<u32>() {
            Ok(i) => return i,
            Err(..) => println!("Vous n'avez pas entré un entier: {}", trimmed)
        };
    }
}

fn read_trimmed_line() -> String{
    String::from(read_line().unwrap().trim())
}
 
fn get_result(chosen_number: u32, number_to_guess: u32) -> Answer{
    return if chosen_number < number_to_guess {
        Answer::new(String::from("Le nombre choisi est trop petit"), true)
    } else if chosen_number > number_to_guess {
        Answer::new(String::from("Le nombre choisi est trop grand"), true)
    } else {
        Answer::new(String::from("Félicitations!"), false)
    }
}

fn play(){
    let random_number = rand::random::<u32>() % 100;
        println!("{}", random_number);  

    let mut continu = true;
    while continu {
        println!("Veuillez entrer un nombre:");
        let n = read_number();
        let answer = get_result(n, random_number);
        continu = answer.continu;
        answer.print_message();
    }
}

fn main() {
    println!("Bienvenue!");
    let mut continu = true;
    while continu{
        play();
        println!("Rejouer? o/n");
        continu = read_trimmed_line() == "o";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_equal() {
        let Answer{continu, message} = get_result(42,42);
        assert_eq!(continu, false);
        assert_eq!(message, "Félicitations!");
    }

    #[test]
    fn test_get_result_less() {
        let Answer{continu, message} = get_result(41,42);
        assert_eq!(
            (continu, message), 
            (true, String::from("Le nombre choisi est trop petit"))
        );
    }

    #[test]
    fn test_get_result_greater() {
        assert_eq!(
            Answer{continu: true, message:String::from("Le nombre choisi est trop grand")}, 
            get_result(43,42)
        ); 
    }

}
