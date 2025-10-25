# ai/python/app/api/endpoints/portfolio.py
from fastapi import APIRouter, HTTPException, Depends, status
import logging

from app.models.schemas import (
    PortfolioAnalysisRequest, RiskAnalysisResponse, ErrorResponse
)
from app.api.dependencies import get_risk_service
from app.services.risk_service import RiskService

logger = logging.getLogger(__name__)

router = APIRouter()

@router.post(
    "/portfolio", 
    response_model=RiskAnalysisResponse,
    responses={
        400: {"model": ErrorResponse},
        500: {"model": ErrorResponse}
    }
)
async def analyze_portfolio(
    request: PortfolioAnalysisRequest,
    risk_service: RiskService = Depends(get_risk_service)
):
    """
    Analyze portfolio risk and generate recommendations
    
    - **wallet**: Wallet address to analyze
    - **positions**: List of portfolio positions with amounts and values
    - **total_value**: Total portfolio value in USD
    - **leverage_ratio**: Portfolio leverage ratio (default 1.0)
    """
    try:
        return await risk_service.analyze_portfolio(request)
    except ValueError as e:
        logger.warning(f"Validation error in portfolio analysis: {e}")
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(e)
        )
    except Exception as e:
        logger.error(f"Portfolio analysis error: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Internal server error during portfolio analysis"
        )

@router.get("/portfolio/examples")
async def get_portfolio_examples():
    """Get example portfolios for testing and demonstration"""
    examples = {
        "balanced_portfolio": {
            "name": "Balanced Portfolio",
            "description": "Well-diversified portfolio with moderate risk",
            "wallet": "example_wallet_balanced",
            "positions": [
                {"symbol": "SOL", "amount": 10, "value_usd": 2000, "volatility": 0.25},
                {"symbol": "USDC", "amount": 1000, "value_usd": 1000, "volatility": 0.01},
                {"symbol": "RAY", "amount": 100, "value_usd": 300, "volatility": 0.35},
                {"symbol": "BTC", "amount": 0.05, "value_usd": 1500, "volatility": 0.20}
            ],
            "total_value": 4800,
            "leverage_ratio": 1.0
        },
        "concentrated_portfolio": {
            "name": "Concentrated Portfolio",
            "description": "High-risk portfolio with heavy concentration",
            "wallet": "example_wallet_concentrated",
            "positions": [
                {"symbol": "SOL", "amount": 50, "value_usd": 10000, "volatility": 0.25},
                {"symbol": "USDC", "amount": 500, "value_usd": 500, "volatility": 0.01}
            ],
            "total_value": 10500,
            "leverage_ratio": 2.0
        }
    }
    return examples