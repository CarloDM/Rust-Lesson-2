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

    // STRUCT
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


    // VECTOR
    // standar con questa syntassi ha il tipo fisso puo contenerte solo uyn tipo di dato
    // sara possibile in rust avere anche il tipo dinamico ma non ora.
    let mut mio_vettore: Vec<u32> = Vec::new();
    println!("{:?}", mio_vettore);
    mio_vettore.push(33);
    mio_vettore.push(43);
    mio_vettore.push(77);

    for n in mio_vettore.iter(){
        print!("{}, ", n);
    }

    let num_di_vettore = mio_vettore[1];
    println!("secondo numero del vettore {}", num_di_vettore);

    // OPTION è un enum fatto di -- > Some() e di --> None
    // ottimo quando dobbiamo cercare qualcosa che puo esserci oppure no

    let frutta = ["mela", "pera", "cocco", "ananas"];

    let risultato_ricerca_frutta = cerca_frutto("ananas", &frutta);
    
    match risultato_ricerca_frutta {
        Some(value) => println!("trovato il frutto: {}", value),
        None => println!("nessun frutto trovato"),
    }

    let risultato_ricerca_frutta_str = cerca_frutto_str("cocco", &frutta);

    match risultato_ricerca_frutta_str {
        Some(value) => println!("trovato str frutto: {}", value),
        None => println!("nessun frutto str trovato"),
    };

    // RESULT è SAEMPRE UN ENUM CON 2 VALORI --> Ok --> Err
    // operazioni che possono anche non andare a buon fine tipo apertura di un file

    let result_apriporta = apri_porta("comune");

    println!("result: {:?}", result_apriporta);
    
    match result_apriporta {
        Ok(value) => println!("{}", value),
        Err(value)=> println!("{}", value),
    }

    let result_apriporta = apri_porta("blindata");
    match result_apriporta {
        Ok(value) => println!("{}", value),
        Err(value)=> println!("{}", value),
    }

}

fn apri_porta(tipo: &str) -> Result<&str, &str> {
    if tipo == "comune" {
        return Ok("porta comune aperta!");
    } else {
        return Err("porta chiusa!")
    }
}

fn cerca_frutto(nome: &str, lista : &[&str]) -> Option<String> {
    for &element in lista {
        if element == nome {
            return Some(format!("{}", element));
        }
    }
    None
}

//  Rust chiede garanzie sul life time del dato concetto da approfondire 
// 'a risultato valido fin che è valida la lista in cui l ho preso
fn cerca_frutto_str<'a>(nome: &str, lista : &'a [&'a str]) -> Option<&'a str> {
    for &element in lista {
        if element == nome {
            return Some(element);
        }
    }
    None
}

fn crea_persona(_nome : String ,_cognome : String, _eta: u8) -> Person {
    Person{
        nome :_nome ,
        cognome : _cognome,
        eta : _eta ,
    }
}
