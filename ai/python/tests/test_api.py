# ai/python/tests/test_api.py
import pytest
from fastapi.testclient import TestClient
from app.main import app

# Fixture to provide a TestClient
@pytest.fixture
def client():
    return TestClient(app)

def test_health_check(client: TestClient):
    """Test health check endpoint"""
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    # Match your current endpoint implementation
    assert data["status"] == "ok"

def test_portfolio_analysis(client: TestClient):
    """Test portfolio analysis endpoint"""
    portfolio_data = {
        "wallet": "test_wallet_123",
        "positions": [
            {"symbol": "SOL", "amount": 10.0, "value_usd": 2000.0, "volatility": 0.25},
            {"symbol": "USDC", "amount": 1000.0, "value_usd": 1000.0, "volatility": 0.01},
        ],
        "total_value": 3000.0,
        "leverage_ratio": 1.0,
    }
    response = client.post("/analyze/portfolio", json=portfolio_data)
    assert response.status_code == 200
    data = response.json()
    assert "risk_score" in data
    assert "risk_level" in data
    assert "alerts" in data
    assert "recommendations" in data
    assert data["wallet"] == "test_wallet_123"

def test_trade_analysis(client: TestClient):
    """Test trade analysis endpoint"""
    trade_data = {
        "wallet": "test_wallet_123",
        "input_token": "SOL",
        "output_token": "USDC",
        "amount": 2.5,
        "trade_size_usd": 500.0,
    }
    response = client.post("/analyze/trade", json=trade_data)
    assert response.status_code == 200
    data = response.json()
    assert "trade_risk_score" in data
    assert "alerts" in data
    assert "recommendations" in data

def test_portfolio_examples(client: TestClient):
    """Test portfolio examples endpoint"""
    response = client.get("/analyze/portfolio/examples")
    assert response.status_code == 200
    data = response.json()
    assert "balanced_portfolio" in data
    assert "concentrated_portfolio" in data
