## Domanda 1
Viene utilizzata Mutex e  Condition Variable CondVar.
Quest'ultima è condivisa tra più thread ed è legata ad un LockGuard. Difatto il funzionamento è il medesimo attraverso i metodi:
- guard = cv.wait(guard); aspettiamo che ci arrivi una notifica di un determinato evento, attesa senza consumo di cpu, rilascio il LockGuard che avevo preso e me lo riprendo a notifica ricevuta.
- cv.notify_one/notify_all ; notifichiamo che un evento è avvenuto.

Quando un thread X aspetta che ci siano dei cambiamenti sul mutex aspetta una notifica da thread Y con cv.wait, sarà poi Y a fare cv.notify. Attenzione però alla perdita di notifiche (nel caso ad esempio la notify venga fatta prima di una eventuale wait). In quel caso dobbiamo controllare lo stato del mutex per cui stiamo aspettando la condvar, e possiamo farlo con un while condizione(guard) oppure usando il metodo cv.wait_while o cv.wait_while_timeout se vogliamo anche avere anche una attesa massima e poi procedere comunque (e distinguere i casi con un WaitTimeoutResult).

I mutex appunto li utilizziamo per proteggere e legare temporalmente senza race condition e lost update delle zone di memoria/struct condivise tra thread. Utilizzando il metodo .lock otterremo accesso alla zona di memoria finchè non rilasceremo il lock, nel caso ci siano già un accesso contemporaneo saremo bloccati fino a che l'altro thread non avrà rilasciato il lock. Nel caso di panic dopo aver preso il lock, grazie alla RAII rilasceremo comunque il lock senza lasciare in deadlock, ma il mutex sarà poison per avvisare che ci sono stati problemi di panic e che il dato potrebbe essere manomesso/non completo dell'operazione precedente. 
E bene comunque ricordarsi di fare dei drop(guard) prima di azioni lunghe dove non è più necessario il guard per creare attese inutili.

## Domanda 2
Non è stato fatto quest'anno process. avanti

## Domanda 3
Il problema delle dipendenze cicliche è dovuto per l'appunto a tutte quelle strutture (per esempio i Nodi) che si fanno riferimento tra loro, e la gestione da parte di Rust della liberazione della memoria e possibili memory leak.

prendiamo in esempio

pub struct Node{
    pub next: Option<Arc<Node>> // per velocizzare le assegnazioni nel main per esempio
}

fn main(){
    let mut father: Arc::new(Node{next: None});
    let mut son: Arc::new(Node{next: Some(father.clone)});

    father.next = son.clone();
}

Andiamo a vedere esattamente cosa succede; Abbiamo una relazione Father -> Son ma anche Son -> Father, quindi ciclica.
Stiamo usando anche degli Arc, quindi abbiamo dopo l'ultima istruzione

father strong = 2; weak =0;
son strong = 2; weak = 0;

Strong = 1 per il primo new e poi entrambi i clone aumentano di 1 il corrispettivo contatore strong dell'altro.

a fine scope del main, con il tratto drop perderemo un count strong per entrambi. Ma avremo comunque strong = 1 su entrambi e quindi la memoria non verrà rilasciata (a meno che non si arrivi a Strong =0 ), per questo dovremmo utilizzare dei puntatori Weak, ottenuti attraverso un downgrade partendo da un puntatore strong. I weak possono essere portati a Strong solo se è ancora disponibile uno Strong, e non possono accedere come weak direttamente alla memoria.

pub struct Node{
    pub next: Option<Weak<Node>>
}

fn main(){
    let mut father: Arc::new(Node{next: None});
    let mut son: Arc.new(node {next: some(Arc::downgrade(father.clone))});

    father.next = Arc::downgrade(son.clone);
}

Cosi facendo avremo
strong = 1 e weak =1 per entrambe, e a fine scope avremo strong=0 e conseguente liberazione mem
