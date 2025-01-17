use crate::utils;
use bytes_utils::Str;

#[cfg(feature = "client-tracking")]
use crate::{
  error::{RedisError, RedisErrorKind},
  types::{Message, RedisKey, RedisValue, Server},
};

/// The type of clients to close.
///
/// <https://redis.io/commands/client-kill>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientKillType {
  Normal,
  Master,
  Replica,
  Pubsub,
}

impl ClientKillType {
  pub(crate) fn to_str(&self) -> Str {
    utils::static_str(match *self {
      ClientKillType::Normal => "normal",
      ClientKillType::Master => "master",
      ClientKillType::Replica => "replica",
      ClientKillType::Pubsub => "pubsub",
    })
  }
}

/// Filters provided to the CLIENT KILL command.
///
/// <https://redis.io/commands/client-kill>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientKillFilter {
  ID(String),
  Type(ClientKillType),
  User(String),
  Addr(String),
  LAddr(String),
  SkipMe(bool),
}

impl ClientKillFilter {
  pub(crate) fn to_str(&self) -> (Str, Str) {
    let (prefix, value) = match *self {
      ClientKillFilter::ID(ref id) => ("ID", id.into()),
      ClientKillFilter::Type(ref kind) => ("TYPE", kind.to_str()),
      ClientKillFilter::User(ref user) => ("USER", user.into()),
      ClientKillFilter::Addr(ref addr) => ("ADDR", addr.into()),
      ClientKillFilter::LAddr(ref addr) => ("LADDR", addr.into()),
      ClientKillFilter::SkipMe(ref b) => ("SKIPME", match *b {
        true => utils::static_str("yes"),
        false => utils::static_str("no"),
      }),
    };

    (utils::static_str(prefix), value)
  }
}

/// Filters for the CLIENT PAUSE command.
///
/// <https://redis.io/commands/client-pause>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientPauseKind {
  Write,
  All,
}

impl ClientPauseKind {
  pub(crate) fn to_str(&self) -> Str {
    utils::static_str(match *self {
      ClientPauseKind::Write => "WRITE",
      ClientPauseKind::All => "ALL",
    })
  }
}

/// Arguments for the CLIENT REPLY command.
///
/// <https://redis.io/commands/client-reply>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientReplyFlag {
  On,
  Off,
  Skip,
}

impl ClientReplyFlag {
  pub(crate) fn to_str(&self) -> Str {
    utils::static_str(match *self {
      ClientReplyFlag::On => "ON",
      ClientReplyFlag::Off => "OFF",
      ClientReplyFlag::Skip => "SKIP",
    })
  }
}

/// Arguments to the CLIENT UNBLOCK command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientUnblockFlag {
  Timeout,
  Error,
}

impl ClientUnblockFlag {
  pub(crate) fn to_str(&self) -> Str {
    utils::static_str(match *self {
      ClientUnblockFlag::Timeout => "TIMEOUT",
      ClientUnblockFlag::Error => "ERROR",
    })
  }
}

/// An `ON|OFF` flag used with client tracking commands.
#[cfg(feature = "client-tracking")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-tracking")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Toggle {
  On,
  Off,
}

#[cfg(feature = "client-tracking")]
impl Toggle {
  pub(crate) fn to_str(&self) -> &'static str {
    match self {
      Toggle::On => "ON",
      Toggle::Off => "OFF",
    }
  }

  pub(crate) fn from_str(s: &str) -> Option<Self> {
    Some(match s {
      "ON" | "on" => Toggle::On,
      "OFF" | "off" => Toggle::Off,
      _ => return None,
    })
  }
}

#[cfg(feature = "client-tracking")]
impl TryFrom<&str> for Toggle {
  type Error = RedisError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Toggle::from_str(value).ok_or(RedisError::new(RedisErrorKind::Parse, "Invalid toggle value."))
  }
}

#[cfg(feature = "client-tracking")]
impl TryFrom<String> for Toggle {
  type Error = RedisError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Toggle::from_str(&value).ok_or(RedisError::new(RedisErrorKind::Parse, "Invalid toggle value."))
  }
}

#[cfg(feature = "client-tracking")]
impl TryFrom<&String> for Toggle {
  type Error = RedisError;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    Toggle::from_str(&value).ok_or(RedisError::new(RedisErrorKind::Parse, "Invalid toggle value."))
  }
}

#[cfg(feature = "client-tracking")]
impl From<bool> for Toggle {
  fn from(value: bool) -> Self {
    if value {
      Toggle::On
    } else {
      Toggle::Off
    }
  }
}

/// A [client tracking](https://redis.io/docs/manual/client-side-caching/) invalidation message from the provided server.
#[cfg(feature = "client-tracking")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-tracking")))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invalidation {
  pub keys:   Vec<RedisKey>,
  pub server: Server,
}

#[cfg(feature = "client-tracking")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-tracking")))]
impl Invalidation {
  pub(crate) fn from_message(message: Message, server: &Server) -> Option<Invalidation> {
    Some(Invalidation {
      keys:   match message.value {
        RedisValue::Array(values) => values.into_iter().filter_map(|v| v.try_into().ok()).collect(),
        RedisValue::String(s) => vec![s.into()],
        RedisValue::Bytes(b) => vec![b.into()],
        RedisValue::Double(f) => vec![f.into()],
        RedisValue::Integer(i) => vec![i.into()],
        RedisValue::Boolean(b) => vec![b.into()],
        RedisValue::Null => vec![],
        _ => {
          trace!("Dropping invalid invalidation message.");
          return None;
        },
      },
      server: server.clone(),
    })
  }
}
