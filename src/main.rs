use crate::square_attacker::SquareAttacker;
mod encryption_service;
mod square_attacker;

fn main() {
    let mut aes_key: [u8; 16] = *b"hinterherdackeln";
    let square_attacker: SquareAttacker = SquareAttacker::new(&mut aes_key);
    square_attacker.execute_full_attack();
}