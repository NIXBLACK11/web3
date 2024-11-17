import '@dialectlabs/blinks/index.css';
import { useState, useEffect } from 'react';
import { Action, Blink, ActionsRegistry, useAction } from "@dialectlabs/blinks";
import { useActionSolanaWalletAdapter } from "@dialectlabs/blinks/hooks/solana"

// needs to be wrapped with <WalletProvider /> and <WalletModalProvider />
export const BlinkComp = () => {
  const [localAction, setLocalAction] = useState<Action | null>(null);
  // const actionApiUrl = 'https://solabule-nix.vercel.app/'; // Replace with your actual API URL
  // useAction initiates registry, adapter and fetches the action.
  const { adapter } = useActionSolanaWalletAdapter('https://api.devnet.solana.com');
  const { action: fetchedAction } = useAction({url: actionApiUrl, adapter});    

  useEffect(() => {
    if (fetchedAction) {
      setLocalAction(fetchedAction);
    }
  }, [fetchedAction]);

  return localAction ? <Blink action={localAction} websiteText={new URL(actionApiUrl).hostname} /> : null;
}