npx ts-node src/index.ts

https://wormholescan.io/#/tx/<ID>?network=Testnet&view=overview

# Solana to Avalanche

## Wormhole scan link
[Wormhole](https://wormholescan.io/#/tx/3M2Jd4iGTnvcPWP7PECn6dsRVWTCVBhiSE6S3A9NkCQS3DU18yYTdiB7PgH9zLesoMcuVxhaF3ojDLjgMx4mAmEV?network=Testnet&view=overview)

## Raw data
{
  sourceToken: {
    token: { chain: 'Solana', address: [SolanaAddress] },
    amount: 50000n
  },
  destinationToken: {
    token: { chain: 'Avalanche', address: [EvmAddress] },
    amount: 50000n
  },
  warnings: undefined,
  eta: 12805
}
Starting transfer
Signing: TokenBridge.TransferTokens for 6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5
[]
PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)] {
  _bn: <BN: 4a91bed434313d311d795434a998f11cb74c19e28e578768479afa19953add04>
}
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
Data:  0450c3000000000000
Keys:  [
  [
    {
      pubkey: [PublicKey [PublicKey(HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM)]],
      isSigner: false,
      isWritable: true
    },
    'HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(3VFdJkFuzrcwCwdxhKRETGxrDtUVAipNmYcLvRBDcQeH)]],
      isSigner: false,
      isWritable: false
    },
    '3VFdJkFuzrcwCwdxhKRETGxrDtUVAipNmYcLvRBDcQeH'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isSigner: true,
      isWritable: false
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ]
]
Program DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe
Data:  040000000050c300000000000000000000000000000000000000000000000000008ffcf7e7109416f2c34b6e226556a58fa68bd7790600
Keys:  [
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isWritable: true,
      isSigner: true
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(8PFZNjn19BBYVHNp4H31bEW7eAmu78Yf2RKV8EeA461K)]],
      isWritable: false,
      isSigner: false
    },
    '8PFZNjn19BBYVHNp4H31bEW7eAmu78Yf2RKV8EeA461K'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM)]],
      isWritable: true,
      isSigner: false
    },
    'HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isWritable: true,
      isSigner: true
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(GQtMXZxnuacCFTXVeTvyHi6P9F6chbtzhVc8JgD8hv7c)]],
      isWritable: true,
      isSigner: false
    },
    'GQtMXZxnuacCFTXVeTvyHi6P9F6chbtzhVc8JgD8hv7c'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(9a9vEiUq7yicBBLQjHpv7iviYwk2JJM1wquuQjZtb9D8)]],
      isWritable: false,
      isSigner: false
    },
    '9a9vEiUq7yicBBLQjHpv7iviYwk2JJM1wquuQjZtb9D8'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(3VFdJkFuzrcwCwdxhKRETGxrDtUVAipNmYcLvRBDcQeH)]],
      isWritable: false,
      isSigner: false
    },
    '3VFdJkFuzrcwCwdxhKRETGxrDtUVAipNmYcLvRBDcQeH'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(6bi4JGDoRwUs9TYBuvoA7dUVyikTJDrJsJU1ew6KVLiu)]],
      isWritable: true,
      isSigner: false
    },
    '6bi4JGDoRwUs9TYBuvoA7dUVyikTJDrJsJU1ew6KVLiu'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(DwtXQ5u4bczu5WWDdEstraFY1R3bntQufdHsVHgkCgdK)]],
      isWritable: true,
      isSigner: true
    },
    'DwtXQ5u4bczu5WWDdEstraFY1R3bntQufdHsVHgkCgdK'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(4yttKWzRoNYS2HekxDfcZYmfQqnVWpKiJ8eydYRuFRgs)]],
      isWritable: false,
      isSigner: false
    },
    '4yttKWzRoNYS2HekxDfcZYmfQqnVWpKiJ8eydYRuFRgs'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(9QzqZZvhxoHzXbNY9y2hyAUfJUzDwyDb7fbDs9RXwH3)]],
      isWritable: true,
      isSigner: false
    },
    '9QzqZZvhxoHzXbNY9y2hyAUfJUzDwyDb7fbDs9RXwH3'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(7s3a1ycs16d6SNDumaRtjcoyMaTDZPavzgsmS3uUZYWX)]],
      isWritable: true,
      isSigner: false
    },
    '7s3a1ycs16d6SNDumaRtjcoyMaTDZPavzgsmS3uUZYWX'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarC1ock11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarC1ock11111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarRent111111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarRent111111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    '11111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)]],
      isWritable: false,
      isSigner: false
    },
    'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5)]],
      isWritable: false,
      isSigner: false
    },
    '3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5'
  ]
]
Submitting transactions 
Waiting for confirmation for:  [
  '3M2Jd4iGTnvcPWP7PECn6dsRVWTCVBhiSE6S3A9NkCQS3DU18yYTdiB7PgH9zLesoMcuVxhaF3ojDLjgMx4mAmEV'
]
Started transfer:  [
  '3M2Jd4iGTnvcPWP7PECn6dsRVWTCVBhiSE6S3A9NkCQS3DU18yYTdiB7PgH9zLesoMcuVxhaF3ojDLjgMx4mAmEV'
]
Getting Attestation
Retrying Wormholescan:GetVaaBytes, attempt 0/30 
Retrying Wormholescan:GetVaaBytes, attempt 1/30 
Retrying Wormholescan:GetVaaBytes, attempt 2/30 
Retrying Wormholescan:GetVaaBytes, attempt 3/30 
Got Attestation:  [
  {
    chain: 'Solana',
    emitter: UniversalAddress { address: [Uint8Array] },
    sequence: 30041n
  }
]
Completing Transfer
Signing: TokenBridge.completeTransfer for 0x8ffcF7E7109416F2c34b6e226556a58Fa68BD779
Completed Transfer:  [
  '0x8441920401d38b034e8cc1e6dac14b4d52dde84f0de18cf7252ced9500a2ca06'
]
WaitLog: Current trasfer state:  DestinationFinalized
{
  from: 'Solana',
  to: 'Avalanche',
  state: 8,
  originTxs: [
    {
      chain: 'Solana',
      txid: '3M2Jd4iGTnvcPWP7PECn6dsRVWTCVBhiSE6S3A9NkCQS3DU18yYTdiB7PgH9zLesoMcuVxhaF3ojDLjgMx4mAmEV'
    }
  ],
  attestation: {
    id: { chain: 'Solana', emitter: [UniversalAddress], sequence: 30041n },
    attestation: {
      protocolName: 'TokenBridge',
      payloadName: 'Transfer',
      payloadLiteral: 'TokenBridge:Transfer',
      guardianSet: 0,
      signatures: [Array],
      timestamp: 1733631159,
      nonce: 0,
      emitterChain: 'Solana',
      emitterAddress: [UniversalAddress],
      sequence: 30041n,
      consistencyLevel: 32,
      payload: [Object],
      hash: [Uint8Array]
    }
  },
  destinationTxs: [
    {
      chain: 'Avalanche',
      txid: '0x8441920401d38b034e8cc1e6dac14b4d52dde84f0de18cf7252ced9500a2ca06'
    }
  ]
}

# Solana to Avalanche

## Wormhole scan link
[Wormhole](https://wormholescan.io/#/tx/oQvxJUKoYzLsFnnjod9zsvFCnCUNQuqkoQzVTpYpXAWYGpDS51GLKAK4tWLXwPYV2jvgA5inNziK5ma1MG2dk9b?network=Testnet&view=overview)

## Raw data

{
  sourceToken: {
    token: { chain: 'Avalanche', address: [EvmAddress] },
    amount: 50000n
  },
  destinationToken: {
    token: { chain: 'Solana', address: [SolanaAddress] },
    amount: 50000n
  },
  warnings: undefined,
  eta: 5
}
Starting transfer
Signing: TokenBridge.Approve for 0x8ffcF7E7109416F2c34b6e226556a58Fa68BD779
Signing: TokenBridge.transferTokens for 0x8ffcF7E7109416F2c34b6e226556a58Fa68BD779
Started transfer:  [
  '0x941ac5d107aa098ea8dddcc1fdd224334433dfb4585c0e6bb99277a25774e285',
  '0x62b1042fbb9e4e3593e1a0ed3cef6477d65da819deb0ebcb71b69907b1a0ddc0'
]
Getting Attestation
Got Attestation:  [
  {
    chain: 'Avalanche',
    emitter: UniversalAddress { address: [Uint8Array] },
    sequence: 18974n
  }
]
Completing Transfer
Signing: Core.VerifySignature for 6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5
[]
PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)] {
  _bn: <BN: 4a91bed434313d311d795434a998f11cb74c19e28e578768479afa19953add04>
}
Program KeccakSecp256k11111111111111111111111111111
Data:  010c00004d00006100200000920ea5a6c62635e8d23ef0fb9dfccb3629b630165a0cd75594805d76e3f7b2c72b574c4ea3a9bc4a0f46a24da13cad6ffcde2e934bd8c0215cb807e310b311470013947bd48b18e53fdaeee77f3473391ac727c638c071de0824df7f5a8c17390c90a0cb613de86981b0046694b4bdecb617d21ce5
Keys:  []
Program 3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5
Data:  0700ffffffffffffffffffffffffffffffffffff
Keys:  [
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isWritable: true,
      isSigner: true
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(dxZtypiKT5D9LYzdPxjvSZER9MgYfeRVU5qpMTMTRs4)]],
      isWritable: false,
      isSigner: false
    },
    'dxZtypiKT5D9LYzdPxjvSZER9MgYfeRVU5qpMTMTRs4'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(FSZUQ4EAr9tAnjpxz4RbBaEg6F4eZQSrCFfpsTZuJevG)]],
      isWritable: true,
      isSigner: true
    },
    'FSZUQ4EAr9tAnjpxz4RbBaEg6F4eZQSrCFfpsTZuJevG'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(Sysvar1nstructions1111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'Sysvar1nstructions1111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarRent111111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarRent111111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    '11111111111111111111111111111111'
  ]
]
Submitting transactions 
Waiting for confirmation for:  [
  '27HUxWYxBn7RNbYhYzv4CbafWvpyE1PJhMBLfeDLk1jCGEeGDHkpSKykSFmkKrvwfjkBtbWNxCtjgP2pLJpEDiio'
]
Signing: Core.PostVAA for 6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5
[]
PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)] {
  _bn: <BN: 4a91bed434313d311d795434a998f11cb74c19e28e578768479afa19953add04>
}
Program 3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5
Data:  020100000000651d556700000000060000000000000000000000000061e44e506ca5659e6c0bba9b678586fa2d7297561e4a000000000000018500000001000000000000000000000000000000000000000000000000000000000000c3500000000000000000000000005425890298aed601595a70ab815c96711a31bc650006f1d9493c8edf189c978c9970bb6fc7753c26f6d51f0e27a572d7ae68f7f84fb600010000000000000000000000000000000000000000000000000000000000000000
Keys:  [
  [
    {
      pubkey: [PublicKey [PublicKey(dxZtypiKT5D9LYzdPxjvSZER9MgYfeRVU5qpMTMTRs4)]],
      isWritable: false,
      isSigner: false
    },
    'dxZtypiKT5D9LYzdPxjvSZER9MgYfeRVU5qpMTMTRs4'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(6bi4JGDoRwUs9TYBuvoA7dUVyikTJDrJsJU1ew6KVLiu)]],
      isWritable: false,
      isSigner: false
    },
    '6bi4JGDoRwUs9TYBuvoA7dUVyikTJDrJsJU1ew6KVLiu'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(FSZUQ4EAr9tAnjpxz4RbBaEg6F4eZQSrCFfpsTZuJevG)]],
      isWritable: false,
      isSigner: false
    },
    'FSZUQ4EAr9tAnjpxz4RbBaEg6F4eZQSrCFfpsTZuJevG'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(9uoLda9eedGU7UTDjoEVEqCNmn1Pa4vjYQ3Fi5iatL5q)]],
      isWritable: true,
      isSigner: false
    },
    '9uoLda9eedGU7UTDjoEVEqCNmn1Pa4vjYQ3Fi5iatL5q'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isWritable: true,
      isSigner: true
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarC1ock11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarC1ock11111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarRent111111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarRent111111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    '11111111111111111111111111111111'
  ]
]
Submitting transactions 
Waiting for confirmation for:  [
  '4aNjvuVfMJ1exwXZ5Skij1UUxtVdHgbMNTcrWJD2tf3yZ5seJZghYqbqA8qQ8jxZfs6WW4fD3eRHkTVcyDK33THM'
]
Signing: Solana.RedeemTransfer for 6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5
[]
PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)] {
  _bn: <BN: 4a91bed434313d311d795434a998f11cb74c19e28e578768479afa19953add04>
}
Program DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe
Data:  03
Keys:  [
  [
    {
      pubkey: [PublicKey [PublicKey(6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5)]],
      isWritable: true,
      isSigner: true
    },
    '6264vVvtWg8CqBRegBt83ttcPPK61LurXNs7cqF56Gf5'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(8PFZNjn19BBYVHNp4H31bEW7eAmu78Yf2RKV8EeA461K)]],
      isWritable: false,
      isSigner: false
    },
    '8PFZNjn19BBYVHNp4H31bEW7eAmu78Yf2RKV8EeA461K'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(9uoLda9eedGU7UTDjoEVEqCNmn1Pa4vjYQ3Fi5iatL5q)]],
      isWritable: false,
      isSigner: false
    },
    '9uoLda9eedGU7UTDjoEVEqCNmn1Pa4vjYQ3Fi5iatL5q'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(3PuX9mb4eZenvvVLt6wDAEbBhgYy7MNmxXTGC87gjQ5m)]],
      isWritable: true,
      isSigner: false
    },
    '3PuX9mb4eZenvvVLt6wDAEbBhgYy7MNmxXTGC87gjQ5m'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(4boZev6ACP6j8yVWmKg4W8E5nUqTbDpdHdm2kMjSKpxE)]],
      isWritable: false,
      isSigner: false
    },
    '4boZev6ACP6j8yVWmKg4W8E5nUqTbDpdHdm2kMjSKpxE'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM)]],
      isWritable: true,
      isSigner: false
    },
    'HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM)]],
      isWritable: true,
      isSigner: false
    },
    'HH5RbaR3uthzjfMhu3iUKwAjwqJkdGxsfnLV9EaqfdNM'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(GQtMXZxnuacCFTXVeTvyHi6P9F6chbtzhVc8JgD8hv7c)]],
      isWritable: true,
      isSigner: false
    },
    'GQtMXZxnuacCFTXVeTvyHi6P9F6chbtzhVc8JgD8hv7c'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(9a9vEiUq7yicBBLQjHpv7iviYwk2JJM1wquuQjZtb9D8)]],
      isWritable: false,
      isSigner: false
    },
    '9a9vEiUq7yicBBLQjHpv7iviYwk2JJM1wquuQjZtb9D8'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(rRsXLHe7sBHdyKU3KY3wbcgWvoT1Ntqudf6e9PKusgb)]],
      isWritable: false,
      isSigner: false
    },
    'rRsXLHe7sBHdyKU3KY3wbcgWvoT1Ntqudf6e9PKusgb'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(SysvarRent111111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    'SysvarRent111111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(11111111111111111111111111111111)]],
      isWritable: false,
      isSigner: false
    },
    '11111111111111111111111111111111'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)]],
      isWritable: false,
      isSigner: false
    },
    'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
  ],
  [
    {
      pubkey: [PublicKey [PublicKey(3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5)]],
      isWritable: false,
      isSigner: false
    },
    '3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5'
  ]
]
Submitting transactions 
Waiting for confirmation for:  [
  'oQvxJUKoYzLsFnnjod9zsvFCnCUNQuqkoQzVTpYpXAWYGpDS51GLKAK4tWLXwPYV2jvgA5inNziK5ma1MG2dk9b'
]
Completed Transfer:  [
  '27HUxWYxBn7RNbYhYzv4CbafWvpyE1PJhMBLfeDLk1jCGEeGDHkpSKykSFmkKrvwfjkBtbWNxCtjgP2pLJpEDiio',
  '4aNjvuVfMJ1exwXZ5Skij1UUxtVdHgbMNTcrWJD2tf3yZ5seJZghYqbqA8qQ8jxZfs6WW4fD3eRHkTVcyDK33THM',
  'oQvxJUKoYzLsFnnjod9zsvFCnCUNQuqkoQzVTpYpXAWYGpDS51GLKAK4tWLXwPYV2jvgA5inNziK5ma1MG2dk9b'
]
WaitLog: Current trasfer state:  DestinationFinalized
{
  from: 'Avalanche',
  to: 'Solana',
  state: 8,
  originTxs: [
    {
      chain: 'Avalanche',
      txid: '0x941ac5d107aa098ea8dddcc1fdd224334433dfb4585c0e6bb99277a25774e285'
    },
    {
      chain: 'Avalanche',
      txid: '0x62b1042fbb9e4e3593e1a0ed3cef6477d65da819deb0ebcb71b69907b1a0ddc0'
    }
  ],
  attestation: {
    id: {
      chain: 'Avalanche',
      emitter: [UniversalAddress],
      sequence: 18974n
    },
    attestation: {
      protocolName: 'TokenBridge',
      payloadName: 'Transfer',
      payloadLiteral: 'TokenBridge:Transfer',
      guardianSet: 0,
      signatures: [Array],
      timestamp: 1733631333,
      nonce: 0,
      emitterChain: 'Avalanche',
      emitterAddress: [UniversalAddress],
      sequence: 18974n,
      consistencyLevel: 1,
      payload: [Object],
      hash: [Uint8Array]
    }
  },
  destinationTxs: [
    {
      chain: 'Solana',
      txid: '27HUxWYxBn7RNbYhYzv4CbafWvpyE1PJhMBLfeDLk1jCGEeGDHkpSKykSFmkKrvwfjkBtbWNxCtjgP2pLJpEDiio'
    },
    {
      chain: 'Solana',
      txid: '4aNjvuVfMJ1exwXZ5Skij1UUxtVdHgbMNTcrWJD2tf3yZ5seJZghYqbqA8qQ8jxZfs6WW4fD3eRHkTVcyDK33THM'
    },
    {
      chain: 'Solana',
      txid: 'oQvxJUKoYzLsFnnjod9zsvFCnCUNQuqkoQzVTpYpXAWYGpDS51GLKAK4tWLXwPYV2jvgA5inNziK5ma1MG2dk9b'
    }
  ]
}