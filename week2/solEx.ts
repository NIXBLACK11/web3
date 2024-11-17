import { Keypair } from "@solana/web3.js";
import nacl from "tweetnacl";

const keypair = Keypair.generate();

const publicKey = keypair.publicKey.toString();
const secretKey = keypair.secretKey;

const message = new TextEncoder().encode("hello world");

const signature = nacl.sign.detached(message, secretKey);

const result = nacl.sign.detached.verify(
    message,
    signature,
    keypair.publicKey.toBytes(),
);

console.log(result);