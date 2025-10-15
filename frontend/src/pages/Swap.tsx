import { SwapForm } from "@/components/swap/SwapForm";
import { Card } from "@/components/ui/card";
import { TrendingUp, Activity, DollarSign } from "lucide-react";

const recentSwaps = [
  { from: "SOL", to: "USDC", amount: "2.5", value: "$245.00", time: "2 mins ago" },
  { from: "USDC", to: "RAY", amount: "100", value: "$100.00", time: "15 mins ago" },
  { from: "RAY", to: "SOL", amount: "50", value: "$87.50", time: "1 hour ago" },
];

const Swap = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Swap Simulator</h1>
        <p className="text-muted-foreground">Simulate token swaps with real-time pricing and slippage calculation</p>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <Card className="glass-card p-6">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 rounded-lg bg-primary/20">
              <TrendingUp className="w-5 h-5 text-primary" />
            </div>
            <h3 className="font-semibold">Best Rate</h3>
          </div>
          <p className="text-2xl font-bold">1 SOL = 98 USDC</p>
          <p className="text-sm text-muted-foreground mt-1">Live market rate</p>
        </Card>
        
        <Card className="glass-card p-6">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 rounded-lg bg-accent/20">
              <Activity className="w-5 h-5 text-accent" />
            </div>
            <h3 className="font-semibold">24h Volume</h3>
          </div>
          <p className="text-2xl font-bold">$2.4M</p>
          <p className="text-sm text-success mt-1">+12.5% from yesterday</p>
        </Card>
        
        <Card className="glass-card p-6">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 rounded-lg bg-warning/20">
              <DollarSign className="w-5 h-5 text-warning" />
            </div>
            <h3 className="font-semibold">Avg Slippage</h3>
          </div>
          <p className="text-2xl font-bold">0.12%</p>
          <p className="text-sm text-muted-foreground mt-1">Based on last 100 swaps</p>
        </Card>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <SwapForm />
        
        <Card className="glass-card p-6">
          <h3 className="text-lg font-semibold mb-4">Recent Simulations</h3>
          <div className="space-y-3">
            {recentSwaps.map((swap, index) => (
              <div key={index} className="flex items-center justify-between p-3 rounded-lg bg-secondary/50">
                <div className="flex items-center gap-3">
                  <div className="flex items-center gap-1">
                    <span className="font-semibold">{swap.from}</span>
                    <span className="text-muted-foreground">→</span>
                    <span className="font-semibold">{swap.to}</span>
                  </div>
                  <span className="text-sm text-muted-foreground">{swap.amount}</span>
                </div>
                <div className="text-right">
                  <p className="font-semibold">{swap.value}</p>
                  <p className="text-xs text-muted-foreground">{swap.time}</p>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </div>
    </div>
  );
};

export default Swap;
