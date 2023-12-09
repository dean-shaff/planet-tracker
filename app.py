from pathlib import Path
from datetime import datetime
import logging
import os

import geocoder
from msgspec import Struct
import ephem
from litestar import Litestar, get
from litestar.static_files.config import StaticFilesConfig


__version__ = "4.0.0"


log = logging.getLogger(__name__)

public_dir = Path("./client").resolve()

if "MODE" in os.environ:
    if os.environ["MODE"] == "production" or os.environ["MODE"] == "prod":
        public_dir = Path("./dist").resolve()

log.info(f"os.environ['MODE']={os.environ.get('MODE')}")
log.info(f"public_dir={public_dir}")


class AstronObjectRequest(Struct):
    name: str
    lon: str
    lat: str
    elevation: float
    when: datetime 


class AstronObjectResponse(Struct):
    name: str
    magnitude: str
    size: str
    az: str
    el: str
    ra: str
    dec: str
    setting_time: datetime
    rising_time: datetime
    when: datetime


class SearchItem(Struct):
    name: str
    country: str
    sub_division: str
    lat: float
    lon: float


class SearchResponse(Struct):
    items: list[SearchItem]


def init_observer():
    observer = ephem.Observer()
    observer.pressure = 0
    observer.epoch = ephem.J2000
    return observer


@get("/search")
async def search(
    q: str,
    max_results: int = 1,
    fuzzy: float = 1.0
) -> SearchResponse:

    results = geocoder.geonames(q, key="dillpickle", maxRows=max_results, fuzzy=fuzzy)
    items = [SearchItem(
        name=g.address, 
        country=g.country, 
        sub_division=g.state,
        lat=float(g.lat),
        lon=float(g.lng)
    ) for g in results]
    return SearchResponse(items=items)


@get('/get_astron_object_data')
async def get_astron_object_data(
    name: str,
    lon: float,
    lat: float,
    elevation: float,
    when: datetime,
) -> AstronObjectResponse:
    log.debug(f"get_astron_object_data")
    observer = init_observer()
    # we have to do a string conversion for pyephem to work!
    observer.lon = str(lon)
    observer.lat = str(lat)
    observer.elevation = elevation
    observer.date = when
    astron_obj = getattr(ephem, name.capitalize())()
    astron_obj.compute(observer)

    res = AstronObjectResponse(
        astron_obj.name,
        astron_obj.mag,
        astron_obj.size,
        astron_obj.az,
        astron_obj.alt,
        astron_obj.ra,
        astron_obj.dec,
        datetime.strptime(str(observer.next_setting(astron_obj)), "%Y/%m/%d %H:%M:%S"),
        datetime.strptime(str(observer.next_rising(astron_obj)), "%Y/%m/%d %H:%M:%S"),
        when
    )
   
    log.debug(f"get_astron_object_data: {res=}")
    return res 


app = Litestar(
    route_handlers=[search, get_astron_object_data],
    static_files_config=[
        StaticFilesConfig(
            directories=[public_dir / "dist"],
            path="/",
            html_mode=True
        ),
    ]
)
