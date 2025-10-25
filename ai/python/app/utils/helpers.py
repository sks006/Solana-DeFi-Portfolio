# Helper functions
# ai/python/app/utils/helpers.py
from typing import Dict, Any, List
import datetime

def format_currency(amount: float) -> str:
    """Format currency amount"""
    return f"${amount:,.2f}"

def format_percentage(value: float) -> str:
    """Format percentage value"""
    return f"{value:.1%}"

def calculate_weighted_average(values: List[float], weights: List[float]) -> float:
    """Calculate weighted average"""
    if len(values) != len(weights):
        raise ValueError("Values and weights must have same length")
    
    total_weight = sum(weights)
    if total_weight == 0:
        return 0.0
    
    return sum(v * w for v, w in zip(values, weights)) / total_weight

def safe_divide(numerator: float, denominator: float, default: float = 0.0) -> float:
    """Safe division with default value"""
    if denominator == 0:
        return default
    return numerator / denominator

def timestamp() -> str:
    """Get current timestamp in ISO format"""
    return datetime.datetime.now().isoformat()