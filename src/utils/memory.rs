use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;

/*
mem_used = (mem_total + shmem - mem_free - mem_buffers - mem_cached - mem_srecl
thanks for htop's developer on stackoverflow for providing this algorithm to
calculate used memory.
*/
pub async fn get_memory() -> ThreadsData {
    let buf = match read_to_string("/proc/meminfo") {
        Ok(data) => data,
        _ => return ThreadsData::Memory(String::from("Error Reading memory!")),
    };

    let mut mem_total: u32 = 0;
    let mut shmem: u32 = 0;
    let mut mem_free: u32 = 0;
    let mut mem_buffers: u32 = 0;
    let mut mem_cached: u32 = 0;
    let mut mem_srecl: u32 = 0;

    for line in buf.lines() {
        if mem_total > 0
            && shmem > 0
            && mem_free > 0
            && mem_buffers > 0
            && mem_cached > 0
            && mem_srecl > 0
        {
            break;
        }
        if line.starts_with("MemTotal") {
            assign_val(line, &mut mem_total);
        }
        if line.starts_with("SReclaimable") {
            assign_val(line, &mut mem_srecl)
        }
        if line.starts_with("Cached") {
            assign_val(line, &mut mem_cached)
        }

        if line.starts_with("Shmem") {
            assign_val(line, &mut shmem);
        }

        if line.starts_with("MemFree") {
            assign_val(line, &mut mem_free);
        }
        if line.starts_with("Buffers") {
            assign_val(line, &mut mem_buffers);
        }
    }

    let mem_used = (mem_total + shmem - mem_free - mem_buffers - mem_cached - mem_srecl) / 1024;
    let result: String;
    if mem_used > 1000 {
        result = format!(
            "  {}  {:.1}G  {}",
            CONFIG.memory.icon,
            mem_used as f32 / 1000.0,
            CONFIG.seperator
        );
    } else {
        result = format!(
            "  {}  {}M  {}",
            CONFIG.memory.icon, mem_used, CONFIG.seperator
        );
    }
    ThreadsData::Memory(result)
}

/*
this helper function will split the line(first argument) by the character(:)
and then parse the right splited item as u32
then assign that to the "assignable"(2nd argument).
*/
fn assign_val(line: &str, assignable: &mut u32) {
    let parsed: u32 = line.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    *assignable = parsed;
}
