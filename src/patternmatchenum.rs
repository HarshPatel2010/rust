
// Fill in the blank and fix the errors
#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
pub enum Tar {
    Mo = 9,
}

pub fn main() {
    let msgs: [Message;3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    let xt = Tar::Mo;
    println!("{:?}",xt);

}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}