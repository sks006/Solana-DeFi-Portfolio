import { RiskAlertCard } from "@/components/risk/RiskAlertCard";
import { RiskMetrics } from "@/components/risk/RiskMetrics";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

const alerts = [
  {
    severity: "high" as const,
    title: "High Volatility Detected",
    description: "SOL price volatility has increased by 45% in the last hour. Consider reducing position size.",
    timestamp: "5 mins ago"
  },
  {
    severity: "medium" as const,
    title: "Concentration Risk Warning",
    description: "65% of your portfolio is allocated to SOL. Diversification recommended.",
    timestamp: "1 hour ago"
  },
  {
    severity: "low" as const,
    title: "Network Congestion",
    description: "Solana network experiencing higher than average transaction volume. Expect slight delays.",
    timestamp: "2 hours ago"
  },
  {
    severity: "medium" as const,
    title: "Price Impact Alert",
    description: "Large swap detected that may impact token price. Review slippage settings.",
    timestamp: "3 hours ago"
  },
];

const Risk = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Risk Management</h1>
        <p className="text-muted-foreground">Monitor portfolio risks and receive real-time alerts</p>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <Card className="glass-card p-6">
          <div className="flex items-center justify-between mb-2">
            <h3 className="font-semibold">Overall Risk Score</h3>
            <Badge variant="outline" className="bg-warning/20 text-warning">Medium</Badge>
          </div>
          <p className="text-4xl font-bold mb-1">42/100</p>
          <p className="text-sm text-muted-foreground">Lower is safer</p>
        </Card>
        
        <Card className="glass-card p-6">
          <div className="flex items-center justify-between mb-2">
            <h3 className="font-semibold">Active Alerts</h3>
            <Badge variant="destructive">4</Badge>
          </div>
          <p className="text-4xl font-bold mb-1">{alerts.length}</p>
          <p className="text-sm text-muted-foreground">Requires attention</p>
        </Card>
        
        <Card className="glass-card p-6">
          <div className="flex items-center justify-between mb-2">
            <h3 className="font-semibold">Portfolio Health</h3>
            <Badge className="bg-success">Good</Badge>
          </div>
          <p className="text-4xl font-bold mb-1">78%</p>
          <p className="text-sm text-muted-foreground">Well diversified</p>
        </Card>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 space-y-4">
          <h2 className="text-xl font-semibold">Active Alerts</h2>
          {alerts.map((alert, index) => (
            <RiskAlertCard key={index} {...alert} />
          ))}
        </div>
        
        <div>
          <RiskMetrics />
        </div>
      </div>
    </div>
  );
};

export default Risk;
