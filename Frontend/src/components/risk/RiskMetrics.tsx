import { Card } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";

interface RiskMetric {
  label: string;
  value: number;
  max: number;
  status: "safe" | "warning" | "danger";
}

const metrics: RiskMetric[] = [
  { label: "Concentration Risk", value: 35, max: 100, status: "safe" },
  { label: "Volatility Exposure", value: 68, max: 100, status: "warning" },
  { label: "Liquidity Risk", value: 22, max: 100, status: "safe" },
  { label: "Smart Contract Risk", value: 15, max: 100, status: "safe" },
];

export const RiskMetrics = () => {
  const getColor = (status: string) => {
    switch (status) {
      case "danger":
        return "bg-destructive";
      case "warning":
        return "bg-warning";
      default:
        return "bg-success";
    }
  };

  return (
    <Card className="glass-card p-6">
      <h3 className="text-lg font-semibold mb-4">Risk Metrics</h3>
      <div className="space-y-4">
        {metrics.map((metric) => (
          <div key={metric.label} className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">{metric.label}</span>
              <span className="font-medium">{metric.value}%</span>
            </div>
            <div className="relative">
              <Progress value={metric.value} className="h-2" />
              <div 
                className={`absolute top-0 left-0 h-2 rounded-full ${getColor(metric.status)} transition-all`}
                style={{ width: `${metric.value}%` }}
              />
            </div>
          </div>
        ))}
      </div>
    </Card>
  );
};
