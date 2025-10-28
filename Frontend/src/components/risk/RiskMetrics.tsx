/** @format */

import { Card } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { AlertTriangle, AlertCircle, Info } from "lucide-react";
import { Badge } from "@/components/ui/badge";

// RiskMetrics component (your first component)
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
          <Card className='glass-card p-6'>
               <h3 className='text-lg font-semibold mb-4'>Risk Metrics</h3>
               <div className='space-y-4'>
                    {metrics.map((metric) => (
                         <div key={metric.label} className='space-y-2'>
                              <div className='flex justify-between text-sm'>
                                   <span className='text-muted-foreground'>
                                        {metric.label}
                                   </span>
                                   <span className='font-medium'>
                                        {metric.value}%
                                   </span>
                              </div>
                              <div className='relative'>
                                   <Progress
                                        value={metric.value}
                                        className='h-2'
                                   />
                                   <div
                                        className={`absolute top-0 left-0 h-2 rounded-full ${getColor(
                                             metric.status,
                                        )} transition-all`}
                                        style={{ width: `${metric.value}%` }}
                                   />
                              </div>
                         </div>
                    ))}
               </div>
          </Card>
     );
};

// RiskAlertCard component (your second component)
interface RiskAlertCardProps {
     severity: "high" | "medium" | "low";
     title: string;
     description: string;
     timestamp: string;
}

export const RiskAlertCard = ({
     severity,
     title,
     description,
     timestamp,
}: RiskAlertCardProps) => {
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
          <Card
               className={`glass-card p-4 border-l-4 ${
                    severity === "high"
                         ? "border-l-destructive"
                         : severity === "medium"
                         ? "border-l-warning"
                         : "border-l-primary"
               }`}>
               <div className='flex gap-3'>
                    <div
                         className={`${config.bg} ${config.color} p-2 rounded-lg h-fit`}>
                         <Icon className='w-5 h-5' />
                    </div>
                    <div className='flex-1'>
                         <div className='flex items-start justify-between mb-2'>
                              <h4 className='font-semibold'>{title}</h4>
                              <Badge
                                   variant={config.badge}
                                   className={
                                        severity === "medium"
                                             ? "bg-warning/20 text-warning"
                                             : ""
                                   }>
                                   {severity.toUpperCase()}
                              </Badge>
                         </div>
                         <p className='text-sm text-muted-foreground mb-2'>
                              {description}
                         </p>
                         <p className='text-xs text-muted-foreground'>
                              {timestamp}
                         </p>
                    </div>
               </div>
          </Card>
     );
};
