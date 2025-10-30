# ai/python/app/services/risk_service.py
from typing import List, Dict, Any
import datetime
import logging
from app.models.schemas import (
    PortfolioAnalysisRequest, RiskAnalysisResponse, Alert, 
    PortfolioMetrics, RiskLevel, AlertSeverity
)
from app.core.config import settings

logger = logging.getLogger(__name__)

class RiskService:
    """Service layer for risk analysis business logic"""
    
    def __init__(self):
        self.risk_engine = PurePythonRiskEngine()
    
    async def analyze_portfolio(self, request: PortfolioAnalysisRequest) -> RiskAnalysisResponse:
        """Analyze portfolio risk"""
        logger.info(f"Analyzing portfolio for wallet: {request.wallet}")
        
        try:
            # Convert to dict using Pydantic v2 method
            portfolio_data = {
                'positions': [pos.model_dump() for pos in request.positions],
                'total_value': request.total_value
            }
            
            # Perform analysis
            analysis = self.risk_engine.analyze_portfolio(portfolio_data)
            
            # Convert to response model
            return RiskAnalysisResponse(
                risk_score=analysis['risk_score'],
                risk_level=RiskLevel(analysis['risk_level']),
                alerts=[Alert(**alert) for alert in analysis['alerts']],
                recommendations=analysis['recommendations'],
                metrics=PortfolioMetrics(**analysis['metrics']),
                analysis_timestamp=datetime.datetime.now(),
                wallet=request.wallet
            )
            
        except Exception as e:
            logger.error(f"Portfolio analysis failed for wallet {request.wallet}: {e}")
            raise

    async def analyze_trade(self, trade_data: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze trade risk"""
        return self.risk_engine.analyze_trade(trade_data)

class PurePythonRiskEngine:
    """Pure Python risk engine implementation"""
    
    def analyze_portfolio(self, portfolio_data: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze portfolio risk using pure Python"""
        positions = portfolio_data.get('positions', [])
        total_value = portfolio_data.get('total_value', 0)
        
        if total_value == 0 or not positions:
            return self._empty_portfolio_response()
        
        # Calculate basic metrics using pure Python
        metrics = self._calculate_metrics_pure_python(positions, total_value)
        
        # Calculate risk score (0-100)
        risk_score = self._calculate_risk_score_pure(metrics)
        
        # Determine risk level
        risk_level = self._get_risk_level(risk_score)
        
        # Generate alerts
        alerts = self._generate_alerts(metrics, risk_level)
        
        # Generate recommendations
        recommendations = self._generate_recommendations(metrics, risk_level)
        
        return {
            'risk_score': risk_score,
            'risk_level': risk_level,
            'alerts': alerts,
            'recommendations': recommendations,
            'metrics': metrics
        }
    
    def analyze_trade(self, trade_data: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze trade risk using pure Python"""
        trade_size = trade_data.get('trade_size_usd', 0)
        
        # Simple trade risk assessment
        size_risk = min(trade_size / 10000, 1.0)  # Normalize by $10k
        risk_score = size_risk * 100
        
        alerts = []
        if size_risk > 0.7:
            alerts.append({
                'severity': 'high',
                'message': 'Large trade detected - consider splitting into smaller trades',
                'type': 'trade_size'
            })
        elif size_risk > 0.3:
            alerts.append({
                'severity': 'medium',
                'message': 'Moderate trade size - monitor price impact',
                'type': 'trade_size'
            })
        
        return {
            'trade_risk_score': risk_score,
            'size_risk': size_risk,
            'alerts': alerts,
            'suggested_slippage': f"{max(0.001, size_risk * 0.01):.3%}",
            'recommendations': [
                'Check current liquidity before trading',
                'Monitor price impact during execution'
            ]
        }
    
    def _calculate_metrics_pure_python(self, positions: List[Dict], total_value: float) -> Dict[str, float]:
        """Calculate portfolio metrics using pure Python"""
        weights = [pos.get('value_usd', 0) / total_value for pos in positions]
        
        concentration = sum(w ** 2 for w in weights)
        weighted_vol = sum(pos.get('volatility', 0.02) * weights[i] 
                          for i, pos in enumerate(positions))
        
        num_positions = len(positions)
        max_position_ratio = max(weights) if weights else 0
        
        volatilities = [pos.get('volatility', 0.02) for pos in positions]
        avg_volatility = sum(volatilities) / len(volatilities) if volatilities else 0
        
        return {
            'concentration': concentration,
            'weighted_volatility': weighted_vol,
            'avg_volatility': avg_volatility,
            'num_positions': num_positions,
            'max_position_ratio': max_position_ratio,
            'diversity_score': 1.0 - concentration,
            'total_value': total_value
        }
    
    def _calculate_risk_score_pure(self, metrics: Dict[str, float]) -> float:
        """Calculate overall risk score using pure Python"""
        concentration_risk = metrics['concentration'] * 0.4
        volatility_risk = metrics['weighted_volatility'] * 0.3
        concentration_penalty = metrics['max_position_ratio'] * 0.2
        size_penalty = min(metrics['total_value'] / 100000, 0.1)
        
        base_score = (concentration_risk + volatility_risk + concentration_penalty + size_penalty) * 100
        
        # Adjust for diversification benefit
        if metrics['num_positions'] > 5:
            base_score *= 0.9
        elif metrics['num_positions'] < 2:
            base_score *= 1.2
            
        return min(base_score, 100)
    
    def _get_risk_level(self, risk_score: float) -> str:
        """Convert risk score to level"""
        if risk_score >= settings.HIGH_RISK_THRESHOLD:
            return "high"
        elif risk_score >= settings.MEDIUM_RISK_THRESHOLD:
            return "medium"
        else:
            return "low"
    
    def _generate_alerts(self, metrics: Dict[str, float], risk_level: str) -> List[Dict]:
        """Generate risk alerts"""
        alerts = []
        
        # Risk level alerts
        if risk_level == "high":
            alerts.append({
                'severity': 'high',
                'message': 'Portfolio has high overall risk',
                'type': 'overall_risk'
            })
        
        # Concentration alerts
        if metrics['concentration'] > 0.7:
            alerts.append({
                'severity': 'high',
                'message': f'Very high concentration: {metrics["concentration"]:.1%}',
                'type': 'concentration'
            })
        elif metrics['concentration'] > 0.5:
            alerts.append({
                'severity': 'medium',
                'message': f'High concentration: {metrics["concentration"]:.1%}',
                'type': 'concentration'
            })
        
        # Volatility alerts
        if metrics['weighted_volatility'] > 0.3:
            alerts.append({
                'severity': 'high',
                'message': f'High volatility: {metrics["weighted_volatility"]:.1%}',
                'type': 'volatility'
            })
        elif metrics['weighted_volatility'] > 0.15:
            alerts.append({
                'severity': 'medium',
                'message': f'Moderate volatility: {metrics["weighted_volatility"]:.1%}',
                'type': 'volatility'
            })
        
        # Diversification alerts
        if metrics['num_positions'] < 3:
            alerts.append({
                'severity': 'medium',
                'message': f'Low diversification: only {metrics["num_positions"]} positions',
                'type': 'diversification'
            })
        
        return alerts
    
    def _generate_recommendations(self, metrics: Dict[str, float], risk_level: str) -> List[str]:
        """Generate risk mitigation recommendations"""
        recommendations = []
        
        # General recommendations based on risk level
        if risk_level == "high":
            recommendations.extend([
                "Consider reducing position sizes in largest holdings",
                "Diversify into different asset classes",
                "Set stop-loss orders for volatile positions",
                "Review and potentially reduce leverage"
            ])
        elif risk_level == "medium":
            recommendations.extend([
                "Monitor portfolio concentration regularly",
                "Consider adding hedging strategies",
                "Review asset allocation monthly",
                "Set price alerts for key positions"
            ])
        
        # Specific recommendations based on metrics
        if metrics['concentration'] > 0.5:
            recommendations.append("Diversify portfolio to reduce concentration risk")
        
        if metrics['weighted_volatility'] > 0.2:
            recommendations.append("Consider adding less volatile assets to portfolio")
        
        if metrics['num_positions'] < 4:
            recommendations.append("Consider adding more positions for better diversification")
        
        # Remove duplicates
        return list(dict.fromkeys(recommendations))
    
    def _empty_portfolio_response(self) -> Dict[str, Any]:
        """Response for empty portfolio"""
        return {
            'risk_score': 0,
            'risk_level': 'low',
            'alerts': [{
                'severity': 'info',
                'message': 'Portfolio is empty or has no value',
                'type': 'info'
            }],
            'recommendations': ['Start by adding some positions to your portfolio'],
            'metrics': {
                'concentration': 0,
                'weighted_volatility': 0,
                'avg_volatility': 0,
                'num_positions': 0,
                'max_position_ratio': 0,
                'diversity_score': 0,
                'total_value': 0
            }
        }