import { encoding, signSendWait, wormhole } from '@wormhole-foundation/sdk';
import solana from '@wormhole-foundation/sdk/solana';
import evm from '@wormhole-foundation/sdk/evm';
import { getSigner } from './helpers/index';
(async function () {
  try {
    console.log("Initializing Wormhole instance...");
    const wh = await wormhole('Testnet', [solana, evm]);

    // Source Chain (Solana)
    console.log("Setting up Solana chain...");
    const solanaChain = wh.getChain('Solana');
    const { signer: solanaSigner, address: solanaAddress } = await getSigner(solanaChain);

    console.log(`Solana Signer Address: ${solanaAddress.address}`);

    // Target Chain (Avalanche)
    console.log("Setting up Avalanche chain...");
    const avalancheChain = wh.getChain('Avalanche');
    const { signer: avalancheSigner, address: avalancheAddress } = await getSigner(avalancheChain);

    console.log(`Avalanche Signer Address: ${avalancheAddress.address}`);

    // Get the core messaging bridge for Solana
    console.log("Retrieving core messaging bridge for Solana...");
    const coreBridgeSolana = await solanaChain.getWormholeCore();

    // Prepare the message
    console.log("Preparing the message...");
    const messagePayload = encoding.bytes.encode("Hello from Solana to Avalanche!");
    const publishTxs = coreBridgeSolana.publishMessage(
      solanaAddress.address, // Sender's address (emitter in VAA)
      messagePayload,        // Message payload
      42,                    // Nonce (example value for uniqueness)
      0                      // Consistency level (0 for low, 1 for higher finality)
    );

    console.log("Signing and sending the message from Solana...");
    const txids = await signSendWait(solanaChain, publishTxs, solanaSigner);
    console.log(`Transaction IDs from Solana: ${txids.map(tx => tx.txid).join(", ")}`);
    console.log(`Link for solana explorer: https://explorer.solana.com/tx/${txids.map(tx => tx.txid).join(", ")}?cluster=devnet`)

    // Take the last txid to retrieve the Wormhole message ID
    const txid = txids[txids.length - 1];
    console.log(`Last Solana Transaction ID: ${txid.txid}`);

    // Parse the transaction to get the Wormhole message ID (whm)
    console.log("Parsing the transaction to retrieve the Wormhole Message ID...");
    const [wormholeMessageId] = await solanaChain.parseTransaction(txid.txid);
    console.log(`Wormhole Message ID: ${wormholeMessageId}`);

    // Wait for the VAA to be signed by the guardians
    console.log("Waiting for the VAA to be signed...");
    const vaa = await wh.getVaa(wormholeMessageId, 'Uint8Array', 60_000);
    console.log("Received VAA:");
    console.log(vaa);

    // Submit the VAA to the Avalanche chain
    console.log("Submitting the VAA to Avalanche...");
    const coreBridgeAvalanche = await avalancheChain.getWormholeCore();
    const verifyTxs = coreBridgeAvalanche.verifyMessage(avalancheAddress.address, vaa!);
    const avalancheTxids = await signSendWait(avalancheChain, verifyTxs, avalancheSigner);

    console.log(`Transaction IDs from Avalanche: ${avalancheTxids.map(tx => tx.txid).join(", ")}`);
    console.log(`Search at avalanche: https://testnet.avascan.info/blockchain/c/tx/${avalancheTxids.map(tx => tx.txid).join(", ")}`)

    console.log("Message successfully sent from Solana to Avalanche and verified on Avalanche!");

  } catch (error) {
    console.error("An error occurred:", error);
  }
})();
