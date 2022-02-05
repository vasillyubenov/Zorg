use std::collections::HashMap;
 
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}
 
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
 
#[derive(Debug, Clone)]
pub struct Neighbours{
    pub west: String,
    pub east: String,
    pub south: String,
    pub north: String,
}
 
impl Neighbours {
    pub fn clone(&self) -> Neighbours{
        Neighbours{
            west: self.west.clone(),
            east: self.east.clone(),
            north: self.north.clone(),
            south: self.south.clone(),
        }
    }
 
    pub fn new() -> Self {
        Neighbours{
            north: String::from(""),
            west: String::from(""),
            east: String::from(""),
            south: String::from("")
        }
    }
    //setters
    pub fn set_west(&mut self, room_name: String) {
        self.west = room_name;
    }
 
    pub fn set_east(&mut self, room_name: String) {
        self.east = room_name;
    }
 
    pub fn set_north(&mut self, room_name: String) {
        self.north = room_name;
    }
 
    pub fn set_south(&mut self, room_name: String) {
        self.south = room_name;
    }
 
    //getters
    pub fn get_west(self) -> Option<String> {
        if self.west == ""{
            return None;
        }
        Some(self.west.clone())
    }
 
    pub fn get_east(self) -> Option<String>{
        if self.east == ""{
            return None;
        }
        Some(self.east.clone())
    }
 
    pub fn get_north(self) -> Option<String>{
        if self.north == ""{
            return None;
        }
        Some(self.north.clone())
    }
 
    pub fn get_south(self) -> Option<String>{
        if self.south == ""{
            return None;
        }
        Some(self.south.clone())
    }
}
 
#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub next_to: Neighbours
}
 
impl Room {
    pub fn new(name: String) -> Self {
        Room{
            name,
            next_to: Neighbours::new()
        }
    }
}
 
pub struct Dungeon {
    rooms: HashMap<String, Room>
}
 
impl Dungeon {
    pub fn new() -> Self {
        Dungeon{
            rooms: HashMap::<String, Room>::new()
        }    
    }
 
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        if self.rooms.contains_key(&name.to_string()) {
            Err(Errors::DuplicateRoom(name.to_string()))
        }
        else{
            self.rooms.insert(name.to_string(), Room::new(name.to_string()));
            Ok(())
        }
    }
 
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        if self.rooms.contains_key(&room_name.to_string()) {
            Ok(self.rooms.get(&room_name.to_string()).unwrap())
        }
        else{
            Err(Errors::UnknownRoom(room_name.to_string()))
        }
    }
 
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        if self.rooms.contains_key(&room_name.to_string()){
            if self.rooms.contains_key(&other_room_name.to_string()){
                let mut current = self.rooms.get_mut(&room_name.to_string()).unwrap();
                //let mut second = self.rooms.get_mut(&other_room_name.to_string()).unwrap();;
                match direction{
                    Direction::East => {
                        current.next_to.set_east(other_room_name.to_string());
                        //change the current room to the second one
                        current = self.rooms.get_mut(&other_room_name.to_string()).unwrap();
                        current.next_to.set_west(room_name.to_string());
                    }
                    Direction::West => {
                        current.next_to.set_west(other_room_name.to_string());
                        //change the current room to the second one 
                        current = self.rooms.get_mut(&other_room_name.to_string()).unwrap();
                        current.next_to.set_east(room_name.to_string());
                    }
                    Direction::South => {
                        current.next_to.set_south(other_room_name.to_string());
                        //change the current room to the second one 
                        current = self.rooms.get_mut(&other_room_name.to_string()).unwrap();
                        current.next_to.set_north(room_name.to_string());
                    }
                    Direction::North => {
                        current.next_to.set_north(other_room_name.to_string());
                        //change the current room to the second one
                        current = self.rooms.get_mut(&other_room_name.to_string()).unwrap();
                        current.next_to.set_south(other_room_name.to_string());
                    }
                }
            }
            else{
                return Err(Errors::UnknownRoom(other_room_name.to_string()));
            }
        }
        else{
            return Err(Errors::UnknownRoom(room_name.to_string()));
        }
        Ok(())
    }
 
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        if self.rooms.contains_key(&room_name.to_string()) {
            let get_room = self.rooms.get(&room_name.to_string());
            match direction{
                Direction::North => {
                    if get_room.unwrap().next_to.clone().get_north() == None {
                        Ok(None)
                    }
                    else{
                        let neighbour_room_name = get_room.unwrap().next_to.clone().get_north().unwrap();
                        Ok(Some(self.rooms.get(&neighbour_room_name.to_string())).unwrap())
                    }
                }
                Direction::East => {
                    if get_room.unwrap().next_to.clone().get_east() == None {
                        Ok(None)
                    }
                    else{
                        let neighbour_room_name = get_room.unwrap().next_to.clone().get_east().unwrap();
                        Ok(Some(self.rooms.get(&neighbour_room_name.to_string())).unwrap())
                    }
                }
                Direction::West => {
                    if get_room.unwrap().next_to.clone().get_west() == None {
                        Ok(None)
                    }
                    else{
                        let neighbour_room_name = get_room.unwrap().next_to.clone().get_west().unwrap();
                        Ok(Some(self.rooms.get(&neighbour_room_name.to_string())).unwrap())
                    }
 
                }
                Direction::South => {
                    if get_room.unwrap().next_to.clone().get_south() == None {
                        Ok(None)
                    }
                    else{
                        let neighbour_room_name = get_room.unwrap().next_to.clone().get_south().unwrap();
                        Ok(Some(self.rooms.get(&neighbour_room_name.to_string())).unwrap())
                    }
                }
            }
        }
        else{
            Err(Errors::UnknownRoom(room_name.to_string()))
        } 
    }
}
 
use std::io::BufRead;
//helper functions
fn str_to_direction(dir: &str) -> Direction{
    match dir {
        "North" => Direction::North,
        "East" => Direction::East,
        "South" => Direction::South,
        "West" => Direction::West,
        _ => unreachable!()
    }
}
 
impl Dungeon {
 
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        //if we have an empty reader
        let mut new_dungeon = Dungeon::new();
 
        let mut row_counter = 0;
        //we have a vector of all the lines and we have checked it for any io::errors
        let lines_vec = reader.lines()
                                  .map(|l| {
                                      if let Err(val) = l{
                                          return Err(Errors::IoError(val));
                                      }
                                      return Ok(l.unwrap());
                                    })
                                   .map(|l| l.unwrap()).collect::<Vec<String>>();
        //cheking for empty buffer
        if lines_vec.len() == 0 {
            return Err(Errors::LineParseError{ line_number: row_counter });
        }
        row_counter += 1;

        let mut reached_nl = false;
        let mut row_reached = 0;
        for mut line in lines_vec { 
            line = line.trim().to_string();
            if !reached_nl{
                if row_counter == 1{
                    if line != "## Rooms"{
                        return Err(Errors::LineParseError{ line_number: row_counter });
                    }
                    else {
                        row_counter += 1;
                        continue;
                    }
                }
                else {
                    if line.as_bytes().len() == 0 {
                        //we have reached a newline
                        reached_nl = true;
                        row_reached = row_counter;
                    }
                    else{
                        //geting and the first letter and checking for the correct format wihich is /- <name>/
                        if line.as_bytes()[0] as char != '-'{
                            return Err(Errors::LineParseError{ line_number: row_counter });
                        }
                        else {
                            new_dungeon.add_room(&line[2..])?;
                        }
                        row_counter += 1;
                    }
                }
            }
            else{
                if row_counter == row_reached {
                    if line != "## Links"{
                        return Err(Errors::LineParseError{ line_number: row_counter });
                    }
                    else{
                        row_counter += 1;
                        continue;
                    }
                }
                //geting and the first letter and checking for the correct format wihich is /- <name>/
                else if line.as_bytes()[0] as char != '-' {
                    return Err(Errors::LineParseError{ line_number: row_counter });
                }
                else {
                    row_counter += 1;
                    let words: Vec<&str> = line[2..].split(" -> ").collect();
                    if words.len() != 3 {
                        
                        return Err(Errors::LineParseError{ line_number: row_counter });
                    }
 
                    if words[1].trim() != "West" && words[1].trim() != "East" && words[1].trim() != "North" && words[1].trim() != "South" {
                        return Err(Errors::DirectionParseError(words[1].to_string()));
                    }   
 
                    let dir = str_to_direction(words[1]);
 
                    new_dungeon.set_link(words[0], dir, words[2])?;
                }
            }
        }
        return Ok(new_dungeon);
    }
}

fn is_not_visited(target: String, path: Vec<String>) -> bool{
    for x in path{
        if x == target{
            return false;
        }
    }   
    return true;
}
 
fn findpaths<'a>(g: &'a HashMap<String, Vec<String>>, src: &'a str, dst: &'a str) -> Vec<String>{
    let mut q = Vec::<Vec::<String>>::new();
 
    // path vector to store the current path
    let mut path = Vec::<String>::new();
    path.push(src.to_string());
    q.insert(0, path);
    while !q.is_empty() {
        path = q[q.len() - 1].clone();
        q.pop();
        let last: String = path[path.len() - 1].clone();
 
        //we have reached the end
        if last == dst{
            return path;
        }
                
        for name in g[&last].clone() {
            if is_not_visited(name.to_string(), path.clone()) {
                let mut newpath = Vec::<String>::new();
                for road in path.clone(){
                    newpath.push(road);
                }
                newpath.push(name.to_string());
                q.push(newpath);
            }
        }
    }
    return vec![];
}

//finding path algorythm part
impl Dungeon {

    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {

        let mut graph = HashMap::<String, Vec<String>>::new();
        for n in &self.rooms.clone(){
            let mut current_neigbours = Vec::<String>::new();
            //n.1 - is a Room
            if n.1.next_to.clone().get_west() != None{
                current_neigbours.push(n.1.next_to.clone().get_west().unwrap());
            }
            if n.1.next_to.clone().get_east() != None{
                current_neigbours.push(n.1.next_to.clone().get_east().unwrap());
            }
            if n.1.next_to.clone().get_north() != None{
                current_neigbours.push(n.1.next_to.clone().get_north().unwrap());
            }
            if n.1.next_to.clone().get_south() != None{
                current_neigbours.push(n.1.next_to.clone().get_south().unwrap());
            }
            graph.insert(n.0.to_string(), current_neigbours);
        }
        let path_in_str_vec = findpaths(&graph, start_room_name, end_room_name);

        let mut result = Vec::<&Room>::new();
        for room in path_in_str_vec{
            result.push(self.rooms.get(&room).unwrap());
        }

        //in case we have nothing in the path
        if result.len() == 0 {
            return Ok(None);
        }

        Ok(Some(result))
    }
}

// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го
// на този ред:

use std::io::{self, Read};

macro_rules! timeout {
    ($time:expr, $body:block) => {
        use std::panic::catch_unwind;

        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            if let Err(e) = catch_unwind(|| $body) {
                sender.send(Err(e)).unwrap();
                return;
            }

            match sender.send(Ok(())) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });

        if let Err(any) = receiver.recv_timeout(std::time::Duration::from_millis($time)).unwrap() {
            panic!("{}", any.downcast_ref::<String>().unwrap());
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West
];

fn all_links<'a>(dungeon: &'a Dungeon, room: &'a Room) -> Vec<&'a str> {
    ALL_DIRECTIONS.iter().
        flat_map(|dir| dungeon.get_next_room(&room.name, *dir).unwrap()).
        map(|r| r.name.as_str()).
        collect::<Vec<&'a str>>()
}

struct ErroringReader {}

impl Read for ErroringReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "read error!"))
    }
}

impl BufRead for ErroringReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
    }

    fn consume(&mut self, _amt: usize) { }
}

#[test]
fn test_adding_rooms_1() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Laboratory").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
    assert_eq!(dungeon.get_room("Laboratory").unwrap().name, "Laboratory");
}

#[test]
fn test_adding_rooms_2() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway 1").unwrap();
    dungeon.add_room("Side Closet").unwrap();
    dungeon.add_room("Hallway 2").unwrap();
    dungeon.add_room("Treasure Room").unwrap();

    dungeon.set_link("Entrance", Direction::East, "Hallway 1").unwrap();
    dungeon.set_link("Hallway 1", Direction::East, "Side Closet").unwrap();
    dungeon.set_link("Hallway 1", Direction::North, "Hallway 2").unwrap();
    dungeon.set_link("Hallway 2", Direction::South, "Side Closet").unwrap();
    dungeon.set_link("Hallway 2", Direction::West, "Treasure Room").unwrap();
    dungeon.set_link("Side Closet", Direction::South, "Entrance").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway 1").unwrap().name, "Hallway 1");
    assert_eq!(dungeon.get_room("Side Closet").unwrap().name, "Side Closet");
    assert_eq!(dungeon.get_room("Hallway 2").unwrap().name, "Hallway 2");
    assert_eq!(dungeon.get_room("Treasure Room").unwrap().name, "Treasure Room");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway 1");
    assert_eq!(dungeon.get_next_room("Hallway 1", Direction::West).unwrap().unwrap().name, "Entrance");

    assert_eq!(dungeon.get_next_room("Hallway 1", Direction::North).unwrap().unwrap().name, "Hallway 2");
    // Overwrite "South" link:
    assert_eq!(dungeon.get_next_room("Hallway 2", Direction::South).unwrap().unwrap().name, "Side Closet");

    assert_eq!(dungeon.get_next_room("Side Closet", Direction::South).unwrap().unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::North).unwrap().unwrap().name, "Side Closet");
}

#[test]
fn test_room_links() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();

    assert_eq!(all_links(&dungeon, dungeon.get_room("Entrance").unwrap()), Vec::<&str>::new());

    dungeon.add_room("Hallway 1").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 1").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1"]
    );

    dungeon.add_room("Hallway 2").unwrap();
    dungeon.set_link("Entrance", Direction::South, "Hallway 2").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2"]
    );

    dungeon.add_room("Hallway 3").unwrap();
    dungeon.set_link("Entrance", Direction::East, "Hallway 3").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2", "Hallway 3"]
    );

    dungeon.add_room("Hallway 4").unwrap();
    dungeon.set_link("Entrance", Direction::West, "Hallway 4").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1", "Hallway 2", "Hallway 3", "Hallway 4"]
    );
}

#[test]
fn test_overwriting_a_room_link() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();

    assert_eq!(all_links(&dungeon, dungeon.get_room("Entrance").unwrap()), Vec::<&str>::new());

    dungeon.add_room("Hallway 1").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 1").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 1"]
    );

    dungeon.add_room("Hallway 2").unwrap();
    dungeon.set_link("Entrance", Direction::North, "Hallway 2").unwrap();
    assert_eq!(
        all_links(&dungeon, dungeon.get_room("Entrance").unwrap()),
        vec!["Hallway 2"]
    );
}

#[test]
fn test_cyrillic_room_names() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Антре").unwrap();
    dungeon.add_room("Хол").unwrap();
    dungeon.set_link("Антре", Direction::North, "Хол").unwrap();

    assert_eq!(dungeon.get_room("Антре").unwrap().name, "Антре");
    assert_eq!(dungeon.get_room("Хол").unwrap().name, "Хол");
    assert!(matches!(dungeon.get_room("Кухня"), Err(Errors::UnknownRoom(_))));

    assert_eq!(dungeon.get_next_room("Антре", Direction::North).unwrap().unwrap().name, "Хол");
    assert_eq!(dungeon.get_next_room("Хол", Direction::South).unwrap().unwrap().name, "Антре");
}

#[test]
fn test_room_errors() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    assert!(matches!(dungeon.add_room("Entrance"), Err(Errors::DuplicateRoom(_))));
    assert!(matches!(dungeon.get_room("Exit"), Err(Errors::UnknownRoom(_))));
}

#[test]
fn test_io_error() {
    timeout!(2000, {
        let dungeon = Dungeon::from_reader(ErroringReader {});
        assert!(matches!(dungeon, Err(Errors::IoError(_))));
    });
}

const TEST_INPUT_1: &str = "
## Rooms
- Entrance
- Hallway
## Links
- Entrance -> East -> Hallway
- Hallway -> West -> Entrance
";

#[test]
fn test_parsing_rooms() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Entrance");
}

const TEST_INPUT_2: &str = "
## Rooms
## Links
";

const TEST_INPUT_3: &str = "
## Rooms
- Pantry
- Kitchen
## Links
";

#[test]
fn test_parsing_no_rooms_or_links() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_2.trim().as_bytes()).unwrap();
    assert!(matches!(dungeon.get_room("Entrance"), Err(Errors::UnknownRoom(_))));

    let dungeon = Dungeon::from_reader(TEST_INPUT_3.trim().as_bytes()).unwrap();
    assert_eq!(all_links(&dungeon, dungeon.get_room("Pantry").unwrap()), Vec::<&str>::new());
}

const TEST_INPUT_4: &str = "
## Chambers
## Links
";

const TEST_INPUT_5: &str = "
## Rooms
## Neighbours
";

const TEST_INPUT_6: &str = "
## Rooms
- Entrance
- Treasure Room
## Links
- Closet -> North -> Bathroom
";

const TEST_INPUT_7: &str = "
## Rooms
- Entrance
- Treasure Room
## Links
- Entrance -> North-west -> Treasure Room
";

#[test]
fn test_invalid_parsing() {
    assert!(matches!(Dungeon::from_reader("".as_bytes()), Err(Errors::LineParseError { line_number: 0 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_4.trim().as_bytes()), Err(Errors::LineParseError { line_number: 1 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_5.trim().as_bytes()), Err(Errors::LineParseError { line_number: 3 })));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_6.trim().as_bytes()), Err(Errors::UnknownRoom(_))));
    assert!(matches!(Dungeon::from_reader(TEST_INPUT_7.trim().as_bytes()), Err(Errors::DirectionParseError(_))));
}

const TEST_INPUT_8: &str = "
## Rooms
- Вход
- Хол
## Links
- Вход -> West -> Хол
";

#[test]
fn test_parsing_cyrillic_rooms() {
    let dungeon = Dungeon::from_reader(TEST_INPUT_8.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Вход").unwrap().name, "Вход");
    assert_eq!(dungeon.get_room("Хол").unwrap().name, "Хол");
    assert!(matches!(dungeon.get_room("Кухня"), Err(Errors::UnknownRoom(_))));

    assert_eq!(dungeon.get_next_room("Вход", Direction::West).unwrap().unwrap().name, "Хол");
    assert_eq!(dungeon.get_next_room("Хол", Direction::East).unwrap().unwrap().name, "Вход");
}

#[test]
fn test_finding_a_direct_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();
        dungeon.set_link("Entrance", Direction::West, "Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();
        assert_eq!(path.into_iter().map(|p| &p.name).collect::<Vec<_>>(), ["Entrance", "Treasure Room"]);
    });
}

#[test]
fn test_finding_an_indirect_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Hallway 1").unwrap();
        dungeon.add_room("Side Closet").unwrap();
        dungeon.add_room("Hallway 2").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        dungeon.set_link("Entrance", Direction::East, "Hallway 1").unwrap();
        dungeon.set_link("Hallway 1", Direction::East, "Side Closet").unwrap();
        dungeon.set_link("Hallway 1", Direction::North, "Hallway 2").unwrap();
        dungeon.set_link("Hallway 2", Direction::South, "Side Closet").unwrap();
        dungeon.set_link("Hallway 2", Direction::West, "Treasure Room").unwrap();
        dungeon.set_link("Side Closet", Direction::South, "Entrance").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();

        let mut path_iter = path.iter();
        let mut first = path_iter.next().unwrap();
        while let Some(second) = path_iter.next() {
            let first_neighbours = all_links(&dungeon, &first);
            let second_neighbours = all_links(&dungeon, &second);

            assert!(first_neighbours.contains(&second.name.as_str()));
            assert!(second_neighbours.contains(&first.name.as_str()));

            first = second;
        }

        assert_eq!(path[0].name, "Entrance");
        assert_eq!(path[path.len() - 1].name, "Treasure Room");
    });
}

#[test]
fn test_finding_a_reflexive_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Entrance").unwrap().unwrap();
        assert_eq!(path[0].name, "Entrance");
        assert_eq!(path.len(), 1);

        let path = dungeon.find_path("Treasure Room", "Treasure Room").unwrap().unwrap();
        assert_eq!(path[0].name, "Treasure Room");
        assert_eq!(path.len(), 1);
    });
}

#[test]
fn test_finding_no_path() {
    timeout!(2000, {
        let mut dungeon = Dungeon::new();

        dungeon.add_room("Entrance").unwrap();
        dungeon.add_room("Treasure Room").unwrap();

        let path = dungeon.find_path("Entrance", "Treasure Room");
        assert!(path.unwrap().is_none());

        let path = dungeon.find_path("Entrance", "Mystery Room");
        assert!(path.is_err());
        let path = dungeon.find_path("Mystery Room", "Treasure Room");
        assert!(path.is_err());
    });
}