pub mod eee;

mod entities {

    use super::eee;

    pub struct U8ToStringEntity {
        output_environment : eee::SharedEnvironment<String>
    }

    impl U8ToStringEntity {
        pub fn new(session : &eee::EEESession) -> Self {
            let output_environment = eee::SUPERVISOR_STRING.get_environment(&session, "CONSOLE");
            U8ToStringEntity { output_environment }
        }

        pub fn spin_up(session : &eee::EEESession) {
            let input_environment = eee::SUPERVISOR_U8.get_environment(&session, "STATS");
            let entity = Self::new(session);
            input_environment.add_entity(Box::new(entity));
        }

        fn convert(&self, &effect : &u8) -> String {
            match effect {
                0 => "zero",
                1 => "one",
                2 => "two",
                3 => "three",
                _ => "a lot"
            }.to_string()
        }
    }

    impl eee::Entity<u8> for U8ToStringEntity {
        fn on_effect(&mut self, &effect : &u8) {
            let result = self.convert(&effect);
            self.output_environment.send_effect( result);
        }
    }

    pub struct ConsoleEntity {}

    impl eee::Entity<String> for ConsoleEntity {
        fn on_effect(&mut self, effect : &String) {
            println!("> {}", effect)
        }
    }

    impl ConsoleEntity {
        pub fn spin_up(session : &eee::EEESession) {
            let input_environment = eee::SUPERVISOR_STRING.get_environment(&session, "CONSOLE");
            input_environment.add_entity(Box::new(ConsoleEntity{}));
        }
    }
}

fn main() {

    // create new session
    let session = eee::EEESession::new();

    // spin up entities
    entities::U8ToStringEntity::spin_up(&session);
    entities::ConsoleEntity::spin_up(&session);

    // trigger entities by sending effects
    let first_env = eee::SUPERVISOR_U8.get_environment(&session, "STATS");

    first_env.send_effect(1);
    first_env.send_effect(2);
    first_env.send_effect(3);
}