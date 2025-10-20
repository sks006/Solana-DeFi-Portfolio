# 📂 Total Project Tree — Solana DeFi Portfolio + Swap Simulator with Risk Alerts with Ai.

solana-defi-portfolio/

│

├── Cargo.toml                  # Root workspace config (Rust crates)

├── Makefile                    # Build/test/deploy commands

├── README.md                   # Project overview

├── LICENSE

├── Dockerfile

├── docker-compose.yml

├── .gitignore

│
├── frontend/                   # React + shadcn/ui + Vite

│   ├── index.html

│   ├── package.json

│   ├── tsconfig.json

│   ├── vite.config.ts

│   ├── postcss.config.js

│   ├── tailwind.config.js      # used internally by shadcn/ui

│   ├── public/

│   │   ├── icons/

│   │   └── images/

│   ├── src/

│   │   ├── main.tsx            # entry point

│   │   ├── App.tsx             # layout shell

│   │   ├── routes/             # Portfolio.tsx, Swap.tsx, Risk.tsx

│   │   ├── components/

│   │   │   ├── ui/             # shadcn components (Button, Card,
Dialog, etc.)

│   │   │   ├── portfolio/      # BalanceTable, PositionCard, PnLChart

│   │   │   ├── swap/           # SwapForm, SlippageCalc

│   │   │   └── risk/           # RiskPanel, AlertList

│   │   ├── lib/                # api.ts, solana.ts, websocket.ts

│   │   ├── hooks/              # useWallet, usePortfolio, useSwapSim,
useRiskAlerts

│   │   ├── styles/             # globals.css, theme.css

│   │   └── __tests__/          # Unit tests

│   └── tests/                  # Playwright e2e tests

│

├── backend/                    # Leptos + Rust API + WebSocket

│   ├── Cargo.toml

│   ├── src/

│   │   ├── main.rs

│   │   ├── server_functions/   # portfolio.rs, swap.rs, risk.rs

│   │   ├── ws/                 # manager.rs, messages.rs

│   │   ├── services/           # solana_client.rs, amm_math.rs,
risk_service.rs, ai_alerts.rs

│   │   ├── models/             # Portfolio, SwapQuote, RiskAlert

│   │   ├── database/           # connection.rs, migrations/

│   │   └── utils/              # error.rs, validation.rs, math.rs

│   ├── config/                 # default.toml, dev.toml,
production.toml

│   ├── tests/

│   │   ├── unit/

│   │   ├── integration/

│   │   └── load/

│   └── benches/                # Performance benchmarks

├── programs/                         # Solana on-chain logic
│   ├── Cargo.toml                    # Workspace config
│   │
│   ├── amm-pool/                     # AMM swap logic
│   │   ├── Cargo.toml
│   │   ├── Anchor.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── pool_state.rs
│   │   │   ├── swap_math.rs
│   │   │   ├── fees.rs
│   │   │   └── context/
│   │   │       ├── initialize_pool.rs
│   │   │       └── execute_swap.rs
│   │   └── tests/
│   │       └── swap_test.rs
│   │
│   └── portfolio-program/           # Position tracking
│       ├── Cargo.toml
│       ├── Anchor.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── state.rs
│       │   ├── processor.rs
│       │   ├── types.rs
│       │   └── context/
│       │       ├── update_position.rs
│       │       └── record_trade.rs
│       └── tests/
│           └── position_test.rs

│

├── ai/                         # AI risk alert logic

│   ├── model/

│   │   ├── train.py            # ML model training (optional)

│   │   ├── predict.py          # Inference logic

│   │   └── risk_model.pkl      # Saved model

│   ├── data/

│   │   └── sample_trades.csv   # Historical trade data

│   └── README.md               # AI module overview

│

├── docs/                       # Showcase-ready documentation

│   ├── README_DEMO.md          # Quickstart for judges

│   ├── architecture.md

│   ├── architecture.png

│   ├── features.md

│   ├── api.md

│   └── usage.md

│

└── .github/                    # workflows




