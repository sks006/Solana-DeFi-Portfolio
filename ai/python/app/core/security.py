# Security utilities
# ai/python/app/core/security.py
from fastapi import HTTPException, status
from typing import Optional

def verify_api_key(api_key: Optional[str] = None) -> bool:
    """
    Basic API key verification.
    In production, use proper authentication like JWT or OAuth2
    """
    if not api_key:
        return False
    
    # Simple validation - in production, validate against database
    return len(api_key) > 10

async def get_api_key(api_key: Optional[str] = None) -> str:
    """Dependency for API key validation"""
    if not verify_api_key(api_key):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid or missing API key",
        )
    return api_key