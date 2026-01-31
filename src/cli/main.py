"""Level CLI Client"""

import typer
from pathlib import Path
from typing import Optional
import toml
import httpx

app = typer.Typer(help="Level Project Management CLI")

# Configuration file location
CONFIG_FILE = Path.home() / ".level" / "config.toml"

class CLIConfig:
    """CLI configuration manager"""

    @staticmethod
    def load() -> dict:
        """Load configuration from ~/.level/config.toml"""
        if CONFIG_FILE.exists():
            return toml.load(CONFIG_FILE)
        return {
            "api_base_url": "http://localhost:8000",
            "api_key": "",
            "default_sow": "",
            "output_format": "table"
        }

    @staticmethod
    def save(config: dict) -> None:
        """Save configuration"""
        CONFIG_FILE.parent.mkdir(parents=True, exist_ok=True)
        with open(CONFIG_FILE, 'w') as f:
            toml.dump(config, f)

class HTTPClient:
    """HTTP client with X-API-Key injection"""

    def __init__(self, base_url: str, api_key: str):
        self.base_url = base_url
        self.api_key = api_key

    async def get(self, endpoint: str) -> dict:
        """GET request with API key"""
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}{endpoint}",
                headers={"X-API-Key": self.api_key}
            )
            return response.json()

    async def post(self, endpoint: str, data: dict) -> dict:
        """POST request with API key"""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{self.base_url}{endpoint}",
                json=data,
                headers={"X-API-Key": self.api_key}
            )
            return response.json()

@app.command()
def config_get(key: Optional[str] = None):
    """Get configuration value"""
    config = CLIConfig.load()
    if key:
        typer.echo(f"{key}: {config.get(key)}")
    else:
        for k, v in config.items():
            typer.echo(f"{k}: {v}")

@app.command()
def config_set(key: str, value: str):
    """Set configuration value"""
    config = CLIConfig.load()
    config[key] = value
    CLIConfig.save(config)
    typer.echo(f"✓ {key} = {value}")

if __name__ == "__main__":
    app()
