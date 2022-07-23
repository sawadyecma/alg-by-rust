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

    let maze = input.replace(" ", "");
    let maze = maze.split("\n").collect::<Vec<&str>>();

    println!("{:?}", maze);
}
