/** @format */

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export function usePortfolio(wallet?: string) {
     return useQuery({
          queryKey: ["portfolio", wallet],
          queryFn: () => api.portfolioByWallet(wallet!),
          enabled: !!wallet,
          refetchOnWindowFocus: false,
     });
}
