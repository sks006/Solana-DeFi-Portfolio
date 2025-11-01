import { AlertTriangle, AlertCircle, Info } from "lucide-react";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

interface RiskAlertCardProps {
  severity: "high" | "medium" | "low";
  title: string;
  description: string;
  timestamp: string;
}

export const RiskAlertCard = ({ severity, title, description, timestamp }: RiskAlertCardProps) => {
  const severityConfig = {
    high: {
      icon: AlertTriangle,
      color: "text-destructive",
      bg: "bg-destructive/10",
      badge: "destructive" as const,
    },
    medium: {
      icon: AlertCircle,
      color: "text-warning",
      bg: "bg-warning/10",
      badge: "outline" as const,
    },
    low: {
      icon: Info,
      color: "text-primary",
      bg: "bg-primary/10",
      badge: "outline" as const,
    },
  };
  
  const config = severityConfig[severity];
  const Icon = config.icon;
  
  return (
    <Card className={`glass-card p-4 border-l-4 ${severity === 'high' ? 'border-l-destructive' : severity === 'medium' ? 'border-l-warning' : 'border-l-primary'}`}>
      <div className="flex gap-3">
        <div className={`${config.bg} ${config.color} p-2 rounded-lg h-fit`}>
          <Icon className="w-5 h-5" />
        </div>
        <div className="flex-1">
          <div className="flex items-start justify-between mb-2">
            <h4 className="font-semibold">{title}</h4>
            <Badge variant={config.badge} className={severity === 'medium' ? 'bg-warning/20 text-warning' : ''}>
              {severity.toUpperCase()}
            </Badge>
          </div>
          <p className="text-sm text-muted-foreground mb-2">{description}</p>
          <p className="text-xs text-muted-foreground">{timestamp}</p>
        </div>
      </div>
    </Card>
  );
};
