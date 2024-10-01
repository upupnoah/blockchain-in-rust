use blockchain_in_rust::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4); // Difficulty level set to 4

    println!("Mining block 1...");
    blockchain.add_block(String::from("First block after genesis"));

    println!("Mining block 2...");
    blockchain.add_block(String::from("Second block"));

    println!("Mining block 3...");
    blockchain.add_block(String::from("Third block"));

    // Verify blockchain integrity
    println!("Is blockchain valid? {}", blockchain.is_valid());

    // Print the blockchain
    for block in blockchain.blocks.iter() {
        println!("{:#?}", block);
    }
}
