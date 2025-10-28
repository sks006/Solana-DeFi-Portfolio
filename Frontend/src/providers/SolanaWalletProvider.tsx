/** @format */

import { ReactNode, useMemo } from "react";
import {
     ConnectionProvider,
     WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import {
     PhantomWalletAdapter,
     SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { BackpackWalletAdapter } from "@solana/wallet-adapter-backpack";
const network = (import.meta.env.VITE_SOLANA_NETWORK ??
     "devnet") as WalletAdapterNetwork;
const endpoint =
     import.meta.env.VITE_SOLANA_RPC ?? "https://api.devnet.solana.com";

export default function SolanaWalletProvider({
     children,
}: {
     children: ReactNode;
}) {
     const wallets = useMemo(
          () => [
               new PhantomWalletAdapter(),
               new SolflareWalletAdapter(),
               new BackpackWalletAdapter(),
          ],
          [],
     );

     return (
          <ConnectionProvider endpoint={endpoint}>
               <WalletProvider wallets={wallets} autoConnect>
                    <WalletModalProvider>{children}</WalletModalProvider>
               </WalletProvider>
          </ConnectionProvider>
     );
}
