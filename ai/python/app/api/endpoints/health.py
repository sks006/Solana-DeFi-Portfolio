# ai/python/app/api/endpoints/health.py
from fastapi import APIRouter
from datetime import datetime
import psutil
import os

from app.models.schemas import HealthResponse
from app.core.config import settings

router = APIRouter()

@router.get("/health", response_model=HealthResponse)
async def health_check():
    """Basic health check endpoint"""
    return HealthResponse(
        status="healthy",
        version=settings.VERSION,
        timestamp=datetime.now(),
        service="risk-ai"
    )

@router.get("/health/detailed")
async def detailed_health_check():
    """Detailed health check with system information"""
    process = psutil.Process(os.getpid())
    memory_info = process.memory_info()
    
    return {
        "status": "healthy",
        "service": "risk-ai",
        "version": settings.VERSION,
        "environment": settings.ENVIRONMENT,
        "timestamp": datetime.now().isoformat(),
        "system": {
            "memory_usage_mb": memory_info.rss / 1024 / 1024,
            "cpu_percent": process.cpu_percent(),
            "threads": process.num_threads(),
        },
        "dependencies": {
            "risk_engine": "operational",
            "anomaly_detector": "operational",
            "database": "not_configured"
        }
    }