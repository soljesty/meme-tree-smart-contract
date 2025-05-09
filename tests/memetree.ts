import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Memetree } from "../target/types/memetree";
import bs58 from 'bs58'

const { Keypair, Connection, clusterApiUrl, sendAndConfirmTransaction, PublicKey } = web3

const MyWallet = Keypair.fromSecretKey(bs58.decode("5EnuQjNurFZ5mrsfoAZBhyb13rd36FTZNdRZT6TYPKNgBqkQxufWgVkjZFcPLU6Ln9TXLmwLczyGg8ZWtoykEjGg"))
const TreasuryWallet = new PublicKey("2XLfuctDXuwXmaFHiCbRQXgCmJozs5457EkAbZ9JeXkW")
const MyToken = new PublicKey("HzwqbKZw8HxMN6bF2yFZNrht3c2iXXzpKcFu7uBEDKtr")
const Target = new PublicKey("3N6yinTdLsCTiwwNo2JmAGYTFJWCbwzeWnopC9fpn1RK")
const connection = new Connection(clusterApiUrl("devnet"))


console.log("My wallet is : ", MyWallet.publicKey)

describe("memetree", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Memetree as Program<Memetree>;
  const [pdaWallet] = PublicKey.findProgramAddressSync(
    [MyWallet.publicKey.toBytes(),
    MyToken.toBytes()],
    program.programId
  )

  const [marketingWallet] = PublicKey.findProgramAddressSync(
    [Buffer.from("wallet"),
    MyWallet.publicKey.toBytes(),
    MyToken.toBytes()],
    program.programId
  )

  it("Is initialized!", async () => {
    console.log("pdaWallet ; ", pdaWallet, " , ", marketingWallet)

    // Add your test here.
    const tx = await program.methods
      .initialize(TreasuryWallet)
      .accounts({
        pdaWallet,
        marketingWallet,
        myToken: MyToken,
        treasury: TreasuryWallet
      })
      .signers([MyWallet])
      .transaction();
    tx.feePayer = MyWallet.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    console.log(await connection.simulateTransaction(tx))

    sendAndConfirmTransaction(connection, tx, [MyWallet])
  });

  it("Withdraw", async () => {
    console.log("pdaWallet ; ", pdaWallet, " , ", marketingWallet)
    const tx = await program.methods
      .withdraw(new BN(100_000_000))
      .accounts({
        pdaWallet,
        marketingWallet,
        myToken: MyToken,
        treasury: TreasuryWallet,
        targetWallet: Target
      })
      .signers([MyWallet])
      .transaction();
    tx.feePayer = MyWallet.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    console.log(await connection.simulateTransaction(tx))

    sendAndConfirmTransaction(connection, tx, [MyWallet])
  })
});
