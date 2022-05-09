from enum import Enum
from pydantic import BaseModel
from typing import Dict, List, Optional


class MeasureType(str, Enum):
    COUNT = 'count'
    NUMBER = 'number'


class Format(str, Enum):
    PERCENT = 'percent'


class DimensionType(str, Enum):
    STRING = 'string'


class Filter(BaseModel):
    sql: str


class Measure(BaseModel, use_enum_values=True):
    type: MeasureType
    sql: str
    filters: List[Filter] = None
    format: Optional[Format] = None


class Dimension(BaseModel, use_enum_values=True):
    type: DimensionType
    sql: str
    filters: List[Filter] = None


class Cube(BaseModel):
    sql: str
    measures: Dict[str, Measure]
    dimensions: Dict[str, Dimension]


class Spec(BaseModel):
    cubes: Dict[str, Cube]


class Query(BaseModel):
    cube: str
    measures: List[str]
    dimensions: List[str]


