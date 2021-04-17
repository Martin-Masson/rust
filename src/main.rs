struct Court
{
    name: String,
    occupation: Vec<i32>
}

struct Club
{
    courts: Vec<Court>,
    open: i32,
    close: i32  
}

impl Club
{
    // Constructor
    fn new_club(opening: i32, closure: i32) -> Club
    {
        Club {
            open: opening,
            close: closure,
            courts: Vec::new()
        }
    }

    // Methods
    fn add_court(&mut self, name: &str) -> bool
    {
        self.courts.push(
            Court {
                name: name.to_string(),
                occupation: Vec::new()
            }
        );
        return true;
    }
}

fn main()
{
    let my_club = Club::new_club(8, 20);
}