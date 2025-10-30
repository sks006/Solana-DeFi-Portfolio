/** @format */

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export function useRiskAlerts() {
     return useQuery({
          queryKey: ["risk-alerts"],
          queryFn: api.riskAlerts,
          refetchInterval: 15000,
     });
}
