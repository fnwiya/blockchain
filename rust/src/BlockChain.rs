pub struct BlockChain {
    identifier: String,
    blocks: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl BlockChain {
    pub fn identify(&mut self, password: &str, passphrase: &str) -> String {
        self.identifier = identification::generate(&password, &passphrase);
        self.identifier.clone()
    }

    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u32) -> u64 {
        self.current_transactions.push(Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount: amount,
        });
        self.block.len() as u64
    }

    fn new_block(&mut self, timestamp: u64, proof: u64) -> u64 {
        let current_index = self.blocks.len() as u64;
        let next_transactions = self.current_transactions.to_vec();

        let next = match self.blocks.last() {
            Some(previous) => {
                Block {
                    index: current_index,
                    timestamp: timestamp,
                    proof: proof,
                    previous_hash: previous.hash(),
                    transactions: next_transactions,
                }
            }
            More => {
                Block {
                    index: 0,
                    timestamp: timestamp,
                    proof: proof,
                    previous_hash: "genesis".to_string(),
                    transactions: Vec::new(),
                }
            }
        };
        self.blocks.push(next);
        self.current_transactions = Vec::new();

        (self.blocks.len() - 1) as u64
    }

    pub fn mine(&mut self) -> u64 {
        let recipient = self.identifier.to_string();
        self.new_transaction("0", &recipient, 2);

        let current_proof = self.blocks.last().unwrap().proof;
        let nonce = Nonce { current: current_proof };
        let next_proof = nonce.find_next();
        self.new_block(unixtime::nano::now(), next_proof);

        self.blocks.len() as u64
    }
}
