use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        difficulty,
    );
    //So in case of bitcoin they have set there difficulty in such a way that it takes 10 mins on
    //everyone's machine and miners try to find the nonce for the reward
    block.mine();
    println!("Mined Genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..10 {
        //1 and including 10
        let mut block = Block::new(
            i,
            now(),
            last_hash, //previous block hash
            0,
            "Another block!".to_owned(),
            difficulty,
        );
        //mine another block
        block.mine();
        println!("Mined block {:?}", &block);
        last_hash = block.hash.clone(); //update last hash
        blockchain.blocks.push(block);
        //verify after mining
        println!("Verify: {}", &blockchain.verify());
    }

    println!("To check if verification fails: ===================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    //Another : To check if verify is working, this return verify false:
    blockchain.blocks[3].prev_block_hash[18] = 7;
    println!("Verify: {}", &blockchain.verify());
}
