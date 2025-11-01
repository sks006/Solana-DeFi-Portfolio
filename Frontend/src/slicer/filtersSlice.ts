import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type TimeRange = "24h" | "7d" | "30d" | "all";
export type Risk = "low" | "medium" | "high" | "all";

export interface FiltersState {
  search: string;
  brands: string[];       // example facet
  tokens: string[];       // e.g. SOL, USDC
  protocols: string[];    // AMM, Lending, etc.
  risk: Risk;
  timeRange: TimeRange;
}

const initialState: FiltersState = {
  search: "",
  brands: [],
  tokens: [],
  protocols: [],
  risk: "all",
  timeRange: "7d",
};

const filtersSlice = createSlice({
  name: "filters",
  initialState,
  reducers: {
    setSearch(state, action: PayloadAction<string>) { state.search = action.payload; },
    toggleBrand(state, { payload }: PayloadAction<string>) {
      const i = state.brands.indexOf(payload);
      if (i === -1) state.brands.push(payload); else state.brands.splice(i, 1);
    },
    toggleToken(state, { payload }: PayloadAction<string>) {
      const i = state.tokens.indexOf(payload);
      if (i === -1) state.tokens.push(payload); else state.tokens.splice(i, 1);
    },
    toggleProtocol(state, { payload }: PayloadAction<string>) {
      const i = state.protocols.indexOf(payload);
      if (i === -1) state.protocols.push(payload); else state.protocols.splice(i, 1);
    },
    setRisk(state, { payload }: PayloadAction<Risk>) { state.risk = payload; },
    setTimeRange(state, { payload }: PayloadAction<TimeRange>) { state.timeRange = payload; },
    resetFilters(state) {
      state.search = "";
      state.brands = [];
      state.tokens = [];
      state.protocols = [];
      state.risk = "all";
      state.timeRange = "7d";
    },
    setAllFilters(state, { payload }: PayloadAction<Partial<FiltersState>>) {
      Object.assign(state, payload);
    }
  }
});

export const { setSearch, toggleBrand, toggleToken, toggleProtocol, setRisk, setTimeRange, resetFilters, setAllFilters } = filtersSlice.actions;
export default filtersSlice.reducer;
