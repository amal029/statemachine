#[derive(Debug)]
enum AP {
    P,
    Q,
    R,
}

#[derive(Debug)]
enum States {
    S0,
    S1,
    S2,
}

#[derive(Debug)]
enum Inputs {
    A,
    B,
}

#[derive(Debug)]
enum Outputs {
    O1,
    O2,
}

fn _relation(s: &States, i: Inputs) -> (States, Option<Outputs>) {
    match (s, i) {
        (States::S0, Inputs::B) => (States::S1, Some(Outputs::O2)),
        (States::S1, Inputs::A) => (States::S2, Some(Outputs::O2)),
        (States::S2, Inputs::A) => (States::S0, Some(Outputs::O1)),
        (States::S2, Inputs::B) => (States::S2, None),
        (States::S0, Inputs::A) => (States::S0, None),
        (States::S1, Inputs::B) => (States::S1, None),
    }
}

fn _label(s: &States) -> AP {
    match s {
        States::S0 => AP::P,
        States::S1 => AP::R,
        States::S2 => AP::Q,
    }
}

// XXX: Executes the state machine every tick
fn _exec(s: &States, i: Inputs) -> (States, Option<Outputs>, AP) {
    let oo = _relation(&s, i);
    let ap = _label(&oo.0);
    (oo.0, oo.1, ap)
}

fn main() {
    let mut _s = States::S0;
    // XXX: Example execution trace with some inputs
    println!("{:?} {:?}", &_s, _label(&_s));
    for i in 0..3 {
        if i == 0 {
            let (_s1, _o, _ap) = _exec(&_s, Inputs::B);
            println!("{:?} {:?}, {:?}", &_s1, _o, _ap);
            _s = _s1;
        } else if i == 2 {
            let (_s1, _o, _ap) = _exec(&_s, Inputs::A);
            println!("{:?} {:?}, {:?}", &_s1, _o, _ap);
            _s = _s1;
        } else {
            let (_s1, _o, _ap) = _exec(&_s, Inputs::A);
            println!("{:?} {:?}, {:?}", &_s1, _o, _ap);
            _s = _s1;
        }
    }
}
