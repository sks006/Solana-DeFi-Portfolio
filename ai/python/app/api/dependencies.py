# Shared dependencies
# ai/python/app/api/dependencies.py
from fastapi import Header, HTTPException, Depends
from typing import Optional
from app.core.security import get_api_key

async def get_optional_api_key(api_key: Optional[str] = Header(None)) -> Optional[str]:
    """Optional API key dependency"""
    if api_key:
        return await get_api_key(api_key)
    return None

def get_risk_service():
    """Dependency for risk service"""
    from app.services.risk_service import RiskService
    return RiskService()

def get_anomaly_service():
    """Dependency for anomaly service"""
    from app.services.anomaly_service import AnomalyService
    return AnomalyService()