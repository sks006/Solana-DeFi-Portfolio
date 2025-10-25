# ai/python/test_installation.py
import sys
import importlib.metadata

def check_installation():
    print("🔍 Checking package installation...")
    
    requirements = {
        'fastapi': '0.120.0',
        'pydantic': '2.12.3', 
        'uvicorn': '0.38.0',
        'pydantic-settings': '2.1.0'
    }
    
    all_ok = True
    for package, expected_version in requirements.items():
        try:
            installed_version = importlib.metadata.version(package)
            status = "✅" if installed_version == expected_version else "⚠️"
            print(f"{status} {package}: {installed_version} (expected: {expected_version})")
            if installed_version != expected_version:
                all_ok = False
        except importlib.metadata.PackageNotFoundError:
            print(f"❌ {package}: Not installed")
            all_ok = False
    
    if all_ok:
        print("\n🎉 All packages installed correctly!")
    else:
        print("\n❌ Some packages have issues.")
    
    return all_ok

if __name__ == "__main__":
    check_installation()