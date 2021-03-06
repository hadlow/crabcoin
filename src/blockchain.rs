use crate::block;

pub struct BlockChain
{
    blocks: Vec<block::Block>,
}

impl BlockChain
{
    pub fn is_valid(&self, previous_block: &block::Block) -> bool
    {
        // Previous block exists and is valid

        // self.timestamp > previous_block.timestamp && self.timestamp < now + 2hrs
        
        // PoW is valid

        // S[0] = state at end of previous_block

        // Loop self.transactions as index, TX
        // > S[index + 1] = TRANSACT(S[index], TX]
        //
        // Another way
        // > S = TRANSACT(S, TX)
        
        // self.state = S

        true
    }


}
