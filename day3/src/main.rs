#![allow(dead_code)]
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("./input.txt");
    let mut game: Game = Game {symbols : HashMap::new(), numbers: HashMap::new()};
    let mut line_nb: usize = 0;
    for line in lines{
        let res = line_parser(&line.unwrap());
        game.add_numbers(res.1, line_nb);
        game.add_symbols(res.0, line_nb);
        line_nb += 1;
    }
    
    let mut res: i32 = 0;
    for entry in &game.symbols{
        for symbol in entry.1{
            let next_y = entry.0 + 1;
            let tot: i32 = find_neighbours_inline(&symbol, game.numbers.get(&entry.0).unwrap()).into_iter().sum();
            res += tot;
            
            if next_y <= line_nb - 1{
            let tot: i32 = find_neighbours(&symbol, game.numbers.get(&next_y).unwrap()).into_iter().sum();
            res += tot;     
            
            }
            if entry.0 != &0 {
            let tot: i32 = find_neighbours(&symbol, game.numbers.get(&(entry.0 - 1)).unwrap())
                    .into_iter().sum();    
            res += tot;         
            }
        }
    }
    
    let mut gear_res: i32 = 0;
    for entry in &game.symbols{
        for symbol in entry.1.iter().filter(|spec_symbol| spec_symbol.symbol == '*'){
            let next_y = entry.0 + 1;
            let mut neighbours: Vec<i32> = find_neighbours_inline(&symbol, game.numbers.get(&entry.0).unwrap());
            if next_y <= line_nb - 1{
            neighbours.append(&mut find_neighbours(&symbol, game.numbers.get(&next_y).unwrap()));
            }
            if entry.0 != &0 {
                neighbours.append(&mut find_neighbours(&symbol, game.numbers.get(&(entry.0 - 1)).unwrap()));
            }
            gear_res += find_gear(neighbours);
        }
    }

    println!("Part 1: {} \n Part 2: {}", res, gear_res);
}

fn find_gear(neighbours: Vec<i32>) -> i32{
    let mut res: i32 = 0;
    if neighbours.len() == 2{
        res += neighbours[0] * neighbours[1];
    }
    res
}
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn line_parser(line: &String) -> (Vec<Symbol>, Vec<Number>){
    let chars: Vec<char> = line.chars().collect();
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];
    let mut i: usize = 0;
    while i < chars.len(){
        if is_symbol(&chars[i]){
            symbols.push(Symbol {j: i, symbol: chars[i]});
            i += 1;
            continue;
        }
        if chars[i].is_numeric(){
            let res = find_number(&chars, i);
            if res.0.is_none(){
                println!("bob");
            }
            numbers.push(res.0.unwrap());
            i = res.1;
            continue;
        }
        i += 1;
    }
    (symbols, numbers)
}

fn find_number(chars: &Vec<char>, beg_pos: usize) -> (Option<Number>, usize){
    let mut nb: String = chars[beg_pos].to_string();
    for i in beg_pos + 1..chars.len(){
        if !chars[i].is_numeric(){
            let number = Some(Number {j: beg_pos, len: i - beg_pos - 1, nb: nb.parse::<i32>().unwrap()});
            let exit_index = i;
            return (number, exit_index);
        }
        nb.push(chars[i]);
    }
    let exit_index = chars.len();
    let number = Some(Number {j: beg_pos, len: exit_index - beg_pos - 1, nb: nb.parse::<i32>().unwrap()});
    (number, exit_index)
}

fn is_symbol(char: &char) -> bool{
    !char.is_alphanumeric() && !char.is_whitespace() && char != &'.'
}

struct Symbol{
    j: usize,
    symbol: char
}

fn find_neighbours<'a>(symbol: &'a Symbol, numbers: &'a Vec<Number>) -> Vec<i32>{
    let mut neighbours: Vec<i32> = vec![];
    for number in numbers{
        let left_index = if number.j == 0  {0} else {number.j -1};
        if !(symbol.j < left_index || symbol.j > number.j + number.len + 1){
            neighbours.push(number.nb);
            continue;
        }
        if !(symbol.j < number.j || symbol.j > number.j + number.len){
            neighbours.push(number.nb);
        }
    }
    
    neighbours
}

fn find_neighbours_inline<'a>(symbol: &'a Symbol, numbers: &'a Vec<Number>) -> Vec<i32>{
    let mut neighbours: Vec<i32> = vec![];
    for number in numbers{
        let left_index = if number.j == 0  {0} else {number.j -1};
        if !(symbol.j < left_index || symbol.j > number.j + number.len + 1){
            neighbours.push(number.nb);
        }
    }

    neighbours
}

#[derive(PartialEq, Eq, Hash)]
struct Number{
    j: usize,
    len: usize,
    nb: i32
}

struct Game{
    symbols: HashMap<usize, Vec<Symbol>>,
    numbers: HashMap<usize, Vec<Number>>
}

impl Game{
    fn add_symbols(&mut self, symbols: Vec<Symbol>, index: usize){
        self.symbols.insert(index, symbols);
    }
    fn add_numbers(& mut self, numbers: Vec<Number>, index: usize){
        self.numbers.insert(index, numbers);
    }
}