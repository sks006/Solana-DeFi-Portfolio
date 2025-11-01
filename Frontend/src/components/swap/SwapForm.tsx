import { useState } from "react";
import { ArrowDownUp } from "lucide-react";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";

export const SwapForm = () => {
  const [fromAmount, setFromAmount] = useState("");
  const [toAmount, setToAmount] = useState("");
  const [slippage, setSlippage] = useState(0.5);
  
  const estimatedOutput = fromAmount ? (parseFloat(fromAmount) * 0.98).toFixed(6) : "0";
  
  return (
    <Card className="glass-card p-6">
      <h3 className="text-lg font-semibold mb-4">Swap Tokens</h3>
      
      <div className="space-y-4">
        {/* From Token */}
        <div className="space-y-2">
          <Label>From</Label>
          <div className="flex gap-2">
            <Button variant="outline" className="w-32">
              SOL
            </Button>
            <Input 
              type="number" 
              placeholder="0.00"
              value={fromAmount}
              onChange={(e) => setFromAmount(e.target.value)}
              className="flex-1"
            />
          </div>
          <p className="text-xs text-muted-foreground">Balance: 10.5432 SOL</p>
        </div>
        
        {/* Swap Direction */}
        <div className="flex justify-center">
          <Button variant="ghost" size="icon" className="rounded-full">
            <ArrowDownUp className="w-4 h-4" />
          </Button>
        </div>
        
        {/* To Token */}
        <div className="space-y-2">
          <Label>To</Label>
          <div className="flex gap-2">
            <Button variant="outline" className="w-32">
              USDC
            </Button>
            <Input 
              type="number" 
              placeholder="0.00"
              value={estimatedOutput}
              readOnly
              className="flex-1"
            />
          </div>
          <p className="text-xs text-muted-foreground">Balance: 1,234.56 USDC</p>
        </div>
        
        {/* Slippage */}
        <div className="flex items-center justify-between p-3 rounded-lg bg-secondary/50">
          <span className="text-sm">Slippage Tolerance</span>
          <div className="flex gap-2">
            {[0.1, 0.5, 1.0].map((value) => (
              <Button
                key={value}
                variant={slippage === value ? "default" : "outline"}
                size="sm"
                onClick={() => setSlippage(value)}
              >
                {value}%
              </Button>
            ))}
          </div>
        </div>
        
        {/* Swap Details */}
        {fromAmount && (
          <div className="space-y-2 p-3 rounded-lg bg-secondary/50 text-sm">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Rate</span>
              <span>1 SOL = 98 USDC</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Price Impact</span>
              <Badge variant="outline" className="bg-success/20 text-success">
                0.12%
              </Badge>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Network Fee</span>
              <span>0.000005 SOL</span>
            </div>
          </div>
        )}
        
        <Button className="w-full bg-gradient-primary hover:opacity-90 transition-opacity">
          Simulate Swap
        </Button>
      </div>
    </Card>
  );
};
