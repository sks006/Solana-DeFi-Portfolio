/** @format */

import { useEffect, useState } from "react";
import { useRiskAlerts } from "@/hooks/useRiskAlerts";
import { openRiskWS } from "@/lib/websocket";
import { AlertList } from "@/components/risk/AlertList";

export default function Risk() {
     const { data } = useRiskAlerts();
     // eslint-disable-next-line @typescript-eslint/no-explicit-any
     const [live, setLive] = useState<any[]>([]);

     useEffect(() => {
          const ws = openRiskWS((evt) => {
               if (evt?.type === "risk_alert")
                    setLive((prev) => [evt, ...prev].slice(0, 20));
          });
          return () => ws.close();
     }, []);

     // eslint-disable-next-line @typescript-eslint/no-explicit-any
     const alerts = [...(live ?? []), ...((data?.alerts ?? []) as any[])];

     return <AlertList alerts={alerts} />;
}
