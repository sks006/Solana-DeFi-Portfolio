import { TrendingUp, TrendingDown } from "lucide-react";
import { Card } from "@/components/ui/card";

interface BalanceCardProps {
  title: string;
  amount: string;
  change: number;
  changePercent: number;
}

export const BalanceCard = ({ title, amount, change, changePercent }: BalanceCardProps) => {
  const isPositive = change >= 0;
  
  return (
    <Card className="glass-card p-6">
      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">{title}</p>
        <h3 className="text-3xl font-bold">{amount}</h3>
        <div className={cn(
          "flex items-center gap-1 text-sm font-medium",
          isPositive ? "text-success" : "text-destructive"
        )}>
          {isPositive ? (
            <TrendingUp className="w-4 h-4" />
          ) : (
            <TrendingDown className="w-4 h-4" />
          )}
          <span>{isPositive ? "+" : ""}{change.toFixed(2)} SOL</span>
          <span>({isPositive ? "+" : ""}{changePercent.toFixed(2)}%)</span>
        </div>
      </div>
    </Card>
  );
};

function cn(...classes: (string | boolean | undefined)[]) {
  return classes.filter(Boolean).join(' ');
}
