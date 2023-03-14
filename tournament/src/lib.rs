use std::collections::HashMap;

pub struct Team<'a> {
    name: &'a str,
    games: u8,
    wins: u8,
    losses: u8,
    draws: u8,
    points: u8
}

impl<'a> Team<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            games: 0,
            wins: 0,
            losses: 0,
            draws: 0,
            points: 0
        }
    }

    pub fn add_win(&mut self) {
        self.games += 1;
        self.wins += 1;
        self.points += 3;
    }

    pub fn add_loss(&mut self) {
        self.games += 1;
        self.losses += 1;
    }

    pub fn add_draw(&mut self) {
        self.games += 1;
        self.draws += 1;
        self.points += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, Team> = HashMap::new();
    
    for result in match_results.split('\n') {
        if result.is_empty() {
            break;
        }

        let mut res = result.split(';');
        let team1 = res.next().unwrap();
        let team2 = res.next().unwrap();

        match res.next().unwrap() {
            "win" => {
                teams.entry(team1).or_insert_with(|| Team::new(team1)).add_win();
                teams.entry(team2).or_insert_with(|| Team::new(team2)).add_loss();
            },
            "loss" => {
                teams.entry(team2).or_insert_with(|| Team::new(team2)).add_win();
                teams.entry(team1).or_insert_with(|| Team::new(team1)).add_loss();
            },
            _ => {
                teams.entry(team1).or_insert_with(|| Team::new(team1)).add_draw();
                teams.entry(team2).or_insert_with(|| Team::new(team2)).add_draw();
            }
        }
    }

    let mut tally = String::from("Team                           | MP |  W |  D |  L |  P\n");
    let mut teams: Vec<Team> = teams.into_values().collect();
    teams.sort_by(|a, b| (-(a.points as i16), a.name).cmp(&(-(b.points as i16), b.name)));
    
    tally.extend(
        teams.into_iter()
            .map(|team| format!("{:<31}|  {} |  {} |  {} |  {} |  {}\n", team.name, team.games, team.wins, team.draws, team.losses, team.points))
    );
    
    tally.pop();
    tally
}
