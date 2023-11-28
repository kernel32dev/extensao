use rand::Rng;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc::Sender;
use warp::filters::ws::Message;

use crate::command::{self, Answer, ServerCommand};

// safe because this app is single threaded
unsafe impl Sync for Rooms {}

struct Rooms {
    rooms: RefCell<BTreeMap<String, Room>>,
}

struct Room {
    last_interaction: Instant,
    game: Game,
    event_time: u32,
    question_pool: String,
    members: Vec<Member>,
    group_false_name: String,
    group_false_color: String,
    group_true_name: String,
    group_true_color: String,
    /// skcid zero connections
    conns: Connections,
}

enum Game {
    Idle,
    Started { start: Instant, extra: u32 },
    Ended(Message),
}

struct Member {
    sckid: u32,
    online: usize,
    name: String,
    group: bool,
    /// skcid non zero connections
    conns: Connections,
    answers: BTreeMap<u32, u32>,
    kicked: bool,
    x: f32,
    y: f32,
}

struct Connections {
    senders: Vec<Arc<(Sender<Message>, AtomicBool)>>,
}

lazy_static::lazy_static! {
    static ref STATE: Rooms = {
        Rooms {
            rooms: RefCell::new(BTreeMap::new())
        }
    };
}

fn random_room_code() -> String {
    let mut rng = rand::thread_rng();
    fn gen_consonant(rng: &mut impl rand::Rng) -> char {
        loop {
            let char = rng.gen_range('A'..='Z');
            if !matches!(char, 'A' | 'E' | 'I' | 'O' | 'U') {
                return char;
            }
        }
    }
    fn gen_vowel(rng: &mut impl rand::Rng) -> char {
        const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
        VOWELS[rng.gen_range(0..5)]
    }
    String::from_iter(IntoIterator::into_iter([
        gen_consonant(&mut rng),
        gen_vowel(&mut rng),
        gen_consonant(&mut rng),
    ]))
}

impl Game {
    fn is_started(&self) -> bool {
        matches!(self, Self::Started { .. })
    }
}

impl Room {
    fn new() -> Self {
        Self {
            last_interaction: Instant::now(),
            game: Game::Idle,
            event_time: 300,
            question_pool: "default".to_owned(),
            conns: Connections::new(),
            group_false_name: "Grupo Vermelho".to_owned(),
            group_false_color: "170,68,68".to_owned(),
            group_true_name: "Grupo Azul".to_owned(),
            group_true_color: "68,68,170".to_owned(),
            members: Vec::new(),
        }
    }
    fn send_all(&mut self, message: &Message) {
        self.send_master(message);
        self.send_members(message);
    }
    fn send_master(&mut self, message: &Message) {
        self.conns.send(message);
    }
    fn send_members(&mut self, message: &Message) {
        for member in &mut self.members {
            member.send(message);
        }
    }
    fn interacted(&mut self) -> &mut Self {
        self.last_interaction = Instant::now();
        self
    }
    fn clear_answers(&mut self) {
        for member in &mut self.members {
            member.answers.clear();
        }
    }

    fn get_answers(&self) -> Vec<command::MemberAnswers> {
        self.members
            .iter()
            .filter(|x| x.online != 0 && !x.kicked)
            .map(|x| command::MemberAnswers {
                member: x.into(),
                answers: x
                    .answers
                    .iter()
                    .map(|(&question, &answer)| command::Answer { question, answer })
                    .collect(),
            })
            .collect()
    }

    fn get_group_members(&self) -> Vec<command::Member> {
        self.members
            .iter()
            .filter(|x| x.online != 0 && !x.kicked)
            .map(|x| x.into())
            .collect()
    }

    fn into_message(&self) -> command::ServerCommand {
        command::ServerCommand::RoomChanged {
            game_time: self.event_time,
            question_pool: self.question_pool.clone(),
            group_false_name: self.group_false_name.clone(),
            group_false_color: self.group_false_color.clone(),
            group_true_name: self.group_true_name.clone(),
            group_true_color: self.group_true_color.clone(),
        }
    }
}

impl From<&Member> for command::Member {
    fn from(value: &Member) -> Self {
        Self {
            sckid: value.sckid,
            name: value.name.clone(),
            group: value.group.clone(),
            x: value.x,
            y: value.y,
            answers: value.answers.len() as u32,
        }
    }
}

impl Member {
    fn new(index: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            sckid: index as u32 + 1,
            online: 0,
            name: format!("Aluno #{}", index + 1),
            group: rand::random(),
            conns: Connections::new(),
            answers: BTreeMap::new(),
            kicked: false,
            x: rng.gen_range(0.0..=100.0),
            y: rng.gen_range(0.0..=100.0),
        }
    }
    fn send(&mut self, message: &Message) {
        self.conns.send(message);
    }
}

impl Connections {
    fn new() -> Self {
        Self {
            senders: Vec::new(),
        }
    }
    fn push(&mut self, sender: Sender<Message>) {
        self.senders.push(Arc::new((sender, AtomicBool::new(true))));
    }
    fn send(&mut self, message: &Message) {
        let mut index = 0;
        while index < self.senders.len() {
            let sender = &self.senders[index];
            if sender.1.load(Ordering::Relaxed) {
                let sender = Arc::clone(sender);
                index += 1;
                let message = message.clone();
                tokio::spawn(async move {
                    if sender.0.send(message).await.is_err() {
                        sender.1.store(false, Ordering::Relaxed);
                    }
                });
            } else {
                self.senders.swap_remove(index);
            }
        }
    }
    fn close(&mut self) {
        self.senders.clear()
    }
}

pub fn create_room() -> Result<String, ()> {
    let mut rooms = STATE.rooms.borrow_mut();
    for _ in 0..30 {
        let code = random_room_code();
        if !rooms.contains_key(&code) {
            rooms.insert(code.clone(), Room::new());
            return Ok(code);
        }
    }
    Err(())
}

pub fn join_room(room: &str) -> Result<u32, ()> {
    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms.get_mut(room).ok_or(())?.interacted();
    if room.game.is_started() {
        return Err(());
    }
    let index = room.members.len();
    room.members.push(Member::new(index));
    Ok(index as u32 + 1)
}

pub fn check_exists(room: &str, sckid: u32) -> bool {
    let rooms = STATE.rooms.borrow();
    if let Some(room) = rooms.get(room) {
        sckid == 0
            || (sckid as usize - 1 < room.members.len() && !room.members[sckid as usize - 1].kicked)
    } else {
        false
    }
}

pub fn connect_room(room: &str, sckid: u32) -> Result<tokio::sync::mpsc::Receiver<Message>, ()> {
    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms.get_mut(room).ok_or(())?.interacted();
    if sckid as usize > room.members.len() {
        return Err(());
    }
    if sckid != 0 && room.members[sckid as usize - 1].kicked {
        return Err(());
    }
    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    let mut messages: Vec<Message> = Vec::with_capacity(5);

    messages.push(room.into_message().into());
    messages.push(
        ServerCommand::MembersChanged {
            members: room.get_group_members(),
        }
        .into(),
    );
    for member in &room.members {
        if member.online != 0 && !member.kicked {
            messages.push(
                ServerCommand::MemberUpdated {
                    member: member.into(),
                }
                .into(),
            );
        }
    }

    match &room.game {
        Game::Idle => {}
        Game::Started { start, extra } => {
            let answers = if sckid == 0 {
                room.get_answers()
            } else {
                let member = (&room.members[sckid as usize - 1]).into();
                let answers = room.members[sckid as usize - 1]
                    .answers
                    .iter()
                    .map(|(&question, &answer)| Answer { question, answer })
                    .collect();
                vec![crate::command::MemberAnswers { member, answers }]
            };
            messages.push(ServerCommand::AnswersChanged { answers }.into());
            messages.push(
                ServerCommand::Started {
                    remaining: (Duration::from_secs((room.event_time + extra) as u64)
                        - start.elapsed())
                    .as_secs() as u32,
                }
                .into(),
            );
        }
        Game::Ended(message) => messages.push(message.clone()),
    }

    tokio::spawn({
        let sender = sender.clone();
        async move {
            for message in messages {
                let _ = sender.send(message).await;
            }
        }
    });

    if sckid == 0 {
        room.conns.push(sender);
    } else {
        let member = &mut room.members[sckid as usize - 1];
        member.conns.push(sender);
    }
    Ok(receiver)
}

pub fn increment_online(room: &str, sckid: u32) -> Result<(), ()> {
    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms.get_mut(room).ok_or(())?.interacted();
    if sckid == 0 {
        // connection count of sckid 0 is not tracked
        return Ok(());
    }
    if sckid as usize - 1 < room.members.len() {
        let member = &mut room.members[sckid as usize - 1];
        if !member.kicked {
            member.online += 1;
            if member.online == 1 {
                let updated = ServerCommand::MemberUpdated {
                    member: (&*member).into(),
                };
                let changed = ServerCommand::MembersChanged {
                    members: room.get_group_members(),
                };
                room.send_all(&updated.into());
                room.send_all(&changed.into());
            }
        }
        Ok(())
    } else {
        Err(())
    }
}

pub fn decrement_online(room: &str, sckid: u32) -> Result<(), ()> {
    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms.get_mut(room).ok_or(())?.interacted();
    if sckid == 0 {
        // connection count of sckid 0 is not tracked
        return Ok(());
    }
    if sckid as usize - 1 < room.members.len() {
        let member = &mut room.members[sckid as usize - 1];
        if !member.kicked {
            if member.online > 0 {
                member.online -= 1;
            }
            if member.online == 0 {
                let removed = ServerCommand::MemberRemoved { sckid };
                let changed = ServerCommand::MembersChanged {
                    members: room.get_group_members(),
                };
                room.send_all(&removed.into());
                room.send_all(&changed.into());
            }
        }
        Ok(())
    } else {
        Err(())
    }
}

pub struct AnyError {
    pub err: String,
}

impl<T: std::fmt::Debug> From<T> for AnyError {
    fn from(value: T) -> Self {
        Self {
            err: format!("{:?}", value),
        }
    }
}

/// called every second
pub fn periodic_routine(tick: usize) {
    if tick % 1 == 0 {
        let mut rooms = STATE.rooms.borrow_mut();
        for room in rooms.values_mut() {
            if let Game::Started { start, extra } = room.game {
                let elapsed = start.elapsed();
                if elapsed > Duration::from_secs((room.event_time + extra) as u64) {
                    let message = ServerCommand::Finished {
                        answers: room.get_answers(),
                    }
                    .into();
                    room.send_all(&message);
                    room.game = Game::Ended(message);
                    println!("[T] Game ended");
                    dbg!(elapsed);
                    dbg!(elapsed.as_secs());
                    dbg!(room.event_time);
                    dbg!(extra);
                }
            }
        }
    }
    if tick % 3 == 0 {
        let mut rooms = STATE.rooms.borrow_mut();
        while let Some(key) = rooms
            .iter_mut()
            .filter(|(_, room)| room.last_interaction.elapsed() > Duration::from_secs(3600))
            .map(|(key, _)| key.clone())
            .next()
        {
            if let Some(mut room) = rooms.remove(&key) {
                room.send_all(&ServerCommand::RoomClosed.into());
            }
            println!("[T] Room removed");
        }
    }
}

pub fn handle_message(room_id: &str, sckid: u32, message: Message) -> Result<(), AnyError> {
    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms
        .get_mut(room_id)
        .ok_or_else(|| format!("Room {} does not exist", room_id))?
        .interacted();
    if sckid == 0 {
        use crate::command::MasterCommand as Cmd;
        match serde_json::from_slice(message.as_bytes())? {
            Cmd::Start => {
                room.clear_answers();
                let member_updated = room
                    .members
                    .iter()
                    .filter(|x| x.online != 0 && !x.kicked)
                    .map(|x| ServerCommand::MemberUpdated { member: x.into() })
                    .collect::<Vec<_>>();
                for i in member_updated {
                    room.send_master(&i.into());
                }
                room.send_all(
                    &ServerCommand::Started {
                        remaining: room.event_time,
                    }
                    .into(),
                );
                room.game = Game::Started {
                    start: Instant::now(),
                    extra: 0,
                };
            }
            Cmd::Finish => {
                if room.game.is_started() {
                    let message = ServerCommand::Finished {
                        answers: room.get_answers(),
                    }
                    .into();
                    room.send_all(&message);
                    room.game = Game::Ended(message);
                }
            }
            Cmd::ExtraTime { seconds } => {
                if let Game::Started { start: _, extra } = &mut room.game {
                    *extra += seconds;
                    room.send_all(&ServerCommand::ExtraTime { seconds }.into());
                }
            }
            Cmd::CloseRoom => {
                room.send_all(&ServerCommand::RoomClosed.into());
                rooms.remove(room_id);
            }
            Cmd::SetGroupName { group, name } => {
                if group {
                    room.group_true_name = name;
                } else {
                    room.group_false_name = name;
                }
                room.send_all(&room.into_message().into());
            }
            Cmd::SetGroupColor { group, color } => {
                if group {
                    room.group_true_color = color;
                } else {
                    room.group_false_color = color;
                }
                room.send_all(&room.into_message().into());
            }
            Cmd::SetTime { seconds } => {
                if room.event_time != seconds {
                    room.event_time = seconds;
                    room.send_all(&room.into_message().into());
                }
            }
            Cmd::SetQuestionPool { question_pool } => {
                if room.question_pool != question_pool {
                    room.question_pool = question_pool;
                    room.send_all(&room.into_message().into());
                }
            }
            Cmd::Kick { sckid } => {
                if sckid != 0 && sckid as usize - 1 < room.members.len() {
                    let member = &mut room.members[sckid as usize - 1];
                    member.send(&ServerCommand::RoomClosed.into());
                    member.online = 0;
                    member.answers.clear();
                    member.name.clear();
                    member.conns.close();
                    member.kicked = true;
                    room.send_all(&ServerCommand::MemberRemoved { sckid }.into());
                    room.send_all(
                        &ServerCommand::MembersChanged {
                            members: room.get_group_members(),
                        }
                        .into(),
                    );
                }
            }
        }
        Ok(())
    } else if sckid as usize - 1 < room.members.len() {
        let member = &mut room.members[sckid as usize - 1];
        use crate::command::MemberCommand as Cmd;
        match serde_json::from_slice(message.as_bytes())? {
            Cmd::SetName { name } => {
                if member.name != name {
                    member.name = name;
                    let message = ServerCommand::MemberUpdated {
                        member: (&*member).into(),
                    }
                    .into();
                    room.send_all(&message);
                    room.send_all(
                        &ServerCommand::MembersChanged {
                            members: room.get_group_members(),
                        }
                        .into(),
                    );
                }
            }
            Cmd::SetGroup { group } => {
                if member.group != group {
                    member.group = group;
                    let message = ServerCommand::MemberUpdated {
                        member: (&*member).into(),
                    }
                    .into();
                    room.send_all(&message);
                    room.send_all(
                        &ServerCommand::MembersChanged {
                            members: room.get_group_members(),
                        }
                        .into(),
                    );
                }
            }
            Cmd::SetPos { x, y } => {
                member.x = x;
                member.y = y;
                let message = ServerCommand::MemberUpdated {
                    member: (&*member).into(),
                }
                .into();
                room.send_all(&message);
            }
            Cmd::Answer { question, answer } => {
                member.answers.insert(question, answer);
                let member: command::Member = (&*member).into();
                room.send_master(
                    &ServerCommand::MemberUpdated {
                        member: member.clone(),
                    }
                    .into(),
                );
                room.send_master(
                    &ServerCommand::AnswerUpdated {
                        answer: Answer { question, answer },
                        member,
                    }
                    .into(),
                );
                room.send_master(
                    &ServerCommand::AnswersChanged {
                        answers: room.get_answers(),
                    }
                    .into(),
                );
            }
        }
        Ok(())
    } else {
        Err(format!("Member {} of Room {} does not exist", sckid, room_id).into())
    }
}
