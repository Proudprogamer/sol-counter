import * as borsh from "borsh";
import { Connection, Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { test, expect } from "bun:test";
import { COUNTER_SIZE, schema } from "./types";


const adminAccount = new Keypair();
const dataAccount = new Keypair();

const program_id = new PublicKey("3f4HyugnBtiY5JZjDTbS9toqYBjkuUJ4UkKUt1wmUFkp");

test("initializing", async()=>{
    const connection = new Connection("http://127.0.0.1:8899");
    const txn = await connection.requestAirdrop(adminAccount.publicKey, 1*1000000000);
    await connection.confirmTransaction(txn);
    //airdrop done

    const lamports =await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const ix = SystemProgram.createAccount({
        fromPubkey : adminAccount.publicKey,
        lamports,
        space : COUNTER_SIZE,
        programId : program_id,
        newAccountPubkey : dataAccount.publicKey
    });

    const transaction = new Transaction();
    transaction.add(ix);
    const signature = await connection.sendTransaction(transaction, [adminAccount, dataAccount]);

    await connection.confirmTransaction(signature);

    console.log(dataAccount.publicKey.toBase58());

    const dataAccountInfo =await connection.getAccountInfo(dataAccount.publicKey);

    
    const deserialized_info = borsh.deserialize(schema, dataAccountInfo?.data);
    const value = deserialized_info?.count;
    expect(value).toBe(0);
    
})