

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

            #[derive(Debug, Clone)]
            pub struct Node {
                label: String,
                attrs: Attrs,
            }

            impl Node {
                pub fn new(label: &str) -> Node {
                    Node {
                        label: label.to_string(),
                        attrs: Attrs::new(),
                    }
                }

                pub fn with_attrs(&mut self, attrs: AttrsArgs) -> &mut Self {
                    self.attrs = to_attrs(attrs);
                    self
                }
            }
        }

        pub mod edge {
            use crate::graph::{to_attrs, Attrs, AttrsArgs};

            #[derive(Debug, Clone)]
            pub struct Edge {
                nodes: (String, String),
                attrs: Attrs,
            }

            impl Edge {
                pub fn new(node_1: &str, node_2: &str) -> Edge {
                    Edge {
                        nodes: (node_1.to_string(), node_2.to_string()),
                        attrs: Attrs::new(),
                    }
                }

                pub fn with_attrs(&mut self, attrs: AttrsArgs) -> &mut Self {
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

        pub fn with_nodes(&mut self, nodes: &[Node]) -> &mut Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_attrs(&mut self, attrs: AttrsArgs) -> &mut Self {
            self.attrs = to_attrs(attrs);
            self
        }

        pub fn with_edges(&mut self, edges: &[Edge]) -> &mut Self {
            self.edges.extend(edges.iter().cloned());
            self
        }
    }
}
