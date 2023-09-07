use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc::Sender;
use warp::filters::ws::Message;

use crate::command::{Answer, ServerCommand};

// safe because this app is single threaded
unsafe impl Sync for Rooms {}

struct Rooms {
    rooms: RefCell<HashMap<String, Room>>,
}

struct Room {
    last_interaction: Instant,
    game: Game,
    group_count: u32,
    event_time: u32,
    members: Vec<Member>,
    /// skcid zero connections
    conns: Connections,
}

enum Game {
    Idle,
    Started { start: Instant, extra: u32 },
    Ended(Message),
}

struct Member {
    online: usize,
    name: String,
    group: u32,
    /// skcid non zero connections
    conns: Connections,
    answers: HashMap<u32, u32>,
}

struct Connections {
    senders: Vec<Arc<(Sender<Message>, AtomicBool)>>,
}

lazy_static::lazy_static! {
    static ref STATE: Rooms = {
        Rooms {
            rooms: RefCell::new(HashMap::new())
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
            group_count: 0,
            event_time: 300,
            conns: Connections::new(),
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
}

impl Member {
    fn new(index: usize) -> Self {
        Self {
            online: 0,
            name: format!("Aluno #{}", index + 1),
            group: 0,
            conns: Connections::new(),
            answers: HashMap::new(),
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
        sckid == 0 || sckid as usize - 1 < room.members.len()
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
    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    let mut messages: Vec<Message> = Vec::with_capacity(room.members.len() + 4);

    messages.push(
        ServerCommand::GroupCountChanged {
            group_count: room.group_count,
        }
        .into(),
    );
    messages.push(
        ServerCommand::GameTimeChanged {
            game_time: room.event_time,
        }
        .into(),
    );

    for (index, member) in room.members.iter().enumerate() {
        if member.online > 0 {
            messages.push(
                ServerCommand::MemberUpdated {
                    sckid: index as u32 + 1,
                    name: member.name.clone(),
                    group: member.group,
                }
                .into(),
            );
        }
    }

    match &room.game {
        Game::Idle => {}
        Game::Started { start, extra: _ } => messages.push(
            ServerCommand::Started {
                remaining: (Duration::from_secs(room.event_time as u64) - start.elapsed()).as_secs()
                    as u32,
            }
            .into(),
        ),
        Game::Ended(message) => messages.push(message.clone()),
    }

    if sckid == 0 {
        for (index, member) in room.members.iter().enumerate() {
            for (question, answer) in &member.answers {
                messages.push(
                    ServerCommand::QuestionAnswered(Answer {
                        sckid: index as u32 + 1,
                        name: member.name.clone(),
                        group: member.group,
                        question: *question,
                        answer: *answer,
                    })
                    .into(),
                );
            }
        }
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
        member.online += 1;
        if member.online == 1 {
            let name = member.name.clone();
            let group = member.group.clone();
            room.send_all(&ServerCommand::MemberUpdated { sckid, name, group }.into());
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
        if member.online > 0 {
            member.online -= 1;
        }
        if member.online == 0 {
            room.send_all(&ServerCommand::MemberLeft { sckid }.into());
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
                        answers: collect_answers(&room),
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
    let message: crate::command::RawCommand = serde_json::from_slice(message.as_bytes())?;

    let mut rooms = STATE.rooms.borrow_mut();
    let room = rooms
        .get_mut(room_id)
        .ok_or_else(|| format!("Room {} does not exist", room_id))?
        .interacted();
    if sckid == 0 {
        use crate::command::MasterCommand as Cmd;
        match Cmd::try_from(message)? {
            Cmd::Start => {
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
                room.clear_answers();
            }
            Cmd::Finish => {
                if room.game.is_started() {
                    let message = ServerCommand::Finished {
                        answers: collect_answers(&room),
                    }
                    .into();
                    room.send_all(&message);
                    room.game = Game::Ended(message);
                }
            }
            Cmd::ExtraTime(seconds) => {
                if let Game::Started { start: _, extra } = &mut room.game {
                    *extra += seconds;
                    room.send_all(&ServerCommand::ExtraTime { seconds }.into());
                }
            }
            Cmd::CloseRoom => {
                room.send_all(&ServerCommand::RoomClosed.into());
                rooms.remove(room_id);
            }
            Cmd::SetGroupCount(group_count) => {
                if room.group_count != group_count {
                    room.send_all(&ServerCommand::GroupCountChanged { group_count }.into());
                    room.group_count = group_count;
                }
            }
            Cmd::SetTime(seconds) => {
                if room.event_time != seconds {
                    room.send_all(&ServerCommand::GameTimeChanged { game_time: seconds }.into());
                    room.event_time = seconds;
                }
            }
            Cmd::Kick { sckid } => {
                if sckid != 0 && sckid as usize - 1 < room.members.len() {
                    let member = &mut room.members[sckid as usize - 1];
                    member.send(&ServerCommand::RoomClosed.into());
                    member.online = 0;
                    member.answers.clear();
                    member.group = 0;
                    member.name.clear();
                    member.conns.close();
                    room.send_all(&ServerCommand::MemberLeft { sckid }.into());
                }
            }
        }
        Ok(())
    } else if sckid as usize - 1 < room.members.len() {
        let member = &mut room.members[sckid as usize - 1];
        use crate::command::MemberCommand as Cmd;
        match Cmd::try_from(message)? {
            Cmd::SetName(name) => {
                if member.name != name {
                    member.name = name;
                    let message = ServerCommand::MemberUpdated {
                        sckid,
                        name: member.name.clone(),
                        group: member.group,
                    }
                    .into();
                    room.send_all(&message);
                }
            }
            Cmd::SetGroup(group) => {
                if member.group != group {
                    member.group = group;
                    let message = ServerCommand::MemberUpdated {
                        sckid,
                        name: member.name.clone(),
                        group: member.group,
                    }
                    .into();
                    room.send_all(&message);
                }
            }
            Cmd::Answer { question, answer } => {
                member.answers.insert(question, answer);
                let name = member.name.clone();
                let group = member.group;
                room.send_master(
                    &ServerCommand::QuestionAnswered(Answer {
                        sckid,
                        name,
                        group,
                        question,
                        answer,
                    })
                    .into(),
                );
            }
        }
        Ok(())
    } else {
        Err(format!("Member {} of Room {} does not exist", sckid, room_id).into())
    }
}

fn collect_answers(room: &Room) -> Vec<Answer> {
    let mut answers = Vec::new();
    for (index, member) in room.members.iter().enumerate() {
        for (question, answer) in &member.answers {
            answers.push(Answer {
                sckid: index as u32 + 1,
                name: member.name.clone(),
                group: member.group,
                question: *question,
                answer: *answer,
            });
        }
    }
    answers
}
