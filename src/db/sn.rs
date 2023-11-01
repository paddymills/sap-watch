
use ftlog::{info, trace};
use std::env;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use super::BurnedPart;

/// Sigmanest database interface
#[derive(Debug)]
pub struct Sndb {
    conn: super::MssqlClient
}

impl Sndb {
    /// Initialize Sigmanest database connection
    pub async fn init() -> tiberius::Result<Self> {
        info!(">> initializing Sigmanest database connector");
        
        trace!("building config");
        let user = env::var("SNDB_USER").expect("environment variable `SNDB_USER` not defined");
        let pass = env::var("SNDB_PWD").expect("environment variable `SNDB_PWD` not defined");
        let mut config = Config::new();
        config.host("hiiwinbl18");
        config.database("SNDBase91");
        config.authentication(AuthMethod::sql_server(user, pass));
        config.trust_cert(); // on production, it is not a good idea to do this
    
        trace!("opening TCP stream");
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
    
        // To be able to use Tokio's tcp, we're using the `compat_write` from
        // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
        // traits from the `futures` crate.
        let conn = Client::connect(config, tcp.compat_write()).await?;

        info!(">> Sigmanest connection successful");
    
        Ok( Self { conn } )
    }

    /// get all the parts burned in Sigmanest for the past week
    pub async fn get_parts_burned_for_week(&mut self) -> tiberius::Result<Vec<BurnedPart>> {
        trace!("fetching parts burned in the previous week");
        let results = self.conn
            .simple_query(include_str!("sql/get_parts_burned_for_week.sql"))
            .await?
            .into_first_result()
            .await?;
        
        let mut res = Vec::<BurnedPart>::new();
        for x in results {
            res.push(BurnedPart::try_from(&x)?)
        }

        Ok(res)
    }

    /// get the number of pieces burned for a given `part` name
    pub async fn get_part_burned_qty(&mut self, part: &str) -> tiberius::Result<i32> {
        trace!("fetching part burned quantity for `{}`", part);

        match self.conn
            .query(
                "select isnull( sum(QtyProgram), 0 ) from PartArchive where PartName=@P1",
                &[&part]
            )
                .await?
            .into_row()
                .await?.unwrap() // should not panic because of `isnull` in sql statement
            .get(0) {
                Some(val) => Ok(val),
                None => Ok(0)
            }
    }
}