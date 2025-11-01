/** @format */

import { createRoot } from "react-dom/client";
import { Provider } from "react-redux";
import { store } from "@/slicer/store";
import App from "./App";
import "./index.css";

import "@solana/wallet-adapter-react-ui/styles.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import SolanaWalletProvider from "./providers/SolanaWalletProvider";

const qc = new QueryClient();

createRoot(document.getElementById("root")!).render(
     <Provider store={store}>
          <SolanaWalletProvider>
               <QueryClientProvider client={qc}>
                    <App />
               </QueryClientProvider>
          </SolanaWalletProvider>
     </Provider>,
);
