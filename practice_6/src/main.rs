#[derive(Debug, Clone, PartialEq)]
enum Command {
    SetPixel { index: u16, r: u8, g: u8, b: u8 },
    Fill { r: u8, g: u8, b: u8 },
    SetBrightness(u8),
    ReadPixel { index: u16 },
}

impl Command {
    fn kind(&self) -> CommandId {
        match self {
            Command::SetPixel { .. } => CommandId::SetPixel,
            Command::Fill { .. } => CommandId::Fill,
            Command::SetBrightness { .. } => CommandId::SetBrightness,
            Command::ReadPixel { .. } => CommandId::SetPixel,
        }
    }

    fn id(&self) -> u8 {
        self.kind() as u8
    }

    fn encode(&self) -> Vec<u8> {
        // this is the full command byte structure
        match self {
            Command::SetPixel { index, r, g, b } => {
                let hi = (index << 8) as u8;
                let lo = *index as u8;
                vec![hi, lo, *r, *g, *b]
            }
            Command::Fill { r, g, b } => vec![*r, *g, *b],
            Command::SetBrightness(b) => vec![*b],
            Command::ReadPixel { index } => {
                let hi = (index << 8) as u8;
                let lo = *index as u8;
                vec![hi, lo]
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ProtoError {
    BadChecksum { expected: u8, got: u8 },
    UnknownComamnd(u8),
    BadLength { cmd: u8, len: usize },
    Truncated,
}
impl std::fmt::Display for ProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProtoError::BadChecksum { expected, got } => write!(
                f,
                "Error: bad checksum. expected {expected}, got {got} instead"
            ),
            ProtoError::UnknownComamnd(c) => write!(f, "Error: unknown command {c}"),
            ProtoError::BadLength { cmd, len } => {
                write!(f, "Error: size {len} illegal for command {cmd}")
            }
            ProtoError::Truncated => write!(f, "Error: command truncated"),
        }
    }
}
impl std::error::Error for ProtoError {}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum CommandId {
    SetPixel = 0x2,
    Fill = 0x3,
    SetBrightness = 0x4,
    ReadPixel = 0x5,
}

impl TryFrom<u8> for CommandId {
    type Error = ProtoError;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        use CommandId::*;
        [SetPixel, Fill, SetBrightness, ReadPixel]
            .into_iter()
            .find(|id| *id as u8 == b)
            .ok_or(ProtoError::UnknownComamnd(b))
    }
}

struct RawFrame<'a> {
    cmd: u8,
    payload: &'a [u8],
}

impl<'a> RawFrame<'a> {
    fn to_command(&self) -> Result<Command, ProtoError> {
        let command = CommandId::try_from(self.cmd);
        match command {
            Ok(c) => match c {
                CommandId::SetPixel => Ok(Command::SetPixel {
                    index: ((self.payload[0] as u16) << 8) | (self.payload[1] as u16),
                    r: self.payload[2],
                    g: self.payload[3],
                    b: self.payload[4],
                }),
                CommandId::Fill => Ok(Command::Fill {
                    r: self.payload[0],
                    g: self.payload[1],
                    b: self.payload[2],
                }),
                CommandId::SetBrightness => Ok(Command::SetBrightness(self.payload[0])),
                CommandId::ReadPixel => Ok(Command::ReadPixel {
                    index: ((self.payload[0] as u16) << 8 | (self.payload[1] as u16)),
                }),
            },
            Err(e) => Err(e),
        }
    }
}

struct Frames<'a> {
    buf: &'a [u8],
    pos: usize,
}

fn frames(buf: &[u8]) -> Frames<'_> {
    Frames { buf, pos: 0 }
}

impl<'a> Iterator for Frames<'a> {
    type Item = Result<RawFrame<'a>, ProtoError>;
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos + self.buf[self.pos..].iter().position(|&b| b == 0xAA)?;
        let frame = &self.buf[start + 1..];

        if frame.is_empty() {
            return None;
        }

        let len = frame[0];

        if frame.len() < (len as usize) + 3 {
            return None;
        }

        let frame = &frame[..len as usize + 3];
        let cmd = frame[1];
        let payload = &frame[2..len as usize + 2];
        self.pos = start + 1 + frame.len();

        let exp_check = payload.iter().fold(len ^ cmd, |acc, b| acc ^ b);

        let check = frame[frame.len() - 1];
        if check == exp_check {
            return Some(Ok(RawFrame { cmd, payload }));
        }

        Some(Err(ProtoError::BadChecksum {
            expected: exp_check,
            got: check,
        }))
    }
}

struct StreamDecoder {
    buf: Vec<u8>,
}

impl StreamDecoder {
    fn new() -> Self {
        StreamDecoder { buf: Vec::new() }
    }
    fn feed(&mut self, b: &[u8]) -> Vec<Result<Command, ProtoError>> {
        self.buf.extend_from_slice(b);
        let mut frames = frames(self.buf.as_slice());
        let mut cmds: Vec<Result<Command, ProtoError>> = Vec::new();
        for f in &mut frames {
            match f {
                Ok(r) => cmds.push(r.to_command()),
                Err(e) => cmds.push(Err(e)),
            }
        }
        let pos = frames.pos;
        self.buf.drain(..pos);
        cmds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() {
    println!("Hello, world!");
}
