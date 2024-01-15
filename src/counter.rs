use std::collections::VecDeque;

pub struct ReqCounter {
    req_num_per_sec: VecDeque<u32>
}
impl ReqCounter{
    pub fn new() -> Self {
        ReqCounter{req_num_per_sec: VecDeque::from([0;8])}
    }
    pub fn get_rps(&self) -> u32 {
        let mut num_full: u32 = 0;
        for e in self.req_num_per_sec.iter().collect::<Vec<_>>() {
            num_full += e;
        }
        num_full / 8
    }
    pub fn change(&mut self, new_num: u32) {
        self.req_num_per_sec.pop_front();
        self.req_num_per_sec.push_back(new_num);
    }
}