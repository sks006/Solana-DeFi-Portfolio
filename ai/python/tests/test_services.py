# Service tests
# ai/python/tests/test_services.py
import pytest
import asyncio
from app.services.risk_service import RiskService
from app.models.schemas import PortfolioAnalysisRequest, Position

@pytest.fixture
def risk_service():
    return RiskService()

@pytest.mark.asyncio
async def test_risk_service_analysis(risk_service):
    """Test risk service portfolio analysis"""
    request = PortfolioAnalysisRequest(
        wallet="test_wallet",
        positions=[
            Position(symbol="SOL", amount=10, value_usd=2000, volatility=0.25),
            Position(symbol="USDC", amount=1000, value_usd=1000, volatility=0.01)
        ],
        total_value=3000.0
    )
    
    response = await risk_service.analyze_portfolio(request)
    
    assert response.risk_score >= 0
    assert response.risk_score <= 100
    assert response.risk_level in ["low", "medium", "high"]
    assert isinstance(response.alerts, list)
    assert isinstance(response.recommendations, list)
    assert response.wallet == "test_wallet"

@pytest.mark.asyncio
async def test_empty_portfolio(risk_service):
    """Test risk service with empty portfolio"""
    request = PortfolioAnalysisRequest(
        wallet="empty_wallet",
        positions=[],
        total_value=0.0
    )
    
    response = await risk_service.analyze_portfolio(request)
    
    assert response.risk_score == 0
    assert response.risk_level == "low"
    assert len(response.alerts) > 0

def test_risk_engine_trade_analysis(risk_service):
    """Test risk engine trade analysis"""
    trade_data = {
        'trade_size_usd': 5000.0
    }
    
    analysis = risk_service.risk_engine.analyze_trade(trade_data)
    
    assert 'trade_risk_score' in analysis
    assert 'alerts' in analysis
    assert 'recommendations' in analysis