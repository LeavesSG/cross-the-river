#[derive(PartialEq, Eq, Debug)]
enum Person {
    Male(&'static str),
    Female(&'static str),
}

use std::vec::IntoIter;

use crate::dfs::Connect;
use Person::*;

const FATHER: Person = Male("Father");
const MOTHER: Person = Female("Mother");
const BROTHER: Person = Male("Brother");
const SISTER: Person = Female("Sister");
const JIE_GE: Person = Male("JieGe");

struct BoatPassenger<'a> {
    pub driver: &'a Person,
    pub another: Option<&'a Person>,
}

impl BoatPassenger<'_> {
    pub fn new<'a>(driver: &'a Person, passenger: Option<&'a Person>) -> BoatPassenger<'a> {
        BoatPassenger {
            driver,
            another: passenger,
        }
    }
    pub fn to_tuple<'a>(&'a self) -> (&'a Person, Option<&'a Person>) {
        (self.driver, self.another)
    }
    pub fn to_vec<'a>(&'a self) -> Vec<&'a Person> {
        let mut vec = vec![self.driver];
        if let Some(another) = self.another {
            vec.push(another);
        }
        vec
    }
}

struct State<'a> {
    this: Vec<&'a Person>,
    that: Vec<&'a Person>,
    dir: bool,
}

impl State<'_> {
    fn validate_on_boat(boat: &BoatPassenger) -> bool {
        match boat.to_tuple() {
            (&SISTER, _) => false,
            (&FATHER, Some(&SISTER)) => false,
            (&BROTHER, Some(&SISTER)) => false,
            (&MOTHER, Some(&Male(_))) => false,
            (&Male(_), Some(&MOTHER)) => false,
            (&JIE_GE, Some(&BROTHER)) => false,
            (_, _) => true,
        }
    }

    fn side_io(&self) -> (&Vec<&Person>, &Vec<&Person>) {
        if self.dir {
            (&self.this, &self.that)
        } else {
            (&self.that, &self.this)
        }
    }

    fn pick_passenger<'a>(&'a self) -> Vec<BoatPassenger<'a>> {
        let candidates = self.side_io().0;
        let mut picked = vec![];
        let mut cross;
        for driver in candidates {
            for pas in candidates
                .iter()
                .map(|p| if driver.eq(p) { None } else { Some(*p) })
            {
                cross = BoatPassenger::new(*driver, pas);
                if Self::validate_on_boat(&cross) {
                    picked.push(cross);
                }
            }
        }
        picked
    }

    fn next_state<'a>(&'a self, cross: BoatPassenger<'a>) -> State<'a> {
        let this = self
            .side_io()
            .0
            .clone()
            .into_iter()
            .filter(|p| (*p).ne(cross.driver) && Some(*p).ne(&cross.another))
            .collect();
        let mut that = self.side_io().1.clone();
        that.push(cross.driver);
        State {
            this,
            that,
            dir: !self.dir,
        }
    }
}

pub fn cross_the_river() {
    let init = State {
        this: vec![&FATHER, &MOTHER, &BROTHER, &SISTER, &JIE_GE],
        that: vec![],
        dir: true,
    };
    let res = init.pick_passenger();
}

impl<'a> Connect<'a, State<'a>, IntoIter<State<'a>>> for State<'a> {
    fn get_adj(&self) -> IntoIter<State<'a>> {
        let pairs = self.pick_passenger();
        let s = pairs
            .into_iter()
            .map(|pair| self.next_state(pair))
            .collect::<Vec<_>>()
            .into_iter();
        s
    }
}

#[test]
fn test() {
    cross_the_river();
}
