
pub struct TransactionInput
{
    outpoint: [u8; 32],
    signature: [u8; 64],
}

pub struct TransactionOutput
{
    value: u32,
    public_key: [u8; 32],
}

pub struct Transaction
{
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}
