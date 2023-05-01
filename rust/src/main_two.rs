#[derive(Debug)]
struct Dirt {
    is_like: String,
}

#[derive(Debug)]
enum YouAreSomething {
    Nice(f64, String),
    Dirty(Dirt),
}

struct DeSuNya {
    name: String,
    cringe: bool,
    uwu: u128,
}

fn other_context() {
    println!("other context beginning...");

    let nice_instance = YouAreSomething::Nice(2.0, String::from("de su nyan"));

    let dirt = Dirt {
        is_like: (String::from("water")),
    };

    let like_the_dirty_in_the_earth = YouAreSomething::Dirty(dirt);

    println!(
        "debug call for Nice instance :OOOOO -> {:#?}",
        nice_instance
    );
    println!(
        "debug call for Dirty instance :00000 -> {:#?}",
        like_the_dirty_in_the_earth
    );

    println!(
        "yeah, I'm so used, my name is \"is_like\" and my value in Dirt instance is nothing...",
    );

    println!("other context end...");
}

fn returning_something(polyglot: bool) -> f64 {
    if polyglot {
        return 12453.54;
    }

    522.4
}

fn main() {
    other_context();

    println!("Kill me now");

    {
        println!("whoami")
    }

    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    println!("returned stuff is... {}", returning_something(true));
    println!(
        "and if I'm not a polyglot then it's... {}",
        returning_something(false)
    );

    let dst = "Jasmín";
    let src = "Luis";

    hi_ho(dst, src);

    let smiley_face = '😃';

    println!("char is {}", smiley_face);

    let tpl = (9i8, "password", "pipeline", false);

    println!(
        "value two of tuple is {}, fourth {}, one {} and third {}",
        tpl.1, tpl.3, tpl.0, tpl.2
    );

    let andrea_furra = DeSuNya {
        name: String::from("Andrea Furrita"),
        cringe: true,
        uwu: 5,
    };

    println!(
        "your name is {}, cringe mode: {}, uwu counter: {}",
        andrea_furra.name, andrea_furra.cringe, andrea_furra.uwu
    );

    structs_exercise()
}

fn hi_ho(arg1: &str, arg2: &str) {
    println!("miiauuuuu, so, you're {} and I {}", arg1, arg2);
}

struct SuperTuple(bool, i16, String, String);

struct Cup {
    name: String,
    capacity: u8,
}

impl std::fmt::Display for Cup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, capacity: {}", self.name, self.capacity)
    }
}

fn structs_exercise() {
    let tuple_one = SuperTuple(false, 0, String::from("wow"), String::from("nya"));

    println!("{}", tuple_one.2);

    let cup_one = Cup {
        name: String::from("whoami"),
        capacity: 2,
    };

    println!("{}", cup_one);
}
