## Teoria 1
Si spieghi il concetto di possesso in relazione agli smart pointers. Come viene gestito il ciclo delle
risorse quando si utilizzano smart pointers?
Infine, si espliciti il ciclo di vita delle risorse nel seguente esempio:
```rust
{
    let mut i = 10;
    let bi1 = Box::new(i);
    let mut bi2 = Box::new(*bi1);
    *bi2 = 20;
    i = *bi2;
    println!("{} {:?} {:?}", i, bi1, bi2);
}
```

Gli smart pointer sono realizzati mediante struct, implementano i tratti Deref e DerefMut e permettono di avere comporntamenti e regole di ownership più complesse e flessibili rispetto ai puntatori nativi, per esempio mediante:
- Box<T>, il ciclo di vita viene passato alla variabile che prende il controllo del puntatore, potendo così estendere il ciclo di vita.

Nel caso del codice:
- riga 1, definiamo i come mutabile e con valore 10
- riga 2, definiamo una variabile bi1, creiamo una seconda variabile, con bi1 che punta a una zona di memoria nell'heap che contiene il valore copiato da i, 10, essendo i con tratto Copy. 
- riga 3, definiamo bi2 che copia il valore per deferenziazione di bi1, prendendo quindi con copia il valore 10.
- riga 4, modifichiamo il valore di Box<T> nell'heap puntato da bi2 con 20
- riga 5, diamo il valore con deferenziazione di bi2 ad i, che quindi per il tratto copy prenderà un 20.
- riga 6, stampa di "20, Box{10}, Box{20}"

alla fine della println i, bi1, bi2 punteranno tutti a zone diverse, i avrà uno spazio per l'intero in stack, bi1 e bi2 avranno i propri puntatori in stack che punteranno a 2 valori diversi nello heap.
Son 3 lifetime diversi, che termineranno tutti a fine scope, cioè le parentesi graffe.
(bi1 e bi2 faranno un drop per liberare i valori nello heap).

## Teoria 2
Si descriva la gestione della memoria in Rust e si spieghi come vengono evitati i problemi di
sicurezza comuni come le violazioni di accesso o la presenza di puntatori nulli.

Rust si fonda sul paradigma RAII, ovvero ogni struttura che agisce su delle risorse è responsabile 
di liberarle quando questa viene distrutta.
Si è esteso questo concetto definendo il concetto di ownership, dove una risorsa "possiede" un 
altra e una volta che l'owner si distrugge così verrà fatto con quello possedutto ed inoltre nessun 
altro può accedere o manipolare questa risorsa se non l'owner. A questo concetto si affianca 
quello del borrowing, dove il permesso di accedere a un dato non posseduto viene reso possibile 
se si rispettano delle regole controllate a tempo di compilazione:
- una risorsa può essere prestata in lettura se non ci sono altre che la possano modificare
- l'owner non può modificare la risorsa mentre quando ha un prestito attivo
- solo uno alla volta può chiedere un prestito in scrittura
Queste regole servono a garantire che ci sia sempre al massimo uno scrittore al fronte di più 
lettori che non danno problemi e avere questi forti controlli in fase di compilazione non rende 
possibile una classe di errori comuni.
Ad esempio non è possibile avere un dangling pointer (puntatore mantenuto verso una area che è 
stata già deallocata) o una double release (liberare più volte la stessa area di memoria, quando 
dalla precedenza deallocazione non dovremmo più averne "possesso").

In caso di accesso a zone di memoria non riconosciute come nostre, il BOundary checker controlla a Runtime che siano soddisfate le condizioni (es array da 5 elementi e provo ad accedere al sesto con v[5]). Se l'indice non è valido il programma va in panic![]
I riferimenti &T vengono controllati in fase di compilazione, non possiamo avere null pointer. Unica eccezzione utilizzando Option<ptr> dove Option può contenere Some o None, e in quel caso il None può essere sfruttato per indicare che non abbiamo puntatore valido.


## Teoria 3
Si illustri come sia possibile gestire correttamente le situazioni di errore in Rust, distinguendo tra
Option e Result.

In Rust non vengono utilizzate eccezioni per segnalare gli errori, che qui vengono definiti come irrecuperabili e recuperabili.
Nel caso di irrecuperabili, eseguiamo una macro "panic!({},mess)" che dopo essere stata chiamata mostrerà un messaggio su sdterr.
Nel caso di errori recuperabili, usiamo la struttura Result<R,E>. Questo perchè se non abbiamo errori ritornemo un Ok{Valore} altrimenti un Error{Errore}. Questa struttura realizzata attraverso enum ci permette quindi di poter ritornare un errore al chiamante dando quindi al chiamante stesso l'onere di gestirlo (e di propagarlo a sua volta).
Nel caso invece volessimo ritornare se abbiamo un risultato valido oppure nessun risultato, possiamo usare Option<Some(),None>, anche qui un enum. Se ad esempio in un database troviamo un risultato o meno da una query potrebbe usare un Some(Valore) per fornire il risultato, altrimenti None. Volendo essere più specifici possiamo incapsulare a loro volta Result<Option<Some,None>, Error>, cosi da coprire le casistiche in caso di errore, e in caso non ci siano errori specificare se abbiamo effettivamente un risultato pronto oppure non abbiamo corrispondenze per la funzione effetuata.
Nel caso di option possiamo anche sfruttare la NullPointerOptimization, che ci permette di non usare byte per memorizzare il tag essendo che non possiamo avere puntatori nulli in Rust, e sfruttiamo il valore nullo come None.

Per gestire le situazioni di Option e Result possiamo usare match funzione_ritorna_result_option {
    Ok(t) -> possiamo usare t,
    Err(e) -> possiamo usare e a nostro piacimento.
} o if let Some(t) == funzione_ritorna_result_option{
    // possiamo usare t
} 

```rust
fn main() {
    let mut l = HashMap::<String, usize>::new();
    l.push("Marco".to_string(), 10)

    match find(l, "Marco".to_string()){
        Ok(some_value) => {
            if let Some(value) = some_value{
                // Some(value)   find the value inside
            } else{
                // None()
            }
        },
        Err(e) => { //Err 
            panic!("{}", e)
        } 
    }
}

fn find( l : Hashmap::<String, usize> , s: String) -> Result<Option<usize>, String>{
    if s.is_empty() {
        return Err("La stringa non può essere vuota".to_string());
    }

    // ipotizzando get_value ritorni un Option<usize>
    return Ok(l.get(s)) 
}

// Definiamo i nostri tipi
#[derive(Debug)]
struct Utente {
    id: i32,
    nome: String,
}

#[derive(Debug)]
enum DbError {
    Timeout,
    ConnessionePersa,
}

// 1. Simuliamo la funzione di basso livello che interroga il DB
fn interroga_db(id: i32) -> Result<Option<Utente>, DbError> {
    match id {
        -1 => Err(DbError::ConnessionePersa), // Errore di sistema
        42 => Ok(Some(Utente { id: 42, nome: String::from("Alice") })), // Trovato
        _ => Ok(None), // Nessun errore, ma nessun utente trovato
    }
}

// 2. La nostra funzione che usa il costrutto `?`
fn ottieni_nome_utente(id: i32) -> Result<Option<String>, DbError> {
    
    // IL COSTRUTTO ?:
    // Agisce sul livello più esterno (il Result).
    // - Se interroga_db ritorna Err, l'operatore ? fa un "return Err(...)" invisibile e la funzione finisce qui.
    // - Se ritorna Ok(Option), il ? estrae l'Option e lo mette in `utente_opt`.
    let utente_opt: Option<Utente> = interroga_db(id)?;

    // Ora dobbiamo gestire l'Option interno (il dato logico).
    // Possiamo usare il classico match:
    match utente_opt {
        Some(utente) => Ok(Some(utente.nome)),
        None => Ok(None),
    }
}
```

