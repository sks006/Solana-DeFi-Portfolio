# ai/python/app/api/endpoints/trade.py
from fastapi import APIRouter, HTTPException, Depends, status
import logging

from app.models.schemas import (
    TradeAnalysisRequest, TradeAnalysisResponse, ErrorResponse
)
from app.api.dependencies import get_risk_service
from app.services.risk_service import RiskService

logger = logging.getLogger(__name__)

router = APIRouter()

@router.post(
    "/trade",
    response_model=TradeAnalysisResponse,
    responses={
        400: {"model": ErrorResponse},
        500: {"model": ErrorResponse}
    }
)
async def analyze_trade(
    request: TradeAnalysisRequest,
    risk_service: RiskService = Depends(get_risk_service)
):
    """
    Analyze individual trade risk
    
    - **wallet**: Wallet address executing the trade
    - **input_token**: Token being sold
    - **output_token**: Token being bought  
    - **amount**: Amount of input token to trade
    - **trade_size_usd**: Total trade size in USD
    - **daily_volume**: Daily trading volume for price impact calculation
    """
    try:
        trade_data = {
            'wallet': request.wallet,
            'input_token': request.input_token,
            'output_token': request.output_token,
            'amount': request.amount,
            'trade_size_usd': request.trade_size_usd,
            'daily_volume': request.daily_volume
        }
        
        analysis = await risk_service.analyze_trade(trade_data)
        
        return TradeAnalysisResponse(
            trade_risk_score=analysis['trade_risk_score'],
            size_risk=analysis['size_risk'],
            alerts=analysis['alerts'],
            suggested_slippage=analysis['suggested_slippage'],
            recommendations=analysis['recommendations']
        )
        
    except ValueError as e:
        logger.warning(f"Validation error in trade analysis: {e}")
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(e)
        )
    except Exception as e:
        logger.error(f"Trade analysis error: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Internal server error during trade analysis"
        )