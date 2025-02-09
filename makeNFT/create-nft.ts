import { createNft, fetchDigitalAsset, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { airdropIfRequired, getExplorerLink, getKeypairFromFile } from "@solana-developers/helpers";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { generateSigner, keypairIdentity, percentAmount, publicKey } from "@metaplex-foundation/umi";

const connection = new Connection(clusterApiUrl("devnet"));

const user = await getKeypairFromFile();

await airdropIfRequired(connection, user.publicKey, 1*LAMPORTS_PER_SOL, 0.5*LAMPORTS_PER_SOL);

console.log("Loaded user", user.publicKey.toBase58());

const umi = createUmi(connection.rpcEndpoint);
umi.use(mplTokenMetadata());

const umiUser = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
umi.use(keypairIdentity(umiUser));

console.log("Set to Umi instance for user");

const collectionAddress = publicKey("FECQnpMisT7efTuqxUPFscYwLigkwXfdAtfz1qv3q2bD");

console.log(`Creating NFT...`);

const mint = generateSigner(umi);

const transcation = await createNft(umi, {
    mint,
    name: "My First NFT",
    uri: "https://raw.githubusercontent.com/NIXBLACK11/web3/refs/heads/main/makeNFT/nft-metadata.json",
    sellerFeeBasisPoints: percentAmount(0),
    collection: {
        key: collectionAddress,
        verified: false,
    },
});

await transcation.sendAndConfirm(umi);

const createdNft = await fetchDigitalAsset(umi, mint.publicKey);

console.log(`Created NFT! Address is ${getExplorerLink("address", createdNft.mint.publicKey, "devnet")}`);