//% Senza traits
#[derive(Debug, PartialEq)]
struct CommonShape {
    id: u32,
    description: String,
    color: String,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    common: CommonShape,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq)]
struct Square {
    common: CommonShape,
    side: u32,
}

/* impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
 */

/* impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
} */

//% Con traits
trait Shape {
    // Definizione dei metodi che devono essere implementati dalle struct
    fn area(&self) -> u32;

    // Metodo di default
    fn perimeter(&self) -> u32 {
        println!("Perimeter not implemented for this shape. Returning dummy value.");
        0.0 as u32
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        println!("Area del rettangolo: {}", self.width * self.height);
        self.width * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        println!("Area del quadrato: {}", self.side * self.side);
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        println!("Perimeter del quadrato: {}", self.side * 4);
        self.side * 4
    }
}

trait GetId {
    fn get_id(&self) -> u32;
}

impl GetId for Rectangle {
    fn get_id(&self) -> u32 {
        self.common.id
    }
}

//% Trait Bound
fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

// Impl Trait Syntax
fn print_perimeter(shape: &impl Shape) {
    println!("Perimeter: {}", shape.perimeter());
}

// Multi Trait Bound
fn get_info<T: Shape + GetId>(shape: &T) {
    println!("Id: {}", shape.get_id());
}

// Where clause
fn print_area_and_perimeter<T>(shape: &T)
where T: Shape
{
    print_area(shape);
    print_perimeter(shape);
}

fn get_info_where<T, U>(shape1: &T, shape2: &U)
where
    T: Shape + std::fmt::Debug,
    U: Shape + std::fmt::Debug,
{
    if shape1.area() > shape2.area() {
        println!("Shape1 is bigger, it is a {:?}", shape1);
    } else {
        println!("Shape2 is bigger, it is a {:?}", shape2);
    }
}

//# Return Type Polymorphism
fn returns_shape_1() -> impl Shape {
    Rectangle {
        width: 10,
        height: 20,
        common: CommonShape {
            id: 1,
            description: "Rectangle".to_string(),
            color: "Red".to_string(),
        },
    }
}

fn returns_shape_2(condition: bool) -> impl Shape {
    if condition {
        Rectangle {
            width: 10,
            height: 20,
            common: CommonShape {
                id: 1,
                description: "Rectangle".to_string(),
                color: "Red".to_string(),
            }
        }
    } else {
        Rectangle { // stesso tipo concreto di sopra
            width: 5,
            height: 5,
            common: CommonShape {
                id: 2,
                description: "Square".to_string(),
                color: "Blue".to_string(),
            }
        }
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

trait Designable {
    fn print(&self);
}

//% Super trait
trait Drawable: Designable + Shape {
    fn draw(&self);
}

// Implementare il trait Designable da cui Drawable dipende
impl Designable for Circle {
    fn print(&self) {
        println!("Circle");
    }
}

// Implementare il trait Shape da cui Drawable dipende
impl Shape for Circle {
    fn area(&self) -> u32 {
        println!(
            "Area del cerchio: {}",
            (self.radius * self.radius * std::f64::consts::PI) as u32
        );
        (self.radius * self.radius * std::f64::consts::PI) as u32
    }
}

// Poi possiamo implementare il trait Drawable
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a rectangle");
    }
}

fn get_properties<T>(object: &T)
where
    // Grazie ai super trait non è necessario listarli tutti nei generics
    // T: Drawable + Shape + Designable,
    T: Drawable + std::fmt::Debug,
{
    println!("Object: {:?}", object.area());
}

//# Trait Object - Dynamic Dispatch
fn print_shape_info(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn print_area_dyn_box(shape: Box<dyn Shape>) {
    println!("Area: {}", shape.area());
}

fn get_dynamic_info(dimension: Vec<u32>) -> Box<dyn Shape + 'static> {
    match dimension.len() {
        2 => Box::new(Rectangle {
            width: dimension[0],
            height: dimension[1],
            common: CommonShape {
                id: 3,
                description: "Rectangle".to_string(),
                color: "Red".to_string(),
            },
        }),
        1 => Box::new(Square {
            side: dimension[0],
            common: CommonShape {
                id: 4,
                description: "Square".to_string(),
                color: "Blue".to_string(),
            },
        }),
        _ => panic!("Dimensioni non valide per creare una forma dinamica!"),
    }
}


//% Marker Trait
trait Sendable {}
trait Cloneable: Sendable + PartialEq + Shape {}

// Associated Types in Traits
#[derive(Debug)]
struct Kilometer(f64);

#[derive(Debug)]
struct Mile(f64);

trait Speed {
    // Il tipo associato rappresenta l'unità della velocità (ad esempio, chilometri o miglia)
    type Distance;

    // Metodo che restituisce la velocità espressa come tipo associato
    fn get_speed(&self) -> Self::Distance;
}

trait Distance {
    type Unit; // tipo associato che rappresenterà l'unità di misura della distanza

    // Un metodo che restituisce una distanza espressa nel tipo associato
    fn compute(&self) -> Self::Unit;
}

struct CarKm {
    speed: f64, // velocità in km/h
}

impl Speed for CarKm {
    // In questa implementazione, il tipo di distanza associato è 'Kilometer'
    type Distance = Kilometer;

    fn get_speed(&self) -> Self::Distance {
        Kilometer(self.speed)
    }
}

struct CarMiles {
    speed: f64, // velocità in mph
}

impl Speed for CarMiles {
    // Qui il tipo associato è 'Mile'
    type Distance = Mile;

    fn get_speed(&self) -> Self::Distance {
        Mile(self.speed)
    }
}

pub fn run() {
    /* let rectangle = Rectangle {
        width: 10,
        height: 20,
    }; */

    // let square = Square { side: 5 };

    // Composition over inheritance
    let rectangle = Rectangle {
        width: 10,
        height: 20,
        common: CommonShape {
            id: 1,
            description: "Rectangle".to_string(),
            color: "Red".to_string(),
        },
    };

    println!("Rectangle area: {:?}", rectangle.area());

    let square = Square {
        side: 5,
        common: CommonShape {
            id: 2,
            description: "Square".to_string(),
            color: "Blue".to_string(),
        },
    };

    rectangle.area();
    square.area();
    square.perimeter();

    // Trait Bound
    print_area(&rectangle);
    print_area(&square);
    print_perimeter(&square);
    get_info(&rectangle);
    print_area_and_perimeter(&square);
    get_info_where(&rectangle, &square);

    returns_shape_1();
    returns_shape_2(true);

    // Super Trait
    let c = Circle { radius: 5.0 };

    c.print(); // Metodo di Designable
    c.draw(); // Metodo di Drawable
    get_properties(&c);

    // Trait Object - Dynamic Dispatch
    print_shape_info(&rectangle);
    let boxed_rect: Box<dyn Shape> = Box::new(rectangle);
    print_area_dyn_box(boxed_rect);

    // Ritorno di un trait object dinamico
    let dynamic_shape = get_dynamic_info(vec![10, 20]);
    print_area_dyn_box(dynamic_shape);

    // Esempio con 'static
    let dynamic_static_shape = get_dynamic_info(vec![15]);
    print_area_dyn_box(dynamic_static_shape);

    // Derived traits

    // Possiamo confrontare due istanze di Rectangle grazie a PartialEq
    let rectangle2 = Rectangle {
        width: 10,
        height: 20,
        common: CommonShape {
            id: 1,
            description: "Rectangle".to_string(),
            color: "Red".to_string(),
        },
    };

    /* if rectangle == rectangle2 {
        println!("I due rettangoli sono uguali!");
    } else {
        println!("I due rettangoli sono diversi!");
    } */

    // Associated Types in Traits
    let car1 = CarKm { speed: 100.0 };
    let car2 = CarMiles { speed: 60.0 };

    println!("Velocità CarKm: {:?}", car1.get_speed());
    println!("Velocità CarMiles: {:?}", car2.get_speed());
}
