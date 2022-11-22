
#[derive(Clone, Debug)]
struct Player {
    name: String,
    rating: f32,
}

#[derive(Debug)]
struct Team {
    players: Vec<Player>,
}

impl Team{
    fn new() -> Team{
        Team{
            players: Vec::new()
        }
    }

    fn print(&self){
        for player in self.players.iter(){
            println!("{}", player.name);
        }
    }

    fn rating(&self) -> f32 {
        let mut rating = 0.0;
        for player in self.players.iter(){
            rating += player.rating;
        }
        rating
    }
}

fn main() {
    // Create player list
    let players = vec![
        Player{ name: String::from("Jason"), rating: 0.3},
        Player{ name: String::from("Max"), rating: 0.4},
        Player{ name: String::from("Kevin"), rating: -0.4},
        Player{ name: String::from("Jake"), rating: -0.1},
        Player{ name: String::from("Emilio"), rating: 0.2},
        Player{ name: String::from("Yi-Khy"), rating: 0.13},
        Player{ name: String::from("Josh"), rating: 0.38},
        Player{ name: String::from("Younis"), rating: -0.03},
        Player{ name: String::from("Heather"), rating: 0.04},
    ];

    // Create two teams
    let mut team1: Team = Team::new();
    let mut team2: Team = Team::new();
    for (i, player) in players.iter().enumerate(){
        if i % 2 == 0 {
            team1.players.push(player.clone());
        }
        else{
            team2.players.push(player.clone());
        }
    }

    // Output the teams
    println!("Team 1: {}", team1.rating());
    team1.print();
    println!("\n");

    println!("Team 2: {}", team2.rating());
    team2.print();
}

