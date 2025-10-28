import { useDispatch, useSelector } from "react-redux";
import { RootState } from "@/slicer/store";
import { setSearch, setRisk, setTimeRange, toggleBrand, toggleProtocol, toggleToken, resetFilters } from "@/slicer/filtersSlice";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Separator } from "@/components/ui/separator";
import { X } from "lucide-react";
import { useMemo } from "react";

type Props = {
  availableTokens?: string[];
  availableProtocols?: string[];
  availableBrands?: string[];
};

export default function SlicerBar({ availableTokens = [], availableProtocols = [], availableBrands = [] }: Props) {
  const dispatch = useDispatch();
  const filters = useSelector((s: RootState) => s.filters);

  const tokenItems = useMemo(()=> Array.from(new Set(availableTokens)), [availableTokens]);
  const protocolItems = useMemo(()=> Array.from(new Set(availableProtocols)), [availableProtocols]);
  const brandItems = useMemo(()=> Array.from(new Set(availableBrands)), [availableBrands]);

  return (
    <div className="w-full rounded-2xl border bg-background/60 backdrop-blur px-4 py-3 shadow-sm">
      <div className="flex flex-col gap-3 md:flex-row md:items-center md:gap-4">
        <div className="flex-1">
          <Input
            placeholder="Search…"
            value={filters.search}
            onChange={(e)=> dispatch(setSearch(e.target.value))}
          />
        </div>

        <Select value={filters.timeRange} onValueChange={(v:any)=> dispatch(setTimeRange(v))}>
          <SelectTrigger className="w-[140px]">
            <SelectValue placeholder="Range" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="24h">24h</SelectItem>
            <SelectItem value="7d">7d</SelectItem>
            <SelectItem value="30d">30d</SelectItem>
            <SelectItem value="all">All</SelectItem>
          </SelectContent>
        </Select>

        <Select value={filters.risk} onValueChange={(v:any)=> dispatch(setRisk(v))}>
          <SelectTrigger className="w-[140px]">
            <SelectValue placeholder="Risk" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All risk</SelectItem>
            <SelectItem value="low">Low</SelectItem>
            <SelectItem value="medium">Medium</SelectItem>
            <SelectItem value="high">High</SelectItem>
          </SelectContent>
        </Select>

        <Separator orientation="vertical" className="hidden md:block h-6" />

        <Popover>
          <PopoverTrigger asChild>
            <Button variant="outline">Tokens ({filters.tokens.length})</Button>
          </PopoverTrigger>
          <PopoverContent className="w-64">
            <div className="flex flex-wrap gap-2">
              {tokenItems.map(tok => {
                const active = filters.tokens.includes(tok);
                return (
                  <Badge key={tok} variant={active ? "default":"secondary"} className="cursor-pointer"
                    onClick={()=> dispatch(toggleToken(tok))}
                  >
                    {tok}
                    {active && <X className="ml-1 h-3 w-3" />}
                  </Badge>
                );
              })}
            </div>
          </PopoverContent>
        </Popover>

        <Popover>
          <PopoverTrigger asChild>
            <Button variant="outline">Protocols ({filters.protocols.length})</Button>
          </PopoverTrigger>
          <PopoverContent className="w-64">
            <div className="flex flex-wrap gap-2">
              {protocolItems.map(p => {
                const active = filters.protocols.includes(p);
                return (
                  <Badge key={p} variant={active ? "default":"secondary"} className="cursor-pointer"
                    onClick={()=> dispatch(toggleProtocol(p))}
                  >
                    {p}
                    {active && <X className="ml-1 h-3 w-3" />}
                  </Badge>
                );
              })}
            </div>
          </PopoverContent>
        </Popover>

        <Popover>
          <PopoverTrigger asChild>
            <Button variant="outline">Brands ({filters.brands.length})</Button>
          </PopoverTrigger>
          <PopoverContent className="w-64">
            <div className="flex flex-wrap gap-2">
              {brandItems.map(b => {
                const active = filters.brands.includes(b);
                return (
                  <Badge key={b} variant={active ? "default":"secondary"} className="cursor-pointer"
                    onClick={()=> dispatch(toggleBrand(b))}
                  >
                    {b}
                    {active && <X className="ml-1 h-3 w-3" />}
                  </Badge>
                );
              })}
            </div>
          </PopoverContent>
        </Popover>

        <Button variant="ghost" onClick={()=> dispatch(resetFilters())}>Reset</Button>
      </div>
    </div>
  );
}
