mod get;
pub use get::Get;

mod publish;
pub use publish::Publish;

mod set;
pub use set::Set;

mod subscribe;
pub use subscribe::{Subscribe, Unsubscribe};

mod ping;
pub use ping::Ping;

mod unknown;
pub use unknown::Unknown;

mod reload_save;
pub use reload_save::ReloadSave;

mod shutdown_no_save;
pub use shutdown_no_save::ShutdownNoSave;

mod shutdown_save;
pub use shutdown_save::ShutdownSave;

use crate::{connection::Connection, db::Db, frame::Frame, parser::Parser, shutdown::Shutdown};

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Publish(Publish),
    Set(Set),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Ping(Ping),
    ShutdownSave(ShutdownSave),
    ShutdownNoSave(ShutdownNoSave),
    ReloadSave(ReloadSave),
    Unknown(Unknown),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parser::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
            "unsubscribe" => Command::Unsubscribe(Unsubscribe::parse_frames(&mut parse)?),
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            "shutdownsave" => Command::ShutdownSave((ShutdownSave::parse_frames(&mut parse)?)),
            "shutdownnosave" => {
                Command::ShutdownNoSave((ShutdownNoSave::parse_frames(&mut parse)?))
            }
            "reloadsave" => Command::ReloadSave((ReloadSave::parse_frames(&mut parse)?)),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_name)));
            }
        };

        parse.finish()?;

        Ok(command)
    }

    pub(crate) async fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> crate::Result<()> {
        use Command::*;

        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            Publish(cmd) => cmd.apply(db, dst).await,
            Set(cmd) => cmd.apply(db, dst).await,
            Subscribe(cmd) => cmd.apply(db, dst, shutdown).await,
            Ping(cmd) => cmd.apply(dst).await,
            Unknown(cmd) => cmd.apply(dst).await,

            ShutdownSave(cmd) => cmd.apply(db, dst).await,
            ShutdownNoSave(cmd) => cmd.apply(db, dst).await,
            ReloadSave(cmd) => cmd.apply(db, dst).await,

            Unsubscribe(_) => Err("`Unsubscribe` is unsupported in this context".into()),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        match self {
            Command::Get(_) => "get",
            Command::Publish(_) => "pub",
            Command::Set(_) => "set",
            Command::Subscribe(_) => "subscribe",
            Command::Unsubscribe(_) => "unsubscribe",
            Command::Ping(_) => "ping",
            Command::ShutdownSave(_) => "shutdownsave",
            Command::ShutdownNoSave(_) => "shutdownnosave",
            Command::ReloadSave(_) => "reloadsave",
            Command::Unknown(cmd) => cmd.get_name(),
        }
    }
}
