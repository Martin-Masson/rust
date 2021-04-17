struct Court {
    name: String,
    occupation: Vec<i32>,
}

impl Court
{
    fn new_court(n: String, o: Vec<i32>) -> Court {
        Court {
            name: String::from(n),
            occupation: o.to_vec(),
        }
    }

    fn add_reservation(&mut self, time: i32){
       self.occupation.push(time)
    }
}

fn is_true() -> bool {
    true
}

fn main()
{
    let mut occupations = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &occupations[1..3];
    println!("{:?}", slice);
    for x in occupations.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }
    println!("{:?}", is_true());
    let mut jean_did_court = Court::new_court("court de jean did".to_string(), [].to_vec());
    jean_did_court.add_reservation(16);
    println!("{}", jean_did_court.occupation[0]);
}