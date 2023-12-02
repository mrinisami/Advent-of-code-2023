#![allow(dead_code)]
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let lines = read_lines("./input.txt");
    let validator: HashMap<String, u32> = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14)
    ]);
    let mut res = 0;
    let mut res_mult = 0;
    for line in lines{
        let game_data = game_parser(&line.unwrap());
        let game_id = game_data.add_game_id(&validator);
        res += game_id;
        res_mult += game_data.draw_product();
    }

    print!("Part 1: {res}, Part 2: {res_mult}");
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

struct Game{
    game_id: u32,
    data: Vec<Draw>
}

impl Game {
    fn add_game_id(&self, validator: &HashMap<String, u32> ) -> u32{
        for draw in &self.data{
            if !validate(draw, &validator){
                return 0;
            }
        }
        return self.game_id;
    }
}

impl Game {
    fn draw_product(&self) -> u32{
        let max_draw = find_max_draw(&self);

        max_draw.values().product()
    }
}

fn find_max_draw(game: &Game) -> HashMap<String, u32>{
    let mut max_draw: HashMap<String, u32> = HashMap::from([
        (String::from("red"), 1),
        (String::from("green"), 1),
        (String::from("blue"), 1)
    ]);
    for draw in &game.data{
        let is_above = max_draw.get(&draw.colour).is_some_and(|count| count < &draw.count);
        if is_above {
            max_draw.insert(String::from(&draw.colour), draw.count);
        }
    }

    max_draw
}

fn validate(draw: &Draw, validator: &HashMap<String, u32>) -> bool {
    let res = validator.get(&draw.colour);

    return res.is_some_and(|res| *res >= draw.count);
}

fn game_parser(data: &String) -> Game{
    let id_text: Vec<&str> = data.split(":").collect();
    let id = id_parser(id_text[0]);
    
    let draws : Vec<&str> = id_text[1].split(";").collect();
    Game {game_id: id, data:draw_parser(draws)}
}

fn draw_parser(data: Vec<&str>) -> Vec<Draw>{
    let mut game_draws: Vec<Draw> = vec![];
    data.into_iter().for_each(|line_data| {
        let mut draw = draw_line_parser(line_data);
        game_draws.append(&mut draw);
    });
    
    return game_draws;
}

fn draw_line_parser(data: &str) -> Vec<Draw>{
    let balls: Vec<&str> = data.split(",").collect();
    let draws: Vec<Draw> = balls.into_iter().map(|ball| {
        let ball_info: Vec<&str> = ball.split(' ').collect();

        return Draw {colour: ball_info[2].to_string(), count: ball_info[1].parse::<u32>().unwrap()}
    }).collect();

    return draws;
}

struct Draw{
    colour: String,
    count: u32
}

fn id_parser(data: &str) -> u32{
    let id: Vec<&str> = data.split(" ").collect();

    id[1].parse::<u32>().unwrap()
}
