/* 
--- Day 3: Toboggan Trajectory ---
With the toboggan login problems resolved, you set off toward the airport. 
While travel by toboggan might be easy, it's certainly not safe: there's 
very minimal steering and the area is covered in trees. You'll need to see 
which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer 
coordinates in a grid. You make a map (your puzzle input) of the open 
squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
These aren't the only trees, though; due to something you read about once 
involving arboreal genetics and biome stability, the same pattern repeats 
to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach 
the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper 
model that prefers rational numbers); start by counting all the trees you 
would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is 
right 3 and down 1. Then, check the position that is right 3 and down 1 
from there, and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where 
there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
In this example, traversing the map using this slope would cause you to 
encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 
3 and down 1, how many trees would you encounter?
*/

static empty: char = '.';
static tree: char = '#';

pub struct TobogganMap {
    pub width: usize,
    pub rows: Vec<String>
}

impl TobogganMap {
    pub fn new(lines: Vec<String>) -> TobogganMap {
        let w = lines.first().unwrap().len();
        let mut toboggan_map = TobogganMap { rows: lines, width: w };
        return toboggan_map;
    }
    fn len(self: &TobogganMap) -> usize {
        return self.rows.len();
    }
    pub fn count_trees_while_sliding_down_hill(self: &TobogganMap, slope: (usize, usize)) -> u16 {
        let mut x: usize = 0; // x grows without bound, to the right
        let mut y: usize = 0; // y grows from 0 to rows.len(), in steps of slope.1
        let mut trees = 0;
        while y < self.len() {
            x += slope.0;
            y += slope.1;
            if let Some(line) = self.rows.get(y) {
                if let Some(c) = line.chars().nth(x % self.width) {
                    if c == tree {
                        trees += 1;
                    }
                }
            }
        }
        return trees;
    }
    pub fn print_rows(self: &TobogganMap) {
        for row in self.rows.iter() {
            for col in 0..6 {
                print!("{}", row);
            }
            println!(" -->");
        }
    }
}

#[test]
fn test_parse_lines() {
    let mut lines: Vec<String> = Vec::new();
    let raw_lines: &[&str] = &[
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#"
    ];
    for i in 0..raw_lines.len() {
        lines.push(String::from(raw_lines[i].clone()));
    }

    let toboggan_map = TobogganMap::new(lines);
    assert_eq!(11, toboggan_map.len());
    assert_eq!(7, toboggan_map.count_trees_while_sliding_down_hill((3,1)));
}

/*
--- Part Two ---
Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

Right 1, down 1.
Right 3, down 1. (This is the slope you already checked.)
Right 5, down 1.
Right 7, down 1.
Right 1, down 2.
In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?


*/


//In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
#[test]
fn test_part1() {
    let mut lines: Vec<String> = Vec::new();
    let raw_lines: &[&str] = &[
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#"
    ];
    for i in 0..raw_lines.len() {
        lines.push(String::from(raw_lines[i].clone()));
    }

    let toboggan_map = TobogganMap::new(lines);
    let answer = toboggan_map.count_trees_while_sliding_down_hill((1, 1)) *
        toboggan_map.count_trees_while_sliding_down_hill((3, 1)) *
        toboggan_map.count_trees_while_sliding_down_hill((5, 1)) *
        toboggan_map.count_trees_while_sliding_down_hill((7, 1)) *
        toboggan_map.count_trees_while_sliding_down_hill((1, 2));
    assert_eq!(336, answer);
}
