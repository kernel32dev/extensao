use std::{borrow::Cow, collections::HashMap};

use serde_json::Map;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RawCommand {
    pub cmd: String,
    pub args: HashMap<String, serde_json::Value>,
}

pub enum MasterCommand {
    Start,
    Finish,
    /// seconds
    ExtraTime(u32),
    CloseRoom,
    SetGroupCount(u32),
    /// seconds
    SetTime(u32),
    Kick {
        sckid: u32,
    },
}

pub enum MemberCommand {
    SetName(String),
    SetGroup(u32),
    Answer { question: u32, answer: u32 },
}

pub enum ServerCommand {
    Started {
        remaining: u32,
    },
    Finished {
        answers: Vec<Answer>,
    },
    ExtraTime {
        seconds: u32,
    },
    RoomClosed,
    QuestionAnswered(Answer),
    GameTimeChanged {
        game_time: u32,
    },
    GroupCountChanged {
        group_count: u32,
    },
    MemberUpdated {
        sckid: u32,
        name: String,
        group: u32,
    },
    MemberLeft {
        sckid: u32,
    },
}

pub struct Answer {
    pub sckid: u32,
    pub name: String,
    pub group: u32,
    pub question: u32,
    pub answer: u32,
}

impl TryFrom<RawCommand> for MasterCommand {
    type Error = Cow<'static, str>;
    fn try_from(mut value: RawCommand) -> Result<Self, Self::Error> {
        match value.cmd.as_str() {
            "start" => Ok(Self::Start),
            "finish" => Ok(Self::Finish),
            "extra_time" => Ok(Self::ExtraTime(value.take("seconds")?)),
            "close_room" => Ok(Self::CloseRoom),
            "set_group_count" => Ok(Self::SetGroupCount(value.take("group_count")?)),
            "set_time" => Ok(Self::SetTime(value.take("seconds")?)),
            "kick" => Ok(Self::Kick {
                sckid: value.take("sckid")?,
            }),
            cmd => Err(format!(
                "expected cmd to be set_name, set_group or set_answer, found {cmd} instead"
            )
            .into()),
        }
    }
}

impl TryFrom<RawCommand> for MemberCommand {
    type Error = Cow<'static, str>;
    fn try_from(mut value: RawCommand) -> Result<Self, Self::Error> {
        match value.cmd.as_str() {
            "set_name" => Ok(Self::SetName(value.take("name")?)),
            "set_group" => Ok(Self::SetGroup(value.take("group")?)),
            "answer" => Ok(Self::Answer {
                question: value.take("question")?,
                answer: value.take("answer")?,
            }),
            cmd => Err(format!(
                "expected cmd to be set_name, set_group or set_answer, found {cmd} instead"
            )
            .into()),
        }
    }
}

impl From<ServerCommand> for RawCommand {
    fn from(value: ServerCommand) -> Self {
        match value {
            ServerCommand::Started { remaining } => {
                Self::new("started", [("remaining", remaining.into())])
            }
            ServerCommand::Finished { answers } => Self::new(
                "finished",
                [(
                    "answers",
                    serde_json::Value::Array(
                        answers
                            .into_iter()
                            .map(|x| {
                                serde_json::Value::Object(Map::from_iter(IntoIterator::into_iter(
                                    [
                                        ("sckid".to_owned(), x.sckid.into()),
                                        ("name".to_owned(), x.name.into()),
                                        ("group".to_owned(), x.group.into()),
                                        ("question".to_owned(), x.question.into()),
                                        ("answer".to_owned(), x.answer.into()),
                                    ],
                                )))
                            })
                            .collect(),
                    ),
                )],
            ),
            ServerCommand::ExtraTime { seconds } => {
                Self::new("extra_time", [("seconds", seconds.into())])
            }
            ServerCommand::RoomClosed => Self::new("room_closed", []),
            ServerCommand::QuestionAnswered(Answer {
                sckid,
                name,
                group,
                question,
                answer,
            }) => Self::new(
                "question_answered",
                [
                    ("sckid", sckid.into()),
                    ("name", name.into()),
                    ("group", group.into()),
                    ("question", question.into()),
                    ("answer", answer.into()),
                ],
            ),
            ServerCommand::GameTimeChanged { game_time } => {
                Self::new("game_time_changed", [("game_time", game_time.into())])
            }
            ServerCommand::GroupCountChanged { group_count } => {
                Self::new("group_count_changed", [("group_count", group_count.into())])
            }
            ServerCommand::MemberUpdated { sckid, name, group } => Self::new(
                "member_updated",
                [
                    ("sckid", sckid.into()),
                    ("name", name.into()),
                    ("group", group.into()),
                ],
            ),
            ServerCommand::MemberLeft { sckid } => {
                Self::new("member_left", [("sckid", sckid.into())])
            }
        }
    }
}

impl From<ServerCommand> for warp::ws::Message {
    fn from(value: ServerCommand) -> Self {
        let raw = RawCommand::from(value);
        let json =
            serde_json::to_string(&raw).expect("RawCommand is always serializable into json");
        warp::ws::Message::text(json)
    }
}

impl RawCommand {
    fn new<const N: usize>(
        cmd: impl Into<String>,
        args: [(&'static str, serde_json::Value); N],
    ) -> Self {
        Self {
            cmd: cmd.into(),
            args: IntoIterator::into_iter(args)
                .map(|(x, y)| (x.to_owned(), y))
                .collect(),
        }
    }
    fn take<T>(&mut self, key: &'static str) -> Result<T, Cow<'static, str>>
    where
        T: TryFromValue,
    {
        let value = self
            .args
            .remove(key)
            .ok_or(format!("key {key} not found"))?;
        T::try_from_value(value)
    }
}

trait TryFromValue: Sized {
    fn try_from_value(value: serde_json::Value) -> Result<Self, Cow<'static, str>>;
}

impl TryFromValue for u32 {
    fn try_from_value(value: serde_json::Value) -> Result<Self, Cow<'static, str>> {
        match value.as_u64() {
            Some(x) => Ok(x as u32),
            None => Err("expected unsigned number".into()),
        }
    }
}

impl TryFromValue for String {
    fn try_from_value(value: serde_json::Value) -> Result<Self, Cow<'static, str>> {
        match value.as_str() {
            Some(x) => Ok(x.to_owned()),
            None => Err("expected string".into()),
        }
    }
}
