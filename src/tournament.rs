use std::collections::HashMap;

fn main() {
    let input: &[&str] = &[
        "Allegoric Alaskans;Blithering Badgers;win",
        "Devastating Donkeys;Courageous Californians;draw",
        "Devastating Donkeys;Allegoric Alaskans;win",
        "Courageous Californians;Blithering Badgers;loss",
        "Blithering Badgers;Devastating Donkeys;loss",
        "Allegoric Alaskans;Courageous Californians;win",
    ];
    let input = input.join("\n");
    let output = tally(&input);
    let expected = [
        "Team                           | MP |  W |  D |  L |  P",
        "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
        "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
        "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
        "Courageous Californians        |  3 |  0 |  1 |  2 |  1",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[derive(Debug, Default)]
struct TeamStats {
    matches: u32,
    wins: u32,
    draws: u32,
    loses: u32,
    points: u32,
}

impl TeamStats {
    pub fn new() -> TeamStats {
        TeamStats {
            matches: 0,
            wins: 0,
            draws: 0,
            loses: 0,
            points: 0,
        }
    }

    /**
     * Result can be 'win', 'draw' or 'lose'.
     */
    pub fn process_result(&mut self, outcome: &str) {
        self.matches += 1;

        match outcome {
            "win" => {
                self.wins += 1;
                self.points += 3;
            },
            "draw" => {
                self.draws += 1;
                self.points += 1;
            },
            "loss" => {
                self.loses += 1;
            },
            &_ => {}
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<String, TeamStats> = HashMap::new();

    for line in match_results.to_string().lines() {
        process_match(line.to_string(), &mut table);
    }

    let formatted_table = format_table(&mut table);

    formatted_table
}

/**
 * Processes a line representing one match result.
 * Updates table.
 */
fn process_match(match_result: String, table: &mut HashMap<String, TeamStats>) {
    let match_result = match_result
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let team1_name = match_result[0].clone();
    let team2_name = match_result[1].clone();
    let team1_outcome = match_result[2].clone();

    let mut team2_outcome = "draw";

    if team1_outcome == "win" {
        team2_outcome = "loss"
    } else if team1_outcome == "loss" {
        team2_outcome = "win"
    }

    {
        let team1_stats = table.entry(team1_name.clone()).or_insert_with(TeamStats::new);
        team1_stats.process_result(&team1_outcome);
    }

    {
        let team2_stats = table.entry(team2_name.clone()).or_insert_with(TeamStats::new);
        team2_stats.process_result(team2_outcome);
    }
}

/**
 * Sorts table and formats it.
 */
fn format_table(table: &mut HashMap<String, TeamStats>) -> String {
    let mut sorted_teams: Vec<(&String, &TeamStats)> = table.iter().collect();
    sorted_teams.sort_by(|a, b| {
        b.1.points.cmp(&a.1.points)
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut table_string = "Team                           | MP |  W |  D |  L |  P".to_string();

    for (name, stats) in &sorted_teams {
        table_string += &format_current_team(name, stats);
    }

    table_string
}

fn format_current_team(name: &&String, stats: &&TeamStats) -> String {
    let mut formatted_row = "\n".to_string();

    // Remaining space characters on 'name' column
    let name_spaces = 31 - name.len() as u32;

    formatted_row += name;
    formatted_row += &(0..name_spaces).map(|_| " ").collect::<String>();
    formatted_row += "|  ";
    formatted_row += &stats.matches.to_string();
    formatted_row += " |  ";
    formatted_row += &stats.wins.to_string();
    formatted_row += " |  ";
    formatted_row += &stats.draws.to_string();
    formatted_row += " |  ";
    formatted_row += &stats.loses.to_string();
    let mut spaces_for_points_column = " ";
    if &stats.points > &9 {
        spaces_for_points_column = "";
    }

    formatted_row += " | ";
    formatted_row += spaces_for_points_column;
    formatted_row += &stats.points.to_string();

    formatted_row
}
