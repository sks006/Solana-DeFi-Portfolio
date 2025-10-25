# ai/python/test_api_simple.py
import sys
import os

# Add current directory to Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def test_basic_imports():
    """Test if we can import the main modules"""
    try:
        from app.core.config import settings
        print("✅ Config imported successfully")
        
        from app.models.schemas import PortfolioAnalysisRequest, Position
        print("✅ Schemas imported successfully")
        
        from app.services.risk_service import RiskService
        print("✅ Risk service imported successfully")
        
        from app.main import app
        print("✅ FastAPI app imported successfully")
        
        return True
        
    except Exception as e:
        print(f"❌ Import failed: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    print("🔍 Testing basic imports...")
    if test_basic_imports():
        print("\n🎉 All imports successful!")
    else:
        print("\n💥 Import issues found!")