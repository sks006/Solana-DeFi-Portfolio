/** @format */

import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export const Header = () => {
     return (
          <header className='border-b border-border/50 bg-card/30 backdrop-blur-xl sticky top-0 z-50'>
               <div className='container mx-auto px-4 h-16 flex items-center justify-between'>
                    <div className='flex items-center gap-2'>
                         <div className='w-8 h-8 rounded-lg bg-gradient-primary flex items-center justify-center'>
                              <span className='text-lg font-bold'>S</span>
                         </div>
                         <span className='text-xl font-bold bg-gradient-primary bg-clip-text text-transparent'>
                              Solana DeFi
                         </span>
                    </div>
                    <WalletMultiButton className='rounded-xl' />
               </div>
          </header>
     );
};
