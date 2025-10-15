import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

interface PositionCardProps {
  token: string;
  symbol: string;
  amount: number;
  value: string;
  change24h: number;
  icon?: string;
}

export const PositionCard = ({ token, symbol, amount, value, change24h }: PositionCardProps) => {
  const isPositive = change24h >= 0;
  
  return (
    <Card className="glass-card p-4 hover:border-primary/50 transition-all cursor-pointer">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-gradient-primary flex items-center justify-center">
            <span className="font-bold">{symbol.slice(0, 1)}</span>
          </div>
          <div>
            <h4 className="font-semibold">{token}</h4>
            <p className="text-sm text-muted-foreground">{symbol}</p>
          </div>
        </div>
        <Badge variant={isPositive ? "default" : "destructive"} className={isPositive ? "bg-success" : ""}>
          {isPositive ? "+" : ""}{change24h.toFixed(2)}%
        </Badge>
      </div>
      <div className="flex justify-between items-end">
        <div>
          <p className="text-xs text-muted-foreground">Amount</p>
          <p className="font-semibold">{amount.toFixed(4)} {symbol}</p>
        </div>
        <div className="text-right">
          <p className="text-xs text-muted-foreground">Value</p>
          <p className="font-semibold">{value}</p>
        </div>
      </div>
    </Card>
  );
};
