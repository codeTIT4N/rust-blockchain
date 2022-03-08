use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        0x0000fffffffffffffffffffffffffff,
    );
    block.hash = block.hash();

    println!("{:?}", block);
    block.mine();
    println!("{:?}", block);

    //So in case of bitcoin they have set there difficulty in such a way that it takes 10 mins on
    //everyone's machine and miners try to find the nonce for the reward
}
