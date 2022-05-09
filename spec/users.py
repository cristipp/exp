spec = {
    'Users': Cube(
        sql='SELECT * FROM users',
        measures={
            'count': Measure(
                sql='{{CUBE}}.id',
                type=MeasureType.COUNT
            ),
            'payingCount': Measure(
                sql='{{CUBE}}.id',
                type=MeasureType.COUNT,
                filters=[
                    Filter(sql='{{CUBE}}.paying = true'),
                ],
            ),
            'payingPercentage': Measure(
                sql='100.0 * {{payingCount}} / {{count}}',
                type=MeasureType.NUMBER,
                format=Format.PERCENT,
            ),
        },
        dimensions={
            'city': Dimension(
                sql='{{CUBE}}.city',
                type=DimensionType.STRING
            ),
            'companyName': Dimension(
                sql='{{CUBE}}.company_name',
                type=DimensionType.STRING,
            ),
        },
    ),
}