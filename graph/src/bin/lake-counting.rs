fn main() {
    let fields: Vec<Vec<&str>> = vec![
        vec!["W", ".", ".", ".", ".", ".", ".", ".", ".", "W", "W", "."],
        vec![".", "W", "W", "W", ".", ".", ".", ".", ".", "W", "W", "W"],
        vec![".", ".", ".", ".", "W", "W", ".", ".", ".", "W", "W", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "W", "W", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "W", ".", "."],
        vec![".", ".", "W", ".", ".", ".", ".", ".", ".", "W", ".", "."],
        vec![".", "W", ".", "W", ".", ".", ".", ".", ".", "W", "W", "."],
        vec!["W", ".", "W", ".", "W", ".", ".", ".", ".", ".", "W", "."],
        vec![".", "W", ".", "W", ".", ".", ".", ".", ".", ".", "W", "."],
        vec![".", ".", "W", ".", ".", ".", ".", ".", ".", ".", "W", "."],
    ];
    let mut mut_fields = fields;

    let mut cnt = 0;
    for y in 0..mut_fields.len() {
        let row = &mut_fields[y];
        for x in 0..row.len() {
            if mut_fields[y][x] == "W" {
                println!("x:{},y:{}", x, y);

                dfs(&mut mut_fields, &x, &y);
                cnt += 1;
            }
        }
    }

    // println!("mut_fields: {:?}", &mut_fields);
    println!("cnt: {}", &cnt);
}

fn dfs(fields: &mut Vec<Vec<&str>>, x: &usize, y: &usize) {
    // let cell = fields[*y][*x];

    let dry = stringify!('.');
    fields[*y][*x] = dry;

    for dx in 0..=2 {
        for dy in 0..=2 {
            let opt_nx = (*x).checked_add(dx).unwrap().checked_sub(1);
            let opt_ny = (*y).checked_add(dy).unwrap().checked_sub(1);

            let ny = match opt_ny {
                None => {
                    continue;
                }
                Some(ny) => {
                    if ny >= fields.len() {
                        continue;
                    }
                    ny
                }
            };

            let nx = match opt_nx {
                None => {
                    continue;
                }
                Some(nx) => {
                    if nx >= fields[ny].len() {
                        continue;
                    }
                    nx
                }
            };

            if fields[ny][nx] == "W" {
                dfs(fields, &nx, &ny);
            }
        }
    }

    return;
}
