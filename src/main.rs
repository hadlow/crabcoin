mod block;
mod blockchain;
mod transaction;
mod address;

fn main()
{
    let previous_hash: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,];

    let transactions: Vec<transaction::Transaction> = Vec::new();

    let block: block::Block = block::Block::new(1, previous_hash, 1000, transactions);

    let hash = block.hash();

    //println!("{:?}", hash);

    let address: address::Address = address::Address::new();

    println!("{:?}", address.get());
}
