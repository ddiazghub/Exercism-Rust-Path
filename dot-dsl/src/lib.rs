

pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attributes: HashMap<&'a str, &'a str>
            }


            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self {
                        name, 
                        attributes: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attributes: &[(&'a str, &'a str)]) -> Self {
                    self.attributes.extend(
                        attributes.into_iter()
                            .map(|&attribute| attribute)
                    );
                    
                    self
                }

                pub fn attr(&'a self, name: &str) -> Option<&'a str> {
                    self.attributes.get(name).copied()
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge<'a> {
                pub nodes: (&'a str, &'a str),
                pub attributes: HashMap<&'a str, &'a str>
            }


            impl<'a> Edge<'a> {
                pub fn new(node1: &'a str, node2: &'a str) -> Self {
                    Self {
                        nodes: (node1, node2), 
                        attributes: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attributes: &[(&'a str, &'a str)]) -> Self {
                    self.attributes.extend(
                        attributes.into_iter()
                            .map(|&attribute| attribute)
                    );
                    
                    self
                }

                pub fn attr(&'a self, name: &str) -> Option<&'a str> {
                    self.attributes.get(name).copied()
                }
            }
        }
    }
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attributes: &[(&'a str, &'a str)]) -> Self {
            self.attrs.extend(
                attributes.into_iter()
                    .map(|&(attr, value)| (attr.to_string(), value.to_string()))
            );

            self
        }

        pub fn node(&'a self, name: &str) -> Option<&'a Node> {
            (&self.nodes)
                .into_iter()
                .find(|&node| node.name == name)
        }
    }
}
