/** @format */

import { useWallet } from "@solana/wallet-adapter-react";
import { usePortfolio } from "@/hooks/usePortfolio";
import { BalanceCard } from "@/components/portfolio/BalanceCard";
import { PositionCard } from "@/components/portfolio/PositionCard";
import { PortfolioChart } from "@/components/portfolio/PortfolioChart";

export default function Portfolio() {
     const { publicKey, connected } = useWallet();
     const wallet = publicKey?.toBase58();
     const { data, isLoading, error } = usePortfolio(wallet);

     if (!connected)
          return (
               <div className='container mx-auto px-4 py-8'>
                    Connect your wallet to view your portfolio.
               </div>
          );
     if (isLoading)
          return <div className='container mx-auto px-4 py-8'>Loading…</div>;
     if (error)
          return (
               <div className='container mx-auto px-4 py-8 text-red-600'>
                    Failed to load portfolio.
               </div>
          );

     const positions = data?.positions ?? [];
     const total = data?.total_value ?? 0;

     return (
          <div className='container mx-auto px-4 py-8 space-y-8'>
               <BalanceCard total={total} />
               <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
                    <PortfolioChart series={data?.pnl_series ?? []} />
                    <div className='space-y-4'>
                         {positions.map((p: any, i: number) => (
                              <PositionCard key={i} position={p} />
                         ))}
                    </div>
               </div>
          </div>
     );
}
