# ai/python/check_files.py
import os
import sys

def check_required_files():
    """Check if all required files exist"""
    required_files = [
        "app/__init__.py",
        "app/main.py", 
        "app/core/__init__.py",
        "app/core/config.py",
        "app/utils/__init__.py", 
        "app/utils/logging.py",
        "app/models/__init__.py",
        "app/models/schemas.py",
        "app/services/__init__.py",
        "app/services/risk_service.py",
        "app/api/__init__.py",
        "app/api/endpoints/__init__.py",
        "app/api/endpoints/health.py",
        "app/api/endpoints/portfolio.py",
        "app/api/dependencies.py"
    ]
    
    missing_files = []
    for file_path in required_files:
        if not os.path.exists(file_path):
            missing_files.append(file_path)
    
    if missing_files:
        print("❌ Missing files:")
        for file in missing_files:
            print(f"   - {file}")
        return False
    else:
        print("✅ All required files are present!")
        return True

if __name__ == "__main__":
    check_required_files()