use std::{borrow::Cow, collections::HashMap};

pub struct RawCommand(HashMap<String, serde_json::Value>);

impl serde::Serialize for RawCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for RawCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <HashMap<String, serde_json::Value> as serde::Deserialize>::deserialize(deserializer)
            .map(RawCommand)
    }
}

pub enum MasterCommand {
    Start,
    Finish,
    ExtraTime {
        seconds: u32,
    },
    CloseRoom,
    CreateGroup,
    ModifyGroupName {
        group_id: u32,
        name: String,
    },
    ModifyGroupColor {
        group_id: u32,
        color: String,
    },
    RemoveGroup {
        group_id: u32,
    },
    SetTime {
        seconds: u32,
    },
    SetQuestions {
        questions: String,
    },
    Kick {
        sckid: u32,
    },
}

pub enum MemberCommand {
    SetName { name: String },
    SetGroup { group: u32 },
    Answer { question: u32, answer: u32 },
}

pub enum ServerCommand {
    Started {
        remaining: u32,
    },
    Finished {
        answers: Vec<GroupAnswers>,
    },
    ExtraTime {
        seconds: u32,
    },
    GameConfigChanged {
        game_time: u32,
        questions: String,
    },
    AnswersChanged {
        answers: Vec<GroupAnswers>,
    },
    MembersChanged {
        groups: Vec<GroupMembers>,
    },
    AnswerUpdated {
        answer: Answer,
        member: Member,
        group: Group,
    },
    MemberUpdated {
        member: Member,
    },
    GroupUpdated {
        group: Group,
    },
    MemberRemoved {
        sckid: u32,
    },
    GroupRemoved {
        group: u32,
    },
    RoomClosed,
}

pub struct Member {
    pub sckid: u32,
    pub name: String,
    pub group: u32,
}

pub struct Group {
    pub group_id: u32,
    pub name: String,
    pub color: String,
}

pub struct Answer {
    pub question: u32,
    pub answer: u32,
}
pub struct GroupMembers {
    pub group: Group,
    pub members: Vec<Member>,
}

pub struct GroupAnswers {
    pub group: Group,
    pub answers: Vec<MemberAnswers>,
}

pub struct MemberAnswers {
    pub member: Member,
    pub answers: Vec<Answer>,
}

impl TryFrom<RawCommand> for MasterCommand {
    type Error = Cow<'static, str>;
    fn try_from(mut value: RawCommand) -> Result<Self, Self::Error> {
        match value.cmd()?.as_str() {
            "start" => Ok(Self::Start),
            "finish" => Ok(Self::Finish),
            "extra_time" => Ok(Self::ExtraTime {
                seconds: value.take("seconds")?,
            }),
            "close_room" => Ok(Self::CloseRoom),
            "create_group" => Ok(Self::CreateGroup),
            "modify_group_name" => Ok(Self::ModifyGroupName {
                group_id: value.take("group")?,
                name: value.take("name")?,
            }),
            "modify_group_color" => Ok(Self::ModifyGroupColor {
                group_id: value.take("group")?,
                color: value.take("color")?,
            }),
            "remove_group" => Ok(Self::RemoveGroup {
                group_id: value.take("group")?,
            }),
            "set_time" => Ok(Self::SetTime {
                seconds: value.take("seconds")?,
            }),
            "set_questions" => Ok(Self::SetQuestions {
                questions: value.take("questions")?,
            }),
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
        match value.cmd()?.as_str() {
            "set_name" => Ok(Self::SetName {
                name: value.take("name")?,
            }),
            "set_group" => Ok(Self::SetGroup {
                group: value.take("group")?,
            }),
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
                Self::new("started", [("remaining", remaining.into_value())])
            }
            ServerCommand::Finished { answers } => {
                Self::new("finished", [("answers", answers.into_value())])
            }
            ServerCommand::ExtraTime { seconds } => {
                Self::new("extra_time", [("seconds", seconds.into())])
            }
            ServerCommand::RoomClosed => Self::new("room_closed", []),
            ServerCommand::GameConfigChanged {
                game_time,
                questions,
            } => Self::new(
                "game_config_changed",
                [
                    ("game_time", game_time.into()),
                    ("questions", questions.into()),
                ],
            ),
            ServerCommand::AnswersChanged { answers } => {
                Self::new("answers_changed", [("answers", answers.into_value())])
            }
            ServerCommand::MembersChanged { groups } => {
                Self::new("members_changed", [("groups", groups.into_value())])
            }
            ServerCommand::AnswerUpdated {
                answer,
                member,
                group,
            } => Self::new(
                "answer_updated",
                [
                    ("answer", answer.into_value()),
                    ("member", member.into_value()),
                    ("group", group.into_value()),
                ],
            ),
            ServerCommand::MemberUpdated { member } => {
                Self::new("member_updated", [("member", member.into_value())])
            }
            ServerCommand::GroupUpdated { group } => {
                Self::new("group_updated", [("group", group.into_value())])
            }
            ServerCommand::MemberRemoved { sckid } => {
                Self::new("member_removed", [("sckid", sckid.into_value())])
            }
            ServerCommand::GroupRemoved { group } => {
                Self::new("group_removed", [("group", group.into_value())])
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
        cmd: &'static str,
        args: [(&'static str, serde_json::Value); N],
    ) -> Self {
        Self(
            IntoIterator::into_iter(args)
                .map(|(x, y)| (x.to_owned(), y))
                .chain(std::iter::once(("cmd".to_string(), cmd.into())))
                .collect(),
        )
    }
    fn cmd(&mut self) -> Result<String, Cow<'static, str>> {
        self.take("cmd")
    }
    fn take<T>(&mut self, key: &'static str) -> Result<T, Cow<'static, str>>
    where
        T: TryFromValue,
    {
        let value = self.0.remove(key).ok_or(format!("key {key} not found"))?;
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

trait IntoValue {
    fn into_value(self) -> serde_json::Value;
}

impl IntoValue for u32 {
    fn into_value(self) -> serde_json::Value {
        self.into()
    }
}
impl IntoValue for String {
    fn into_value(self) -> serde_json::Value {
        self.into()
    }
}
impl IntoValue for Member {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("sckid".to_owned(), self.sckid.into_value()),
                ("name".to_owned(), self.name.into_value()),
                ("group".to_owned(), self.group.into_value()),
            ])
            .collect(),
        )
    }
}
impl IntoValue for Group {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("group".to_owned(), self.group_id.into_value()),
                ("name".to_owned(), self.name.into_value()),
                ("color".to_owned(), self.color.into_value()),
            ])
            .collect(),
        )
    }
}
impl IntoValue for Answer {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("question".to_owned(), self.question.into_value()),
                ("answer".to_owned(), self.answer.into_value()),
            ])
            .collect(),
        )
    }
}
impl IntoValue for GroupMembers {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("group".to_owned(), self.group.into_value()),
                ("members".to_owned(), self.members.into_value()),
            ])
            .collect(),
        )
    }
}
impl IntoValue for GroupAnswers {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("group".to_owned(), self.group.into_value()),
                ("answers".to_owned(), self.answers.into_value()),
            ])
            .collect(),
        )
    }
}
impl IntoValue for MemberAnswers {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(
            IntoIterator::into_iter([
                ("member".to_owned(), self.member.into_value()),
                ("answers".to_owned(), self.answers.into_value()),
            ])
            .collect(),
        )
    }
}
impl<T: IntoValue> IntoValue for Vec<T> {
    fn into_value(self) -> serde_json::Value {
        serde_json::Value::Array(self.into_iter().map(T::into_value).collect())
    }
}
