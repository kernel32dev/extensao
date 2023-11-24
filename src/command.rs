#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "cmd")]
pub enum MasterCommand {
    Start,
    Finish,
    ExtraTime { seconds: u32 },
    CloseRoom,
    SetGroupName { group: bool, name: String },
    SetGroupColor { group: bool, color: String },
    SetTime { seconds: u32 },
    SetQuestions { questions: String },
    Kick { sckid: u32 },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "cmd")]
pub enum MemberCommand {
    SetName { name: String },
    SetGroup { group: bool },
    SetPos { x: f32, y: f32, },
    Answer { question: u32, answer: u32 },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "cmd")]
pub enum ServerCommand {
    Started {
        remaining: u32,
    },
    Finished {
        answers: Vec<MemberAnswers>,
    },
    ExtraTime {
        seconds: u32,
    },
    RoomChanged {
        game_time: u32,
        questions: String,
        group_false_name: String,
        group_false_color: String,
        group_true_name: String,
        group_true_color: String,
    },
    AnswersChanged {
        answers: Vec<MemberAnswers>,
    },
    MembersChanged {
        members: Vec<Member>,
    },
    AnswerUpdated {
        answer: Answer,
        member: Member,
    },
    MemberUpdated {
        member: Member,
    },
    MemberRemoved {
        sckid: u32,
    },
    RoomClosed,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Member {
    pub sckid: u32,
    pub name: String,
    pub group: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Answer {
    pub question: u32,
    pub answer: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemberAnswers {
    pub member: Member,
    pub answers: Vec<Answer>,
}

impl From<ServerCommand> for warp::ws::Message {
    fn from(value: ServerCommand) -> Self {
        let json =
            serde_json::to_string(&value).expect("ServerCommand is always serializable into json");
        warp::ws::Message::text(json)
    }
}
