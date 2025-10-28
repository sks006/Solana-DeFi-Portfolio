/** @format */

// src/components/risk/AlertList.tsx
import { Info } from "lucide-react";
import { Card } from "../ui/card";
import { RiskAlertCard } from "./RiskMetrics";
import { Badge } from "../ui/badge";

interface AlertItem {
     id: string;
     type: string;
     title: string;
     message: string;
     timestamp: number;
     severity: "high" | "medium" | "low";
}

interface AlertListProps {
     alerts: AlertItem[];
}

export function AlertList({ alerts }: AlertListProps) {
     if (alerts.length === 0) {
          return (
               <Card className='glass-card p-8 text-center'>
                    <div className='text-muted-foreground'>
                         <Info className='w-12 h-12 mx-auto mb-4 opacity-50' />
                         <h3 className='text-lg font-medium mb-2'>
                              No Risk Alerts
                         </h3>
                         <p className='text-sm'>
                              All systems are operating normally
                         </p>
                    </div>
               </Card>
          );
     }

     return (
          <div className='space-y-4'>
               <div className='flex items-center justify-between'>
                    <h2 className='text-2xl font-bold'>Risk Alerts</h2>
                    <Badge variant='outline' className='px-3 py-1'>
                         {alerts.length} Active
                    </Badge>
               </div>

               <div className='space-y-4 max-h-[600px] overflow-y-auto'>
                    {alerts.map((alert) => (
                         <RiskAlertCard
                              key={alert.id}
                              severity={alert.severity}
                              title={alert.title}
                              description={alert.message}
                              timestamp={new Date(
                                   alert.timestamp,
                              ).toLocaleString()}
                         />
                    ))}
               </div>
          </div>
     );
}
