use crate::moving::Move;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::{Arc, Mutex},
};

pub struct Engine {
    pub id: Arc<str>,
    pub author: Arc<str>,
    etc: HashMap<String, String>,
}

pub struct Handler {
    pub engine: Option<Engine>,
    pub process: Child,
    messages: Arc<Mutex<VecDeque<MessageKind>>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GoSubcommand {
    SearchMoves(Vec<Move>),
    Ponder,
    WTime(u64),
    BTime(u64),
    WInc(u64),
    BInc(u64),
    MovesToGo(u64),
    Depth(u64),
    Nodes(u64),
    Mate(u64),
    MoveTime(u64),
    Infinite,
}

#[derive(PartialEq, Eq, Debug)]
pub enum IdSubcommand {
    Name(Arc<str>),
    Author(Arc<str>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum ScoreSubcommand {
    Cp(i64),
    Mate(i64),
}

#[derive(PartialEq, Eq, Debug)]
pub enum InfoSubcommand {
    Depth(u64),
    SelDepth(u64),
    Time(u64),
    Nodes(u64),
    Pv(Vec<Move>),
    Score(ScoreSubcommand),
    CurrMove(Move),
    CurrMoveNumber(u64),
    HashFull(u64),
    Nps(u64),
}

#[derive(PartialEq, Eq, Debug)]
pub enum OptionType {
    Check(bool),
    Spin(i64, i64, i64),
    Combo(Arc<str>, Vec<String>),
    Button,
    String(),
}

#[derive(PartialEq, Eq, Debug)]
pub enum MessageKind {
    // GUI -> Engine
    Uai,
    Debug(bool),
    IsReady,
    SetOption {
        name: Arc<str>,
        value: Arc<str>,
    },
    UaiNewGame,
    Position {
        fen: Arc<str>,
        moves: Vec<Move>,
    },
    Go(Vec<GoSubcommand>),
    Stop,
    PonderHit,
    Quit,

    // Engine -> GUI
    Id(IdSubcommand),
    UaiOk,
    ReadyOk,
    BestMove {
        bestmove: Move,
        ponder: Option<Move>,
    },
    Info(Vec<InfoSubcommand>),
    Option {
        name: Arc<str>,
        type_: OptionType,
    },
}

#[derive(Debug)]
pub enum MessageError {
    Malformed(String),
    Invalid(String),
    WrongType(String),
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageError::Malformed(s) => write!(f, "malformed message: {}", s),
            MessageError::Invalid(s) => write!(f, "invalid message: {}", s),
            MessageError::WrongType(s) => write!(f, "wrong type: {}", s),
        }
    }
}

impl Error for MessageError {}

impl MessageKind {
    fn parse_id(words: &[&str]) -> Result<Self, MessageError> {
        let mut iter = words.iter();
        match *iter.next().unwrap() {
            "name" => {
                // Steal rest of the words, whitespaces can occur in names
                let name = iter.map(|s| *s).collect::<Vec<_>>().join(" ");
                Ok(MessageKind::Id(IdSubcommand::Name(name.into())))
            }
            "author" => {
                let author = iter.map(|s| *s).collect::<Vec<_>>().join(" ");
                Ok(MessageKind::Id(IdSubcommand::Author(author.into())))
            }
            t => Err(MessageError::Invalid(format!(
                "invalid id subcommand {}",
                t
            ))),
        }
    }

    fn parse_best_move(words: &[&str]) -> Result<Self, MessageError> {
        let mut iter = words.iter();
        let bestmove = (*iter.next().unwrap()).try_into().unwrap();
        let mut ponder = None;
        if let Some(word) = iter.next() {
            if *word == "ponder" {
                let p = *iter.next().unwrap();
                ponder = Some(p.try_into().unwrap());
            }
        }
        Ok(MessageKind::BestMove { bestmove, ponder })
    }

    fn parse_info(words: &[&str]) -> Result<Self, MessageError> {
        let mut iter = words.iter().peekable();
        let mut sub_commands = Vec::new();
        while let Some(word) = iter.next() {
            match *word {
                "depth" => {
                    let depth = iter.clone().next().unwrap().parse().unwrap();
                    sub_commands.push(InfoSubcommand::Depth(depth));
                }
                "seldepth" => {
                    let seldepth = iter.next().unwrap().parse().unwrap();
                    sub_commands.push(InfoSubcommand::SelDepth(seldepth));
                }
                "time" => {
                    let time = iter.next().unwrap().parse().unwrap();
                    sub_commands.push(InfoSubcommand::Time(time));
                }
                "nodes" => {
                    let nodes = iter.next().unwrap().parse().unwrap();
                    sub_commands.push(InfoSubcommand::Nodes(nodes));
                }
                "pv" => {
                    let mut pv = Vec::new();
                    while let Some(word) = iter.peek() {
                        if let Ok(m) = (**word).try_into() {
                            pv.push(m);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    sub_commands.push(InfoSubcommand::Pv(pv));
                }
                "score" => match *iter.next().unwrap() {
                    "cp" => {
                        let cp = iter.next().unwrap().parse().unwrap();
                        sub_commands.push(InfoSubcommand::Score(ScoreSubcommand::Cp(cp)));
                    }
                    "mate" => {
                        let mate = iter.next().unwrap().parse().unwrap();
                        sub_commands.push(InfoSubcommand::Score(ScoreSubcommand::Mate(mate)));
                    }
                    _ => panic!("invalid score subcommand"),
                },
                "currmove" => {
                    let currmove = (*iter.next().unwrap()).try_into().unwrap();
                    sub_commands.push(InfoSubcommand::CurrMove(currmove));
                }
                "currmovenumber" => {
                    let currmovenumber = (*iter.next().unwrap()).parse().unwrap();
                    sub_commands.push(InfoSubcommand::CurrMoveNumber(currmovenumber));
                }
                "hashfull" => {
                    let hashfull = (*iter.next().unwrap()).parse().unwrap();
                    sub_commands.push(InfoSubcommand::HashFull(hashfull));
                }
                "nps" => {
                    let nps = (*iter.next().unwrap()).parse().unwrap();
                    sub_commands.push(InfoSubcommand::Nps(nps));
                }
                t => {
                    Err(MessageError::Invalid(format!(
                        "invalid info subcommand {}",
                        t
                    )))?;
                }
            }
        }
        Ok(MessageKind::Info(sub_commands))
    }
}

impl TryFrom<String> for MessageKind {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let words: Vec<&str> = value.split_ascii_whitespace().collect();
        let cmd = words
            .first()
            .ok_or(MessageError::Malformed(value.clone()))?;
        match *cmd {
            "id" => MessageKind::parse_id(&words[1..]),
            "bestmove" => MessageKind::parse_best_move(&words[1..]),
            "uaiok" => Ok(MessageKind::UaiOk),
            _ => Err(MessageError::Malformed(value)),
        }
    }

    type Error = MessageError;
}

impl From<MessageKind> for String {
    fn from(value: MessageKind) -> Self {
        match value {
            MessageKind::Uai => "uai".into(),
            MessageKind::IsReady => "isready".into(),
            MessageKind::UaiNewGame => "newgame".into(),
            MessageKind::Position { fen, moves } => {
                let mut s = "position".to_string();
                s.push_str(&format!(" fen {}", fen));
                if !moves.is_empty() {
                    s.push_str(&format!(
                        " moves {}",
                        moves
                            .iter()
                            .map(|m| String::from(m))
                            .collect::<Vec<_>>()
                            .join(" ")
                    ));
                }
                s
            }
            MessageKind::Go(subcommands) => {
                let mut s = "go".to_string();
                for command in subcommands {
                    match command {
                        GoSubcommand::SearchMoves(moves) => {
                            s.push_str(&format!(
                                " searchmoves {}",
                                moves
                                    .iter()
                                    .map(|m| String::from(m))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            ));
                        }
                        GoSubcommand::Ponder => s.push_str(" ponder"),
                        GoSubcommand::WTime(time) => {
                            s.push_str(&format!(" wtime {}", time));
                        }
                        GoSubcommand::BTime(time) => {
                            s.push_str(&format!(" btime {}", time));
                        }
                        GoSubcommand::WInc(_) => todo!(),
                        GoSubcommand::BInc(_) => todo!(),
                        GoSubcommand::MovesToGo(_) => todo!(),
                        GoSubcommand::Depth(_) => todo!(),
                        GoSubcommand::Nodes(_) => todo!(),
                        GoSubcommand::Mate(_) => todo!(),
                        GoSubcommand::MoveTime(_) => todo!(),
                        GoSubcommand::Infinite => s.push_str(" infinite"),
                    }
                }
                s
            }
            MessageKind::Stop => "stop".into(),
            t => unimplemented!("{:?}", t),
        }
    }
}

impl Handler {
    pub fn new(path: PathBuf, args: Vec<String>) -> Self {
        let mut child = Command::new(path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .args(args)
            .spawn()
            .expect("failed to execute process");
        let mut stdout = child.stdout.take().unwrap();
        let mut stdout_reader = BufReader::new(stdout);
        let arc = Arc::new(Mutex::new(VecDeque::new()));
        std::thread::spawn({
            let arc = arc.clone();
            move || loop {
                let message = {
                    let stdout: &mut BufReader<ChildStdout> = &mut stdout_reader;
                    let mut buffer = String::new();
                    stdout.read_line(&mut buffer);

                    MessageKind::try_from(buffer)
                };
                {
                    if let Ok(message) = message {
                        arc.lock().unwrap().push_back(message);
                    }
                }
            }
        });
        Self {
            engine: None,
            process: child,
            messages: arc,
        }
    }

    pub fn auth(&mut self) -> Result<(), Box<dyn Error>> {
        self.write(MessageKind::Uai)?;
        let mut name = None;
        let mut author = None;
        loop {
            let msg = self.read_blocking();
            match msg {
                MessageKind::Id(subcommand) => match subcommand {
                    IdSubcommand::Name(n) => name = Some(n),
                    IdSubcommand::Author(a) => author = Some(a),
                },
                MessageKind::UaiOk => {
                    if name.is_some() && author.is_some() {
                        self.engine = Some(Engine {
                            id: name.unwrap(),
                            author: author.unwrap(),
                            etc: HashMap::new(),
                        });
                        break;
                    } else {
                        return Err(Box::new(MessageError::WrongType("uaiok".into())));
                    }
                }
                m => {
                    return Err(Box::new(MessageError::WrongType(format!("{:?}", m))));
                }
            }
        }
        Ok(())
    }

    pub fn read(&self) -> Option<MessageKind> {
        let msg = self.messages.lock().unwrap().pop_front();
        msg
    }

    pub fn read_blocking(&self) -> MessageKind {
        loop {
            if let Some(message) = self.read() {
                return message;
            }
        }
    }

    pub fn write(&mut self, message: MessageKind) -> Result<(), Box<dyn Error>> {
        let mut stdin = self.process.stdin.take().unwrap();
        let mut s: String = message.into();

        s.push('\n');
        stdin.write_all(s.as_bytes())?;
        self.process.stdin = Some(stdin);
        Ok(())
    }

    pub fn get_name(&self) -> &str {
        &*self.engine.as_ref().unwrap().id
    }
}
