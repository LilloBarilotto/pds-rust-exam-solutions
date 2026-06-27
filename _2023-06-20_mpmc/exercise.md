## Teoria 1
Si definisca il problema delle referenze cicliche nell’uso degli smart pointers. Si fornisca quindi un
esempio in cui tale problema sia presente.

Il problema delle referenze cicliche (o reference cycles) è uno dei pochi modi in cui è ancora possibile causare un memory leak (perdita di memoria) in un linguaggio memory-safe come Rust.

Questo fenomeno si verifica quando due o più smart pointer si uniscono tra loro formano un ciclo chiuso di riferimenti. Di conseguenza, il conteggio dei riferimenti (reference count) di ciascun nodo non scenderà mai a zero, impedendo al sistema di deallocare la memoria occupata, anche quando i nodi diventano completamente irraggiungibili dal resto del programma.

Quando un Rc viene clonato, il contatore sale; quando un Rc esce dal codice (va out of scope), il contatore scende. La memoria viene liberata solo quando il contatore raggiunge lo zero.

Se però combiniamo la proprietà condivisa con la mutabilità interna (tramite RefCell), possiamo modificare i puntatori dopo la loro creazione per farli puntare a vicenda.

Ecco cosa succede a livello logico:

    Il Nodo A punta al Nodo B (il contatore di B è 1).

    Il Nodo B punta al Nodo A (il contatore di A è 1).

    Anche se le variabili principali nel codice vengono distrutte, A e B continuano a tenersi in vita a vicenda. La memoria è persa.

Esempio pratico in Rust

Ecco un esempio classico: una struttura Node che può avere un puntatore a un altro nodo.
Rust

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    // Usiamo RefCell per poter modificare il link dopo la creazione
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    // 1. Creiamo il Nodo 1
    let nodo1 = Rc::new(Node {
        value: 10,
        next: RefCell::new(None),
    });
    // In questo momento: Conteggio Rc di nodo1 = 1

    // 2. Creiamo il Nodo 2 e facciamolo puntare al Nodo 1
    let nodo2 = Rc::new(Node {
        value: 20,
        next: RefCell::new(Some(Rc::clone(&nodo1))),
    });
    // In questo momento: Conteggio Rc di nodo1 = 2, nodo2 = 1

    // 3. CHIUDIAMO IL CICLO: Facciamo in modo che il Nodo 1 punti al Nodo 2
    if let Some(ref mut next_link) = *nodo1.next.borrow_mut() {
        // Questa riga non verrà eseguita perché è None, usiamo il borrow diretto:
    }
    *nodo1.next.borrow_mut() = Some(Rc::clone(&nodo2));
    // Ora: Conteggio Rc di nodo1 = 2, nodo2 = 2

    println!("Conteggio nodi prima della fine del main:");
    println!("nodo1 ref count = {}", Rc::strong_count(&nodo1));
    println!("nodo2 ref count = {}", Rc::strong_count(&nodo2));

    // Alla fine del main, le variabili `nodo1` e `nodo2` escono dallo scope.
    // Il contatore di entrambi scende da 2 a 1.
    // Poiché il contatore è 1 (e non 0), la memoria NON viene liberata.
    // Abbiamo appena creato un memory leak in Rust!
}


-------------------------------------------
## Teoria 2
I tratti fondamentali della concorrenza sono:
- Sync, il tipo che lo implementa può essere condiviso.
- Send, il tipo che lo implementa può essere spostato tra un thread e un altro

In programmazione concorrente, in caso di risorse immutabili, possiamo usare degli Arc, che ci permettono di clonare il valore su più thread.
In caso di risorse mutabili, possiamo usare Mutex e RwLock per poter far agire contemporaneamente sulla zona di memoria solo uno alla volta.
Nel caso di Mutex, che sia in lettura o scrittura, solo un thread che avrà preso il lock del Mutex potrà accederci, gli altri dovranno aspettare.
Nel caso di RwLock, più letture possono essere fatte in contemporanea, e in caso di scrittura può avvenire solo in mutua esclusione (o 1 scrittura o 1..inf letture in contemporanea).
Quindi in caso di mutabilità/immutabilità delle risorse possiamo aspettarci uno o più lock e attese in base alla implementazione.

-----------------------------------------
## Teoria 3
struct Data {
    Element: AsVector,
    next: Rc<Data>
}
enum AsVector {
    AsVector(Box::<Rc<i32>>),
    None
}

Indicare l’occupazione di memoria di un singolo elemento in termini di: 1. numero di byte
complessivi (caso peggiore, architettura a 64 bit) e 2. posizionamento dei vari byte in memoria
(stack, heap, ecc.).


AsVector:
    stack 8B Box_ptr = 8 +tagEnumSeNecessario
    heap  8B ptrRc + (8+8+4) strong-weak-i32 = 28B

Data:
    stack 8B di AsVector_Box_ptr + 8B next_Rc_ptr = 16B +tagEnumSeNecessario
    heap: heap_AsVector + (8B + 8B + stackData) = 28B + 16B + 16B = 60B
