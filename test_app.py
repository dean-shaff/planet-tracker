from datetime import datetime, timezone

import pytest
from litestar.testing import AsyncTestClient
from litestar.status_codes import HTTP_200_OK

from app import app 


@pytest.fixture(autouse=True)
def anyio_backend():
    return 'asyncio'


async def test_get_astron_object_data():
    async with AsyncTestClient(app=app) as client:
        params = dict(
            name="jupiter",
            lon=str(13.4),
            lat=str(52.5),
            elevation=str(0),
            when=datetime.now(tz=timezone.utc).isoformat(),
        )
        resp = await client.get("/get_astron_object_data", params=params)
        assert resp.status_code == HTTP_200_OK


async def test_search():
    async with AsyncTestClient(app=app) as client:
        params = dict(
            q="Berlin",
            max_results=5
        )
        resp = await client.get("/search", params=params)
        assert resp.status_code == HTTP_200_OK
