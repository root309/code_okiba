#[derive(Debug)]
enum Level {
    Stay,
    LevelUp { n: i32 },
    LevelDown { n: i32 },
    MyLevel(i32),
}
impl Level {
    fn get_level(&self) -> Option<i32> {
        match *self {
            Level::Stay => None,
            Level::LevelUp { n: up } => Some(up),
            Level::LevelDown { n: down } => Some(down),
            Level::MyLevel(my_level) => Some(my_level),
        }
    }
}

fn main() {
    let stay = Level::Stay;
    let level_up = Level::LevelUp { n: 1 };
    let level_down = Level::LevelDown { n: -1 };
    let my_level = Level::MyLevel(5);

    // println!("my_level = {:?}", my_level.0); // no field `0` on type `Level`
    println!("my_level   = {:?}", my_level.get_level()); // Some(5)
    println!("level_down = {:?}", level_down.get_level()); // Some(1)
    println!("level_up   = {:?}", level_up.get_level()); // Some(-1)
    println!("stay       = {:?}", stay.get_level()); // None

    // match: literals
    let code = 100;
    match code {
        0 => println!("success!"),
        1 => println!("error!"),
        n => println!("code:{} is unknown...", n),
    }
    // match: wildcard
    let goods = "key";
    match goods {
        "keyboard" => println!("On the 5th floor"),
        "keypad" => println!("On the 2nd floor"),
        "key" => println!("On the 99th floor"),
        _ => println!("Please go out"),
    }
    // match: tuple
    let password = ("ticktack", "years");
    match password {
        ("ticktack", "days") => println!("who?"),
        (_, "months") => println!("he?"),
        ("ticktick", "years") => println!("her?"),
        (_, _) => println!("Accept! (weak! weak! weak!)"),
    }
    // match: struct
    struct Dim {
        id: i32,
        x: i32,
        y: i32,
        z: i32,
    }
    let d = Dim {
        id: 0,
        x: 0,
        y: 0,
        z: 0,
    };
    match d {
        // Dim {id: id, x: 0, y: 0, z: 0} => println!("id={}", id),
        Dim { id, x, y, z } => println!("id={}", id),
        // Dim {id, ..} => println!("id={}", id),
        _ => println!("Unknown..."),
    }
    // match: reference
    let mut string = String::from("String!");
    match string {
        ref s => {
            println!("s={}", s);
            println!("string.len()={}", string.len());
        }
    }
    match &*string {
        "String!" => println!("String!"), // Error: mismatched types
        _ => println!("Unknown..."),
    }
    match string {
        ref mut s => {
            println!("s={}", s);
            println!("s.make_ascii_uppercase={}", {
                s.make_ascii_uppercase();
                s
            });
        }
    }
    println!("string = {}", string);

    let x = &0;
    match x {
        &0 => println!("zero"),
        &1 => println!("one"),
        _ => println!("Unknown..."),
    }

    // match: range
    let password = "password!";
    match password {
        "password!" | "p@ssw0rd!" | "PASSWORD!" => println!("Accept!"),
        _ => println!("who?"),
    }
    let z = 'z';
    match z {
        '0'..='9' => println!("digit"),
        'a'..='z' => println!("char"),
        _ => println!("?"),
    }

    // match: guard
    let string = String::from("String!");
    match string {
        ref s if s == "String" => println!("s={}", s),
        ref s if s == "String!" => println!("s={}", s),
        _ => println!("Unknown..."),
    }

    // match: @
    let digit = 10;
    match digit {
        d @ 0..=9 => println!("{} is ok!", d),
        d @ 10..=100 => println!("{} is too much!", d),
        _ => println!("who?"),
    }
}
