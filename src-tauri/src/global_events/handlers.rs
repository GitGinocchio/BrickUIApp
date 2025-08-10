use rdev::{simulate, EventType, SimulateError};


#[tauri::command]
fn simulate_event(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}