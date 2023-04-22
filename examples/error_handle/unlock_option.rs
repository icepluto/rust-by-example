struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}
fn main() {
    let p_n = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(12),
                number: 123456,
            }),
        }),
    };
    let area_number = p_n.work_phone_area_code();
    assert_eq!(area_number, Some(12));
}
