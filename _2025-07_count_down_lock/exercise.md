

## Exercise 1
Si considerino le strutture dati e rispettive porzioni di codice. Per ciascuna di esse si indichi la dimensione in memoria.

//Rc<u64> = 
    stack 8B
    heap  8B strong + 8B weak  + 8B di u64 

// Rc2 = rc.clone()
    stack 8B
    heap  0B

// wk = Rc:downgrade(&rc)
    stack 8B
    heap  0B

stack = 24B
heap  = 24B

// vector= vec::<u64>::withcapacity(8);
    stack= 8B ptr + 8B len + 8B capacity
    heap = 8B*8 = 64

// vslice = &vector[1..3]
    stack = 8B ptr + 8B len -> (ptr=vector, len=2)
    heap = 0B

stack = 24 + 16 = 40B
heap = 64B

## Exercise 2
Manca il testo completo..