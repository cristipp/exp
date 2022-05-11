import json
import logging
from pathlib import Path
from textwrap import dedent

import chevron
import yaml

import users
from spec import Cube, Query, MeasureType, Spec


class CubeCompiler:
    def __init__(self, name: str, cube: Cube):
        self.name = name
        self.cube = cube

    def compile(self, query: Query):
        measure_columns = []
        dimension_columns = []
        for name in query.measures:
            sql = self.compile_measure(self.cube.measures[name])
            sql = f'{sql} {self.name}_{name}'
            measure_columns.append(sql)
        for name in query.dimensions:
            sql = self.compile_dimension(self.cube.dimensions[name])
            sql = f'{sql} {self.name}_{name}'
            dimension_columns.append(sql)
        return dedent(f'''\
            SELECT
                {", ".join(dimension_columns)},
                {", ".join(measure_columns)}
            FROM ({self.cube.sql}) AS {self.name}
            GROUP BY {", ".join([str(i) for i, _ in enumerate(dimension_columns)])}
        ''')

    def compile_measure(self, measure, depth=3):
        sql = self.render(measure.sql, depth)
        if measure.filters:
            filters = [self.render(filter.sql, depth) for filter in measure.filters]
            sql = f'CASE WHEN ({" AND ".join(filters)}) THEN {sql} END'
        match measure.type:
            case MeasureType.COUNT:
                sql = f'count({sql})'
            case MeasureType.NUMBER:
                pass
        return sql

    def compile_dimension(self, dimension, depth=3):
        sql = self.render(dimension.sql, depth)
        return sql

    def render(self, template: str, depth: int):
        if depth == 0:
            return 'recursion depth exceeded'
        data = {
            'CUBE': self.name
        }
        for name, measure in self.cube.measures.items():
            data[name] = self.compile_measure(measure, depth - 1)
        return chevron.render(template, data)


class SpecCompiler:
    def __init__(self, spec: Spec):
        self.spec = spec

    def compile(self, query: Query):
        cube = self.spec[query.cube]
        return CubeCompiler(query.cube.lower(), cube).compile(query)


def test_users():
    # https://cube.dev/docs/schema/getting-started
    dict0 = {name: cube.dict(exclude_unset=True) for name, cube in users.spec.items()}
    json0 = json.dumps(dict0, indent=2)
    print(json0)
    yaml0 = yaml.dump(dict0, indent=2)
    print(yaml0)

    assert json0 == Path('users.json').read_text()

    compiler = SpecCompiler(users.spec)
    sql0 = compiler.compile(Query(
        cube='Users',
        measures=['count'],
        dimensions=['city', 'companyName'],
    ))
    print(sql0)
    assert sql0 == dedent('''\
        SELECT
            users.city users_city, users.company_name users_companyName,
            count(users.id) users_count
        FROM (SELECT * FROM users) AS users
        GROUP BY 0, 1
    ''')
    sql1 = compiler.compile(Query(
        cube='Users',
        measures=['payingCount'],
        dimensions=['city'],
    ))
    print(sql1)
    assert sql1 == dedent('''\
        SELECT
            users.city users_city,
            count(CASE WHEN (users.paying = true) THEN users.id END) users_payingCount
        FROM (SELECT * FROM users) AS users
        GROUP BY 0
    ''')
    sql2 = compiler.compile(Query(
        cube='Users',
        measures=['payingPercentage'],
        dimensions=['city'],
    ))
    print(sql2)
    assert sql2 == dedent('''\
        SELECT
            users.city users_city,
            100.0 * count(CASE WHEN (users.paying = true) THEN users.id END) / count(users.id) users_payingPercentage
        FROM (SELECT * FROM users) AS users
        GROUP BY 0
    ''')


def test_yaml():
    users_dict = yaml.safe_load(Path('users.short.yaml').read_text())
    users = {name: Cube(**spec) for name, spec in users_dict.items()}
    print(json.dumps({name: cube.dict() for name, cube in users.items()}, indent=2))
