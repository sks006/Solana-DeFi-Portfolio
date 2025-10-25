# ai/python/check_compatibility.py
import sys
import importlib.metadata

def check_versions():
    requirements = {
        'fastapi': '0.120.0',
        'pydantic': '2.12.3',
        'uvicorn': '0.38.0',
        'pytest': '8.4.2'
    }
    
    for package, expected_version in requirements.items():
        try:
            installed_version = importlib.metadata.version(package)
            print(f"✅ {package}: {installed_version} (expected: {expected_version})")
        except importlib.metadata.PackageNotFoundError:
            print(f"❌ {package}: Not installed")

if __name__ == "__main__":
    check_versions()