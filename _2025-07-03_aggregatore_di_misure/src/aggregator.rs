use std::{collections::HashMap, sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle, sleep}, time::{Duration, Instant}, vec};


pub struct AggregatorMeasuresAverages {
    running: bool,
    measures: HashMap<usize, Vec<(Instant, f64)>>, // mappa che associa a ciascun sensore un vettore di tuple (timestamp, temperatura)
    averages: HashMap<usize, Average>, // mappa che associa a ciascun sensore la media calcolata durante l'ultimo periodo di campionamento
}

pub struct Aggregator {
    sample_time_millis: u64,
    aggregates_measures_averages: Arc<(Mutex<AggregatorMeasuresAverages>, Condvar)>,
    handle_aggregator : Option<JoinHandle<()>>,
}

#[derive(Clone)]
pub struct Average {
   pub sensor_id: usize,
   pub reference_time: Instant,        //indica l'istante temporale in cui è stata calcolata la media
   pub average_temperature: f64,
}
impl Aggregator {
   pub fn new(sample_time_millis: u64) -> Self {

        let aggregates_measures_averages = Arc::new((
            Mutex::new(AggregatorMeasuresAverages{
                measures: HashMap::new(),
                averages: HashMap::new(),
                running: true,
            }),
            Condvar::new()
            ));       

       let aggr_clone = Arc::clone(&aggregates_measures_averages);
        
       let handle_aggregator = thread::spawn(move || {
            
            let (mutex, cvar) = &*aggr_clone;
            let mut data = mutex.lock().unwrap();

            while data.running {
                let instant = Instant::now();
                let sleep_duration = Duration::from_millis(sample_time_millis);
                let next_wakeup = instant + sleep_duration;

                let res = cvar.wait_timeout_while(data, sleep_duration, |d| d.running).unwrap();
                
                // Se il thread è stato svegliato perché è stato notificato/d.running (e non perché è scaduto il timeout), esce dal ciclo.
                data = res.0;
                let timeout_result = res.1;
                if !timeout_result.timed_out() {
                    continue;
                }
                
                let mut new_averages = Vec::<Average>::new();

                // recreate_average
                for (sensor_id, vec_measures) in data.measures.iter_mut() {
                    if vec_measures.is_empty() {
                        continue;
                    }

                    let partition_index = vec_measures.partition_point(|(timestamp, _)| *timestamp <= next_wakeup);
                    if partition_index == 0 {
                        continue;
                    }

                    let vec_measures_partition = vec_measures.drain(..partition_index);
                    
                    let average_temperature = vec_measures_partition.map(|(_, temp)| temp).sum::<f64>() / (partition_index as f64);
                    new_averages.push(Average { sensor_id: *sensor_id, reference_time: next_wakeup, average_temperature });
                }

                for avg in new_averages {
                    data.averages.insert(avg.sensor_id, avg);
                }

            }
       });   

       Self {
           sample_time_millis,
           aggregates_measures_averages: aggregates_measures_averages,
           handle_aggregator: Some(handle_aggregator),
       }

   }

pub fn add_measure(&self, sensor_id: usize, temperature: f64) {
    let timestamp = Instant::now();
    let (lock, _) = &*self.aggregates_measures_averages;
    let mut data = lock.lock().unwrap();

    let vec_sensor = data.measures.entry(sensor_id).or_default();

    // Cerca la posizione di inserimento per mantenere l'ordine.
    // `binary_search_by` ci dà l'indice corretto sia che trovi una corrispondenza esatta (Ok)
    // sia che non la trovi (Err).
    let insertion_index = vec_sensor
        .binary_search_by(|(t, _)| t.cmp(&timestamp))
        .unwrap_or_else(|err_index| err_index);

    // Inserisci la nuova misura nella posizione corretta per mantenere l'ordinamento.
    vec_sensor.insert(insertion_index, (timestamp, temperature));
}

   pub fn get_averages(&self) -> Vec<Average> {
       // restituisce un vettore che riporta la temperatura media di ciascun sensore,
       // calcolata durante l'ultimo periodo di campionamento.
       // Sono presenti solo i sensori che hanno inviato almeno una misura.
       let guard = self.aggregates_measures_averages.0.lock().unwrap();
        guard.averages.values().cloned().collect::<Vec<Average>>()
   }
}


impl Drop for Aggregator {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.aggregates_measures_averages;
        let mut data = lock.lock().unwrap();
        data.running = false;
        cvar.notify_one(); // sveglia il thread dell'aggregatore per farlo terminare

        if let Some(handle) = self.handle_aggregator.take() {
            handle.join().expect("Failed to join aggregator thread");
        }
    }
}