use std::io;
use rand::Rng;

fn main() {
    let mut ip_array: [u32; 4] = [0; 4];
    
    let mut secret_array: [u32; 4] = [0; 4];
    
    secret_array[0] = rand::thread_rng().gen_range(0..256);
    secret_array[1] = rand::thread_rng().gen_range(0..256);
    secret_array[2] = rand::thread_rng().gen_range(0..256);
    secret_array[3] = rand::thread_rng().gen_range(0..256);
    
    let secret_ip = format!("{}.{}.{}.{}", secret_array[0], secret_array[1], secret_array[2], secret_array[3]);
    let mut ip = format!("{}.{}.{}.{}", ip_array[0], ip_array[1], ip_array[2], ip_array[3]);

    while ip_array != secret_array {
        println!("My ip : {}", ip);
        println!("Secret ip : {}", secret_ip);

        println!("Please enter a number between 0 and 255 : ");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if secret_array.contains(&answer) {
            let index = secret_array
                .iter()
                .position(|&x| x == answer)
                .unwrap();

            ip_array[index] = answer;

            ip = format!("{}.{}.{}.{}", ip_array[0], ip_array[1], ip_array[2], ip_array[3]);
        }
    }


    println!("You find the secret ip ! {}", ip);

}
