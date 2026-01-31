"""Configuration settings for Level application"""

from enum import Enum
from typing import Optional
from pydantic_settings import BaseSettings

class DeploymentMode(str, Enum):
    """Deployment modes: Skinny (single DB), Standard (per-org DB), Enterprise (multi-DB federation)"""
    SKINNY = "skinny"
    STANDARD = "standard"
    ENTERPRISE = "enterprise"

class Settings(BaseSettings):
    """Application settings with support for multiple deployment modes"""
    
    # Database
    database_url: str = "postgresql+asyncpg://user:password@localhost/level"
    
    # Deployment mode
    deployment_mode: DeploymentMode = DeploymentMode.STANDARD
    
    # Logging
    log_level: str = "INFO"
    log_format: str = "json"  # json or standard
    
    # API
    api_title: str = "Level API"
    api_version: str = "0.1.0"
    api_debug: bool = False
    
    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"

settings = Settings()
