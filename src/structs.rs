// Questo permette di stampare la struct con println!("{:?}", instance);
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//, Costruttore
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn run() {
    let mut user1: User = User {
        email: String::from("user1@email.com"),
        username: String::from("user_1"),
        active: true,
        sign_in_count: 1,
    };
    user1.active = false; //* ok

    let user_email = user1.email.clone();
    // println!("{user_email}"); // user@email.com

    //# Istanziare struct User usando il constructor build_user()
    let user2: User = build_user(String::from("user2@email.com"), String::from("user_2"));
    // println!("user2: {:?}", user2);

    //# Update struct
    let user3: User = User {
        email: String::from("user3@email.com"),
        username: String::from("user_3"),
        ..user2
    };
    // println!("user3: {:?}", user3);
    // println!("user2.email: {:?}", user2.email);

    //# Istanza tuple struct
    let mut color1 = Color(255, 255, 0);

    //# Istanza Triangle
    /* let tri = Triangle {
        width: 30,
        height: 50
    };
    println!("L'area del triangolo è {} pixel quadrati", triangle_area(&tri)); */

    //# Chiamate a metodi d'istanza
    /* println!("L'area del triangolo è {} pixel quadrati", tri.area());

    let rotated_tri = tri.rotate();
    println!("Dopo la rotazione, il triangolo ha larghezza {} e altezza {}", rotated_tri.width, rotated_tri.height);

    tri.enlarge(2);
    println!("Dopo l'ingrandimento, l'area del triangolo è {} pixel quadrati", tri.area()); */

    //# Chiamata del metodo associato
    /* let default_tri = Triangle::new_default_triangle();
    println!("Nuovo triangolo di default creato: {:#?}", default_tri); */

    //# Invocazione costruttore
    let tri2 = Triangle::new(10, 5);
    println!("Nuovo triangolo di creato: {:#?}", tri2);
}

//, Tuple struct
struct Color(i32, i32, i32);

//, Passaggio istanze struct come parametri
#[derive(Debug)]
struct Triangle {
    width: u32,
    height: u32,
}

fn triangle_area(tri: &Triangle) -> u32 {
    (tri.width * tri.height) / 2
}

//, Metodi delle struct
impl Triangle {
    //# Metodi d'istanza

    // Borrow dei dati d'istanza
    fn area(&self) -> u32 {
        (self.width * self.height) / 2
    }
    // Creare un nuovo triangolo scambiando larghezza e altezza
    fn rotate(self) -> Self {
        Triangle {
            width: self.height,
            height: self.width,
        }
    }
    // Modifica l'istanza, aumentando la larghezza e l'altezza
    fn enlarge(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    //# Metodi associati
    fn new_default_triangle() -> Self {
        Triangle {
            width: 1,
            height: 1,
        }
    }

    //# Costruttori
    fn new(width: u32, height: u32) -> Self {
        Triangle { width, height }
    }
}
