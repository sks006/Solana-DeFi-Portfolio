# Anomaly detection service
# ai/python/app/services/anomaly_service.py
import logging
from typing import Dict, Any, List
import numpy as np

logger = logging.getLogger(__name__)

class AnomalyService:
    """Service for detecting anomalous patterns in portfolios and trades"""
    
    def __init__(self):
        self.anomaly_detector = SimpleAnomalyDetector()
    
    async def detect_portfolio_anomalies(self, portfolio_data: Dict[str, Any]) -> Dict[str, Any]:
        """Detect anomalies in portfolio changes"""
        return self.anomaly_detector.detect_portfolio_anomalies(portfolio_data)
    
    async def detect_trading_anomalies(self, trade_data: Dict[str, Any]) -> Dict[str, Any]:
        """Detect anomalies in trading patterns"""
        return self.anomaly_detector.detect_trading_anomalies(trade_data)

class SimpleAnomalyDetector:
    """Simple anomaly detection using statistical methods"""
    
    def detect_portfolio_anomalies(self, portfolio_data: Dict[str, Any]) -> Dict[str, Any]:
        """Detect portfolio anomalies"""
        current = portfolio_data.get('current', {})
        historical = portfolio_data.get('historical', [])
        
        if len(historical) < 2:
            return {'is_anomaly': False, 'reason': 'Insufficient historical data'}
        
        try:
            # Calculate portfolio change metrics
            current_value = current.get('total_value', 0)
            historical_values = [h.get('total_value', 0) for h in historical[-5:]]
            
            if len(historical_values) > 1:
                value_changes = np.diff(historical_values + [current_value])
                avg_change = np.mean(np.abs(value_changes[:-1]))
                current_change = abs(value_changes[-1])
                
                # Anomaly if current change is 3x average
                is_value_anomaly = current_change > avg_change * 3 if avg_change > 0 else False
            else:
                is_value_anomaly = False
            
            return {
                'is_anomaly': is_value_anomaly,
                'value_anomaly': is_value_anomaly,
                'current_value': current_value,
                'value_change_ratio': current_change / avg_change if avg_change > 0 else 0,
            }
            
        except Exception as e:
            logger.error(f"Portfolio anomaly detection failed: {e}")
            return {'is_anomaly': False, 'error': str(e)}
    
    def detect_trading_anomalies(self, trade_data: Dict[str, Any]) -> Dict[str, Any]:
        """Detect trading anomalies"""
        try:
            avg_trade_size = trade_data.get('avg_trade_size', 1000)
            current_trade_size = trade_data.get('trade_size_usd', 0)
            
            trade_size_ratio = current_trade_size / max(avg_trade_size, 1)
            is_anomaly = trade_size_ratio > 5.0
            
            return {
                'is_anomaly': is_anomaly,
                'anomaly_score': min(trade_size_ratio / 10.0, 1.0),
                'anomaly_type': 'large_trade' if is_anomaly else 'normal',
                'trade_size_ratio': trade_size_ratio
            }
            
        except Exception as e:
            logger.error(f"Trading anomaly detection failed: {e}")
            return {'is_anomaly': False, 'error': str(e)}