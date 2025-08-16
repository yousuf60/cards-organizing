a practice
rust maturin python fastapi

```
mkdir data
uv sync
uv run --offline --verbose maturin dev --verbose --uv &&echo -e "\a" &&uv -n run pytest
uv run fastapi dev

```
