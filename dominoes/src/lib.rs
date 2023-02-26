use std::collections::HashMap;

trait Flip {
    fn flip(&self) -> Self;
}

impl <T: Copy> Flip for (T, T) {
    fn flip(&self) -> Self {
        (self.1, self.0)
    }
}

struct DominoChainTree {
    children: Vec<DominoChainTree>,
    root: (u8, u8)
}

impl DominoChainTree {
    pub fn new(root: (u8, u8), children: HashMap<(u8, u8), u8>) -> Self {
        let mut chil = Vec::new();

        for child in children.keys() {
            let mut childs_children = children.clone();
            *childs_children.get_mut(child).unwrap() -= 1;

            if childs_children[child] == 0 {
                childs_children.remove(child);
            }

            if child.1 == root.1 {
                chil.push(Self::new(child.flip(), childs_children.clone()));
            }

            if child.0 == root.1 {
                chil.push(Self::new(*child, childs_children));
            }
        }

        Self {
            root,
            children: chil
        }
    }

    pub fn chains(&self) -> Vec<Vec<(u8, u8)>> {
        let mut domino_chains = Vec::new();

        for child in self.children.iter() {
            domino_chains.extend(child.chains());
        }

        if domino_chains.len() == 0 {
            domino_chains.push(Vec::new());
        }

        for chain in domino_chains.iter_mut() {
            chain.push(self.root);
        }

        domino_chains
    }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {
        return Some(Vec::new());
    }

    let mut stones: HashMap<(u8, u8), u8> = input
        .iter()
        .fold(HashMap::new(), |mut map, stone| {
            match map.get_mut(stone) {
                Some(v) => *v += 1,
                None => drop(map.insert(*stone, 1))
            };

            map
        });

    for stone in input.iter() {
        let mut other_stones = stones.clone();
        *other_stones.get_mut(stone).unwrap() -= 1;

        if other_stones[stone] == 0 {
            other_stones.remove(stone);
        }

        let trees = [
            DominoChainTree::new(*stone, other_stones.clone()),
            DominoChainTree::new(stone.flip(), other_stones.clone())
        ];

        for tree in trees.into_iter() {
            for chain in tree.chains() {
                if chain.len() == input.len() && chain[0].1 == chain[chain.len() - 1].0 {
                    return Some(chain.into_iter().rev().collect());
                }
            }
        }
    }

    None
}
