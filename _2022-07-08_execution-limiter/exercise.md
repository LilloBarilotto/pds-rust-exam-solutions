## Domanda 1
Si definiscano i concetti di Dangling Pointer, Memory Leakage e Wild Pointer, facendo esempi
concreti, usando dello pseudocodice, che possono generare questi fenomeni


1. Dangling Pointer, punta a una zona di memoria che però non gli appartiene, il cui contenuto è quindi sconosciuto e le conseguenze imprevedibili. Questo lo otteniamo riutilizzando un puntatore che magari la quale memoria è stata già rilasciata e riassegnata ad altri, per esempio tramite una free.
PSEUDO CODICE

int main(){
    int * ptr = malloc(sizeof(int));
    free(ptr);
 
    println("{}", *ptr); // A cosa punta effettivamente ptr dopo la liberazione della memoria? Di chi è quella memoria?

    return 0;
}


2. Memory Leakage. Quando non facciamo deallocazione esplicita della memoria, in questo caso otteniamo una perdita di memoria RAM usabile da altri processi/thread.
int main(){
    int * ptr = malloc( 100*sizeof(f64)); // 800B

    return 0;
} // non abbiamo liberato la memoria di 800B nello heap.

2. Memory Leakage. Questo avviene quando effettuiamo una Double Free, dopo aver copiato un puntatore facciamo la free della memoria usando entrambi i puntatori. In questo caso quello che proviamo a fare è liberare memoria che non è nostra. In RUST ciò viene evitato grazie alla implementazione del tratto Drop che viene chiamato quando si esce da uno scope della variabile e alla mutua esclusione tra i tratti Drop e Copy.

PSEUDO CODICE.

int main(){
    int *ptr = malloc(5);
    int *ptr2 = ptr;

    free(ptr); //liberiamo la memoria assegnata a ptr con il valore 5 dentro.
    free(ptr2); // a chi stiamo togliendo la memoria che non è più di nostra competenza?
    return 0;
}

3. Wild Pointer, se non inizializziamo il puntatore e cerchiamo di usarlo non sappiamo a cosa sta effettivamente puntando. Problematica del codice con puntatori nativi.
PSEUDO

int main (){
    int *ptr;

    println({} , *ptr); //Cosa stiamo leggendo??

    return 0;
}




## Domanda 2
In relazione al concetto di Atomic, si definisca cosa esso mira a garantire, come tale garanzia
possa essere fornite a livello architetturale, e quali siano i suoi limiti
-----
L'atomic, applicato a un tipo/struct, mira a garantire 3 condizioni per poter avere zone di memoria e variabili thread-safe:
1. Non interrompibilità, un atomic permette di ottenere un valore che sarà disponibile agli altri solo dopo che tutti gli step verranno effettuati, se ci sono valori intermedi e si verrà interrotti con un panic sarà come se questi non saranno stati effettuati.
2. Non visibilità degli stati intermedi, altri thread non possono vedere gli stati intermedi di un Atomic quando sta per essere calcolato. 
3. Trasparenza e visibilità del valore finale, il cambiamento di un atomic garantisce in un contesto multi-thread che tutte le cache con l'atomic debbano essere refreshate per poter avere l'ultimo vero valore modificato.

Questa garanzia si ottiene grazie a delle istruzioni che sfruttano le fence e le barriere di memoria, inoltre le operazioni da parte dei processori possono seguire un riordine, in base al tipo di atomic che vogliamo usare e al suo use-case possiamo garantire un ordinamento specifico delle altre istruzioni dello stesso thread dove si trova l'atomic:
 - Ordering Relaxed, nessuna specifica, utile per contatori.
 - Ordering Release-Acquire, indica che tutte le operazioni precedenti all'operazione Release dello stesso thread vengano effettuate effettivamente prima di Release, e che tutte le operazioni di Acquire vengano effettuate dopo acquire, con Acquire che viene eseguito dopo Release.
 - Ordering Seq, seguono l'ordine sequenziale di chiamata come sono state scritte, il metodo più stricted.
Inoltre non è bloccante come tipologia di struttura thread-safe. (implementata ad esempio in Arc rispetto al solo Rc proprio per il conteggio atomico dei contatori di Strong-Weak).

Tra le limitazioni abbiamo sicuramente una gestione da parte del programmatore molto più difficile da gestire e l'imposizione che viene effettuata di fatti solo su tipi base. In caso di strutture e complessità maggiore è preferibile usare i lock con Mutex. (o la libreria adattiva Crossbeam con gli AtomicCell).
Tra le istruzioni uniche che abbiamo negli atomic per esempio abbiamo fetch_add, cmp_, swap.
Per esempio possiamo usare fetch_add in un contatore, aggiunge il valore da noi richiesto e ci restituisce il valore precedente del tipo atomic su cui abbiamo chiamato.


## Domanda 3
All'interno di un programma è definita la seguente struttura dati
struct Bucket {
      data: Vec<i32>,
      threshold: Option<i32>
 }
Usando il debugger si è determinato che, per una istanza di Bucket, essa è memorizzata
all'indirizzo 0x00006000014ed2c0.

Osservando la memoria presente a tale indirizzo, viene mostrato il seguente contenuto (per
blocchi di 32bit):
308a6e01 00600000 03000000 00000000 03000000 00000000 01000000 0a000000
Cosa è possibile dedurre relativamente ai valori contenuti dei vari campi della singola istanza?

--- Memoria a 64 bit, quindi 8B di puntatore ( e deriviamo 8B di len e cap).

Partiamo con data, composta internamente da {ptr, len, cap}.

00006000 016e8a30 -> ptr = indirizzo in heap di dove iniziano i valori di vec.
00000000 00000003 -> numero i32 presenti in vec 
00000000 00000003 -> cap = numero i32 massimi allocabili contiguamente, poi bisognerà aumentare la capacity e riallocare se necessario

Poi abbiamo la Option<i32>, che quindi avrà: 
00000001 -> 4B indica il tag
0000000a  -> 4B Some(i32)












2022-07-08
All'interno di un programma è necessario garantire che non vengano eseguite CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente.
A questo scopo, è stata definita la struttura dati ExecutionLimiter che viene inizializzata con il valore N del limite.
Tale struttura è thread-safe e offre solo il metodo pubblico generico execute( f ), che accetta come unico parametro una funzione f, priva di parametri
che ritorna il tipo generico R. Il metodo execute(...) ha, come tipo di ritorno, lo stesso tipo R restituito da f ed ha il compito di mantere il conteggio
di quante invocazioni sono in corso. Se tale numero è già pari al valore N definito all'atto della costruzione della struttura dati, attende, senza provocare
consumo di CPU, che scenda sotto soglia, dopodiché invoca la funzione f ricevuta come parametro e ne restituisce il valore. Poiché l'esecuzione della funzione f
potrebbe fallire, in tale caso, si preveda di decrementare il conteggio correttamente. Si implementi, usando i linguaggi Rust o C++, tale struttura dati,
garantendo tutte le funzionalità richieste.use std::sync::{Arc, Condvar, Mutex};
