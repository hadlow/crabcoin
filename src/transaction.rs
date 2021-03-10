
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

impl Transaction
{
    pub fn new() -> Self
    {
        Self
        {
            inputs: Vec::new(),
            outputs: Vec::new()
        }
    }

    pub fn serialize(&self) -> Vec<u8>
    {
        let mut transaction: Vec<u8> = Vec::new();

        for input in self.inputs.iter()
        {
            transaction.extend_from_slice(&input.outpoint);
            transaction.extend_from_slice(&input.signature);
        }

        for output in self.outputs.iter()
        {
            transaction.extend_from_slice(&output.value.to_be_bytes());
            transaction.extend_from_slice(&output.public_key);
        }

        transaction
    }
}
