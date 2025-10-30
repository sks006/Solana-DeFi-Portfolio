# Pydantic schemas
# ai/python/app/models/schemas.py
from pydantic import BaseModel, Field, validator
from typing import List, Dict, Any, Optional
from datetime import datetime
from enum import Enum


class RiskLevel(str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"

    def __str__(self):
        return str(self.value)

    def __repr__(self):
        return str(self.value)
    
    def __json__(self):
        return str(self.value)
    
class AlertSeverity(str, Enum):
    INFO = "info"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"

class Position(BaseModel):
    mint: str
    amount: float
    value_usd: float
    volatility: float = 0.02
    
    # Use model_validate instead of parse_obj in v2
    @classmethod
    def from_dict(cls, data: dict):
        return cls.model_validate(data)
    
    # Convert to dict using model_dump
    def to_dict(self):
        return self.model_dump()

class PortfolioAnalysisRequest(BaseModel):
    wallet: str = Field(..., description="Wallet address to analyze")
    positions: List[Position] = Field(..., description="List of portfolio positions with amounts and values")
    total_value: float = Field(..., gt=0, description="Total portfolio value in USD")
    leverage_ratio: float = Field(..., gt=0, description="Portfolio leverage ratio (must be > 0)")

class TradeAnalysisRequest(BaseModel):
    wallet: str = Field(..., description="Wallet address", example="wallet_123abc")
    input_token: str = Field(..., description="Input token symbol", example="SOL")
    output_token: str = Field(..., description="Output token symbol", example="USDC")
    amount: float = Field(..., description="Trade amount", example=2.5)
    trade_size_usd: float = Field(..., description="Trade size in USD", example=500.0)
    daily_volume: Optional[float] = Field(1000000.0, description="Daily volume for the token")

class Alert(BaseModel):
    severity: AlertSeverity = Field(..., description="Alert severity")
    message: str = Field(..., description="Alert message")
    type: str = Field(..., description="Alert type")
    metric: Optional[str] = Field(None, description="Related metric")
    value: Optional[float] = Field(None, description="Metric value")

class PortfolioMetrics(BaseModel):
    concentration: float = Field(..., description="Portfolio concentration", example=0.65)
    weighted_volatility: float = Field(..., description="Weighted volatility", example=0.18)
    num_positions: int = Field(..., description="Number of positions", example=3)
    diversity_score: float = Field(..., description="Diversity score", example=0.35)
    max_position_ratio: float = Field(..., description="Largest position ratio", example=0.5)
    total_value: float = Field(..., description="Total portfolio value", example=5000.0)

class RiskAnalysisResponse(BaseModel):
    risk_score: float = Field(..., description="Overall risk score 0-100", example=65.5)
    risk_level: RiskLevel = Field(..., description="Risk level", example=RiskLevel.MEDIUM)
    alerts: List[Alert] = Field(..., description="Risk alerts")
    recommendations: List[str] = Field(..., description="Risk recommendations")
    metrics: PortfolioMetrics = Field(..., description="Portfolio metrics")
    analysis_timestamp: datetime = Field(..., description="Analysis timestamp")
    wallet: str = Field(..., description="Wallet address")

class TradeAnalysisResponse(BaseModel):
    trade_risk_score: float = Field(..., description="Trade risk score 0-100", example=45.0)
    size_risk: float = Field(..., description="Size-based risk", example=0.3)
    alerts: List[Alert] = Field(..., description="Trade alerts")
    suggested_slippage: str = Field(..., description="Suggested slippage tolerance", example="0.5%")
    recommendations: List[str] = Field(..., description="Trade recommendations")

class HealthResponse(BaseModel):
    status: str = Field(..., description="Service status", example="healthy")
    version: str = Field(..., description="Service version", example="1.0.0")
    timestamp: datetime = Field(..., description="Response timestamp")
    service: str = Field(..., description="Service name", example="risk-ai")

class ErrorResponse(BaseModel):
    detail: str = Field(..., description="Error message")
    error_type: str = Field(..., description="Error type")
    timestamp: datetime = Field(..., description="Error timestamp")