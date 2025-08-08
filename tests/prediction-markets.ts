import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarkets } from "../target/types/prediction_markets";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { getMint } from "@solana/spl-token"

describe("prediction-markets", () => {

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.predictionMarkets as Program<PredictionMarkets>;

  let betCreator: Keypair;

  before(async () => {
    betCreator = Keypair.generate();
    await airdrop(betCreator.publicKey, provider.connection);
  })

  it("it initializes the bet", async () => {

    let title = "Will lxsh wins";
    let oracle_info = "random shit";
    let now_sec = Math.floor(Date.now() / 1000) + 60;
    let start_ts = new anchor.BN(now_sec);
    let end_ts = new anchor.BN(now_sec + 3600 + 60);
    let yes_pool = new anchor.BN(1_000_000);
    let no_pool = new anchor.BN(500_000)
    let connector_weight = 100_000 // eg 10%

    let [betPda, bump] = await PublicKey.findProgramAddressSync(
      [betCreator.publicKey.toBuffer(), Buffer.from(title)],
      program.programId
    )

    const [yesMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("yes_mint"), betPda.toBuffer()],
      program.programId
    );

    const [noMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("no_mint"), betPda.toBuffer()],
      program.programId
    );

    let tx = await program.methods
      .initializeBet(
        title,
        oracle_info,
        start_ts,
        end_ts,
        yes_pool,
        no_pool,
        connector_weight)
      .accounts({
        betCreator: betCreator.publicKey,
      }).signers([betCreator])
      .rpc()

    let betAccount = await program.account.bet.fetch(betPda);
    console.log("✅ bet title : ", betAccount.betTitle);
    console.log("✅ bet resolved status : ", betAccount.resolved);
    console.log("✅ stat ts : ", betAccount.startTs);
    console.log("✅ end ts : ", betAccount.endTs);
    console.log("✅ yes pool : ", betAccount.yesPool);
    console.log("✅ no pool : ", betAccount.noPool);
    console.log("✅ connector weight : ", betAccount.connectorWeight);

    const yesMint = getMint(provider.connection, yesMintPda);
    const noMint = getMint(provider.connection, noMintPda);

    console.log("✅bet pda : ", betPda);
    console.log("✅yes mint : ", (await yesMint).mintAuthority);
    console.log("✅no min : ", (await noMint).mintAuthority);
  });

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
