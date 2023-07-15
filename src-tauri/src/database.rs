    use rusqlite::{Connection, Error, Result};
    use serde::{Deserialize, Serialize};

    trait Model {
        fn create_table(&self, connection: &Connection) -> Result<(), Error>;
        fn drop_table(&self, connection: &Connection) -> Result<(), Error>;
        fn save(&self, connection: &Connection) -> Result<(), Error>;
    }

    #[derive(Serialize, Deserialize)]
    pub struct Day {
        pub(crate) id: i32,
        pub(crate) date: String,
        pub(crate) activities: Vec<Activity>,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct Activity {
        pub(crate) id: i32,
        pub(crate) day_id: i32,
        pub(crate) description: String,
        pub(crate) project: Project,
        pub(crate) task: Task,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct Project {
        pub(crate) id: i32,
        pub(crate) name: String,
        pub(crate) tasks: Vec<Task>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Task {
        pub id: i32,
        pub name: String,
        pub project_id: i32,
    }

    impl Model for Task {
        fn create_table(&self, connection: &Connection) -> Result<(), Error> {
            connection.execute(
                "CREATE TABLE task IF NOT EXISTS(
                id INTEGER PRIMARY KEY,
                name TEXT,
                project_id INTEGER
                FOREIGN KEY(project_id) REFERENCES project(id))",
                (),
            )?;
            Ok(())
        }

        fn drop_table(&self, connection: &Connection) -> Result<(), Error> {
            // TODO: will break activity table
            connection.execute("DROP TABLE task", ())?;
            Ok(())
        }

        fn save(&self, connection: &Connection) -> Result<(), Error> {
            connection.execute(
                "INSERT INTO task(name, project_id) VALUES(1?, 2?)",
                [&self.name, &self.project_id.to_string()],
            )?;
            Ok(())
        }
    }

    impl Model for Project {
        fn create_table(&self, connection: &Connection) -> Result<(), Error> {
            connection.execute(
                "CREATE TABLE project IF NOT EXISTS(
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE)",
                (),
            )?;

            Ok(())
        }

        fn drop_table(&self, connection: &Connection) -> Result<(), Error> {
            // TODO: will break activity table
            connection.execute("DROP TABLE project", ())?;
            Ok(())
        }

        fn save(&self, connection: &Connection) -> Result<(), Error> {
            connection.execute("INSERT INTO project(name) VALUES(?1)", [&self.name])?;
            Ok(())
        }
    }
