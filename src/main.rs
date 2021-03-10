mod block;
mod blockchain;
mod transaction;
mod address;
mod network;

fn main()
{
    let previous_hash: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,
                                   0, 0, 0, 0, 0, 0, 0, 0,];

    let transactions: Vec<transaction::Transaction> = Vec::new();

    let block: block::Block = block::Block::new(1, previous_hash, 1000, transactions);

    let hash = block.hash();

    let private_key: [u8; 32] = *b"1IfrgKcxKHtC1SEVaVKLMIjHWUqjRiX1";
    let address: address::Address = address::Address::new(&private_key);

    println!("{}", address.get());

    // network
    let net: network::Network = network::Network::new();
}
