# ai/python/app/core/config.py
from pydantic_settings import BaseSettings
from typing import List
import os

class Settings(BaseSettings):
    """Application settings"""
    
    # Application
    PROJECT_NAME: str = "Solana DeFi Risk AI"
    DESCRIPTION: str = "AI-powered risk analysis for Solana DeFi portfolios"
    VERSION: str = "1.0.0"
    ENVIRONMENT: str = "development"
    DEBUG: bool = True
    
    # Server
    PORT: int = 8001
    HOST: str = "0.0.0.0"
    
    # CORS
    ALLOWED_HOSTS: List[str] = ["*"]
    
    # Logging
    LOG_LEVEL: str = "INFO"
    LOG_FORMAT: str = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
    
    # Risk Analysis
    HIGH_RISK_THRESHOLD: int = 70
    MEDIUM_RISK_THRESHOLD: int = 40
    
    class Config:
        env_file = ".env"
        case_sensitive = True

# Global settings instance
settings = Settings()