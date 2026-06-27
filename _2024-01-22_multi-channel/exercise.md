## Teoria 1
Sia data la struttura LinkedList<T> definita come:
```rust
pub struct LinkedList<'a, T> {
    pub val: Option<T>,
    pub next: Option<&'a Box<LinkedList<'a, T>>>,
} 
```

Un elemento in lista occupa in memoria:
 stack:
    - val = 1B tag
            xB per T se c'è Some(T)
    - next= 1B tag
            8B per il Some(Box<LinkedList<T>)
            MA IN QUESTO CASO ESSENDO UN Option<Puntatore> posso anche solo salvare il puntatore da 8B senza il tag, o puntatore o nulla (che tanto non può essere null un puntatore).

 heap:
    - next: 1B +xB + 1B + 8B


Il fine lista viene definito con next = None()

## Teoria 2
Si definisca un esempio in cui, data la necessità di creare N thread, si possano evitare  race-
conditions nel momento in cui i thread debbano accedere in scrittura alla stessa risorsa. Si
distingua il caso in cui tale risorsa sia uno scalare e quella in cui sia una struttura più articolata.

Immaginiamo di avere un vettore condiviso:
```rust
count = 0;
while count != 5 {
    count = count + 1;
    print("{}", count);
}
```

Potremmo pensare di scrivere questo per contare fino a 5 con N thread, e poi uscire, ma cosa succedere nel caso in cui (siamo a count = 4 ) ed N1 ed N2 accedono contemporaneamente nel while:
    N1: accede e vede count=4,  fa count = 4+1, poi N2 in mezzo fa il suo calcolo, fa print ma esce un valore strano, fa print 6, e continua non bloccando il while
    N2: accede con count=4, ma quando fa count = count +1, insieme al +1 il nuovo valore di count è quello modificato da N1, quindi fa count = 5+1, vale 6, fa print e magari vale il count successivo di N1 con print 7.

Per evitare ciò si deve fare lock della variabile così da non farla modificare o leggere da altri mentre avvengono delle modifiche.
Oltre alle strutture Mutex che si applicano su tutte le strutture articolate, per una più semplice struttura ci sono le operazioni atomiche, che si appoggiano alle operazioni barrier dei processori.


