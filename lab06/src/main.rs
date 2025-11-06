trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong");
    }
}

struct CountCommand;
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;

        println!("'times' command has been called {} time(s)", self.count);
    }
}

struct SumCommand;
impl Command for SumCommand {
    fn get_name(&self) -> &str {
        "sum"
    }

    fn exec(&mut self, args: &[&str]) {
        let mut sum: i32 = 0;

        for arg in args {
            let mut num: i32 = 0;
            for c in arg.chars() {
                num = num * 10 + c as i32 - '0' as i32
            }
            sum += num;
        }

        if !args.is_empty() {
            print!("{}", args[0]);
            for arg in &args[1..] {
                print!(" + {}", arg);
            }
        }

        println!(" = {}", sum);
    }
}

struct Bookmark {
    name: String,
    url: String,
}

struct BookmarkCommand {
    database: String,
}
impl BookmarkCommand {
    fn add(&self, name: &str, url: &str) -> Result<(), rusqlite::Error> {
        let conn = rusqlite::Connection::open(self.database.as_str())?;

        conn.execute(
            r"
        create table if not exists bookmarks (
            name text primary key,
            url  test not null
        );
        ",
            (),
        )?;

        conn.execute(
            "insert into bookmarks (name, url) values (?1, ?2);",
            (name, url),
        )?;

        Ok(())
    }

    fn search(&self, name: &str) -> Result<(), rusqlite::Error> {
        let conn = rusqlite::Connection::open(self.database.as_str())?;

        let mut stmt = conn.prepare("select name, url from bookmarks where name = ?1")?;
        let bookmarks_iter = stmt.query_map(
            // rusqlite::params![name],
            [name],
            |row| {
                Ok(Bookmark {
                    name: row.get("name")?,
                    url: row.get("url")?,
                })
            },
        )?;

        let bookmarks: Vec<Bookmark> =
            bookmarks_iter.collect::<Result<Vec<Bookmark>, rusqlite::Error>>()?;

        if bookmarks.is_empty() {
            println!("Bookmark [{}] not found", name);
        }
        for i in bookmarks {
            println!("name = {}, url = {}", i.name, i.url);
        }

        Ok(())
    }
}
impl Command for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }

    fn exec(&mut self, args: &[&str]) {
        if args.len() == 3
            && args[0] == "add"
            && let Err(e) = self.add(args[1], args[2])
        {
            eprintln!("ERROR: {}", e);
        } else if args.len() == 2
            && args[0] == "search"
            && let Err(e) = self.search(args[1])
        {
            eprintln!("ERROR: {}", e);
        }
    }
}

struct StopCommand;
impl Command for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("Stop command reached");
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn get_command(&mut self, command: &str) -> Option<&mut Box<dyn Command>> {
        for cmd in &mut self.commands {
            if command == cmd.get_name() {
                return Some(cmd);
            } else if command.to_lowercase() == cmd.get_name() {
                println!(
                    "Misstype command, you entered [{}] instead of [{}]",
                    command,
                    cmd.get_name()
                );
            }
        }

        None
    }

    fn run(&mut self, filepath: &str) -> Result<(), std::io::Error> {
        let file = std::fs::read_to_string(filepath)?;

        for line in file.lines() {
            let mut tokens = line.split_whitespace();

            if let Some(command) = tokens.next() {
                let mut args = Vec::new();

                for token in tokens {
                    args.push(token);
                }

                if let Some(cmd) = self.get_command(command) {
                    cmd.exec(&args);

                    if cmd.get_name() == "stop" {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(SumCommand {}));
    terminal.register(Box::new(BookmarkCommand {
        database: "bookmarks.db".to_string(),
    }));
    terminal.register(Box::new(StopCommand {}));

    if let Err(e) = terminal.run("commands.txt") {
        eprintln!("ERROR: {}", e);
    }
}
