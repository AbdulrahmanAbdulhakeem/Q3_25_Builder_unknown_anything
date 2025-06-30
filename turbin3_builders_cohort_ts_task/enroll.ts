import { Connection,Keypair,PublicKey, SystemProgram } from "@solana/web3.js";
import { Program,Wallet,AnchorProvider } from "@coral-xyz/anchor";
import {IDL,Turbin3Prereq} from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json"
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")
const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111")
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2")

const secretKey = bs58.decode(wallet)
const keypair = Keypair.fromSecretKey(secretKey)
// console.log(keypair.publicKey)

const connection = new Connection("https://api.devnet.solana.com");


const provider = new AnchorProvider(connection,new Wallet(keypair),{
    commitment:"confirmed"
})

const program:Program<Turbin3Prereq> = new Program(IDL,provider) 

const account_seed = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer()
]

const [account_key,_account_bump] = PublicKey.findProgramAddressSync(account_seed,program.programId) 
const mintTs = Keypair.generate();

const authority_seed = [
    Buffer.from("collection"),
    mintCollection.toBuffer()
];


const [authority_key,_authority_bump] = PublicKey.findProgramAddressSync(authority_seed,program.programId);


//Execute Initialize instruction
// (async () => {
//     try {
//         const txhash = await program.methods.initialize("AbdulrahmanAbdulhakeem").accountsPartial({
//             user:keypair.publicKey,
//             account:account_key,
//             system_program:SYSTEM_PROGRAM_ID
//         }).signers([keypair]).rpc()
//         console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
//     } catch (error) {
//         console.error(`OOPS Something went wrong:${error}`)
//     }
// })()

//Execute the submitTs transaction
(async () => {
    try {
        const txhash = await program.methods.submitTs().accountsPartial({
            user:keypair.publicKey,
            account:account_key,
            mint:mintTs.publicKey,
            collection:mintCollection,
            authority:authority_key,
            mpl_core_program:MPL_CORE_PROGRAM_ID,
            system_program:SYSTEM_PROGRAM_ID
        }).signers([keypair,mintTs]).rpc()

        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (error) {
        console.error(`OOPS Something went wrong:${error}`)
    }
})()
