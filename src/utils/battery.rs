use crate::config::CONFIG;
use crate::types::ThreadsData;
use battery::Manager;

// TODO: better error handeling

// getting battery percentage
pub async fn get_battery() -> ThreadsData {
    let battery_manager = if let Ok(manager) = Manager::new() {
        manager
    } else {
        return ThreadsData::Battery(String::from("Cannot Create Battery Manager!"));
    };

    let mut batteries = if let Ok(batteries) = battery_manager.batteries() {
        batteries
    } else {
        return ThreadsData::Battery(String::from("Cannot Get Battery!"));
    };

    let percentage = if let Some(battery) = batteries.next() {
        f32::from(battery.unwrap().state_of_charge()) * 100.0
    } else {
        return ThreadsData::Battery(String::from("Cannot Read Battery!"));
    };

    let result = format!(
        "  {}  {:.0}%  {}",
        CONFIG.battery.icon, percentage, CONFIG.seperator
    );
    ThreadsData::Battery(result)
}
