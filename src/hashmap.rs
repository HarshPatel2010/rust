
// FILL in the blanks and FIX the errors
use std::collections::HashMap;
pub fn main() {
    let mut scores:HashMap<&str,i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score:Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }

    use std::collections::HashMap;
    fn main() {
        let teams:[(&str,i32);3] = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        // IMPLEMENT team_map2 in two ways
        // Tips: one of the approaches is to use `collect` method
        let teams_map2:HashMap<&str,i32> =teams.into_iter().collect();

        assert_eq!(teams_map1, teams_map2);

        println!("Success!");
    }
    main()
}