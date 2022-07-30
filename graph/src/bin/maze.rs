use std::collections::VecDeque;

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];
const INF: i32 = 1000000;

struct Pointer {
    x: usize,
    y: usize,
}

fn main() {
    let input = "#S######.#
    ......#..#
    .#.##.##.#
    .#........
    ##.##.####
    ....#....#
    .#######.#
    ....#.....
    .####.###.
    ....#...G#";

    let n: usize = 10;
    let m: usize = 10;
    let sx: usize = 1;
    let sy: usize = 0;
    let gx: usize = 8;
    let gy: usize = 9;

    let maze = compose_maze(input, n, m);
    println!("{:?}", maze);

    let mut res_vec = init_vec(n, m);

    // スタート地点は0
    res_vec[sy][sx] = 0;

    println!("{:?}", res_vec);
    let mut que: VecDeque<Pointer> = VecDeque::new();
    que.push_back(Pointer { x: sx, y: sy });

    while que.len() > 0 {
        let p = que.pop_front().unwrap();
        if p.x == gx && p.y == gy {
            break;
        }

        for i in 0..DX.len() {
            let nx = (p.x as isize) + DX[i];
            let ny = (p.y as isize) + DY[i];

            println!("nx: {}, ny:{}", nx, ny);

            // mazeの外側であればcontinue
            if !(0 <= nx && nx < (n as isize) && 0 <= ny && ny < (m as isize)) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            // 通れる点で、最短距離が未確定な場合
            if maze[ny][nx] != '#' && res_vec[ny][nx] == INF {
                que.push_front(Pointer { x: nx, y: ny });
                res_vec[ny][nx] = res_vec[p.y][p.x] + 1;
            }
        }
    }

    println!("{:?}", res_vec);

    println!("result: {}", res_vec[gy][gx]);
}

fn init_vec(n: usize, m: usize) -> Vec<Vec<i32>> {
    return vec![vec![INF; m]; n];
}

fn compose_maze(s: &str, n: usize, m: usize) -> Vec<Vec<char>> {
    let replaced = s.replace(" ", "");
    let split = replaced.split("\n").collect::<Vec<&str>>();

    let mut maze: Vec<Vec<char>> = vec![vec!['.'; m]; n];

    for y in 0..maze.len() {
        let row = &maze[y];
        let chars = split[y].as_bytes();
        for x in 0..row.len() {
            maze[y][x] = chars[x] as char;
        }
    }
    return maze;
}
