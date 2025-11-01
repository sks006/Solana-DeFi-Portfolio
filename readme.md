# 📂 Total Project Tree — Solana DeFi Portfolio with Risk Alerts Ai.

solana-defi-portfolio/

├── Makefile                      # dev, test, start-demo targets

├── README.md                     # short overview + demo instructions

├── Dockerfile                    # backend + AI service

├── docker-compose.yml            # services: backend, ai, db, replay

├── .gitignore

├── scripts/

│   ├── replay_events.sh          # demo + load test

│   └── warm_models.sh            # AI bootstrapping


                    🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧

                  

  Live frontend link
                  
                    https://solana-defi-portfolio.vercel.app/
                  
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

                     🟨🟨🟨🟨🟨🟨🟨🟨🟨🟨

├── backend/

│   ├── Cargo.toml

│   ├── src/

│   │   ├── main.rs

│   │   ├── config/

│   │    │    ├── mod.rs

│   │    │   ├── server.rs

│   │    │    ├── solana.rs

│   │    │   ├── ai.rs

│   │    │    ├── database.rs

│   │    │   ├── pipeline.rs

│   │    │    └── risk.rs

│   │   ├── server_functions/

│   │   │   ├── portfolio.rs

│   │   │   ├── swap.rs

│   │   │   └── risk.rs

│   │   ├── ws/

│   │   │   ├── hub.rs

│   │   │   └── client.rs

│   │   ├── ingestion/

│   │   │   ├── solana_ws.rs

│   │   │   └── normalizer.rs

│   │   ├── pipeline/

│   │   │   ├── mpsc_queue.rs

│   │   │   ├── micro_batcher.rs

│   │   │   └── rules_engine.rs

│   │   ├── services/

│   │   │   ├── solana_client.rs

│   │   │   ├── ai_client.rs

│   │   │   └── metrics.rs

│   │   ├── models/

│   │   │   ├── event.rs

│   │   │   └── risk_alert.rs

│   │   └── utils/

│   │       └── telemetry.rs

│   ├── config/

│   │   └── dev.toml

│   └── tests/

│       └── integration/          # backend + ai + replay


✅ ~ https://solana-defi-portfolio.onrender.com/health ~

✅ ~ https://solana-defi-portfolio.onrender.com/info ~

✅ ~ https://solana-defi-portfolio.onrender.com/api/portfolio/<WALLET_ADDRESS> ~

✅ ~ https://solana-defi-portfolio.onrender.com/api/risk/alerts  ~

                   
                 🟨🟨🟨🟨🟨🟨🟨🟨🟨🟨
                   
├── ai/

│   ├── python/

│   │   ├── app/

│   │   │   ├── __init__.py

│   │   │   ├── main.py                          # ✅ RUNNING on port 8001

│   │   │   ├── api/

│   │   │   │   ├── __init__.py

│   │   │   │   ├── dependencies.py

│   │   │   │   └── endpoints/

│   │   │   │       ├── __init__.py

│   │   │   │       ├── health.py

│   │   │   │       ├── portfolio.py

│   │   │   │       └── trade.py

│   │   │   ├── core/

│   │   │   │   ├── __init__.py

│   │   │   │   ├── config.py                    # ⚠️ has import issue

│   │   │   │   └── security.py

│   │   │   ├── services/

│   │   │   │   ├── __init__.py

│   │   │   │   ├── risk_service.py              # ✅ fixed syntax

│   │   │   │   └── anomaly_service.py

│   │   │   ├── models/

│   │   │   │   ├── __init__.py

│   │   │   │   ├── schemas.py

│   │   │   │   └── domain.py

│   │   │   └── utils/

│   │   │       ├── __init__.py

│   │   │       ├── logging.py

│   │   │       └── helpers.py

│   │   ├── tests/

│   │   │   ├── __init__.py

│   │   │   └── test_installation.py             # ✅ working

│   │   ├── check_compatibility.py               # ✅ working

│   │   ├── check_files.py                       # ✅ working

│   │   ├── test_api_simple.py                   # ⚠️ import issue

│   │   └── requirements.txt                     # ✅ installed

│   ├── test_ultra_simple.py                     # ✅ original test

│   └── Dockerfile

```
https://rejwar-solana-defi-ai.hf.space/
health

```

                 🟨🟨🟨🟨🟨🟨🟨🟨🟨🟨

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


├── docs/                       # Showcase-ready documentation

│   ├── README_DEMO.md          # Quickstart for judges

│   ├── architecture.md

│   ├── architecture.png

│   ├── features.md

│   ├── api.md

│   └── usage.md

│

└── .github/                    # workflows

### install frontend
~~~
npm install
~~~

### Build in debug mode (faster for development)
~~~
 cargo build
~~~
### Build in release mode (optimized for production)
~~~
 cargo build --release
~~~

### Ai Backend run
~~~~
uvicorn app.main:app
~~~~

### Run in debug mode
~~~
 cargo run
~~~
### Run in release mode
~~~
 cargo run --release
~~~
### Run tests
~~~
 cargo test
~~~
### Check for compilation errors without building
~~~
 cargo check
~~~
### Format code
~~~
 cargo fmt
~~~
### Check code quality
~~~
 cargo clippy
~~~




