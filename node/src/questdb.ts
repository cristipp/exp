import { Client } from "pg";

async function main() {
  const client = new Client({
    database: "qdb",
    host: "127.0.0.1",
    port: 8812,
    user: "admin",
    password: "quest",
  })
  await client.connect()
  await client.query(`
    CREATE TABLE IF NOT EXISTS orders as (
      with x as (
        select 1 as id, 100 as amount, 'new' status
        UNION ALL
        select 2 as id, 200 as amount, 'new' status
        UNION ALL
        select 3 as id, 300 as amount, 'processed' status
        UNION ALL
        select 4 as id, 500 as amount, 'processed' status
        UNION ALL
        select 5 as id, 600 as amount, 'shipped' status
      )
      select * from x
    );
  `)
  const result = await client.query(`
    SELECT * FROM orders
  `)
  console.log('Rows: ', result.rows)
  await client.end()
}

main()