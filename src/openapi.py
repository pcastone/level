"""OpenAPI/Swagger configuration"""

from fastapi.openapi.utils import get_openapi

def custom_openapi(app):
    if app.openapi_schema:
        return app.openapi_schema

    openapi_schema = get_openapi(
        title="Level API",
        version="0.1.0",
        description="Project Management System with Noun/Verb/Container Grammar",
        routes=app.routes,
    )

    openapi_schema["info"]["x-logo"] = {
        "url": "https://level.example.com/logo.png"
    }

    app.openapi_schema = openapi_schema
    return app.openapi_schema
