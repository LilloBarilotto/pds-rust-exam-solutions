## Domanda 1
- SEGMENTO DI CODICE, comune a tutti i thread, sono contenute le istruzioni del programma 
- STATIC, comune a tutti i thread, contiene tutte le variabili globali e i valori fissi come "stringa" definiti nel codice.
- STACK, diverso per ogni thread, serve per salvare le variabili locali del thread il cui spazio spazio in memoria è noto a tempo di compilazione. Ogni scope sintattico, cioè definito tra le parentesi graffe {}, corrisponde a uno stack frame, e alla fine dello scope viene liberato lo spazio dello stack. Allocazione contigua.
- HEAP, comune a tutti i thread, serve per salvare quelle variabili di cui non è noto il size a tempo di compilazione (Vec, Hashmap, etc.) e in generale per quando ci serve una zona di memoria grossa che possiamo sfruttare. Allocazione non contigua.

fn main(){ // main e le sue variabili saranno in stack

 let v = Vec::<i32>::new();   // v sarà in stack definito come (ptr, capacity, len), il ptr punta alla zona dello heap
     v.push(3);   
     v.push(4);
                                // i valori di v salvati (3,4) si troveranno nello heap;
}

## Domanda 2
Cell<T> contiene al suo interno un T immutabile. Eppure possiede la capacità di Interior Mutability, Mutabilità interna, che ci permette di estrarre il valore e sostituirlo con un altro. Utile quando il contenuto non è mutabile ma vogliamo cambiarne il contenuto.
RefCell<T> permette di ottenere dei riferimenti a T grazie alla implementazione del tratto DeRef, e possiamo quindi ottenere più riferimenti multipli per poter mutare il valore interno con la Interior Mutability.
Entrambi sono tipi definiti in un contesto single-thread.
Lo spazio in memoria di Cell<T> è occupato da (Borrow, T).
Lo spazio in memoria di RefCell<T> è occupato da un puntatore che poi punta a una zona di memoria nello heap che contiene (Strong, Weak, T).


## Domanda 3
Nel caso in cui una sincronizzazione di stati di una variabile Mutex sia gestitata attraverso la ConditionVar, è possibile che alcune notifiche vengano perse. In questo caso è necessario controllare che il valore del Mutex sia quello atteso dopo una segnalazione attesa.
In particolare questo può succedere quando, dopo aver fatto partire due thread, il thread che fa partire la notifica arriva su cv.notify_all/notify_one() prima ancora che i thread che devono aspettare lo stato arrivino sulla cv.wait, questo perchè ci possono essere ottimizzazioni da parte della CPU per mandare avanti un thread rispetto a un altro.
Un esempio 

```rust
fn main(){
 let bool = Arc::new(( Mutex::new(false), Condvar::new());
 let bool_c =       bool.clone();

 let jh = thread::spawn( move || {
        thread::sleep(Duration_from_millis(1000));
        let mut guard = bool.lock().unwrap();
        guard = cv.wait(guard);

        println("bool è {}", *guard);
 }); //drop appena esce da scope del guard;
 
 let mut guard = bool_c.lock().unwrap();
 *guard = true;
  drop(guard);
 cv.notify_one(); // se arrivo qui prima di cv.wait? rimane in attesa infinita il thread interno.
 jh.join().unwrap(); // se interno rimane in attesa qui mi blocco all'infinito pure io.
}
```

La soluzione è cambiare la cv.wait in un while con wait interna oppure usando una cv.wait_while(guard, |s| !*s);

2022-09-08
In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta facendo.
Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher: questo è un oggetto thread-safe che offre il metodo

        dispatch(msg: Msg)

mediante il quale un messaggio di tipo generico Msg (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia sottoscritto.
Un thread interessato a ricevere messaggi può invocare il metodo

        subscribe()

del Dispatcher: otterrà come risultato un oggetto di tipo Subscription mediante il quale potrà leggere i messaggi che da ora in poi saranno pubblicati attraverso
il Dispatcher. Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata (FIFO) di messaggi non ancora letti.
A fronte dell'invocazione del metodo dispatch(msg:Msg), il messaggio viene clonato ed inserito in ciascuna delle code esistenti. L'oggetto Subscription offre il
metodo bloccante

        read() -> Option<Msg>

se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito; se nella coda non è presente nessun messaggio e il Dispatcher esiste
ancora, l'invocazione si blocca fino a che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito il valore corrispondente
all'opzione vuota.

Gli oggetti Dispatcher e Subscription sono in qualche modo collegati, ma devono poter avere cicli di vita indipendenti: la distruzione del Dispatcher non deve impedire la
consumazione dei messaggi già recapitati ad una Subscription, ma non ancora letti; parimenti, la distruzione di una Subscription non deve impedire al Dispatcher di
consegnare ulteriori messaggi alle eventuali altre Subscription presenti.

Si implementino le strutture dati Dispatcher e Subscription, a scelta, nel linguaggio Rust o C++11.
