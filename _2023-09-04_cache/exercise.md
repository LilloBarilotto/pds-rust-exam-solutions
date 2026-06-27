## Teoria 1

Struct Counter {
    counter = Arc<Mutex<usize>>
}


impl Counter {
    pub fn new() -> Self{
        let counter = Arc::new(Mutex::new(0));
        Self{
            counter
        }
    }

    pub fn count(&self){

        let mutex_guard =  self.counter.lock().unwrap();

        *mutex_guard = *mutex_guard + 1;
    }

    pub fn get_counter (&Self) {

    
    }
}

I mutex vengono utilizzati in un contesto thread-safe attraverso l'incapsulamento/wrap in un Arc, così da poter condividere la struttura del mutex attraverso riferimenti forti o deboli, così che la risorsa venga rilasciata effettivamente solo quando l'ultimo riferimento strong verrà rilasciato.
Quello che ogni thread può fare è quindi operare su un clone di una istanza di Counter.
Implementano il paradigma RAII.

Così ogni thread avrà la possibilità di poter accedere al metodo lock() per bloccare l'accesso in sicurezza sulla variabile condivisa.


## Teoria 2
Si descriva il concetto di "ownership" in Rust e come contribuisca a prevenire errori come le race
condition e i dangling pointer rispetto ad altri linguaggi come C++.

In Rust un dato può essere posseduto solo da un unico Owner. Chiunque sia il suo owner ha lo scopo di rilasciare il valore, quando esce dallo scope o quando viene sovrascritto.
Si possono avere più riferimenti non mutabili (sola lettura) contemporaneamente, o al contrario un unico riferimento mutabile.
Si evità così la race condition, grazie al fatto che all'interno di uno scope abbiamo solo un riferimento mutabile e nessuno riferimento in lettura.
Si evita invece il dangling pointer perchè il borrow checker in fase di compilazione controlla attraverso gli scope, che non ci siano riferimenti a valori che sono stati già rilasciati in uno scope precedente.

Per la race condition in caso di programmazione concorrente, utilizzeremo strutture thread safe per evitare race condition.

## Teoria 3
Si dimostri come sia possibile implementare il polimorfismo attraverso i tratti. Si fornisca anche un
esempio concreto che faccia riferimento ad almeno due strutture diverse.

L'uso dei tratti permette di specificare un insieme di metodi, argomenti e tipi di ritorno associati, comuni.
I tratti possono essere quelli già definiti da RUST, e poi implementare dei metodi comuni per le nostre strutture (es. Trait Debug, Clone, Copy, Drop, etc.)
Di sotto un esempio di un tratto "Special" con un metodo comune che ha una definizione di default.

trait Special{
    fn special(&self: S)
    where S: Display {
        println!("{} you are special", self)
    }
}

#[derive(Display, Special)]
struct Person{
    name: String
}

#[derive(Display, Special)]
struct Student{
    matricola: usize
}

fn main(){
    let p = Person{name: "Lillo".to_string()};
    let m = Student{matricola: 300};

    p.special();
    m.special();
}