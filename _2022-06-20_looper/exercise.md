# Teoria

## Domanda 1

> Si definisca ii concetto di Smart Pointer, quindi si fornisca un esempio (Rust o C++)
> che ne evidenzi il ciclo di vita.

Uno smart pointer è una struttura dati che funge da puntatore ma offre funzionalità aggiuntive, come la gestione automatica della memoria o il conteggio dei riferimenti. In Rust, gli smart pointer implementano i trait `Deref` (per accedere al dato puntato) e `Drop` (per gestire la distruzione della risorsa). Si basano sul pattern RAII (*Resource Acquisition Is Initialization*): l'allocazione avviene all'inizializzazione e la deallocazione è legata al ciclo di vita (scope) dell'oggetto.

Esempi principali:

* `Box<T>`: Per l'allocazione di dati nello heap con proprietà esclusiva.
* `Rc<T>` / `Arc<T>`: Per la proprietà condivisa tramite conteggio dei riferimenti (Reference Counting).

**Esempio di ciclo di vita:**

```rust
fn main() {
    println!("Inizio");
    {
        // Allocazione nello heap
        let b = Box::new(5); 
        println!("Valore: {}", b);
    } // 'b' esce dallo scope qui: la memoria heap viene liberata automaticamente
    println!("Fine");
}
```

## Domanda 2

> Si illustrino le differenze nel linguaggio Rust tra std::channel() e std::sync_channel(), indicando quali tipi di sincronizzazione i due meccanismi permettono.

* **std::sync::mpsc::channel()**: Crea un canale **asincrono** con buffer illimitato. La funzione `send` non è mai bloccante (non consuma cicli CPU in attesa di spazio), permettendo al produttore di procedere indipendentemente dal consumatore. Il rischio è l'esaurimento della memoria se il consumatore è troppo lento.
* **std::sync::mpsc::sync_channel(bound)**: Crea un canale **sincrono** con buffer limitato alla dimensione `bound`. La funzione `send` diventa **bloccante** quando il buffer è pieno, fornendo un meccanismo di *backpressure* che sincronizza la velocità del produttore con quella del consumatore. Se il `bound` è 0, il mittente si blocca finché il ricevente non preleva il messaggio (rendezvous).

In sintesi, `channel()` permette una sincronizzazione a senso unico e disaccoppiata, mentre `sync_channel()` permette una sincronizzazione più stretta che regola il flusso di dati tra i thread.

## Domanda 3

> Dato ii seguente frammento di codice Rust (ogni linea e preceduta dal suo indice), si descriva ii contenuto dello stack e dello heap al termine dell'esecuzione della riga
> 15.

```rust
1.  struct Point { 
2.   x: i16, 
3.   y: i16,
4.  }
5.
6.  enum PathCommand { 
7.   Move(Point), 
8.   Line(Point),
9.   Close,
10. }
11. let mut v = Vec::<PathCommand>::new();
12. v.push(PathCommand::Move(Point{x:1,y:1 }));
13. v.push(PathCommand::Line(Point{x:10, y:20}));
14. v.push(PathCommand::Close);
15. let slice = &v[..];
```

Prima di tutto:

* struct Point, sono 2 i16, quindi 2*2B, totale 4B. Va bene per l'align considerando che max_align = 2B tra i16 solo.
* enum PathCommand, 1B di tag, e poi la union tra struct Point fa la struct Point, quindi in totale PARZIALE 1B + 4B = 5B.
Dobbiamo aggiungere del padding per arrivare al minimo multiplo che vada bene per il max_align, quindi essendo max_align quello di Point, quindi 2B, il min per tenere dentro 5B è 6B, quindi metto del padding di 1B dopo il tag. Totale = 6B.
* ipotizziamo che sia una architettura a 64bit e quindi 8B di riferimento per i puntatori, len, cap, etc.

Riga 11. Definisce un vec mutabile di enum PathCommand, i valori andranno nell'heap.
Lo stack di v conterrà 8B puntatore heap, 8B len, 8B cap. = 24B

Riga 12-13-14 Aggiungo 3 enum alla struttura, heap = 3*6B_PathCommand = 18B. (ipotizziamo che cap sia 3 e che non sia stato messo in potenza di 2 quindi con 4 elementi, se cosi fosse heap =4*6B=24B).

Riga 15. slice crea un riferimento &[T], che quindi nello heap non aggiungerà nulla, nello stack avremo 8B ptr ai dati di v nell'heap + 8B len = 16B.

TOTALE BYTE:
- STACK, 24B v + 16B slice = 40B
- HEAP, 18B valori di v. (oppure 24 dipende da cap).

Riguardo a questa discussione del tema del 2022-06-20 looper
La regola generale è (domando):
1. size struct Point = minimo_multiplo del max_alignment tra tutti gli elementi dentro la struct Point, quindi in questo caso elementi da i16=2B, align =2B, in totale fan 4B quindi ho 4B di size Point.
2. size enum = minimo_multiplo del max_align tra tutti gli align delle strutture interne, quindi ho 4B del Point, 1B di tag, per l'align di 2B della struttura point devo aggiungere 1B di padding al tag per avere i 2B align? Quindi totale 6B?

Cioè spiegato sicuramente male ma in pratica per la size mi devo basare (sia per struct che enum) per il primo la min_size che è multipla dell'align maggiore dentro la struct e che contenga tutti gli elementi delle struct, e per l'enum la stessa cosa ma oltre che tutti gli elementi dell'enum ci aggiungo anche 1B di tag.
# Pratica - Looper

Un paradigma frequentemente usato nei sistemi reattivi e costituito dall'astrazione detta Looper.
Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
II thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda,
li estrae a uno a uno nell'ordine di arrivo, e li elabora.

II costruttore di Looper riceve due parametri, entrambi di tipo (puntatore a) funzione: process( ... ) e cleanup().
La prima è una funzione responsabile di elaborare i singoli messaggi ricevuti attraverso la coda;
tale funzione accetta un unico parametro in ingresso di tipo Message e non ritorna nulla;
La seconda e funzione priva di argomenti e valore di ritorno e verra invocata dal thread incapsulato
nel Looper quando esso stara per terminare.

Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne ii ciclo di vita:
send(msg), che accetta come parametro un oggetto generico di tipo Message che verra inserito nella coda
e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione.
Quando un oggetto Looper viene distrutto, occorre fare in modo che ii thread contenuto al suo interno
invochi la seconda funzione passata nel costruttore e poi termini.

Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi
metodi dovranno essere thread-safe.
