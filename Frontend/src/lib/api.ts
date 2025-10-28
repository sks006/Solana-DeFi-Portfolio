/** @format */

const API = import.meta.env.VITE_API_BASE;

async function j(res: Response) {
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json();
}

export const api = {
  portfolioByWallet: (wallet: string) =>
    j(fetch(`${API}/api/portfolio/${wallet}`)),
  swapQuote: (body: {
    input_mint: string;
    output_mint: string;
    in_amount: number;
  }) =>
    j(
      fetch(`${API}/api/swap/quote`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      }),
    ),
  swapExecute: (body: {
    wallet: string;
    input_mint: string;
    output_mint: string;
    in_amount: number;
    slippage_bps: number;
  }) =>
    j(
      fetch(`${API}/api/swap/execute`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      }),
    ),
  riskAlerts: () => j(fetch(`${API}/api/risk/alerts`)),
};
