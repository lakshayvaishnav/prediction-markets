import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarkets } from "../target/types/prediction_markets";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { getMint } from "@solana/spl-token"

describe("prediction-markets", () => {

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.predictionMarkets as Program<PredictionMarkets>;

  


  // ------------ helper function
  async function airdrop(user: PublicKey, connection: Connection) {
    const signature = await connection.requestAirdrop(user, 5 * LAMPORTS_PER_SOL);

    const latestBlockhash = await connection.getLatestBlockhash();

    let tx = await connection.confirmTransaction({
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      blockhash: latestBlockhash.blockhash,
      signature: signature
    })

    console.log(" ✅ airdropped successfull balance is : ", await connection.getBalance(user))
  }
});
