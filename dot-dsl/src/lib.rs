pub mod graph {
    use std::collections::HashMap;
    type Attrs = HashMap<String, String>;
    type AttrsArgs<'a, 'b> = &'a[(&'b str, &'b str)];


    fn to_attrs(attrs: AttrsArgs) -> Attrs {
        attrs.iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    pub mod graph_items {
        pub mod node {
            use crate::graph::{to_attrs, Attrs, AttrsArgs};

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub label: String,
                pub attrs: Attrs,
            }

            // impl PartialEq for Node {
            //     fn eq(&self, other: &Node) -> bool {
            //         self.label == other.label
            //     }
            // }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: label.to_string(),
                        attrs: Attrs::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: AttrsArgs) -> Self {
                    self.attrs = to_attrs(attrs);
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_ref())
                }
            }
        }

        pub mod edge {
            use crate::graph::{to_attrs, Attrs, AttrsArgs};

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                nodes: [&'static str; 2],
                attrs: Attrs,
            }

            impl Edge {
                pub fn new(node_1: &'static str, node_2: &'static str) -> Edge {
                    Edge {
                        nodes: [node_1, node_2],
                        attrs: Attrs::new(),
                    }
                }

                // impl PartialEq for Edge {
                //     fn eq(&self, other: &Edge) -> bool {
                //         self.nodes == other.nodes
                //     }
                // }

                pub fn with_attrs(mut self, attrs: AttrsArgs) -> Self {
                    self.attrs = to_attrs(attrs);
                    self
                }
            }
        }
    }

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    // use {to_attrs, Attrs, AttrsArgs};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Attrs::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: AttrsArgs) -> Self {
            self.attrs = to_attrs(attrs);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            for node in self.nodes.iter() {
                if node.label == name {
                    return Some(node.clone())
                }
            }
            None
        }
    }
}
