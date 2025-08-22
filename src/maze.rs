use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(path: &str) -> Maze {
    let file = File::open(path).expect("maze.txt no encontrado");
    let reader = BufReader::new(file);

    let mut raw: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().trim_end_matches('\r').to_string())
        .collect();

    while raw.first().is_some_and(|s| s.trim().is_empty()) { raw.remove(0); }
    while raw.last().is_some_and(|s| s.trim().is_empty()) { raw.pop(); }

    if raw.is_empty() {
        return vec![
            vec!['#','#','#'],
            vec!['#','.','#'],
            vec!['#','#','#'],
        ];
    }

    let mut rows: Vec<Vec<char>> = Vec::with_capacity(raw.len());
    let mut max_w = 0usize;

    for line in raw {
        let mut row: Vec<char> = Vec::with_capacity(line.len());
        for ch in line.chars() {
            let mapped = match ch {
                ' ' | '.' => '.',           
                'p' | 'g' => ch,            
                '#' | '1' | '2' | '3' | '4' => ch, 
                '+' | '-' | '|' | '=' => '#',      
                _ => '#',                   
            };
            row.push(mapped);
        }
        max_w = max_w.max(row.len());
        rows.push(row);
    }

    for row in rows.iter_mut() {
        if row.len() < max_w {
            row.resize(max_w, '#');
        }
    }

    if !rows.is_empty() {
        let h = rows.len();
        let w = rows[0].len();
        rows[0].fill('#');
        rows[h - 1].fill('#');
        for y in 0..h {
            rows[y][0] = '#';
            rows[y][w - 1] = '#';
        }
    }

    rows
}

pub fn find_player(maze: &Maze) -> (f32, f32) {
    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'p' {
                return (x as f32 + 0.5, y as f32 + 0.5);
            }
        }
    }
    (1.5, 1.5)
}

pub fn is_wall(c: char) -> bool {
    matches!(c, '#' | '1' | '2' | '3' | '4' | '+' | '-' | '|' | '=')
}

pub fn dims(maze: &Maze) -> (usize, usize) {
    let h = maze.len();
    let w = if h > 0 { maze[0].len() } else { 0 };
    (w, h)
}