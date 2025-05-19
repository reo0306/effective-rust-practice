struct S {
    field: Option<i32>,
}

#[derive(Debug, Copy, Clone)]
struct IanaAllocated(pub u64);

fn is_iana_reserved(s: IanaAllocated) -> bool {
    s.0 == 0 || s.0 == 65335
}

#[derive(Debug, Clone)]
pub struct PhoneNumberE164(pub String);

#[derive(Debug)]
pub struct Details {
    pub given_name: String,
    pub preferred_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: String,
    pub mobile_phone: Option<PhoneNumberE164>,
    pub date_of_birth: time::Date,
    pub last_seen: Option<time::OffsetDateTime>,
}

pub struct DetailBuilder(Details);

impl DetailBuilder {
    pub fn new(
        given_name: &str,
        family_name: &str,
        date_of_birth: time::Date,
    ) -> Self {
        DetailBuilder(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone: None,
            date_of_birth,
            last_seen: None,
        })
    }

    pub fn preferred_name(mut self, preferred_name: &str) -> Self {
        self.0.preferred_name = Some(preferred_name.to_owned());
        self
    }

    pub fn middle_name(mut self, middle_name: &str) -> Self {
        self.0.middle_name = Some(middle_name.to_owned());
        self
    }

    pub fn just_seen(mut self) -> Self {
        self.0.last_seen = Some(time::OffsetDateTime::now_utc());
        self
    }

    pub fn build(self) -> Details {
        self.0
    }
}

fn main() {
    let amount_to_add = 3;
    let add_n = |y| {
        y + amount_to_add
    };
    let z = add_n(5);
    println!("{}", z);

    let s = S { field: Some(42) };
    match &s.field {
        Some(i) => println!("field is {i}"),
        None => {}
    }

    if let Some(i) = &s.field {
        println!("field is {i}");
    }

    let s = IanaAllocated(1);
    println!("{:?} reserved? {}", s, is_iana_reserved(s));

    /*let dizzy = Details {
        given_name: "Dizzy".to_owned(),
        preferred_name: None,
        middle_name: None,
        family_name: "Mixer".to_owned(),
        mobile_phone: None,
    };

    println!("{:?}", dizzy);*/

    /*let dizzy2 = Details {
        given_name: "Dizzy".to_owned(),
        family_name: "Mixer".to_owned(),
        ..Default::default()
    };

    println!("{:?}", dizzy2);*/

    let bob = Details {
        given_name: "Dizzy".to_owned(),
        preferred_name: Some("Bob".to_owned()),
        middle_name: Some("the".to_owned()),
        family_name: "Builder".to_owned(),
        mobile_phone: None,
        date_of_birth: time::Date::from_calendar_date(
            1998,
            time::Month::November,
            28,
        ).unwrap(),
        last_seen: None,
    };
    println!("{:?}", bob);

    let also_bob = DetailBuilder::new(
        "Robert",
        "Builder",
        time::Date::from_calendar_date(1998, time::Month::November, 29).unwrap(),
    )
    .middle_name("the")
    .preferred_name("Bob")
    .just_seen()
    .build();
    println!("{:?}", also_bob);
}
