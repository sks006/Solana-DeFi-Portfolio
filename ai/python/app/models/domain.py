# Domain models
# ai/python/app/models/domain.py
from typing import List, Dict, Any
from dataclasses import dataclass
from enum import Enum

class RiskCategory(Enum):
    CONCENTRATION = "concentration"
    VOLATILITY = "volatility"
    LIQUIDITY = "liquidity"
    LEVERAGE = "leverage"
    DIVERSIFICATION = "diversification"

@dataclass
class Portfolio:
    wallet: str
    positions: List[Dict[str, Any]]
    total_value: float
    historical_snapshots: List[Dict[str, Any]]
    leverage_ratio: float

@dataclass
class RiskAssessment:
    score: float
    level: str
    categories: Dict[RiskCategory, float]
    alerts: List[Dict[str, Any]]
    recommendations: List[str]

@dataclass
class Trade:
    wallet: str
    input_token: str
    output_token: str
    amount: float
    trade_size_usd: float
    market_conditions: Dict[str, Any]