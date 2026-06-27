## Teoria 1 (3pt)
1)
Riga 14: "last: {(c8)}"
Riga 15: "res: 3"

2) Riga 5: trasforma il vec in un iterator su cui accediamo in lettura, essendo all'interno numero con il tratto Copy non abbiamo problemi nemmeno a cancellare elementi ma numbers rimane invariato.
Riga 6: filtra e fa passare alla catena successiva solo gli elementi di numbers pari, quindi 2-4-8.
Riga 7: combina l'iterator arrivato dal filtro [2,4,8] con un altro iterator ['a'...'z'], creando un nuovo iterator con la tupla con primo elemento numero e il secondo elemento lettera, in questo caso avrema solo [(2,a), (4,b), (8,c)], vengono presi n elementi con n = min(iterator1.len(), iterator2.len())

3) Se omettiamo la riga 10 non va avanti perchè la .map() consuma il valore interno di res e poi cerchiamo di accedere a res.count(), per questo motivo il borrow checker ci avrebbe bloccato.
Mettiamo quindi .clone() per non avere questo problema


## Teoria 2 (3pt)
1. prima di cvar.notify_one() sarebbe meglio fare drop(started) per liberare il lock.
2. started = cvar.wait(started).unwrap.
   Questa rimane in attesa finchè non gli arriva una notifica, ma per come è attualmente strutturato il codice la cvar.notify_one() potrebbe attivarsi e mandare una notifica prima ancora che si attivi la condizione cvar.wait, riportando quest'ultima in una condizione di wait infinita con la notifica andata persa.

Per correggere il problema, si potrebbe effettuare una cvar.wait_timeout(started, Duration::from_secs(1)), cosi da aspettare un secondo al massimo prima di controllare la condizione e poi vedere il valore di started, ma non risolvere effettivamente il problema ma non ci blocca.

Per risolvere il problema si può fare:
while !*started{
    started = cvar.wait(started).unwrap();
}
Oppure un
cvar.wait_while(started, |s|, !*s)

## Teoria 3 (3pt)
Il codice seguente genera un errore di compilazione: spiegare perché e indicare come 
modificare la struct S (attraverso l'aggiunta di tratti) per renderlo compilabile ed eseguibile. 
```rust
#[derive(Debug)] 
struct S { 
    i: i32, 
} 
 
impl From<i32> for S { fn from(value: i32) -> Self { S { i: value } } } 

fn main() { 
    let mut v = Vec::<S>::new(); 
    let s = 42.into(); 
    for i in 0..3 { 
        v.push(s); 
    } 
    println!("{:?}",v); 
}
```

Come soluzione basterebbe mettere Copy, Clone nella derive
perchè altrimenti s (che viene inferito come : S perchè va dentro il v.push(s)) viene fatto il move con il push, quindi bisogna dargli copy e clone per non fare la move del valore.

