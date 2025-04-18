#[cfg(test)]

mod app {
    use crate::app::rng::{DefaultRandomSource, RandomSource};
    use crate::app::state::{LightSource, LightSourceType};
    use crate::app::state::{ShadowtrackData, TurnEntry};
    use crate::app::systems::{process_light_burn, roll_encounter, roll_light_event};
    use crate::app::ShadowtrackApp;
    use std::ops::Sub;
    use std::time::{Duration, Instant};

    #[test]
    fn test_app_defaults() {
        let app = ShadowtrackApp::default();
        assert!(&app.data.clock_elapsed.eq(&0));
        assert!(&app.data.event_log.is_empty());
        assert!(&app.data.light_sources.is_empty());
        assert!(!&app.data.encounter_table.is_empty());
        assert!(!&app.data.torch_event_table.is_empty());
    }

    #[test]
    fn test_should_tick() {
        let mut app = ShadowtrackApp::default();
        let now = Instant::now();

        // If no time has passed since 'last_tick', we should not tick -> return None
        app.last_tick = now;
        assert_eq!(app.should_tick(now), None);
        
        // If time has passed since 'last_tick', we should tick -> returning the difference. 
        app.last_tick = app.last_tick.sub(Duration::from_secs(30));
        assert_eq!(app.should_tick(now), Some(30));
    }

    #[test]
    fn test_app_game_clock_ticks() {
        let mut app = ShadowtrackApp::default();
        assert_eq!(app.clock_running, false);
        app.toggle_clock();
        assert_eq!(app.clock_running, true);

        app.last_tick = Instant::now().sub(Duration::from_secs(60));
        app.handle_clock_tick();

        assert_eq!(app.data.clock_elapsed, 60);
    }

    #[test]
    fn test_app_handles_intervaled_events() {
        let mock_light_source = LightSource {
            label: "Mock Light Label".to_string(),
            radius_feet: 10,
            light_type: LightSourceType::Spell("Mock Testing Spell".to_string()),
            minutes_remaining: 420,
            last_roll: Some(69),
        };
        let mut app = ShadowtrackApp::default();
        
        // Add a LightSource to test against
        app.data.light_sources.push(mock_light_source.clone());
        app.toggle_clock();

        // Set last_tick to 60 seconds in the past and handle the time change.
        app.last_tick = Instant::now().sub(Duration::from_secs(60));
        app.handle_clock_tick();

        // Did we account for those 60 seconds and properly set up the interval period?
        assert!(app.data.event_log.is_empty());
        assert_eq!(app.data.clock_elapsed, 60);
        assert_eq!(
            app.data.next_process_minutes,
            Some(app.data.process_interval_minutes)
        );

        // Advance the clock 10 minutes, fake a last_tick in the past, handle the time change.
        app.data.clock_elapsed = 0;
        app.last_tick = Instant::now().sub(Duration::from_secs(600));
        app.handle_clock_tick();

        // Did we account for the 10 minutes properly by running the interval processes?
        assert!(!app.data.event_log.is_empty());
        assert!(
            app.data.light_sources.get(0).unwrap().minutes_remaining
                < mock_light_source.minutes_remaining
        );
        assert_eq!(app.data.clock_elapsed, 600);
    }

    #[test]
    fn add_new_encounter_table_entry() {
        let mut data = ShadowtrackData::default();
        let new_entry = "Spectral hound".to_string();
        data.encounter_table.push(new_entry.clone());
        assert!(data.encounter_table.contains(&new_entry));
    }

    #[test]
    fn log_ordering() {
        // TODO: This really just checks that Vec maintains insertion order, and it does.
        // Currently I dont know how to properly setup a testing harness to test the egui widget
        // displaying the log in reverse order. So for now this is just a sanity check.
        let mut data = ShadowtrackData::default();
        let turn_entry_0 = TurnEntry {
            turn: 0,
            events: vec![
                "First".to_string(),
                "Second".to_string(),
                "Third".to_string(),
            ],
        };
        let turn_entry_1 = TurnEntry {
            turn: 1,
            events: vec![
                "Forth".to_string(),
                "Fifth".to_string(),
                "Sixth".to_string(),
            ],
        };
        let turn_entry_2 = TurnEntry {
            turn: 2,
            events: vec![
                "Seventh".to_string(),
                "Eighth".to_string(),
                "Ninth".to_string(),
            ],
        };

        data.event_log.push(turn_entry_0.clone());
        data.event_log.push(turn_entry_1.clone());
        data.event_log.push(turn_entry_2.clone());

        assert!(data.event_log.contains(&turn_entry_0));
        assert!(data.event_log.contains(&turn_entry_1));
        assert!(data.event_log.contains(&turn_entry_2));

        assert_eq!(data.event_log.len(), 3);

        assert_eq!(data.event_log[0], turn_entry_0);
        assert_eq!(data.event_log[1], turn_entry_1);
        assert_eq!(data.event_log[2], turn_entry_2);

        // "Third"
        assert_eq!(data.event_log[0].events[2], turn_entry_0.events[2]);
        // "Forth"
        assert_eq!(data.event_log[1].events[0], turn_entry_1.events[0]);
        // "Eighth"
        assert_eq!(data.event_log[2].events[1], turn_entry_2.events[1]);
    }

    #[test]
    fn light_source_adds_correctly() {
        use crate::app::state::{LightSource, LightSourceType};
        let source = LightSource {
            label: "Lantern".into(),
            light_type: LightSourceType::Lantern,
            radius_feet: 30,
            minutes_remaining: 60,
            last_roll: None,
        };
        assert_eq!(source.label, "Lantern");
        assert_eq!(source.radius_feet, 30);
    }

    #[test]
    fn add_new_torch_event_table_entry() {
        let mut data = ShadowtrackData::default();
        let new_entry = "Shadows attack!".to_string();
        data.torch_event_table.push(new_entry.clone());
        assert!(data.torch_event_table.contains(&new_entry));
    }

    #[test]
    fn save_and_load_game() {
        use crate::app::state::{LightSource, LightSourceType, ShadowtrackData};
        use std::{env, fs};

        let mock_light_source = LightSource {
            label: "Mock Light Label".to_string(),
            radius_feet: 10,
            light_type: LightSourceType::Spell("Mock Testing Spell".to_string()),
            minutes_remaining: 420,
            last_roll: Some(69),
        };

        let test_save_file = std::path::PathBuf::from(format!(
            "{}/test_save.json",
            env::temp_dir().to_str().unwrap()
        ));
        let mut data = ShadowtrackData::default();
        data.clock_elapsed = 69420;
        data.light_sources.push(mock_light_source);

        let _test_save = crate::app::save::write_save(&test_save_file, &data);

        assert_eq!(crate::app::save::load_save(&test_save_file).unwrap(), data);

        // clean up.
        fs::remove_file(test_save_file).unwrap();
    }

    #[test]
    fn rng_range_inclusive() {
        let mut rng = DefaultRandomSource;
        let mut rolls: Vec<u32> = Vec::new();
        let min = 0;
        let max = 5;

        // Let's hope this is enough.
        for _ in 1..100 {
            rolls.push(rng.roll_range(min, max));
        }

        assert!(rolls.contains(&min));
        assert!(rolls.contains(&max));
    }

    #[test]
    fn rng_choose_empty() {
        let mut rng = DefaultRandomSource;
        let empty_set: Vec<String> = Vec::new();
        let choice: Option<&String> = rng.choose(&empty_set);

        assert!(choice.is_none());
    }

    #[test]
    fn rng_choose_inclusive() {
        let mut rng = DefaultRandomSource;
        let mut choices: Vec<String> = Vec::new();
        let choice_set: Vec<String> = vec!["first".into(), "second".into(), "third".into()];

        for _ in 0..100 {
            choices.push(
                rng.choose(&choice_set)
                    .expect("Something Broke Here")
                    .to_string(),
            );
        }

        assert!(!choices.is_empty());
        assert!(choices.contains(&choice_set[0]));
        assert!(choices.contains(&choice_set[1]));
        assert!(choices.contains(&choice_set[2]));
        assert!(!choices.contains(&"fifth".to_string()));
    }

    // A mock RNG source to control test results
    struct MockRng {
        roll_values: Vec<u32>,
        choose_indices: Vec<usize>,
        roll_index: usize,
        choose_index: usize,
    }

    impl MockRng {
        fn new(roll_values: Vec<u32>, choose_indices: Vec<usize>) -> Self {
            Self {
                roll_values,
                choose_indices,
                roll_index: 0,
                choose_index: 0,
            }
        }
    }

    impl RandomSource for MockRng {
        fn roll_range(&mut self, _min: u32, _max: u32) -> u32 {
            let val = self.roll_values[self.roll_index];
            self.roll_index = (self.roll_index + 1) % self.roll_values.len();
            val
        }

        fn choose<'a, T>(&mut self, list: &'a [T]) -> Option<&'a T> {
            if list.is_empty() {
                return None;
            }
            let idx = self.choose_indices[self.choose_index] % list.len();
            self.choose_index = (self.choose_index + 1) % self.choose_indices.len();
            list.get(idx)
        }
    }

    #[test]
    fn test_roll_encounter_with_miss() {
        let mut data = ShadowtrackData::default();
        data.encounter_table.push("Goblin Screamer".to_string());

        let mut rng = MockRng::new(vec![4], vec![0]); // force roll != 1
        roll_encounter(&mut data, &mut rng, false);

        assert_eq!(data.event_log.len(), 1);
        assert_eq!(data.event_log[0].events.len(), 1);
        assert!(data.event_log[0]
            .events
            .contains(&String::from("No encounter")));
    }

    #[test]
    fn test_roll_encounter_with_hit() {
        let mut data = ShadowtrackData::default();
        data.encounter_table.clear();
        data.encounter_table.push("Skeleton Ambush".to_string());

        let mut rng = MockRng::new(vec![1], vec![0]); // force 1 on d6, select 0th entry
        roll_encounter(&mut data, &mut rng, false);

        assert_eq!(data.event_log.len(), 1);
        assert_eq!(data.event_log[0].events.len(), 1);
        assert!(data.event_log[0]
            .events
            .contains(&String::from("!ENCOUNTER! - Skeleton Ambush")));
    }

    #[test]
    fn test_process_torch_burn() {
        let mut data = ShadowtrackData::default();
        data.light_sources.push(crate::app::state::LightSource {
            label: "".to_string(),
            light_type: Default::default(),
            radius_feet: 0,
            minutes_remaining: 30,
            last_roll: None,
        });

        let mut rng = MockRng::new(vec![2], vec![]);
        process_light_burn(&mut data, &mut rng);
        let torch = &data.light_sources[0];

        assert_eq!(torch.minutes_remaining, 10);
        assert_eq!(torch.last_roll, Some(2));
    }

    #[test]
    fn test_roll_torch_event() {
        let mut data = ShadowtrackData::default();
        data.torch_event_table.clear();
        data.torch_event_table
            .push("Torch sputters ominously.".to_string());

        let mut rng = MockRng::new(vec![], vec![0]);
        roll_light_event(&mut data, &mut rng);

        assert_eq!(data.event_log.len(), 1);
        assert_eq!(data.event_log[0].events.len(), 1);
        assert!(data.event_log[0]
            .events
            .contains(&String::from("Torch sputters ominously.")));
    }
}
