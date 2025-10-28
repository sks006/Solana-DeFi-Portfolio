/** @format */

import { useWallet } from "@solana/wallet-adapter-react";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { api } from "@/lib/api";
import { SwapForm } from "@/components/swap/SwapForm";
import { Card } from "@/components/ui/card";

export default function Swap() {
     const { publicKey, connected } = useWallet();
     const [inputMint, setInputMint] = useState("");
     const [outputMint, setOutputMint] = useState("");
     const [inAmount, setInAmount] = useState(0);

     const { data: quote, refetch } = useQuery({
          queryKey: ["quote", inputMint, outputMint, inAmount],
          queryFn: () =>
               api.swapQuote({
                    input_mint: inputMint,
                    output_mint: outputMint,
                    in_amount: inAmount,
               }),
          enabled: connected && !!inputMint && !!outputMint && inAmount > 0,
     });

     useEffect(() => {
          if (connected) refetch();
     }, [connected, refetch]);

     const exec = useMutation({
          mutationFn: () =>
               api.swapExecute({
                    wallet: publicKey!.toBase58(),
                    input_mint: inputMint,
                    output_mint: outputMint,
                    in_amount: inAmount,
                    slippage_bps: 50,
               }),
     });

     return (
          <div className='container mx-auto px-4 py-8 space-y-6'>
               <SwapForm
                    connected={connected}
                    quote={quote}
                    onChange={({ inMint, outMint, amount }) => {
                         setInputMint(inMint);
                         setOutputMint(outMint);
                         setInAmount(amount);
                    }}
                    onSubmit={() => exec.mutate()}
                    isSubmitting={exec.isPending}
                    txSig={(exec.data as any)?.signature}
                    error={exec.error ? String(exec.error) : undefined}
               />
               <Card className='p-4 text-sm text-muted-foreground'>
                    {connected
                         ? "Connected — quotes and execution use your wallet address."
                         : "Connect wallet to get live quotes."}
               </Card>
          </div>
     );
}
