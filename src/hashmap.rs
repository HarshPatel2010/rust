
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

        // FILL in the blanks
        use std::collections::HashMap;
        fn main() {
            // Type inference lets us omit an explicit type signature (which
            // would be `HashMap<&str, u8>` in this example).
            let mut player_stats = HashMap::new();

            // Insert a key only if it doesn't already exist
            player_stats.entry("health").or_insert(100);

            assert_eq!(player_stats["health"], 100);

            // Insert a key using a function that provides a new value only if it
            // doesn't already exist
            player_stats.entry("health").or_insert_with(random_stat_buff);
            assert_eq!(player_stats["health"], 100);

            // Ensures a value is in the entry by inserting the default if empty, and returns
            // a mutable reference to the value in the entry.
            let health:&mut u8 = player_stats.entry("health").or_insert(50);
            assert_eq!(health, &100);
            *health -= 50;
            assert_eq!(*health, 50);

            println!("Success!");

            // FIX the errors
            // Tips: `derive` is usually a good way to implement some common used traits
            use std::collections::HashMap;
#[derive(Debug,Hash,Eq,PartialEq)]
            struct Viking {
                name: String,
                country: String,
            }

            impl Viking {
                /// Creates a new Viking.
                fn new(name: &str, country: &str) -> Viking {
                    Viking {
                        name: name.to_string(),
                        country: country.to_string(),
                    }
                }
            }

            fn main() {
                // Use a HashMap to store the vikings' health points.
                let vikings :HashMap<Viking,i32>= HashMap::from([
                    (Viking::new("Einar", "Norway"), 25),
                    (Viking::new("Olaf", "Denmark"), 24),
                    (Viking::new("Harald", "Iceland"), 12),
                ]);

                // Use derived implementation to print the status of the vikings.
                for (viking, health) in &vikings {
                    println!("{:?} has {} hp", viking, health);
                }

                use std::collections::HashMap;
                fn main() {
                    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
                    map.insert(1, 2);
                    map.insert(3, 4);
                    // Indeed ,the capacity of HashMap is not 100, so we can't compare the equality here.
                    assert!(map.capacity() >= 100);

                    // Shrinks the capacity of the map with a lower limit. It will drop
                    // down no lower than the supplied limit while maintaining the internal rules
                    // and possibly leaving some space in accordance with the resize policy.

                    map.shrink_to(50);
                    assert!(map.capacity() >= 50);

                    // Shrinks the capacity of the map as much as possible. It will drop
                    // down as much as possible while maintaining the internal rules
                    // and possibly leaving some space in accordance with the resize policy.
                    map.shrink_to_fit();
                    assert!(map.capacity() >= 2);
                    println!("Success!");
                    // FIX the errors with least changes
                    // DON'T remove any code line
                    use std::collections::HashMap;
                    fn main() {
                        let v1 = 10;
                        let mut m1 = HashMap::new();
                        m1.insert(v1, v1);
                        println!("v1 is still usable after inserting to hashmap : {}", v1);

                        let v2 = "hello".to_string();
                        let mut m2 = HashMap::new();
                        // Ownership moved here
                        m2.insert(&v2, v1);

                        assert_eq!(v2, "hello");

                        println!("Success2!");
                    }
                    main()
                }
                main()
            }

            main()
        }

        fn random_stat_buff() -> u8 {
            // Could actually return some random value here - let's just return
            // some fixed value for now
            42
        }
        main()
    }
    main()
}