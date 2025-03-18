use crate::{
    connection::Connection,
    db::Db,
    dumper,
    frame::Frame,
    parser::{Parser, ParserError},
};
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ShutdownSave {}

impl ShutdownSave {
    pub fn new() -> ShutdownSave {
        ShutdownSave {}
    }

    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let data = db.get_all();
        dumper::dump_kv_store(data, Some(crate::SAVE_PATH));
        let response = Frame::Simple("OK".to_string());
        debug!(?response);
        dst.write_frame(&response).await?;

        Ok(())
    }

    pub(crate) fn parse_frames(_parse: &mut Parser) -> crate::Result<ShutdownSave> {
        Ok(ShutdownSave {})
    }
}
