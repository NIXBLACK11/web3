import { applyChainsConfigConfigOverrides } from "@wormhole-foundation/sdk-connect";
import * as _solana from "@wormhole-foundation/sdk-solana";
import { PlatformDefinition } from "@wormhole-foundation/sdk/dist/cjs";
/** Platform and protocol definitons for Solana */
const solana: PlatformDefinition<typeof _solana._platform> = {
  Address: _solana.SolanaAddress,
  Platform: _solana.SolanaPlatform,
  getSigner: _solana.getSolanaSignAndSendSigner,
  protocols: {
    WormholeCore: () => import("@wormhole-foundation/sdk-solana-core"),
    TokenBridge: () => import("@wormhole-foundation/sdk-solana-tokenbridge"),
    CircleBridge: () => import("@wormhole-foundation/sdk-solana-cctp"),
  },
  getChain: (network: any, chain: any, overrides?: any) =>
    new _solana.SolanaChain(
      chain,
      new _solana.SolanaPlatform(
        network,
        applyChainsConfigConfigOverrides(network, _solana._platform, { [chain]: overrides }),
      ),
    ),
};
export default solana;