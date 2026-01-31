"""Level - Project management system with Noun/Verb/Container grammar"""

import logging
from fastapi import FastAPI
from contextlib import asynccontextmanager

# Configure structured JSON logging
logging.basicConfig(
    level=logging.INFO,
    format='{"timestamp": "%(asctime)s", "level": "%(levelname)s", "message": "%(message)s"}'
)

logger = logging.getLogger(__name__)

@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("Starting Level application")
    yield
    logger.info("Shutting down Level application")

app = FastAPI(title="Level", version="0.1.0", lifespan=lifespan)

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
