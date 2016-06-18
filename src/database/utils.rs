use postgres;
use postgres::{Connection, SslMode};
use redis::{RedisResult, Connection as RedisConn};
use redis;
use redis::Commands;

static URL:&'static str = "postgres://ipaomian:root@192.168.33.10:5432/smzdh";
//static URL:&'static str = "postgres://ipaomian:root@localhost:5432/smzdh";
static REDIS:&'static str = "redis://192.168.33.10/";
//static REDIS:&'static str = "redis://127.0.0.1/";

fn create_conn(url:&str) -> Result<Connection,postgres::error::ConnectError> {
    Connection::connect(url,SslMode::None)
}

pub fn conn() -> Result<Connection,postgres::error::ConnectError> {
    create_conn(URL)
}

pub fn redis_conn() -> Result<RedisConn,redis::RedisError> {
    redis::Client::open(REDIS).and_then(|c| c.get_connection())
}


pub fn test() {
    let mut c = conn();
    let cc = c.as_mut();

    let result = cc.map(
        |x| {
            x.query("SELECT * from pg_user;", &[]);
        });
    info!("what ? {:?}",result);
}
