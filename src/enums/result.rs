pub fn run() {
    let message = Message::Send;
    match message.is_sent() {
        Ok(_) => println!("Il controllo ha confermato: il messaggio è stato inviato."),
        Err(e) => println!("Errore durante l'invio del messaggio: {}", e),
    } // Il controllo ha confermato: il messaggio è stato inviato.
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

    //, Metodi per Result
    fn is_sent(&self) -> Result<(), String> {
        match self {
            Message::Send => {
                self.actions(); // Invoca l'azione associata a Message::Send
                Ok(())
            }
            _ => Err(String::from("Il messaggio non è stato inviato")),
        }
    }
}

//% Result enum
/* enum Result<T, E> {
    Ok(T),
    Err(E)
} */
