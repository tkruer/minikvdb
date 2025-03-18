use crate::{
    connection::Connection,
    db::Db,
    parser::{Parser, ParserError},
};
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ReloadSave {}

impl ReloadSave {
    pub fn new() -> ReloadSave {
        ReloadSave {}
    }

    pub(crate) async fn apply(self, _db: &Db, _dst: &mut Connection) -> crate::Result<()> {
        // db.set(self.key, self.value, self.expire);
        //
        // let response = Frame::Simple("OK".to_string());
        // debug!(?response);
        // dst.write_frame(&response).await?;
        //
        Ok(())
    }
    pub(crate) fn parse_frames(_parse: &mut Parser) -> crate::Result<ReloadSave> {
        Ok(ReloadSave {})
    }
}
