#[derive(Debug)]
struct Person {
    nome: String,
    cognome : String, 
    eta : u8
}

impl Person {
    fn change_name (&mut self, _nome: String) {
        self.nome = _nome;
    }
}


fn main() {

    // crea persona diretamente, cambia nome con funzione implementata in struttura
    let mut persona_due = Person {
        nome : String::from("Mark"), 
        cognome : String::from("Hall"), 
        eta: 68,
    };

    persona_due.nome = String::from("Ercole");

    println!("persona: {persona_due :?}");

    // crea persona con crea persona, cambia nome con funzione implementata in struttura
    let mut persona_uno = crea_persona(String::from("Fabio"), String::from("Bistolfi"), 64);

    println!("persona: {persona_uno :?}");

    persona_uno.change_name(String::from("Gianni"));

    println!("nome: {}, cognome: {}, eta {}.", persona_uno.nome, persona_uno.cognome, persona_uno.eta);



}

fn crea_persona(_nome : String ,_cognome : String, _eta: u8) -> Person {
    Person{
        nome :_nome ,
        cognome : _cognome,
        eta : _eta ,
    }
}
