use postgres::{Client, NoTls};
use::testcontainers::*;

#[test]
fn postgres_one_plus_one() {
    let docker = clients::Cli::default();
    let postgres_image = images::postgres::Postgres::default();
    let node = docker.run(postgres_image);

    let connection_string = &format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        node.get_host_port(5432).unwrap()
    );

    let mut conn = Client::connect(connection_string, NoTls).unwrap();

    for row in conn.query("SELECT 1 + 1", &[]).unwrap() {
        let first_column: i32 = row.get(0);
        assert_eq!(first_column, 2);
    }
}