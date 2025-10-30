# Pytest fixtures
# ai/python/tests/conftest.py
import pytest
import asyncio
from typing import AsyncGenerator
from fastapi.testclient import TestClient

from app.main import app

@pytest.fixture(scope="session")
def event_loop():
    """Create an instance of the default event loop for the test session"""
    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()


@pytest.fixture(scope="session")
def event_loop_policy():
    """Use the modern event loop policy for pytest-asyncio"""
    import asyncio
    policy = asyncio.DefaultEventLoopPolicy()
    return policy

@pytest.fixture(scope="module")
def client() -> TestClient:
    """Test client fixture"""
    with TestClient(app) as test_client:
        yield test_client