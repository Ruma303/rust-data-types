enum Directions {
    Nord,
    Est,
    Sud,
    West,
}

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn run() {
    //% Enum
    let coo1: Directions = Directions::Nord;
    let coo2 = Directions::West;

    //% Pattern matching
    match coo1 {
        Directions::Nord => println!("Stiamo andando a nord!"),
        Directions::Est => println!("Stiamo andando a est!"),
        Directions::Sud => println!("Stiamo andando a sud!"),
        Directions::West => println!("Stiamo andando a ovest!"),
        _ => println!("Sembra che non ci stiamo muovendo."),
    } // Stiamo andando a nord!

    if let Directions::West = coo1 {
        println!("Stiamo andando a nord!");
    } else {
        println!("Sicuramente, non stiamo andando a nord.");
    }

    //, Metodi delle enums
    println!(
        "Numero di varianti in Directions: {}",
        Directions::variant_count()
    );

    if coo1.is_vertical() {
        println!("coo1: {}", coo1.description());
    }

    if coo2.is_horizontal() {
        println!("coo2: {}", coo2.description());
    }

    //, Implementare enums nelle struct
    let team1 = TravelTeam {
        name: String::from("Team 1"),
        direction: Directions::Sud,
        people: 5,
    };

    println!("{}", team1.team_definition());

    //, Dati associati
    let msg1 = Message::Send;
    msg1.actions();
    let msg2 = Message::ChangeColor(255, 0, 0);
    msg2.actions();
    let msg3 = Message::Move { x: 10, y: 20 };
    msg3.actions();
    let msg4 = Message::Write("Hello, world!".to_string());
    msg4.actions();

    //, PartialEq
    let c1 = Color::Red;
    let c2 = Color::Blue;

    if c1 != c2 {
        println!("I colori sono diversi!");
    }

}

//% Metodi associati
impl Directions {
    fn variant_count() -> usize {
        4
    }

    fn is_vertical(&self) -> bool {
        match self {
            Directions::Nord | Directions::Sud => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Directions::Est | Directions::West => true,
            _ => false,
        }
    }

    fn description(&self) -> &str {
        match self {
            Directions::Nord => "nord",
            Directions::Est => "est",
            Directions::Sud => "sud",
            Directions::West => "ovest",
        }
    }
}

//% Struct vs enums
struct TravelTeam {
    name: String,
    direction: Directions,
    people: u32,
}

impl TravelTeam {
    fn team_definition(&self) -> String {
        format!(
            "Il team {} è composto da {} persone e si muove verso {}.",
            self.name,
            self.people,
            self.direction.description()
        )
    }
}

//, Dati associati
enum Message {
    Send,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn actions(&self) {
        match self {
            Message::Send => println!("Messaggio inviato"),
            Message::ChangeColor(r, g, b) => println!("Cambia colore a: ({}, {}, {})", r, g, b),
            Message::Move { x, y } => println!("Muovi a: ({}, {})", x, y),
            Message::Write(text) => println!("Testo del messaggio: {}", text),
        }
    }
}