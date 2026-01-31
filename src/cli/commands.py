"""CLI command handlers"""

import typer

sow_app = typer.Typer(help="SOW commands")
noun_app = typer.Typer(help="Noun commands")
verb_app = typer.Typer(help="Verb commands")

@sow_app.command()
def list():
    """List SOWs"""
    typer.echo("SOWs:")

@sow_app.command()
def create(name: str, short_name: str):
    """Create SOW"""
    typer.echo(f"Creating SOW: {name}")

@noun_app.command()
def list(sow: str):
    """List nouns"""
    typer.echo(f"Nouns in {sow}:")

@noun_app.command()
def show(reference: str):
    """Show noun details"""
    typer.echo(f"Noun: {reference}")

@verb_app.command()
def complete(reference: str):
    """Complete noun"""
    typer.echo(f"Completing: {reference}")

@verb_app.command()
def escalate(reference: str):
    """Escalate noun"""
    typer.echo(f"Escalating: {reference}")
