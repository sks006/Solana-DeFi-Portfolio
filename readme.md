# solana-defi-portfolio


├── Cargo.toml                   Root workspace config (Rust crates)

├── Makefile                    # Build/test/deploy commands

├── README.md                   # Project overview

├── LICENSE

├── Dockerfile

├── docker-compose.yml

├── .gitignore

│

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

│

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

├── scripts/                    # Automation scripts

│   ├── 01_build.sh

│   ├── 02_deploy.sh

│   ├── 03_test.sh

│   ├── cleanup.sh

│   └── solana/

│       ├── deploy_programs.sh

│       ├── airdrop.sh

│       └── test_validator.sh

│

├── docs/                       # Showcase-ready documentation

│   ├── README_DEMO.md          # Quickstart for judges

│   ├── architecture.md

│   ├── architecture.png

│   ├── features.md

│   ├── api.md

│   └── usage.md

│

└── .github/                    # CI/CD workflows

├── workflows/

│   ├── ci.yml              # lint + test + build

│   ├── deploy.yml          # deploy to devnet

│   └── security.yml        # cargo audit, npm audit

└── dependabot.yml

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

programs/

│

├── amm-pool/                          # AMM swap logic

│   ├── Cargo.toml

│   ├── Anchor.toml

│   ├── src/

│   │   ├── lib.rs                     # Entry point: initialize_pool,
execute_swap

│   │   ├── pool_state.rs             # Pool struct + PoolParams

│   │   ├── swap_math.rs              # Constant product swap logic

│   │   ├── fees.rs                   # Optional: fee tracking

│   │   ├── context/

│   │   │   ├── initialize_pool.rs    # #[derive(Accounts)] for pool
init

│   │   │   └── execute_swap.rs       # #[derive(Accounts)] for swap

│   └── tests/

│       └── swap_test.rs              # Anchor test: pool init + swap
execution

│

├── portfolio-program/                # Portfolio tracking logic

│   ├── Cargo.toml

│   ├── Anchor.toml

│   ├── src/

│   │   ├── lib.rs                    # Entry point: update_position,
record_trade

│   │   ├── state.rs                 # Position struct

│   │   ├── processor.rs             # Logic for position updates and
trade recording

│   │   ├── context/

│   │   │   ├── update_position.rs   # #[derive(Accounts)] for
position update

│   │   │   └── record_trade.rs      # #[derive(Accounts)] for trade
recording

│   │   ├── types.rs                 # TradeData struct

│   └── tests/

│       └── position_test.rs         # Anchor test: update + record