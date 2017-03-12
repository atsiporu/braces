use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    for arg in args {
        println!("Input: {:?} {}", arg, test(arg.as_bytes()));
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Open(usize),
    Close(usize),
    Pair(usize, usize),
}

impl From<(u8, usize)> for State {
    fn from(val: (u8, usize)) -> Self
    {
        match val.0 as char {
            '(' => State::Open(val.1),
            ')' => State::Close(val.1),
            _ => panic!("No state for {:?}", val),
        }
    }
}

fn test(v: &[u8]) -> usize
{
    return find(
        State::Pair(0, 0),
        v, 0,
        0,
    ).1;
}

fn find(prev_state: State, v: &[u8], idx: usize, mut largest: usize) -> (State, usize)
{
    //print!("find {:?}  largest {}  --- ",  prev_state, largest);
    if v.len() <= idx {
        return (State::Pair(idx, idx), largest);
    }

    let state = State::from((v[idx], idx));

    //println!(" --- {:?} ({:?}, {:?}) - largest {}", String::from_utf8_lossy(&v[idx..]), prev_state, state, largest);

    let (state, state_largest) =
    match (prev_state, state) {
        (State::Open(oi), State::Close(idx)) => {
            //println!("pair: {:?}", State::Pair(oi, idx));
            (State::Pair(oi, idx), std::cmp::max((idx-oi+1), largest))
        },
        (prev_state, state @ State::Open(_)) => {
            match (prev_state, find(state, &v, idx+1, largest).0) {
                (State::Open(oi), State::Pair(b, e)) => {
                    //println!("step and back (()");
                    find(State::Open(oi), &v, e+1, std::cmp::max(e-b+1, largest))
                },
                (State::Pair(pb, pe), State::Pair(b, e)) => {
                    //println!("step and back ({}, {})({}, {})", pb, pe, b, e);
                    if pe + 1 >= b {
                        if largest < e - pb + 1 {
                            largest = e - pb + 1;
                        };
                        find(State::Pair(pb, e), &v, e+1, largest)
                    } else {
                        find(State::Pair(b, e), &v, e+1, largest)
                    }
                },
                (_, State::Pair(b, e)) => {
                    if largest < e - b + 1 {
                        largest = e - b + 1;
                    };
                    find(State::Pair(b, e), &v, e+1, largest)
                },
                (_, state) => {
                    (state, largest)
                }
            }
        },
        (_, state @ State::Close(_)) => {
            find(state, &v, idx+1, largest)
        },
        (_, State::Pair(_, _)) => unreachable!()
    };

    (state, std::cmp::max(state_largest, largest))
}
