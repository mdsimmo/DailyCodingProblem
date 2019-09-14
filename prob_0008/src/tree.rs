#[derive(PartialEq, Debug)]
pub struct Tree<T: PartialEq> {
    subs: Link<T>,
}

type Link<T> = Vec<Box<Node<T>>>;

#[derive(PartialEq, Debug)]
pub struct Node<T: PartialEq> {
    val: T,
    subs: Link<T>,
}

impl <T: PartialEq> Tree<T> {
    pub fn count_unival(&self) -> u32 {
        self.subs.iter()
            .map(|n| {
                let res = n.as_unival().0;
                    //.map_or(0, |(cnt, _)| cnt);
                println!("{}", res);
                res
            })
            .sum()
    }
}

impl <T: PartialEq> Node<T> {
    fn as_unival(&self) -> (u32, Option<&T>) {
        let res = self.subs.iter()
            .map(|n| n.as_unival())
            .fold((0, Some(&self.val)),
                |acc, new| {
                    (acc.0 + new.0, acc.1.and_then(|v1| new.1.and_then(|v2|
                        if v1.eq(v2) {
                            Some(v1)
                        } else {
                            None
                        }))
                    )
                }
            );

        // add 1 if everything is the same
        if let (cnt, Some(v)) = res {
            (cnt+1, Some(v))
        } else {
            res
        }
    }
}

#[macro_export]
macro_rules! tree {
    ($($x:expr => $v:tt,)*) => {
        Tree {
            subs: node!([$($x => $v),*])
        }
    };
    ($($x:expr => $v:tt),*) => {
        tree!($($x => $v,)*)
    };

}

#[macro_export]
macro_rules! node {
    ({$($x:expr => $v:tt),*} $val:expr => $sub:tt, $($rest:tt)*) => {
        node!({$($x => $v,)* $val => $sub} $($rest)*)
    };

    ({$($x:expr => $v:tt),*} $val:expr, $($rest:tt)*) => {
        node!({$($x => $v,)* $val => []} $($rest)*)
    };

    (,$($x:tt)*) => {
        node!($($x)*)
    };

    ({$($x:expr => $v:tt),*}) => {
        vec![
            $(Box::new(Node{
                val: $x,
                subs: node![$v],
            })),*
        ]
    };

    ([$($x:tt)*]) => {
        node!({} $($x)*)
    };

    // optional trailing comma
    ($stack:tt $val:expr => $sub:tt) => {
        node!($stack $val => $sub,)
    };
    // optional trailing comma
    ($stack:tt $val:expr) => {
        node!($stack $val,)
    };

}

#[cfg(test)]
mod test_unival {
    use tree::*;

    #[test]
    fn test_node_count() {
        let node: Vec<Box<Node<u8>>> = node!([
            1 => [
                1,
                1
            ],
        ]);

        let result = node.iter()
            .map(|n| n.as_unival().0)
            .fold(0, |c1, c2| c1+c2);

        assert_eq!(3, result)
    }

    #[test]
    fn test_tree_count() {
        let node:Tree<u8> = tree!(
            0 => [
                1,
                0 => [
                    1 => [
                        1,
                        1
                    ],
                    0
                ]
            ]
        );

        assert_eq!(5, node.count_unival())
    }
}

#[cfg(test)]
mod test_tree_creation {
    use tree::*;

    #[test]
    fn create_no_sub() {
        let node = node!([
            1,
        ]);

        assert_eq!(node, vec![Box::new(Node{
            val: 1,
            subs: vec![],
        })]);

    }

    #[test]
    fn test_make_one_node() {
        let node = node!([1=>[],]);

        assert_eq!(node, vec![Box::new(Node {
            val: 1,
            subs: vec![],
        })]);
    }

    #[test]
    fn test_make_empty_node() {
        let node: Vec<Box<Node<i32>>> = node!([]);

        assert_eq!(node, vec![]);
    }

    #[test]
    fn test_make_sub_nodes() {
        let node: Vec<Box<Node<i32>>> = node!([
            1 => [
                2,
                3 => [],
            ],
            4,
        ]);

        assert_eq!(node, vec![
            Box::new(Node {
                val: 1,
                subs: vec![
                    Box::new(Node {
                        val: 2,
                        subs: vec![],
                    }),
                    Box::new(Node {
                        val: 3,
                        subs: vec![],
                    })
                ]
            }),
            Box::new(Node {
                val: 4,
                subs: vec![],
            }),
        ]);
    }

    #[test]
    fn test_optional_trailing_commas() {
        let node1: Vec<Box<Node<i32>>> = node!([
            1 => [
                2 => [],
                3
            ],
            4 => []
        ]);

        let node2: Vec<Box<Node<i32>>> = node!([
            1 => [
                2 => [],
                3,
            ],
            4,
        ]);

        assert_eq!(node1, node2);
    }

    #[test]
    fn test_easy_tree() {
        let tree = tree![
            2 => [],
        ];

        assert_eq!(tree, Tree {
            subs: vec![
                Box::new(Node {
                    val: 2,
                    subs: vec![],
                })
            ],
        });
    }

    #[test]
    fn test_tree_with_subs() {
        let tree = tree![
            1 => [
                2 => [],
            ],
        ];

        assert_eq!(tree, Tree {
            subs: vec![
                Box::new(Node {
                    val: 1,
                    subs: vec![Box::new(Node {
                        val: 2,
                        subs: vec![],
                    })
                    ],
                })
            ],
        });
    }

    #[test]
    fn create_tree() {
        let tree = tree![
            1 => [
                2,
                3 => [
                    32,
                    33,
                ],
                4,
            ],
        ];

        assert_eq!(tree, Tree {
            subs: vec![
                Box::new(Node {
                    val: 1,
                    subs: vec![
                        Box::new(Node {
                            val: 2,
                            subs: vec![],
                        }),
                        Box::new(Node {
                            val: 3,
                            subs: vec![
                                Box::new(Node {
                                    val: 32,
                                    subs: vec![],
                                }),
                                Box::new(Node {
                                    val: 33,
                                    subs: vec![],
                                }),
                            ],
                        }),
                        Box::new(Node {
                            val: 4,
                            subs: vec![],
                        }),
                    ]
                }),
            ],
        });
    }

}