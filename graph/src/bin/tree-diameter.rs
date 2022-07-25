use std::collections::VecDeque;

fn main() {}

#[allow(dead_code)]
const INF: i32 = 1 << 29;

#[allow(dead_code)]
struct Graph {
    edges: Vec<Vec<usize>>,
    n: usize,
}

impl Graph {
    #[allow(dead_code)]
    fn get_bfs_distance(self, start: usize) -> Vec<i32> {
        let mut dist_list: Vec<i32> = Vec::new();
        for _ in 0..=self.n {
            dist_list.push(INF);
        }

        let mut que = VecDeque::new();

        que.push_front(start);

        dist_list[start] = 0;

        while que.len() > 0 {
            let poped = que.pop_front();
            let poped_number = match poped {
                None => {
                    continue;
                }
                Some(poped) => poped,
            };
            println!(
                "poped:{}, poped_number:{}",
                poped.unwrap_or_default(),
                poped_number
            );

            let to = &self.edges.get(poped_number - 1);

            let to = match to {
                None => {
                    continue;
                }
                Some(to) => *to,
            };

            println!("to:{:?}", to);

            for i in 0..to.len() {
                let to_number = to[i];
                println!("to_number:{}", to_number);

                if dist_list[to_number] == INF {
                    dist_list[to_number] = dist_list[poped_number] + 1;
                    que.push_front(to[i]);
                }
            }
        }

        return dist_list;
    }
}

mod tests {

    #[test]

    fn test_1() {
        struct Case {
            graph: crate::Graph,
            expected: usize,
        }

        let cases = [
            Case {
                graph: crate::Graph {
                    edges: vec![vec![1, 2], vec![2, 3]],
                    n: 3,
                },
                expected: 3,
            },
            Case {
                graph: crate::Graph {
                    // TODO: pointごとにどのエッジがあるかまとめる必要がある
                    edges: vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![3, 5]],
                    n: 5,
                },
                expected: 4,
            },
            /*
            Case {
                graph: crate::Graph {
                    edges: vec![
                        vec![1, 2],
                        vec![1, 3],
                        vec![2, 4],
                        vec![4, 5],
                        vec![4, 6],
                        vec![3, 7],
                        vec![7, 8],
                        vec![8, 9],
                        vec![8, 10],
                    ],
                    n: 10,
                },
                expected: 8,
            },
            */
            /*
            Case {
                graph: crate::Graph {
                    edges: vec![
                        vec![1, 2],
                        vec![1, 3],
                        vec![2, 4],
                        vec![2, 5],
                        vec![3, 6],
                        vec![3, 7],
                        vec![4, 8],
                        vec![4, 9],
                        vec![5, 10],
                        vec![5, 11],
                        vec![6, 12],
                        vec![6, 13],
                        vec![7, 14],
                        vec![7, 15],
                        vec![8, 16],
                        vec![8, 17],
                        vec![9, 18],
                        vec![9, 19],
                        vec![10, 20],
                        vec![10, 21],
                        vec![11, 22],
                        vec![11, 23],
                        vec![12, 24],
                        vec![12, 25],
                        vec![13, 26],
                        vec![13, 27],
                        vec![14, 28],
                        vec![14, 29],
                        vec![15, 30],
                        vec![15, 31],
                    ],
                    n: 31,
                },
                expected: 9,
            },
             */
        ];

        for ele in cases {
            println!("===========");
            let dist = ele.graph.get_bfs_distance(1);
            println!("{:?}, expected: {}", &dist, ele.expected);
        }
    }
}
