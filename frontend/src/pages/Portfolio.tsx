import { BalanceCard } from "@/components/portfolio/BalanceCard";
import { PositionCard } from "@/components/portfolio/PositionCard";
import { PortfolioChart } from "@/components/portfolio/PortfolioChart";

const positions = [
  { token: "Solana", symbol: "SOL", amount: 10.5432, value: "$2,895.12", change24h: 5.43 },
  { token: "USD Coin", symbol: "USDC", amount: 1234.56, value: "$1,234.56", change24h: 0.01 },
  { token: "Raydium", symbol: "RAY", amount: 125.89, value: "$445.32", change24h: -2.15 },
  { token: "Serum", symbol: "SRM", amount: 89.45, value: "$178.90", change24h: 3.67 },
];

const Portfolio = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Portfolio Overview</h1>
        <p className="text-muted-foreground">Track your Solana DeFi positions and performance</p>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <BalanceCard 
          title="Total Balance" 
          amount="$4,753.90" 
          change={125.43} 
          changePercent={2.71} 
        />
        <BalanceCard 
          title="24h P&L" 
          amount="$125.43" 
          change={125.43} 
          changePercent={2.71} 
        />
        <BalanceCard 
          title="Total Positions" 
          amount="4" 
          change={0} 
          changePercent={0} 
        />
      </div>
      
      <div className="mb-8">
        <PortfolioChart />
      </div>
      
      <div>
        <h2 className="text-xl font-semibold mb-4">Your Positions</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {positions.map((position) => (
            <PositionCard key={position.symbol} {...position} />
          ))}
        </div>
      </div>
    </div>
  );
};

export default Portfolio;
