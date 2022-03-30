# Rust Blockchain

Basic blockchain implementation in rust with features like mining, transactions, etc.

Reference: Using tutorials in this youtube playlist: https://youtube.com/playlist?list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv

### Mining Process:

Step 1: Generate a new nonce.

Step 2: Add it to the block.

Step 3: Add those bytes together.

Step4: Check the hash against difficulty and if it does not pass the check
change nonce and go through step 1 till 4 till the check passes.

Step 5: Add block to the blockchain.

Step 6: Submit to peers (Not doing for now).

### Verification process:

We will verify 4 things now:

1. Actual index == stored index value (note that Bitcoin blocks don't store their index).

2. Block's hash fits stored difficulty value (we'll just trust the difficulty for now) (insecure).

3. Time is always increasing (In Real Life network latency/sync demands leniency here).

4. Actual previous block's hash == stored prev_block_hash value (except the genesis block).

Note: Bitcoin protocol describes 19 steps for verification.

## To run the project:

Step 1: Make sure you have rust and cargo installed in your system.

Step 2: Run using cargo:

```
cargo run
```

Continue from: 14:17 video 5 : https://www.youtube.com/watch?v=1t4TXnB4Qj4&list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv&index=5
